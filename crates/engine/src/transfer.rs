//! Moving images between a Host and a `.tar` file on this machine
//! (`docker save` and `docker load`), streamed both ways so memory stays
//! flat for images of several gigabytes. The archive travels over the Host's
//! existing connection, so this works for remote Hosts too.

use std::{
    io,
    path::{Path, PathBuf},
};

use bollard::{query_parameters::ImportImageOptionsBuilder, Docker};
use bytes::Bytes;
use futures_util::StreamExt;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use homeport_core::error::{engine, local_file, AppError, AppResult, ErrorCode};

/// Progress is reported at most once per this many bytes.
const REPORT_EVERY: u64 = 1024 * 1024;
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
    /// Finished; `images` are the names saved or loaded.
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
        return Err(AppError::validation("Choose at least one image to save."));
    }
    let file = tokio::fs::File::create(destination)
        .await
        .map_err(|e| local_file(destination, e))?;
    // Removes the file unless the save completes, also when the task is aborted.
    let mut partial = Partial(Some(destination.to_owned()));
    let mut file = tokio::io::BufWriter::new(file);
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    let mut stream = docker.export_images(&refs);
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
            "The Docker Engine sent an empty archive.",
        ));
    }
    sink(TransferEvent::Progress {
        bytes: written,
        total: Some(written),
    });
    partial.keep();
    Ok(())
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
    let file = tokio::fs::File::open(source)
        .await
        .map_err(|e| local_file(source, e))?;
    let total = file.metadata().await.ok().map(|m| m.len());
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u64>(16);
    // The file is read as the Engine takes it; the sizes sent come back on `rx`.
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
            message: format!("{} isn't an image archive the Docker Engine can load.", source.display()),
            hint: Some(homeport_core::error::Hint {
                text: "Choose a .tar made by docker save or by Save in Homeport, or a compressed one (.tar.gz).".into(),
                commands: vec![],
            }),
            ..error
        }
    } else {
        error
    }
}

fn finish(result: AppResult<Vec<String>>, context: &str, sink: &impl Fn(TransferEvent) -> bool) {
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
            AppError::new(ErrorCode::EngineError, "x")
                .with_detail("archive/tar: invalid tar header"),
            Path::new("/tmp/x.tar"),
        );
        assert_eq!(e.code, ErrorCode::BadRequest);
        assert!(e.message.contains("/tmp/x.tar"));
        let other =
            AppError::new(ErrorCode::DiskFull, "full").with_detail("no space left on device");
        assert_eq!(invalid_archive(other.clone(), Path::new("x")), other);
    }

    #[tokio::test]
    async fn saving_to_a_folder_that_does_not_exist_fails_cleanly() {
        let (sink, events) = recorder();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing").join("a.tar");
        save(testing::docker(), vec!["x".into()], path.clone(), sink).await;
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
        let events = events.lock().unwrap();
        assert_eq!(
            events.last(),
            Some(&TransferEvent::Done {
                images: vec![name.clone()]
            }),
            "{events:?}"
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
}
