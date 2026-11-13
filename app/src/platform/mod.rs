//! What the app asks of the system it runs on, beyond drawing a window:
//! the file dialogs, what opens a link or a folder, the files the user
//! picked, and the app's own folders. The former `tauri-plugin-dialog`,
//! `tauri-plugin-opener` and the commands `read_text_file`,
//! `write_text_file`, `open_data_folder` and `open_log_folder`.
//!
//! A page reaches it through `ctx.platform`, a [`Platform`]:
//!
//! ```ignore
//! // in update: ask where to save, then write, off the window's thread
//! Message::Download(lines) => ctx
//!     .platform
//!     .save_file(SaveFile::new(t!("containers.logs.saveTitle"), "logs.txt"))
//!     .map(move |path| Message::SaveTo(path, lines.clone())),
//! // Cancelling is an answer, not a failure: nothing happens.
//! Message::SaveTo(Ok(None), _) => Task::none(),
//! // No dialog could be shown (a Linux desktop with no portal): said.
//! Message::SaveTo(Err(error), _) => { ctx.failed(Surfaced::new(error, "save_file")); Task::none() }
//! Message::SaveTo(Ok(Some(path)), lines) => {
//!     let saving = files::save(path, move |out, meter| lines.write_counted(out, |n| meter.at(n, total)));
//!     Task::batch([
//!         // The toast outlives the page: it is the shell's to say.
//!         ctx.act(saving.done, |saved| match saved {
//!             Ok(()) => Outcome::quiet().saying(|| t!("containers.logs.saved")),
//!             Err(e) => Outcome::quiet().failing(Surfaced::new(e.clone(), "write_text_file")),
//!         }, Message::Saved),
//!         Task::run(saving.progress, Message::Saving),
//!     ])
//! }
//! ```
//!
//! - **Dialogs** ([`dialogs`]) answer with a `Task`: they open over the
//!   app's window. A cancelled dialog answers `Ok(None)`, or no file; a
//!   desktop that can show none answers an `AppError`.
//! - **Files** ([`files`]) are read and written off the window's thread,
//!   and a file is replaced whole or not at all.
//! - **The opener** ([`opener`]) hands a link to the browser and a folder
//!   to the file manager; its failures are `AppError`s of the catalogue.
//! - **Names** ([`names`]) makes what a save dialog suggests safe on
//!   every system.
//!
//! Nothing here knows a page (`tests/layout.rs`). In tests the dialogs
//! and the opener are a [`fake::Fake`]: no dialog of the OS ever opens,
//! and the folders are temporary ones.

pub mod dialogs;
#[cfg(test)]
pub mod fake;
pub mod files;
pub mod folders;
pub mod names;
pub mod opener;

use std::{
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};

use iced::Task;

use homeport_core::{
    error::{local_file, AppResult},
    local_files,
};

use self::{
    dialogs::{Dialogs, OpenFile, OpenFolder, SaveFile},
    files::off_thread,
    opener::Opener,
};
use crate::app::AppState;

/// The system the app runs on, as the app sees it. Cheap to clone: the
/// window keeps one and lends it to every page.
#[derive(Clone)]
pub struct Platform {
    dialogs: Arc<dyn Dialogs>,
    opener: Arc<dyn Opener>,
    /// Where the app's log files are.
    log_dir: PathBuf,
}

impl std::fmt::Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Platform")
            .field("log_dir", &self.log_dir)
            .finish_non_exhaustive()
    }
}

/// The system a window starts on: the OS's, and under `cargo test` a
/// [`fake::Fake`] of its own, so that no test opens a dialog of the OS or
/// a folder of the user's by forgetting to bring one.
impl Default for Platform {
    fn default() -> Self {
        #[cfg(test)]
        return fake::Fake::new().platform();
        #[cfg(not(test))]
        Self::native()
    }
}

impl Platform {
    /// The real one: the OS's dialogs and handlers, the app's log folder.
    pub fn native() -> Self {
        Self::of(
            Arc::new(dialogs::Native),
            Arc::new(opener::Native),
            folders::log_dir(),
        )
    }

    /// One whose dialogs, opener and log folder are given: a test's.
    pub fn of(dialogs: Arc<dyn Dialogs>, opener: Arc<dyn Opener>, log_dir: PathBuf) -> Self {
        Self {
            dialogs,
            opener,
            log_dir,
        }
    }

    /// Asks for one file to open; `None` when cancelled (`pickFile`).
    pub fn pick_file(&self, request: OpenFile) -> Task<AppResult<Option<PathBuf>>> {
        self.dialogs
            .open(request, false)
            .map(|picked| picked.map(|picked| picked.into_iter().next()))
    }

    /// Asks for files to open; none when cancelled (`pickFiles`).
    pub fn pick_files(&self, request: OpenFile) -> Task<AppResult<Vec<PathBuf>>> {
        self.dialogs.open(request, true)
    }

    /// Asks for a folder; `None` when cancelled (`pickFolder`).
    pub fn pick_folder(&self, request: OpenFolder) -> Task<AppResult<Option<PathBuf>>> {
        self.dialogs.folder(request)
    }

    /// Asks where to save a file; `None` when cancelled (`pickSavePath`).
    pub fn save_file(&self, request: SaveFile) -> Task<AppResult<Option<PathBuf>>> {
        self.dialogs.save(request)
    }

    /// Opens a web, mail or phone link with what the OS has for it
    /// (`openUrl`). Pages ask with `ctx.open_url`, which also says a
    /// failure in a toast.
    pub fn open_url(&self, url: String) -> impl Future<Output = AppResult<()>> + Send + 'static {
        let opener = self.opener.clone();
        off_thread(move || {
            let link = opener::link(&url)?;
            opener
                .open(link.as_ref())
                .map_err(|e| opener::link_not_opened(link).with_cause(&e))
        })
    }

    /// Shows a file that exists, selected in its folder, in the file
    /// manager (`revealItemInDir`).
    pub fn reveal(&self, path: PathBuf) -> impl Future<Output = AppResult<()>> + Send + 'static {
        let opener = self.opener.clone();
        off_thread(move || {
            exists(&path)?;
            opener
                .reveal(&path)
                .map_err(|e| local_files::folder_not_opened(&path, &e))
        })
    }

    /// Opens `folder` in the file manager, made first if it isn't there.
    ///
    /// Only ever a folder: handing a file to the OS runs it when it is a
    /// program, a script or a shortcut, which the Tauri app never allowed
    /// (`opener:default` has no `open-path`). There is no way to open a
    /// file through the platform; [`Platform::reveal`] shows one where it
    /// lies instead.
    pub fn open_folder(
        &self,
        folder: PathBuf,
    ) -> impl Future<Output = AppResult<()>> + Send + 'static {
        let opener = self.opener.clone();
        off_thread(move || {
            local_files::ensure_folder(&folder)?;
            // What is there is looked at, not trusted: a link to a file
            // is a file.
            let is_folder = std::fs::metadata(&folder).map_err(|e| local_file(&folder, e))?;
            if !is_folder.is_dir() {
                return Err(local_file(
                    &folder,
                    std::io::ErrorKind::NotADirectory.into(),
                ));
            }
            opener
                .open(folder.as_os_str())
                .map_err(|e| local_files::folder_not_opened(&folder, &e))
        })
    }

    /// Opens the folder holding Homeport's database in the file manager.
    pub fn open_data_folder(
        &self,
        state: &Arc<AppState>,
    ) -> impl Future<Output = AppResult<()>> + Send + 'static {
        let (opener, state) = (self.opener.clone(), state.clone());
        off_thread(move || {
            crate::api::app::open_data_folder(&state, |dir| opener.open(dir.as_os_str()))
        })
    }

    /// Opens the folder holding Homeport's log files in the file manager.
    pub fn open_log_folder(&self) -> impl Future<Output = AppResult<()>> + Send + 'static {
        let (opener, dir) = (self.opener.clone(), self.log_dir.clone());
        off_thread(move || {
            crate::api::app::open_log_folder(&dir, |dir| opener.open(dir.as_os_str()))
        })
    }

    /// Where the app's log files are.
    pub fn log_dir(&self) -> &Path {
        &self.log_dir
    }
}

/// `path` is there, or the failure that names it.
fn exists(path: &Path) -> AppResult<()> {
    std::fs::symlink_metadata(path)
        .map(|_| ())
        .map_err(|e| local_file(path, e))
}

#[cfg(test)]
mod tests;
