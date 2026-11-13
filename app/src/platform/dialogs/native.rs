//! The OS's own dialogs, through `rfd`: the XDG desktop portal on Linux
//! (the desktop's file chooser, over D-Bus, with `zenity` when there is
//! no portal), `IFileDialog` on Windows, `NSOpenPanel` and `NSSavePanel`
//! on macOS.

use std::{future::Future, path::PathBuf};

use iced::{window, Task};
use rfd::{AsyncFileDialog, FileHandle};

use super::{Dialogs, Filter, OpenFile, OpenFolder, SaveFile};

/// The dialogs of the system the app runs on.
#[derive(Debug, Clone, Copy, Default)]
pub struct Native;

/// A dialog titled `title`, showing `filters`, starting in `folder`.
fn dialog(title: &str, filters: &[Filter], folder: Option<PathBuf>) -> AsyncFileDialog {
    let mut dialog = AsyncFileDialog::new().set_title(title);
    for filter in filters {
        dialog = dialog.add_filter(&filter.name, &filter.extensions);
    }
    match folder {
        Some(folder) => dialog.set_directory(folder),
        None => dialog,
    }
}

/// Opens `dialog` over the app's window and gives what it answered. The
/// window is asked for on its own thread, which is where macOS wants a
/// sheet started; with no window (it closed meanwhile) the dialog stands
/// alone.
fn ask<T, F>(dialog: AsyncFileDialog, start: fn(AsyncFileDialog) -> F) -> Task<T>
where
    T: Send + 'static,
    F: Future<Output = T> + Send + 'static,
{
    let mut dialog = Some(dialog);
    window::oldest()
        .then(move |id| {
            let Some(dialog) = dialog.take() else {
                return Task::none();
            };
            match id {
                Some(id) => window::run(id, move |window| start(dialog.set_parent(window))),
                None => Task::done(start(dialog)),
            }
        })
        .then(Task::future)
}

fn path(handle: Option<FileHandle>) -> Option<PathBuf> {
    handle.map(|handle| handle.path().to_owned())
}

impl Dialogs for Native {
    // Each dialog is started where `ask` calls, on the window's thread,
    // and only awaited elsewhere.
    fn open(&self, request: OpenFile, many: bool) -> Task<Vec<PathBuf>> {
        let dialog = dialog(&request.title, &request.filters, request.folder);
        if many {
            ask(dialog, |dialog| {
                let asked = dialog.pick_files();
                async move {
                    let picked = asked.await.unwrap_or_default();
                    picked.iter().map(|file| file.path().to_owned()).collect()
                }
            })
        } else {
            ask(dialog, |dialog| {
                let asked = dialog.pick_file();
                async move { path(asked.await).into_iter().collect() }
            })
        }
    }

    fn folder(&self, request: OpenFolder) -> Task<Option<PathBuf>> {
        let dialog = dialog(&request.title, &[], request.folder).set_can_create_directories(true);
        ask(dialog, |dialog| {
            let asked = dialog.pick_folder();
            async move { path(asked.await) }
        })
    }

    fn save(&self, request: SaveFile) -> Task<Option<PathBuf>> {
        let dialog = dialog(&request.title, &request.filters, request.folder)
            .set_file_name(request.name)
            .set_can_create_directories(true);
        ask(dialog, |dialog| {
            let asked = dialog.save_file();
            async move { path(asked.await) }
        })
    }
}
