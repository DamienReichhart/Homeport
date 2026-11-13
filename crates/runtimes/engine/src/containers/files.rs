//! Browsing, downloading and uploading files inside a container.
//!
//! Listing runs `find … -exec stat` in the container (the archive API would
//! return whole directory trees); transfers use the archive API.

use std::path::Path;

use bollard::{
    body_full,
    exec::{StartExecOptions, StartExecResults},
    models::ExecConfig,
    query_parameters::{
        ContainerArchiveInfoOptionsBuilder, DownloadFromContainerOptionsBuilder,
        UploadToContainerOptionsBuilder,
    },
    Docker,
};
use futures_util::StreamExt;
use serde::Serialize;
use tokio::io::AsyncWriteExt;

use homeport_core::{
    error::{local_file, AppError, AppResult, ErrorCode},
    local_files::Replacement,
};

use crate::messages as msg;

/// Go's `os.ModeDir`, as reported by the archive stat endpoint.
const GO_MODE_DIR: u32 = 1 << 31;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum FileKind {
    Directory,
    File,
    Symlink,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub kind: FileKind,
    pub size: u64,
    /// Unix time in seconds.
    pub modified: i64,
    /// `ls`-style permissions, e.g. `-rw-r--r--`.
    pub mode: String,
}

/// Parses `stat -c '%F|%s|%Y|%A|%n'` output; directories first, then by name.
pub fn parse_listing(output: &str) -> Vec<FileEntry> {
    let mut entries: Vec<FileEntry> = output
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(5, '|');
            let kind = match parts.next()? {
                "directory" => FileKind::Directory,
                "regular file" | "regular empty file" => FileKind::File,
                "symbolic link" => FileKind::Symlink,
                _ => FileKind::Other,
            };
            let size = parts.next()?.parse().ok()?;
            let modified = parts.next()?.parse().ok()?;
            let mode = parts.next()?.to_owned();
            let path = parts.next()?.to_owned();
            let name = path.rsplit('/').next().unwrap_or(&path).to_owned();
            Some(FileEntry {
                name,
                path,
                kind,
                size,
                modified,
                mode,
            })
        })
        .collect();
    entries.sort_by(|a, b| {
        (a.kind != FileKind::Directory, &a.name).cmp(&(b.kind != FileKind::Directory, &b.name))
    });
    entries
}

fn absolute(path: &str) -> AppResult<&str> {
    let path = path.trim();
    if !path.starts_with('/') {
        return Err(AppError::validation(msg::FILES_PATH_NOT_ABSOLUTE));
    }
    Ok(path)
}

/// Lists the direct children of `path`.
pub async fn list(docker: &Docker, container_id: &str, path: &str) -> AppResult<Vec<FileEntry>> {
    let path = absolute(path)?;
    let cmd = [
        "find",
        path,
        "-mindepth",
        "1",
        "-maxdepth",
        "1",
        "-exec",
        "stat",
        "-c",
        "%F|%s|%Y|%A|%n",
        "{}",
        "+",
    ];
    let (code, stdout, stderr) = run(docker, container_id, &cmd).await?;
    match code {
        Some(0) => Ok(parse_listing(&stdout)),
        // find/stat missing (distroless images): exec itself fails with 126/127.
        Some(126 | 127) | None => Err(AppError::new(ErrorCode::NoShell, msg::FILES_NO_TOOLS)
            .with_hint(msg::FILES_NO_TOOLS_HINT)
            .with_detail(stderr.trim())),
        Some(_) => Err(listing_failure(path, stderr.trim())),
    }
}

/// Why `find` couldn't list `path`, from its stderr.
fn listing_failure(path: &str, stderr: &str) -> AppError {
    let error = if stderr.contains("No such file or directory") {
        AppError::new(
            ErrorCode::PathNotFound,
            msg::FILES_PATH_MISSING.with("path", path),
        )
    } else if stderr.contains("Permission denied") {
        AppError::new(
            ErrorCode::ContainerPermissionDenied,
            msg::FILES_NOT_ALLOWED.with("path", path),
        )
        .with_hint(msg::FILES_NOT_ALLOWED_HINT)
    } else {
        AppError::new(
            ErrorCode::EngineError,
            msg::FILES_NOT_LISTED.with("path", path),
        )
    };
    error.with_detail(stderr)
}

/// Runs a command without a TTY and collects its output.
async fn run(
    docker: &Docker,
    container_id: &str,
    cmd: &[&str],
) -> AppResult<(Option<i64>, String, String)> {
    let config = ExecConfig {
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
        ..Default::default()
    };
    let exec_id = docker.create_exec(container_id, config).await?.id;
    let StartExecResults::Attached { mut output, .. } = docker
        .start_exec(&exec_id, None::<StartExecOptions>)
        .await?
    else {
        return Err(AppError::new(
            ErrorCode::EngineError,
            msg::FILES_SESSION_NOT_OPENED,
        ));
    };
    let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
    while let Some(chunk) = output.next().await {
        match chunk? {
            bollard::container::LogOutput::StdErr { message } => stderr.extend_from_slice(&message),
            other => stdout.extend_from_slice(&other.into_bytes()),
        }
    }
    let code = docker.inspect_exec(&exec_id).await?.exit_code;
    Ok((
        code,
        String::from_utf8_lossy(&stdout).into_owned(),
        String::from_utf8_lossy(&stderr).into_owned(),
    ))
}

/// Saves `path` to `destination` on this machine: a file as itself, a
/// directory as a `.tar` archive. Returns whether a directory was archived.
pub async fn download(
    docker: &Docker,
    container_id: &str,
    path: &str,
    destination: &Path,
) -> AppResult<bool> {
    download_counted(docker, container_id, path, destination, |_, _| {}).await
}

/// [`download`], telling `progress` how many bytes of the Engine's archive
/// came, of how many if the Engine said (a file's size; a folder's is not
/// known). The archive goes to disk as it comes, so memory stays bounded,
/// and the file is written beside `destination` and put in its place only
/// once whole: a failure, or the future dropped part way, leaves no file
/// and whatever was at `destination` as it was.
pub async fn download_counted(
    docker: &Docker,
    container_id: &str,
    path: &str,
    destination: &Path,
    progress: impl Fn(u64, Option<u64>),
) -> AppResult<bool> {
    let path = absolute(path)?;
    let stat = docker
        .get_container_archive_info(
            container_id,
            Some(
                ContainerArchiveInfoOptionsBuilder::default()
                    .path(path)
                    .build(),
            ),
        )
        .await?;
    let is_dir = stat.file_mode & GO_MODE_DIR != 0;
    let total = (!is_dir).then(|| u64::try_from(stat.size).unwrap_or(0));
    progress(0, total);

    // The archive, as it comes: the folder's own file, or a file of its
    // own beside the destination for a file, read back once whole.
    let archive = Replacement::new(destination)?;
    let mut stream = docker.download_from_container(
        container_id,
        Some(
            DownloadFromContainerOptionsBuilder::default()
                .path(path)
                .build(),
        ),
    );
    let file = archive
        .file()
        .try_clone()
        .map_err(|e| local_file(destination, e))?;
    let mut out = tokio::io::BufWriter::new(tokio::fs::File::from_std(file));
    let mut received = 0_u64;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        out.write_all(&chunk)
            .await
            .map_err(|e| local_file(destination, e))?;
        received += chunk.len() as u64;
        // A file's archive is a little more than the file: its header.
        progress(total.map_or(received, |total| received.min(total)), total);
    }
    out.flush().await.map_err(|e| local_file(destination, e))?;
    drop(out);
    if received == 0 {
        return Err(AppError::new(
            ErrorCode::EngineError,
            msg::FILES_EMPTY_ARCHIVE,
        ));
    }
    if is_dir {
        mode_of(&archive, destination);
        archive.commit()?;
        return Ok(true);
    }
    let destination = destination.to_owned();
    tokio::task::spawn_blocking(move || extract_first(&archive, &destination))
        .await
        .map_err(|e| AppError::from(std::io::Error::other(e)))??;
    Ok(false)
}

/// Writes the first entry of the tar in `archive` to `destination`, whole
/// or not at all. `archive` goes when it is dropped.
fn extract_first(archive: &Replacement, destination: &Path) -> AppResult<()> {
    use std::io::{Seek, SeekFrom};
    let mut file = archive
        .file()
        .try_clone()
        .map_err(|e| local_file(destination, e))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|e| local_file(destination, e))?;
    let mut tar = tar::Archive::new(file);
    let mut entry = tar
        .entries()?
        .next()
        .ok_or_else(|| AppError::new(ErrorCode::EngineError, msg::FILES_EMPTY_ARCHIVE))??;
    let saved = Replacement::new(destination)?;
    let mut out = saved.file();
    std::io::copy(&mut entry, &mut out).map_err(|e| local_file(destination, e))?;
    mode_of(&saved, destination);
    saved.commit()
}

/// The permissions a saved file gets: those of the file it replaces, or
/// what any new file gets (the umask): it was private while written.
/// A file whose permissions can't be set stays private.
fn mode_of(saved: &Replacement, destination: &Path) {
    let set = match std::fs::metadata(destination) {
        Ok(replaced) => saved.keep_mode_of(&replaced),
        Err(_) => saved.take_default_mode(),
    };
    if let Err(e) = set {
        log::debug!("a downloaded file keeps its private mode: {e}");
    }
}

/// Copies local files into the container directory `directory`.
pub async fn upload(
    docker: &Docker,
    container_id: &str,
    directory: &str,
    sources: &[std::path::PathBuf],
) -> AppResult<()> {
    let directory = absolute(directory)?;
    let mut builder = tar::Builder::new(Vec::new());
    for source in sources {
        let name = source
            .file_name()
            .ok_or_else(|| AppError::validation(msg::FILES_NOT_A_FILE.with("path", source)))?;
        if source.is_dir() {
            builder.append_dir_all(name, source)
        } else {
            builder.append_path_with_name(source, name)
        }
        .map_err(|e| local_file(source, e))?;
    }
    let archive = builder.into_inner()?;
    let options = UploadToContainerOptionsBuilder::default()
        .path(directory)
        .build();
    docker
        .upload_to_container(container_id, Some(options), body_full(archive.into()))
        .await
        .map_err(|e| match AppError::from(e) {
            e if e.is(ErrorCode::PathNotFound) => {
                e.with_message(msg::FILES_PATH_MISSING.with("path", directory))
            }
            e => e,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listing_failures_say_why() {
        let e = listing_failure("/data", "find: '/data': No such file or directory");
        assert_eq!(e.code, ErrorCode::PathNotFound);
        assert_eq!(e.message, "/data doesn't exist in the container.");
        let e = listing_failure("/root", "find: '/root': Permission denied");
        assert_eq!(e.code, ErrorCode::ContainerPermissionDenied);
    }
    use crate::{containers, testing};

    #[test]
    fn listings_put_directories_first() {
        let out = "regular file|12|1700000000|-rw-r--r--|/etc/hostname\n\
                   directory|4096|1700000001|drwxr-xr-x|/etc/apk\n\
                   symbolic link|12|1700000002|lrwxrwxrwx|/etc/mtab\n\
                   regular empty file|0|1700000003|-rw-r--r--|/etc/a|b\n\
                   garbage line\n";
        let entries = parse_listing(out);
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["apk", "a|b", "hostname", "mtab"]);
        assert_eq!(entries[0].kind, FileKind::Directory);
        assert_eq!(entries[1].path, "/etc/a|b");
        assert_eq!(entries[3].kind, FileKind::Symlink);
        assert_eq!(entries[2].size, 12);
    }

    #[test]
    fn relative_paths_are_rejected() {
        assert!(absolute("etc").is_err());
        assert_eq!(absolute(" /etc ").unwrap(), "/etc");
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn files_round_trip_through_a_container() {
        let docker = testing::docker();
        let c = testing::container(&docker, &testing::unique("files"), &["sleep", "60"]).await;
        containers::act(&docker, &c.id, containers::ContainerAction::Start)
            .await
            .unwrap();

        let dir = tempfile::tempdir().unwrap();
        let local = dir.path().join("note.txt");
        std::fs::write(&local, "hello from homeport").unwrap();
        upload(&docker, &c.id, "/tmp", &[local]).await.unwrap();

        let listed = list(&docker, &c.id, "/tmp").await.unwrap();
        let note = listed.iter().find(|e| e.name == "note.txt").unwrap();
        assert_eq!((note.kind.clone(), note.size), (FileKind::File, 19));

        let back = dir.path().join("back.txt");
        assert!(!download(&docker, &c.id, "/tmp/note.txt", &back)
            .await
            .unwrap());
        assert_eq!(
            std::fs::read_to_string(&back).unwrap(),
            "hello from homeport"
        );

        let tarball = dir.path().join("tmp.tar");
        assert!(download(&docker, &c.id, "/tmp", &tarball).await.unwrap());
        assert!(std::fs::metadata(&tarball).unwrap().len() > 0);
    }
}
