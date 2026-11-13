//! The Files tab (`FilesTab`, `useFileTransfers`): a running container's
//! folders, a file or a folder saved to this computer, and files of this
//! computer put in the folder that shows.
//!
//! A transfer is the Engine's (`files_download`, `files_upload`, the use
//! cases the Tauri commands called), run by `ctx.act`: its toast and the
//! folder read again are the shell's to say, page left or not. Files are
//! put one at a time, so each one's failure is its own and the count
//! moves; a transfer can be stopped between two files, or in the middle
//! of one, whose request is then dropped.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use iced::{futures::StreamExt, Task};

use tokio::sync::watch;

use homeport_core::error::{AppError, AppResult};
use homeport_engine::containers::files::{FileEntry, FileKind};

use crate::{
    api::engine::files::{files_download, files_upload},
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
    /// How many files of the transfer of this number are put.
    Progress(u64, usize),
    /// The transfer of this number ended, was stopped, or failed: the
    /// shell said which.
    Ended(u64),
    /// The Cancel button of the transfer that runs.
    Cancel,
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
    /// Saving this entry.
    Download(String),
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
        let (number, mut stopped) = self.start(Doing::Download(entry.name.clone()));
        let (state, host, id) = (
            ctx.state.clone(),
            host_id.to_owned(),
            container_id.to_owned(),
        );
        let destination = target.to_string_lossy().into_owned();
        let work = async move {
            let download = files_download(&state, host, id, entry.path, destination);
            tokio::select! {
                result = download => (result, false),
                // The request is dropped: nothing was written yet, the
                // file is written once the whole archive came.
                _ = stopped.wait_for(|stop| *stop) => (Ok(false), true),
            }
        };
        let name = entry.name;
        ctx.act(
            work,
            move |result| downloaded(name, result),
            move |_| Message::Ended(number),
        )
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
        let (number, mut stopped) = self.start(doing);
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
                let path = source.to_string_lossy().into_owned();
                let put = files_upload(&state, host.clone(), id.clone(), to.clone(), vec![path]);
                tokio::select! {
                    result = put => results.push((source, result)),
                    _ = stopped.wait_for(|stop| *stop) => return (results, true),
                }
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
        let steps = tokio_stream_of(steps).map(move |done| Message::Progress(number, done));
        Task::batch([done, Task::run(steps, |message| message)])
    }

    pub fn progress(&mut self, number: u64, files: usize) {
        if let Some(Transfer {
            number: running,
            doing: Doing::Upload { done, .. },
            ..
        }) = &mut self.transfer
        {
            if *running == number {
                *done = files;
            }
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
    pub fn cancel(&self) {
        if let Some(transfer) = &self.transfer {
            let _ = transfer.stop.send(true);
        }
    }
}

/// The values a watch takes after its first, until its sender goes.
fn tokio_stream_of(
    receiver: watch::Receiver<usize>,
) -> impl iced::futures::Stream<Item = usize> + Send + 'static {
    iced::futures::stream::unfold(receiver, |mut receiver| async move {
        receiver.changed().await.ok()?;
        let value = *receiver.borrow_and_update();
        Some((value, receiver))
    })
}
