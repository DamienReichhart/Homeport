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
    io::{self, BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use iced::futures::{stream, Stream};
use tokio::sync::watch;

use homeport_core::{
    error::{local_file, AppError, AppResult, ErrorCode},
    local_files::{self, Replacement},
    messages as msg,
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

/// How many links are followed from the path picked to the file it
/// stands for, as the OS itself allows.
const MAX_LINKS: usize = 40;

/// How old a file this app left unfinished must be to be taken away: no
/// write lasts a day, and one killed half way leaves its file behind.
const STALE: Duration = Duration::from_secs(24 * 60 * 60);

/// The folder a file is in.
fn folder_of(path: &Path) -> &Path {
    match path.parent() {
        Some(folder) if !folder.as_os_str().is_empty() => folder,
        _ => Path::new("."),
    }
}

/// The file `path` stands for: itself, or what the links it is lead to,
/// there or not. A link is written through, as a plain write does, so
/// the link stays and a link to nothing gets its file. Links that lead
/// back to themselves are the OS's failure to tell.
fn target(path: &Path) -> io::Result<PathBuf> {
    let mut target = path.to_owned();
    for _ in 0..MAX_LINKS {
        let is_link = fs::symlink_metadata(&target).is_ok_and(|found| found.is_symlink());
        if !is_link {
            return Ok(target);
        }
        let leads_to = fs::read_link(&target)?;
        target = folder_of(&target).join(leads_to);
    }
    // Too many to follow: the OS says why (a loop).
    Err(fs::metadata(path)
        .err()
        .unwrap_or_else(|| io::ErrorKind::InvalidInput.into()))
}

/// What is at `target` now, if a file is: a folder, a pipe, a device or
/// a socket is never replaced by a file.
fn replaced(path: &Path, target: &Path) -> AppResult<Option<fs::Metadata>> {
    match fs::metadata(target) {
        Ok(found) if found.is_file() => Ok(Some(found)),
        Ok(found) if found.is_dir() => Err(local_file(path, io::ErrorKind::IsADirectory.into())),
        Ok(_) => Err(AppError::validation(
            msg::FILE_NOT_REGULAR.with("path", path),
        )),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(local_file(path, e)),
    }
}

/// Takes away the files this app left unfinished in `folder` long ago
/// (`.homeport-….part`: a write that was killed). Whatever fails is left.
fn sweep(folder: &Path) {
    let Ok(entries) = fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let ours = name
            .to_str()
            .is_some_and(|name| name.starts_with(".homeport-") && name.ends_with(".part"));
        let stale = entry.metadata().is_ok_and(|found| {
            found.is_file()
                && found
                    .modified()
                    .ok()
                    .and_then(|at| at.elapsed().ok())
                    .is_some_and(|age| age > STALE)
        });
        if ours && stale {
            let _ = fs::remove_file(entry.path());
        }
    }
}

/// Writes what `unfinished` holds into the file at `target`, which
/// stays the file it was: for a file that can't be renamed over.
fn in_place(unfinished: &Replacement, target: &Path) -> io::Result<()> {
    let mut from = unfinished.file();
    from.seek(SeekFrom::Start(0))?;
    let mut to = OpenOptions::new().write(true).truncate(true).open(target)?;
    io::copy(&mut from, &mut to)?;
    to.sync_all()
}

/// Writes a file at `path` with what `write` puts in it, whole or not at
/// all: a failure part way leaves what was there untouched and nothing
/// beside it. Blocks; call it off the window's thread ([`write_text`],
/// [`save`]).
///
/// The file is written beside its target, private to the user until it
/// is whole (the workspace's [`Replacement`]), then renamed over it. See
/// `docs/agents/iced-migration.md` for what that keeps of the file that
/// was there, and what it doesn't.
pub fn replace(path: &Path, write: impl FnOnce(&mut dyn Write) -> io::Result<()>) -> AppResult<()> {
    replace_by(path, &Stop::default(), write, Replacement::try_commit).map(|_| ())
}

/// What a save writes through: it refuses the next chunk once the save
/// is stopped, which is how a write that can't be interrupted ends.
struct Stoppable<'a, W> {
    out: W,
    stop: &'a Stop,
}

impl<W: Write> Write for Stoppable<'_, W> {
    fn write(&mut self, chunk: &[u8]) -> io::Result<usize> {
        if self.stop.0.load(Ordering::Relaxed) {
            return Err(io::ErrorKind::Other.into());
        }
        self.out.write(chunk)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

/// [`replace`], with what stops it and what puts the finished file in
/// place given.
fn replace_by(
    path: &Path,
    stop: &Stop,
    write: impl FnOnce(&mut dyn Write) -> io::Result<()>,
    put: impl FnOnce(Replacement) -> Result<(), (io::Error, Replacement)>,
) -> AppResult<Saved> {
    let failed = |e: io::Error| local_file(path, e);
    let target = target(path).map_err(failed)?;
    let replaced = replaced(path, &target)?;
    sweep(folder_of(&target));
    // Private from its first byte: the file may replace a key.
    let unfinished = Replacement::new(&target)?;
    if let Some(replaced) = &replaced {
        unfinished.keep_mode_of(replaced).map_err(failed)?;
    }
    let mut out = Stoppable {
        out: BufWriter::new(unfinished.file()),
        stop,
    };
    let written = write(&mut out).and_then(|()| out.flush());
    drop(out);
    // Stopped, whatever the writer made of it: the unfinished file goes
    // as it is dropped, and what was at `path` is as it was.
    if stop.0.load(Ordering::Relaxed) {
        return Ok(Saved::Stopped);
    }
    written.map_err(failed)?;
    if replaced.is_none() {
        unfinished.take_default_mode().map_err(failed)?;
    }
    match put(unfinished) {
        Ok(()) => {}
        // A file that is a mount of its own (a single file bound into a
        // container, a sandbox) can't be renamed over: it is written
        // into, which is all a plain write ever did.
        Err((e, unfinished)) if e.kind() == io::ErrorKind::ResourceBusy && replaced.is_some() => {
            in_place(&unfinished, &target).map_err(failed)?;
        }
        Err((e, _)) => return Err(failed(e)),
    }
    // The rename itself is on disk once its folder is.
    #[cfg(unix)]
    let _ = File::open(folder_of(&target)).and_then(|folder| folder.sync_all());
    Ok(Saved::Whole)
}

/// How far a file being saved is: `done` of `total`, in whatever the
/// writer counts (lines, bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Progress {
    pub done: u64,
    pub total: u64,
}

impl Progress {
    /// From 0 to 100; 0 until the writer has said how much there is.
    pub fn percent(self) -> u8 {
        if self.total == 0 {
            return 0;
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

/// How a save ended, when it did not fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Saved {
    /// The file is in place, whole.
    Whole,
    /// It was stopped before its end: nothing was written where it was
    /// going, and nothing is left beside it.
    Stopped,
}

/// Stops a save that is under way: [`Saving::stop`].
#[derive(Debug, Clone, Default)]
pub struct Stop(Arc<AtomicBool>);

impl Stop {
    /// Asks the write to end. It does at its next chunk, as
    /// [`Saved::Stopped`].
    pub fn now(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
}

/// Stops the write when whoever waited for it no longer does.
struct StopWhenDropped(Stop);

impl Drop for StopWhenDropped {
    fn drop(&mut self) {
        self.0.now();
    }
}

/// A file being saved: its end, and how far it is meanwhile.
pub struct Saving<Done, Steps> {
    /// Ends when the file is in place, or says why it is not. This is the
    /// work to hand `ctx.act`, so its toast outlives the page. Dropped
    /// before its end (the app closes), it stops the write as
    /// [`Saving::stop`] does: the thread that writes ends at its next
    /// chunk, which is what closing the app then waits for.
    pub done: Done,
    /// The progress, a step a percent at most, ending with the write. For
    /// `Task::run`; dropping it changes nothing to the write.
    pub progress: Steps,
    /// Stops the write: for a Cancel button.
    pub stop: Stop,
}

/// Saves a file at `path` off the window's thread, as [`replace`] does,
/// with `write` saying how far it is on the [`Meter`] it is given. The
/// write is on a thread of its own and can't be interrupted: it is
/// stopped between two chunks of what it writes.
pub fn save(
    path: PathBuf,
    write: impl FnOnce(&mut dyn Write, &Meter) -> io::Result<()> + Send + 'static,
) -> Saving<impl Future<Output = AppResult<Saved>> + Send, impl Stream<Item = Progress> + Send> {
    let (sender, receiver) = watch::channel(Progress::default());
    let meter = Meter { sender };
    let stop = Stop::default();
    let (asked, waited) = (stop.clone(), StopWhenDropped(stop.clone()));
    let done = async move {
        // Held until the write has ended: letting go earlier stops it.
        let _waited = waited;
        let put = Replacement::try_commit;
        off_thread(move || replace_by(&path, &asked, |out| write(out, &meter), put)).await
    };
    let progress = stream::unfold(receiver, |mut receiver| async move {
        // The meter is dropped with the write: the steps end there.
        receiver.changed().await.ok()?;
        let progress = *receiver.borrow_and_update();
        Some((progress, receiver))
    });
    Saving {
        done,
        progress,
        stop,
    }
}

#[cfg(test)]
mod tests;
