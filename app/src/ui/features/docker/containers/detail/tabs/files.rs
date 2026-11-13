//! The Files tab (`FilesTab`, `useFileTransfers`): a running container's
//! folders, a file or a folder saved to this computer, and files of this
//! computer put in the folder that shows.
//!
//! A transfer is the Engine's (`files_download`, `files_upload`, the use
//! cases the Tauri commands called), run by `ctx.act`: its toast and the
//! folder read again are the shell's to say, page left or not, and it
//! goes on to its end when the page is left. Only Cancel stops it.
//!
//! - A download says how many bytes came, of how many for a file; Cancel
//!   drops the Engine's request at once, and no file is left (the file
//!   is written beside its place and put there only once whole).
//! - Files are put one at a time, so each one's failure is its own and
//!   the count moves. Cancel stops before the next file: the one being
//!   put goes in whole, so no file is left cut short in the container,
//!   and what went in is said and the folder read again.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use iced::{futures::StreamExt, Task};

use tokio::sync::watch;

use homeport_core::error::{AppError, AppResult};
use homeport_engine::containers::files::{FileEntry, FileKind};

use crate::{
    api::engine::files::{files_download_counted, files_upload},
    data::Resource,
    platform::{
        dialogs::{OpenFile, SaveFile},
        names,
    },
    t,
    ui::{
        features::docker::containers::detail::queries,
        shell::{Context, Outcome},
        widgets::{
            data::file_browser::{self, Entry, Kind},
            feedback::Surfaced,
        },
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    Browser(file_browser::Message),
    /// "Upload here".
    Upload,
    /// The save dialog of a download answered.
    SaveTo(AppResult<Option<PathBuf>>),
    /// The open dialog of an upload answered.
    UploadFrom(AppResult<Vec<PathBuf>>),
    /// How far the transfer of this number is: files put, or bytes come.
    Progress(u64, Step),
    /// The transfer of this number ended, was stopped, or failed: the
    /// shell said which.
    Ended(u64),
    /// The Cancel button of the transfer that runs.
    Cancel,
}

/// How far a transfer is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Files put in the folder.
    Files(usize),
    /// Bytes of the archive that came, of how many if known.
    Bytes(u64, Option<u64>),
}

/// A dialog that is open: what its answer is about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asking {
    Download(Entry),
    /// Files to put in this folder.
    Upload(String),
}

/// What a transfer that runs is doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Doing {
    /// Saving this entry: how many bytes came, of how many if known.
    Download {
        name: String,
        bytes: u64,
        total: Option<u64>,
    },
    /// Putting files in a folder: how many are put, of how many.
    Upload {
        folder: String,
        done: usize,
        count: usize,
    },
}

/// A transfer that runs, and what stops it.
#[derive(Debug)]
pub struct Transfer {
    pub number: u64,
    pub doing: Doing,
    /// Cancel was pressed: an upload ends after the file being put.
    pub stopping: bool,
    stop: watch::Sender<bool>,
}

/// What became of each file put in a folder; `true` when the transfer
/// was stopped before it ended.
type Uploaded = (Vec<(PathBuf, AppResult<()>)>, bool);

#[derive(Debug, Default)]
pub struct Files {
    pub browser: file_browser::State,
    /// The folder that shows (`useState('/')`).
    pub path: String,
    /// What the listing shows, made from the read it was last given.
    pub entries: Vec<Entry>,
    seen: Option<Arc<Vec<FileEntry>>>,
    /// Whether the folder was read, and why its last read failed.
    pub read: bool,
    pub failure: Option<AppError>,
    pub asking: Option<Asking>,
    pub transfer: Option<Transfer>,
    /// The number of the last transfer started.
    started: u64,
}

/// `FileEntry` as the kit's browser takes it.
pub fn entry(file: &FileEntry) -> Entry {
    Entry {
        name: file.name.clone(),
        path: file.path.clone(),
        kind: match file.kind {
            FileKind::Directory => Kind::Directory,
            FileKind::File => Kind::File,
            FileKind::Symlink => Kind::Symlink,
            FileKind::Other => Kind::Other,
        },
        size: file.size,
        modified: file.modified,
        mode: file.mode.clone(),
    }
}

/// The save dialog of an entry: its name made safe, `name.tar` for a
/// folder, which comes as an archive.
pub fn save_request(entry: &Entry) -> SaveFile {
    let name = match entry.kind {
        Kind::Directory => names::with_extension(&entry.name, "tar", ""),
        _ => entry.name.clone(),
    };
    SaveFile::new(
        t!("containers.files.saveOne", name = entry.name.as_str()),
        &name,
    )
}

/// What the shell says of a download.
fn downloaded(name: String, result: &(AppResult<bool>, bool)) -> Outcome {
    match result {
        (_, true) => Outcome::quiet(),
        (Ok(true), _) => Outcome::quiet()
            .saying(move || t!("containers.files.savedArchive", name = name.as_str())),
        (Ok(false), _) => {
            Outcome::quiet().saying(move || t!("containers.files.saved", name = name.as_str()))
        }
        (Err(error), _) => Outcome::quiet().failing(Surfaced::new(error.clone(), "files_download")),
    }
}

/// The name of a file of this computer, as a failure names it.
fn shown(source: &Path) -> String {
    source.file_name().map_or_else(
        || source.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// What the shell says of an upload: what was put, in one toast; what
/// failed, in one, the first failure's words with every file that failed
/// in its detail; and the folder is read again if anything went in.
pub(crate) fn uploaded(folder: String, listing: crate::data::Key, result: &Uploaded) -> Outcome {
    let (results, _) = result;
    let worked = results.iter().filter(|(_, result)| result.is_ok()).count();
    let failed: Vec<(&PathBuf, &AppError)> = results
        .iter()
        .filter_map(|(source, result)| Some((source, result.as_ref().err()?)))
        .collect();
    let mut told = Outcome::quiet();
    if worked > 0 {
        told = told
            .saying(move || {
                t!(
                    "containers.files.uploaded",
                    count = worked,
                    path = folder.as_str()
                )
            })
            .invalidating(listing);
    }
    if let Some((_, first)) = failed.first() {
        let lines: Vec<String> = failed
            .iter()
            .map(|(source, error)| format!("{}: {}", shown(source), error.message))
            .collect();
        let mut error = (*first).clone();
        error.detail = Some(lines.join("\n"));
        told = told.failing(Surfaced::new(error, "files_upload"));
    }
    told
}

impl Files {
    pub fn new() -> Self {
        Self {
            path: "/".to_owned(),
            ..Self::default()
        }
    }

    /// Takes a listing that was read; nothing is done while it is the
    /// one the entries were made from.
    pub fn take(&mut self, read: &Resource<Arc<Vec<FileEntry>>>) {
        self.failure = read.error().cloned();
        self.read = read.data().is_some();
        let Some(read) = read.data() else { return };
        if self
            .seen
            .as_ref()
            .is_some_and(|seen| Arc::ptr_eq(seen, read))
        {
            return;
        }
        self.entries = read.iter().map(entry).collect();
        self.seen = Some(read.clone());
    }

    /// The folder changed: what showed of the last one goes.
    fn go(&mut self, path: String) {
        if path != self.path {
            self.path = path;
            self.entries.clear();
            self.seen = None;
            self.read = false;
            self.failure = None;
        }
    }

    /// Whether a dialog is open or a transfer runs: a second one waits.
    pub fn is_busy(&self) -> bool {
        self.asking.is_some() || self.transfer.is_some()
    }

    fn start(&mut self, doing: Doing) -> (u64, watch::Receiver<bool>) {
        self.started += 1;
        let (stop, stopped) = watch::channel(false);
        self.transfer = Some(Transfer {
            number: self.started,
            doing,
            stopping: false,
            stop,
        });
        (self.started, stopped)
    }

    /// The browser asked for something. `Navigate` and `Refresh` are the
    /// page's: they are given back.
    pub fn browse(
        &mut self,
        message: file_browser::Message,
        ctx: &Context<'_>,
    ) -> (Task<Message>, Option<file_browser::Event>) {
        match self.browser.update(message, Some(&self.entries)) {
            Some(file_browser::Event::Navigate(path)) => {
                self.go(path.clone());
                (Task::none(), Some(file_browser::Event::Navigate(path)))
            }
            Some(file_browser::Event::Download(_) | file_browser::Event::Upload)
                if self.is_busy() =>
            {
                (Task::none(), None)
            }
            Some(file_browser::Event::Download(entry)) => {
                let request = save_request(&entry);
                self.asking = Some(Asking::Download(entry));
                (ctx.platform.save_file(request).map(Message::SaveTo), None)
            }
            Some(file_browser::Event::Upload) => self.ask_upload(ctx),
            other => (Task::none(), other),
        }
    }

    /// "Upload here": the open dialog, for the folder that shows.
    pub fn ask_upload(
        &mut self,
        ctx: &Context<'_>,
    ) -> (Task<Message>, Option<file_browser::Event>) {
        if self.is_busy() {
            return (Task::none(), None);
        }
        let title = t!("containers.files.uploadTo", path = self.path.as_str());
        self.asking = Some(Asking::Upload(self.path.clone()));
        let asked = ctx.platform.pick_files(OpenFile::new(title));
        (asked.map(Message::UploadFrom), None)
    }

    /// The save dialog answered: the Engine's archive goes there.
    pub fn save_to(
        &mut self,
        target: AppResult<Option<PathBuf>>,
        host_id: &str,
        container_id: &str,
        ctx: &Context<'_>,
    ) -> Task<Message> {
        let Some(Asking::Download(entry)) = self.asking.take() else {
            return Task::none();
        };
        let target = match target {
            Ok(Some(target)) => target,
            // Cancelled: nothing is said.
            Ok(None) => return Task::none(),
            Err(error) => {
                ctx.failed(Surfaced::new(error, "save_file"));
                return Task::none();
            }
        };
        let doing = Doing::Download {
            name: entry.name.clone(),
            bytes: 0,
            total: None,
        };
        let (number, stopped) = self.start(doing);
        let (wrote, steps) = watch::channel((0_u64, None));
        let (state, host, id) = (
            ctx.state.clone(),
            host_id.to_owned(),
            container_id.to_owned(),
        );
        let destination = target.to_string_lossy().into_owned();
        let work = async move {
            let progress = move |bytes, total| {
                let _ = wrote.send((bytes, total));
            };
            let path = entry.path;
            let download = files_download_counted(&state, host, id, path, destination, progress);
            tokio::select! {
                result = download => (result, false),
                // The request is dropped: the file was being written
                // beside its place, and goes.
                () = cancelled(stopped) => (Ok(false), true),
            }
        };
        let name = entry.name;
        let done = ctx.act(
            work,
            move |result| downloaded(name, result),
            move |_| Message::Ended(number),
        );
        let steps = changes(steps)
            .map(move |(bytes, total)| Message::Progress(number, Step::Bytes(bytes, total)));
        Task::batch([done, Task::run(steps, |message| message)])
    }

    /// The open dialog answered: each file goes into the folder in turn.
    pub fn upload_from(
        &mut self,
        sources: AppResult<Vec<PathBuf>>,
        host_id: &str,
        container_id: &str,
        ctx: &Context<'_>,
    ) -> Task<Message> {
        let Some(Asking::Upload(folder)) = self.asking.take() else {
            return Task::none();
        };
        let sources = match sources {
            // No file picked is a cancel.
            Ok(sources) if sources.is_empty() => return Task::none(),
            Ok(sources) => sources,
            Err(error) => {
                ctx.failed(Surfaced::new(error, "pick_files"));
                return Task::none();
            }
        };
        let doing = Doing::Upload {
            folder: folder.clone(),
            done: 0,
            count: sources.len(),
        };
        let (number, stopped) = self.start(doing);
        let (progress, steps) = watch::channel(0_usize);
        let (state, host, id) = (
            ctx.state.clone(),
            host_id.to_owned(),
            container_id.to_owned(),
        );
        let to = folder.clone();
        let work = async move {
            let mut results = vec![];
            for source in sources {
                // Cancel stops before the next file, never in one: the
                // container is never left with a file cut short.
                if *stopped.borrow() {
                    return (results, true);
                }
                let path = source.to_string_lossy().into_owned();
                let put = files_upload(&state, host.clone(), id.clone(), to.clone(), vec![path]);
                results.push((source, put.await));
                let _ = progress.send(results.len());
            }
            (results, false)
        };
        let listing = queries::files_key(host_id, container_id, &folder);
        let done = ctx.act(
            work,
            move |result| uploaded(folder, listing, result),
            move |_| Message::Ended(number),
        );
        let steps = changes(steps).map(move |done| Message::Progress(number, Step::Files(done)));
        Task::batch([done, Task::run(steps, |message| message)])
    }

    pub fn progress(&mut self, number: u64, step: Step) {
        let Some(transfer) = self.transfer.as_mut().filter(|t| t.number == number) else {
            return;
        };
        match (&mut transfer.doing, step) {
            (Doing::Upload { done, .. }, Step::Files(files)) => *done = files,
            (Doing::Download { bytes, total, .. }, Step::Bytes(came, of)) => {
                *bytes = came;
                *total = of;
            }
            _ => {}
        }
    }

    /// A transfer ended: the next can start. One that ended after
    /// another started is not the one that shows.
    pub fn ended(&mut self, number: u64) {
        if self.transfer.as_ref().is_some_and(|t| t.number == number) {
            self.transfer = None;
        }
    }

    /// Stops the transfer that runs; its end says what it did.
    pub fn cancel(&mut self) {
        if let Some(transfer) = &mut self.transfer {
            transfer.stopping = true;
            let _ = transfer.stop.send(true);
        }
    }
}

/// Ends when Cancel was pressed, and only then: a page that goes away
/// drops what would have pressed it, which stops nothing.
async fn cancelled(mut stopped: watch::Receiver<bool>) {
    if stopped.wait_for(|stop| *stop).await.is_err() {
        std::future::pending::<()>().await;
    }
}

/// The values a watch takes after its first, until its sender goes.
fn changes<T: Clone + Send + Sync + 'static>(
    receiver: watch::Receiver<T>,
) -> impl iced::futures::Stream<Item = T> + Send + 'static {
    iced::futures::stream::unfold(receiver, |mut receiver| async move {
        receiver.changed().await.ok()?;
        let value = receiver.borrow_and_update().clone();
        Some((value, receiver))
    })
}
