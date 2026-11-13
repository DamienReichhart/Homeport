//! Files on this computer the user picked in a dialog: text to read (a
//! key, a certificate, a compose file) and files to write (logs, an
//! export), always off the window's thread.
//!
//! A file is written beside where it goes and put in place once whole:
//! until then, and if the write fails, whatever was at that path (last
//! week's logs, picked again in the save dialog) stays as it was.

use std::{
    fs::{self, File, OpenOptions},
    future::Future,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use iced::futures::{stream, Stream};
use tokio::sync::watch;

use homeport_core::{
    error::{local_file, AppError, AppResult, ErrorCode},
    local_files, messages as msg,
};

/// Runs `work` where blocking is allowed, and waits for it.
pub(super) async fn off_thread<T: Send + 'static>(
    work: impl FnOnce() -> AppResult<T> + Send + 'static,
) -> AppResult<T> {
    tokio::task::spawn_blocking(work).await.unwrap_or_else(|e| {
        Err(AppError::new(ErrorCode::Io, msg::IO_FAILED)
            .with_detail(e)
            .logged("a task of the platform layer panicked"))
    })
}

/// Reads a file the user picked in an open dialog, with the rules of
/// `read_text_file`: text only, 1 MB at most.
pub async fn read_text(path: PathBuf) -> AppResult<String> {
    off_thread(move || local_files::read_text(&path)).await
}

/// Writes `contents` to a path the user picked in a save dialog.
pub async fn write_text(path: PathBuf, contents: String) -> AppResult<()> {
    off_thread(move || replace(&path, |out| out.write_all(contents.as_bytes()))).await
}

/// Tells one unfinished file of this process from another.
static PARTS: AtomicU64 = AtomicU64::new(0);

/// A file being written beside the one it will replace. Dropped before
/// [`Part::commit`], it is removed and the target is as it was.
struct Part {
    file: Option<File>,
    temp: PathBuf,
    target: PathBuf,
}

impl Part {
    fn create(path: &Path) -> io::Result<Self> {
        // A link is written through, as a plain write would: what it
        // points at is replaced, not the link.
        let target = if path.is_symlink() {
            fs::canonicalize(path).unwrap_or_else(|_| path.to_owned())
        } else {
            path.to_owned()
        };
        let folder = match target.parent() {
            Some(folder) if !folder.as_os_str().is_empty() => folder.to_owned(),
            _ => PathBuf::from("."),
        };
        // Another instance of the app, or a file left by one that was
        // killed, may hold a name: `create_new` never takes over a file.
        for _ in 0..64 {
            let n = PARTS.fetch_add(1, Ordering::Relaxed);
            let temp = folder.join(format!(".homeport-{}-{n}.part", std::process::id()));
            match OpenOptions::new().write(true).create_new(true).open(&temp) {
                Ok(file) => {
                    return Ok(Self {
                        file: Some(file),
                        temp,
                        target,
                    })
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e),
            }
        }
        Err(io::ErrorKind::AlreadyExists.into())
    }

    /// Flushes the file to disk and puts it where it goes, with the
    /// permissions of the file it replaces when there was one.
    fn commit(mut self) -> io::Result<()> {
        if let Some(file) = self.file.take() {
            file.sync_all()?;
            if let Ok(replaced) = fs::metadata(&self.target) {
                if replaced.is_file() {
                    file.set_permissions(replaced.permissions())?;
                }
            }
        }
        // `self` is dropped on the way out: a rename that failed leaves
        // nothing behind, and one that worked has nothing left to remove.
        fs::rename(&self.temp, &self.target)
    }
}

impl Drop for Part {
    fn drop(&mut self) {
        drop(self.file.take());
        // Gone already when the rename worked.
        let _ = fs::remove_file(&self.temp);
    }
}

/// Writes a file at `path` with what `write` puts in it, whole or not at
/// all: a failure part way leaves what was there untouched and nothing
/// beside it. Blocks; call it off the window's thread ([`write_text`],
/// [`save`]).
pub fn replace(path: &Path, write: impl FnOnce(&mut dyn Write) -> io::Result<()>) -> AppResult<()> {
    let written = (|| {
        let part = Part::create(path)?;
        if let Some(file) = &part.file {
            let mut out = BufWriter::new(file);
            write(&mut out)?;
            out.flush()?;
        }
        part.commit()
    })();
    written.map_err(|e| local_file(path, e))
}

/// How far a file being saved is: `done` of `total`, in whatever the
/// writer counts (lines, bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Progress {
    pub done: u64,
    pub total: u64,
}

impl Progress {
    /// From 0 to 100; 100 when there was nothing to write.
    pub fn percent(self) -> u8 {
        if self.total == 0 {
            return 100;
        }
        let percent = u128::from(self.done.min(self.total)) * 100 / u128::from(self.total);
        u8::try_from(percent).unwrap_or(100)
    }
}

/// Where a writer says how far it is. Only a new percent is passed on,
/// so it can be told after every line.
pub struct Meter {
    sender: watch::Sender<Progress>,
}

impl Meter {
    pub fn at(&self, done: u64, total: u64) {
        let progress = Progress { done, total };
        self.sender.send_if_modified(|shown| {
            let moved = shown.total != total || shown.percent() != progress.percent();
            *shown = progress;
            moved
        });
    }
}

/// A file being saved: its end, and how far it is meanwhile.
pub struct Saving<Done, Steps> {
    /// Ends when the file is in place, or says why it is not. This is the
    /// work to hand `ctx.act`, so its toast outlives the page.
    pub done: Done,
    /// The progress, a hundred steps at most, ending with the write. For
    /// `Task::run`; dropping it changes nothing to the write.
    pub progress: Steps,
}

/// Saves a file at `path` off the window's thread, as [`replace`] does,
/// with `write` saying how far it is on the [`Meter`] it is given.
pub fn save(
    path: PathBuf,
    write: impl FnOnce(&mut dyn Write, &Meter) -> io::Result<()> + Send + 'static,
) -> Saving<impl Future<Output = AppResult<()>> + Send, impl Stream<Item = Progress> + Send> {
    let (sender, receiver) = watch::channel(Progress::default());
    let meter = Meter { sender };
    let done = off_thread(move || replace(&path, |out| write(out, &meter)));
    let progress = stream::unfold(receiver, |mut receiver| async move {
        // The meter is dropped with the write: the steps end there.
        receiver.changed().await.ok()?;
        let progress = *receiver.borrow_and_update();
        Some((progress, receiver))
    });
    Saving { done, progress }
}

#[cfg(test)]
mod tests;
