//! Browsing, downloading and uploading files inside a container.
//!
//! Listing runs `find … -exec stat` in the container (the archive API would
//! return whole directory trees); transfers use the archive API.

use std::{io::Read, path::Path};

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

use crate::error::{local_file, AppError, AppResult, ErrorCode};

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
        return Err(AppError::validation("Paths must start with /."));
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
        Some(126 | 127) | None => Err(AppError::new(
            ErrorCode::NoShell,
            "This container has no find and stat tools, so its files can't be listed.",
        )
        .with_hint("Minimal images (distroless, scratch) leave them out. Copy files with docker cp from a terminal instead.")
        .with_detail(stderr.trim())),
        Some(_) => Err(listing_failure(path, stderr.trim())),
    }
}

/// Why `find` couldn't list `path`, from its stderr.
fn listing_failure(path: &str, stderr: &str) -> AppError {
    let error = if stderr.contains("No such file or directory") {
        AppError::new(
            ErrorCode::PathNotFound,
            format!("{path} doesn't exist in the container."),
        )
    } else if stderr.contains("Permission denied") {
        AppError::new(
            ErrorCode::ContainerPermissionDenied,
            format!("The container's user isn't allowed to read {path}."),
        )
        .with_hint("Files owned by root can only be listed when the container runs as root.")
    } else {
        AppError::new(
            ErrorCode::EngineError,
            format!("{path} can't be listed in the container."),
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
            "The Docker Engine didn't open the command's session.",
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

    let mut archive = Vec::new();
    let mut stream = docker.download_from_container(
        container_id,
        Some(
            DownloadFromContainerOptionsBuilder::default()
                .path(path)
                .build(),
        ),
    );
    while let Some(chunk) = stream.next().await {
        archive.extend_from_slice(&chunk?);
    }
    if is_dir {
        std::fs::write(destination, &archive).map_err(|e| local_file(destination, e))?;
        return Ok(true);
    }
    let mut tar = tar::Archive::new(archive.as_slice());
    let mut entry = tar.entries()?.next().ok_or_else(|| {
        AppError::new(
            ErrorCode::EngineError,
            "The Docker Engine sent an empty archive for this file.",
        )
    })??;
    let mut contents = Vec::new();
    entry.read_to_end(&mut contents)?;
    std::fs::write(destination, contents).map_err(|e| local_file(destination, e))?;
    Ok(false)
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
            .ok_or_else(|| AppError::validation(format!("{} is not a file.", source.display())))?;
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
            e if e.is(ErrorCode::PathNotFound) => AppError {
                message: format!("{directory} doesn't exist in the container."),
                ..e
            },
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
    use crate::services::docker::{containers, testing};

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
