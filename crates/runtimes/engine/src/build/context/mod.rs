//! Packs a folder on this machine into the tar the Engine builds from,
//! leaving out what `.dockerignore` excludes, and streams it in chunks so a
//! large context never sits in memory whole.

use std::{
    fs,
    io::{self, Write},
    path::{Component, Path, PathBuf},
};

use bytes::Bytes;
use tokio::sync::mpsc;

use homeport_core::error::{local_file, AppError, AppResult};

use crate::messages as msg;

use super::ignore::DockerIgnore;

/// Size of the chunks sent to the Engine.
const CHUNK: usize = 64 * 1024;

/// A checked build context: the folder and the Dockerfile inside it.
#[derive(Debug, Clone)]
pub(super) struct Context {
    pub(super) root: PathBuf,
    /// Relative to `root`, `/`-separated, as the Engine expects it.
    pub(super) dockerfile: String,
    ignore: DockerIgnore,
}

impl Context {
    /// Checks that `root` is a folder holding `dockerfile`, and reads its `.dockerignore`.
    pub(super) fn open(root: &Path, dockerfile: &str) -> AppResult<Self> {
        if !root.is_dir() {
            return Err(AppError::validation(
                msg::BUILD_NOT_A_FOLDER.with("path", root),
            ));
        }
        let dockerfile = relative(dockerfile)
            .ok_or_else(|| AppError::validation(msg::BUILD_DOCKERFILE_OUTSIDE))?;
        if !root.join(&dockerfile).is_file() {
            return Err(AppError::validation(
                msg::BUILD_NO_DOCKERFILE
                    .with("dockerfile", &dockerfile)
                    .with("folder", root),
            ));
        }
        let ignore = match fs::read_to_string(root.join(".dockerignore")) {
            Ok(text) => DockerIgnore::parse(&text),
            Err(e) if e.kind() == io::ErrorKind::NotFound => DockerIgnore::default(),
            Err(e) => return Err(local_file(&root.join(".dockerignore"), e)),
        };
        Ok(Self {
            root: root.to_owned(),
            dockerfile,
            ignore,
        })
    }

    /// Whether a path, relative to the root, is sent. The Dockerfile and
    /// `.dockerignore` always are, as the Docker CLI does.
    pub(super) fn includes(&self, path: &str) -> bool {
        path == self.dockerfile || path == ".dockerignore" || !self.ignore.excludes(path)
    }

    /// Starts packing on a blocking thread; the receiver yields the tar's
    /// chunks, then ends (or yields the error that stopped it).
    pub(super) fn stream(self) -> mpsc::Receiver<io::Result<Bytes>> {
        let (tx, rx) = mpsc::channel(4);
        tokio::task::spawn_blocking(move || {
            let writer = ChannelWriter {
                tx: tx.clone(),
                buf: Vec::with_capacity(CHUNK),
            };
            let result = self.pack(writer);
            if let Err(e) = result {
                // The receiver is gone when the build was cancelled.
                let _ = tx.blocking_send(Err(e));
            }
        });
        rx
    }

    fn pack(&self, writer: ChannelWriter) -> io::Result<()> {
        let mut tar = tar::Builder::new(writer);
        tar.follow_symlinks(false);
        self.add_dir(&mut tar, &self.root, "")?;
        let mut writer = tar.into_inner()?;
        writer.flush()
    }

    fn add_dir(
        &self,
        tar: &mut tar::Builder<ChannelWriter>,
        dir: &Path,
        prefix: &str,
    ) -> io::Result<()> {
        let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "{} has a name that isn't valid UTF-8",
                        entry.path().display()
                    ),
                ));
            };
            let rel = if prefix.is_empty() {
                name.to_owned()
            } else {
                format!("{prefix}/{name}")
            };
            let kind = entry.file_type()?;
            if kind.is_dir() {
                if !self.includes(&rel) {
                    // An exception may still bring back something inside.
                    if self.ignore.has_exceptions() {
                        self.add_dir(tar, &entry.path(), &rel)?;
                    }
                    continue;
                }
                tar.append_dir(&rel, entry.path())?;
                self.add_dir(tar, &entry.path(), &rel)?;
            } else if self.includes(&rel) {
                tar.append_path_with_name(entry.path(), &rel)?;
            }
        }
        Ok(())
    }
}

/// A path relative to the context that stays inside it, `/`-separated.
fn relative(path: &str) -> Option<String> {
    let path = Path::new(path.trim());
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(p) => parts.push(p.to_str()?.to_owned()),
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

/// Sends what the tar builder writes as chunks over a channel, waiting when
/// the Engine is slower than the disk.
struct ChannelWriter {
    tx: mpsc::Sender<io::Result<Bytes>>,
    buf: Vec<u8>,
}

impl ChannelWriter {
    fn send(&mut self) -> io::Result<()> {
        if self.buf.is_empty() {
            return Ok(());
        }
        let chunk = Bytes::from(std::mem::replace(&mut self.buf, Vec::with_capacity(CHUNK)));
        self.tx
            .blocking_send(Ok(chunk))
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "the build was stopped"))
    }
}

impl Write for ChannelWriter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(data);
        if self.buf.len() >= CHUNK {
            self.send()?;
        }
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.send()
    }
}

#[cfg(test)]
mod tests;
