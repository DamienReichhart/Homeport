//! Moving images and containers' files between a Host and a `.tar` file on
//! this machine — `docker save` and `docker load`, `docker export` and
//! `docker import` — streamed both ways so memory stays flat for archives of
//! several gigabytes. The archive travels over the Host's existing
//! connection, so this works for remote Hosts too.

use std::{
    io,
    path::{Path, PathBuf},
};

use bollard::{
    query_parameters::{CreateImageOptionsBuilder, ImportImageOptionsBuilder},
    Docker,
};
use bytes::Bytes;
use futures_util::StreamExt;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use homeport_core::error::{engine, local_file, AppError, AppResult, ErrorCode};

use crate::messages as msg;

use crate::{containers::commit, images::split_reference};

/// Progress is reported at most once per this many bytes.
pub(crate) const REPORT_EVERY: u64 = 1024 * 1024;
const CHUNK: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum TransferEvent {
    /// Bytes written to the file (save) or sent to the Engine (load) so far.
    Progress {
        bytes: u64,
        total: Option<u64>,
    },
    /// A line from the Engine, such as `Loaded image: nginx:1.27`.
    Line {
        text: String,
    },
    /// Finished; `images` are the names saved, loaded or imported (none for an export).
    Done {
        images: Vec<String>,
    },
    Error {
        error: AppError,
    },
}

/// Saves `names` into one tar at `destination`, which `docker load` accepts.
/// A failed or cancelled save leaves no file behind.
pub async fn save(
    docker: Docker,
    names: Vec<String>,
    destination: PathBuf,
    sink: impl Fn(TransferEvent) -> bool,
) {
    let result = save_to(&docker, &names, &destination, &sink).await;
    finish(result.map(|()| names), "image save", &sink);
}

async fn save_to(
    docker: &Docker,
    names: &[String],
    destination: &Path,
    sink: &impl Fn(TransferEvent) -> bool,
) -> AppResult<()> {
    if names.is_empty() {
        return Err(AppError::validation(msg::TRANSFER_NOTHING_TO_SAVE));
    }
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    write_file(docker.export_images(&refs), destination, sink).await
}

/// Writes what the Engine streams into `destination`, reporting the bytes.
/// The file is removed unless the stream completes, also when the task is
/// cancelled, so nothing half-written is left behind.
pub(crate) async fn write_file(
    mut stream: impl futures_util::Stream<Item = Result<Bytes, bollard::errors::Error>> + Unpin,
    destination: &Path,
    sink: &impl Fn(TransferEvent) -> bool,
) -> AppResult<()> {
    let file = tokio::fs::File::create(destination)
        .await
        .map_err(|e| local_file(destination, e))?;
    let mut partial = Partial(Some(destination.to_owned()));
    let mut file = tokio::io::BufWriter::new(file);
    let mut written = 0u64;
    let mut reported = 0u64;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)
            .await
            .map_err(|e| local_file(destination, e))?;
        written += chunk.len() as u64;
        if written - reported >= REPORT_EVERY {
            reported = written;
            if !sink(TransferEvent::Progress {
                bytes: written,
                total: None,
            }) {
                return Ok(());
            }
        }
    }
    file.flush().await.map_err(|e| local_file(destination, e))?;
    file.into_inner()
        .sync_all()
        .await
        .map_err(|e| local_file(destination, e))?;
    if written == 0 {
        return Err(AppError::new(
            ErrorCode::EngineError,
            msg::TRANSFER_EMPTY_ARCHIVE,
        ));
    }
    sink(TransferEvent::Progress {
        bytes: written,
        total: Some(written),
    });
    partial.keep();
    Ok(())
}

/// Saves container `id`'s whole filesystem into a flat tar at `destination`,
/// as `docker export` does; `docker import` or [`import`] turn it back into
/// an image. A failed or cancelled export leaves no file behind.
pub async fn export(
    docker: Docker,
    id: String,
    destination: PathBuf,
    sink: impl Fn(TransferEvent) -> bool,
) {
    let result = write_file(docker.export_container(&id), &destination, &sink).await;
    finish(result.map(|()| vec![]), "container export", &sink);
}

/// The file at `source` as a request body read as the Engine takes it, its
/// size, and a channel reporting each chunk's size as it's sent.
pub(crate) async fn file_body(
    source: &Path,
) -> AppResult<(
    impl futures_util::Stream<Item = Result<Bytes, io::Error>> + Send + 'static,
    Option<u64>,
    tokio::sync::mpsc::Receiver<u64>,
)> {
    let file = tokio::fs::File::open(source)
        .await
        .map_err(|e| local_file(source, e))?;
    let total = file.metadata().await.ok().map(|m| m.len());
    let (tx, rx) = tokio::sync::mpsc::channel::<u64>(16);
    let body = futures_util::stream::unfold((file, tx), |(mut file, tx)| async move {
        let mut buf = vec![0u8; CHUNK];
        match file.read(&mut buf).await {
            Ok(0) => None,
            Ok(n) => {
                buf.truncate(n);
                let _ = tx.try_send(n as u64);
                Some((Ok::<_, io::Error>(Bytes::from(buf)), (file, tx)))
            }
            Err(e) => Some((Err(e), (file, tx))),
        }
    });
    Ok((body, total, rx))
}

/// What [`import`] makes: the image's name and Dockerfile changes.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportSpec {
    /// `repo[:tag]` for the new image.
    pub reference: String,
    /// Dockerfile instructions, one per entry, like `CMD ["/bin/sh"]`.
    pub changes: Vec<String>,
}

/// Creates an image from the root filesystem tar at `source` (a container
/// export, `debootstrap`, a vendor appliance), as `docker import` does.
pub async fn import(
    docker: Docker,
    source: PathBuf,
    spec: ImportSpec,
    sink: impl Fn(TransferEvent) -> bool,
) {
    let result = import_from(&docker, &source, &spec, &sink).await;
    finish(result, "image import", &sink);
}

async fn import_from(
    docker: &Docker,
    source: &Path,
    spec: &ImportSpec,
    sink: &impl Fn(TransferEvent) -> bool,
) -> AppResult<Vec<String>> {
    let (repo, tag) = split_reference(&spec.reference);
    if repo.is_empty() || repo.contains('@') {
        return Err(AppError::validation(msg::TRANSFER_IMPORT_NAME));
    }
    let tag = tag.unwrap_or_else(|| "latest".into());
    let changes = commit::changes(&spec.changes)?;
    let (body, total, mut rx) = file_body(source).await?;
    let options = CreateImageOptionsBuilder::default()
        .from_src("-")
        .repo(&repo)
        .tag(&tag)
        .changes(changes)
        .build();
    let mut stream = docker.create_image(Some(options), Some(bollard::body_try_stream(body)), None);
    let (mut sent, mut reported) = (0u64, 0u64);
    loop {
        tokio::select! {
            Some(n) = rx.recv() => {
                sent += n;
                if sent - reported >= REPORT_EVERY {
                    reported = sent;
                    if !sink(TransferEvent::Progress { bytes: sent, total }) {
                        return Ok(vec![]);
                    }
                }
            }
            item = stream.next() => {
                let Some(item) = item else { break };
                let info = item.map_err(|e| not_a_filesystem(AppError::from(e), source))?;
                if let Some(message) = info.error_detail.and_then(|e| e.message) {
                    return Err(not_a_filesystem(
                        engine::from_message(None, &message).with_detail(&message),
                        source,
                    ));
                }
            }
        }
    }
    if let Some(total) = total {
        sink(TransferEvent::Progress {
            bytes: total,
            total: Some(total),
        });
    }
    Ok(vec![format!("{repo}:{tag}")])
}

/// The Engine's words for a file it can't unpack become a clear message.
fn not_a_filesystem(error: AppError, source: &Path) -> AppError {
    let detail = error.detail.as_deref().unwrap_or_default().to_lowercase();
    if detail.contains("archive/tar")
        || detail.contains("unexpected eof")
        || detail.contains("invalid tar header")
    {
        AppError {
            code: ErrorCode::BadRequest,
            ..error
        }
        .with_message(msg::TRANSFER_NOT_A_FILESYSTEM.with("path", source))
        .with_hint(msg::TRANSFER_NOT_A_FILESYSTEM_HINT)
    } else {
        error
    }
}

/// Loads the images in the tar at `source` (plain or compressed, as
/// `docker load` takes) into the Host; reports their names.
pub async fn load(docker: Docker, source: PathBuf, sink: impl Fn(TransferEvent) -> bool) {
    let result = load_from(&docker, &source, &sink).await;
    finish(result, "image load", &sink);
}

async fn load_from(
    docker: &Docker,
    source: &Path,
    sink: &(impl Fn(TransferEvent) -> bool + ?Sized),
) -> AppResult<Vec<String>> {
    // The file is read as the Engine takes it; the sizes sent come back on `rx`.
    let (body, total, mut rx) = file_body(source).await?;
    let options = ImportImageOptionsBuilder::default().quiet(false).build();
    let mut stream = docker.import_image_stream(options, body, None);
    let mut loaded = Vec::new();
    let (mut sent, mut reported) = (0u64, 0u64);
    loop {
        tokio::select! {
            Some(n) = rx.recv() => {
                sent += n;
                if sent - reported >= REPORT_EVERY {
                    reported = sent;
                    if !sink(TransferEvent::Progress { bytes: sent, total }) {
                        return Ok(loaded);
                    }
                }
            }
            item = stream.next() => {
                let Some(item) = item else { break };
                let info = item.map_err(|e| invalid_archive(AppError::from(e), source))?;
                if let Some(message) = info.error_detail.and_then(|e| e.message) {
                    return Err(invalid_archive(
                        engine::from_message(None, &message).with_detail(&message),
                        source,
                    ));
                }
                for line in info.stream.iter().flat_map(|s| s.lines()).map(str::trim) {
                    if line.is_empty() {
                        continue;
                    }
                    if let Some(name) = line
                        .strip_prefix("Loaded image: ")
                        .or_else(|| line.strip_prefix("Loaded image ID: "))
                    {
                        loaded.push(name.trim().to_owned());
                    }
                    if !sink(TransferEvent::Line { text: line.to_owned() }) {
                        return Ok(loaded);
                    }
                }
            }
        }
    }
    if let Some(total) = total {
        sink(TransferEvent::Progress {
            bytes: total,
            total: Some(total),
        });
    }
    Ok(loaded)
}

/// The Engine's words for an archive it can't read become a clear message.
fn invalid_archive(error: AppError, source: &Path) -> AppError {
    let detail = error.detail.as_deref().unwrap_or_default().to_lowercase();
    let unreadable = [
        "archive/tar",
        "unexpected eof",
        "invalid tar header",
        "manifest.json",
        "does not exist",
        "unrecognized",
    ]
    .iter()
    .any(|n| detail.contains(n));
    if unreadable && !error.is(ErrorCode::DiskFull) {
        AppError {
            code: ErrorCode::BadRequest,
            ..error
        }
        .with_message(msg::TRANSFER_NOT_AN_IMAGE_ARCHIVE.with("path", source))
        .with_hint(msg::TRANSFER_NOT_AN_IMAGE_ARCHIVE_HINT)
    } else {
        error
    }
}

pub(crate) fn finish(
    result: AppResult<Vec<String>>,
    context: &str,
    sink: &impl Fn(TransferEvent) -> bool,
) {
    match result {
        Ok(images) => {
            sink(TransferEvent::Done { images });
        }
        Err(error) => {
            sink(TransferEvent::Error {
                error: error.logged(context),
            });
        }
    }
}

/// A file being written, removed when dropped unless kept.
struct Partial(Option<PathBuf>);

impl Partial {
    fn keep(&mut self) {
        self.0 = None;
    }
}

impl Drop for Partial {
    fn drop(&mut self) {
        if let Some(path) = self.0.take() {
            if let Err(e) = std::fs::remove_file(&path) {
                log::warn!("couldn't remove the partial file {}: {e}", path.display());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::{images, testing};

    fn recorder() -> (
        impl Fn(TransferEvent) -> bool,
        std::sync::Arc<Mutex<Vec<TransferEvent>>>,
    ) {
        let events = std::sync::Arc::new(Mutex::new(Vec::new()));
        let sink = {
            let events = events.clone();
            move |e| {
                events.lock().unwrap().push(e);
                true
            }
        };
        (sink, events)
    }

    #[test]
    fn partial_files_are_removed_unless_kept() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.tar");
        std::fs::write(&path, "x").unwrap();
        drop(Partial(Some(path.clone())));
        assert!(!path.exists());
        std::fs::write(&path, "x").unwrap();
        let mut kept = Partial(Some(path.clone()));
        kept.keep();
        drop(kept);
        assert!(path.exists());
    }

    #[test]
    fn unreadable_archives_are_explained() {
        let e = invalid_archive(
            AppError::new(ErrorCode::EngineError, msg::TRANSFER_EMPTY_ARCHIVE)
                .with_detail("archive/tar: invalid tar header"),
            Path::new("/tmp/x.tar"),
        );
        assert_eq!(e.code, ErrorCode::BadRequest);
        assert!(e.message.contains("/tmp/x.tar"));
        let other = AppError::new(ErrorCode::DiskFull, msg::TRANSFER_EMPTY_ARCHIVE)
            .with_detail("no space left on device");
        assert_eq!(invalid_archive(other.clone(), Path::new("x")), other);
    }

    #[tokio::test]
    async fn saving_to_a_folder_that_does_not_exist_fails_cleanly() {
        let (sink, events) = recorder();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing").join("a.tar");
        save(testing::offline(), vec!["x".into()], path.clone(), sink).await;
        let events = events.lock().unwrap();
        assert!(
            matches!(events.last(), Some(TransferEvent::Error { .. })),
            "{events:?}"
        );
        assert!(!path.exists());
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn images_saved_to_a_file_load_back_with_their_names() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let name = format!("{}:v1", testing::unique("save"));
        images::tag(&docker, testing::IMAGE, &name).await.unwrap();
        let _cleanup = testing::ImageName(docker.clone(), name.clone());
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("images.tar");

        let (sink, events) = recorder();
        save(docker.clone(), vec![name.clone()], path.clone(), sink).await;
        assert_eq!(
            events.lock().unwrap().last(),
            Some(&TransferEvent::Done {
                images: vec![name.clone()]
            })
        );
        assert!(std::fs::metadata(&path).unwrap().len() > 0);

        images::remove(&docker, &name, false).await.unwrap();
        let (sink, events) = recorder();
        load(docker.clone(), path, sink).await;
        let last = events.lock().unwrap().last().cloned();
        assert_eq!(
            last,
            Some(TransferEvent::Done {
                images: vec![name.clone()]
            })
        );
        assert!(docker.inspect_image(&name).await.is_ok());
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn loading_a_file_that_is_not_an_archive_says_so() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("notes.tar");
        std::fs::write(&path, "these are not image layers").unwrap();
        let (sink, events) = recorder();
        load(testing::docker(), path, sink).await;
        let events = events.lock().unwrap();
        match events.last() {
            Some(TransferEvent::Error { error }) => {
                assert_eq!(error.code, ErrorCode::BadRequest, "{error:?}");
            }
            other => panic!("{other:?}"),
        }
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn an_exported_container_imports_back_as_a_runnable_image() {
        let docker = testing::docker();
        testing::ensure_image(&docker).await;
        let guard = testing::container(
            &docker,
            &testing::unique("export"),
            &["sh", "-c", "echo exported > /exported.txt && sleep 60"],
        )
        .await;
        crate::containers::act(
            &docker,
            &guard.id,
            crate::containers::ContainerAction::Start,
        )
        .await
        .unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rootfs.tar");

        let (sink, events) = recorder();
        export(docker.clone(), guard.id.clone(), path.clone(), sink).await;
        assert_eq!(
            events.lock().unwrap().last(),
            Some(&TransferEvent::Done { images: vec![] })
        );
        let mut archive = tar::Archive::new(std::fs::File::open(&path).unwrap());
        assert!(archive.entries().unwrap().any(|e| e
            .unwrap()
            .path()
            .unwrap()
            .ends_with("exported.txt")));

        let name = format!("{}:1", testing::unique("imported"));
        let _cleanup = testing::ImageName(docker.clone(), name.clone());
        let (sink, events) = recorder();
        import(
            docker.clone(),
            path,
            ImportSpec {
                reference: name.clone(),
                changes: vec![r#"CMD ["cat", "/exported.txt"]"#.into()],
            },
            sink,
        )
        .await;
        assert_eq!(
            events.lock().unwrap().last(),
            Some(&TransferEvent::Done {
                images: vec![name.clone()]
            })
        );
        let check = crate::containers::run::run(
            &docker,
            crate::containers::run::RunSpec {
                image: name.clone(),
                name: Some(testing::unique("import-check")),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
        let exit = loop {
            let state = docker
                .inspect_container(&check, None)
                .await
                .unwrap()
                .state
                .unwrap();
            if state.running != Some(true) {
                break state.exit_code;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };
        crate::containers::remove(&docker, &check, true, true)
            .await
            .unwrap();
        assert_eq!(exit, Some(0));
    }

    #[tokio::test]
    async fn exporting_to_a_folder_that_does_not_exist_fails_cleanly() {
        let (sink, events) = recorder();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing").join("rootfs.tar");
        export(testing::offline(), "nope".into(), path.clone(), sink).await;
        assert!(matches!(
            events.lock().unwrap().last(),
            Some(TransferEvent::Error { .. })
        ));
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn imports_need_a_name() {
        let (sink, events) = recorder();
        import(
            testing::offline(),
            "/nonexistent.tar".into(),
            ImportSpec {
                reference: " ".into(),
                changes: vec![],
            },
            sink,
        )
        .await;
        let last = events.lock().unwrap().last().cloned();
        match last {
            Some(TransferEvent::Error { error }) => assert_eq!(error.code, ErrorCode::Validation),
            other => panic!("{other:?}"),
        }
    }
}
