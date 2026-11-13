//! The system, for tests: dialogs that answer what the test told them to
//! and an opener that only notes what it was asked, in a folder that goes
//! with the test. No dialog of the OS opens, no browser, no file manager.

use std::{
    collections::VecDeque,
    ffi::{OsStr, OsString},
    io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use iced::Task;

use super::{
    dialogs::{Dialogs, OpenFile, OpenFolder, SaveFile},
    opener::Opener,
    Platform,
};

/// Something a page or the window asked of the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asked {
    /// A dialog for one file, or for several (`true`).
    Open(OpenFile, bool),
    Folder(OpenFolder),
    Save(SaveFile),
    /// A link or a path handed to the OS.
    Opened(OsString),
    Revealed(PathBuf),
}

#[derive(Default)]
struct Script {
    asked: Vec<Asked>,
    /// What the next dialogs answer, in order; a dialog with no answer
    /// left is cancelled.
    answers: VecDeque<Vec<PathBuf>>,
    /// Why the opener fails, when it must.
    refusal: Option<io::ErrorKind>,
}

/// The system a test stands on.
pub struct Fake {
    script: Mutex<Script>,
    /// Where a test saves and what stands for the log folder.
    dir: tempfile::TempDir,
}

impl Fake {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            script: Mutex::default(),
            dir: tempfile::tempdir().expect("a temporary folder"),
        })
    }

    /// The platform a page is given, on this fake.
    pub fn platform(self: &Arc<Self>) -> Platform {
        Platform::of(self.clone(), self.clone(), self.dir.path().join("logs"))
    }

    fn script(&self) -> std::sync::MutexGuard<'_, Script> {
        self.script.lock().expect("the fake's script")
    }

    /// A folder of the test's own, to save in.
    pub fn dir(&self) -> &Path {
        self.dir.path()
    }

    /// The next dialog answers `path`.
    pub fn answer(&self, path: impl Into<PathBuf>) {
        self.answers(vec![path.into()]);
    }

    /// The next dialog answers `paths`: several files.
    pub fn answers(&self, paths: Vec<PathBuf>) {
        self.script().answers.push_back(paths);
    }

    /// From now on the OS has nothing to open a link or a folder with.
    pub fn refuse(&self, kind: io::ErrorKind) {
        self.script().refusal = Some(kind);
    }

    /// What was asked since the last call, in order.
    pub fn took(&self) -> Vec<Asked> {
        std::mem::take(&mut self.script().asked)
    }

    fn answered(&self, asked: Asked) -> Vec<PathBuf> {
        let mut script = self.script();
        script.asked.push(asked);
        script.answers.pop_front().unwrap_or_default()
    }

    fn noted(&self, asked: Asked) -> io::Result<()> {
        let mut script = self.script();
        script.asked.push(asked);
        script.refusal.map_or(Ok(()), |kind| Err(kind.into()))
    }
}

impl Dialogs for Fake {
    fn open(&self, request: OpenFile, many: bool) -> Task<Vec<PathBuf>> {
        let mut picked = self.answered(Asked::Open(request, many));
        if !many {
            picked.truncate(1);
        }
        Task::done(picked)
    }

    fn folder(&self, request: OpenFolder) -> Task<Option<PathBuf>> {
        Task::done(self.answered(Asked::Folder(request)).into_iter().next())
    }

    fn save(&self, request: SaveFile) -> Task<Option<PathBuf>> {
        Task::done(self.answered(Asked::Save(request)).into_iter().next())
    }
}

impl Opener for Fake {
    fn open(&self, target: &OsStr) -> io::Result<()> {
        self.noted(Asked::Opened(target.to_owned()))
    }

    fn reveal(&self, path: &Path) -> io::Result<()> {
        self.noted(Asked::Revealed(path.to_owned()))
    }
}
