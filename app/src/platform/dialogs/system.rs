//! The system's own dialogs on Windows and macOS, through `rfd`:
//! `IFileDialog`, and `NSOpenPanel` and `NSSavePanel`. A Linux desktop is
//! asked through its portal instead (`portal.rs`).

use std::{future::Future, path::PathBuf};

use homeport_core::error::AppResult;
use iced::Task;
use rfd::{AsyncFileDialog, FileHandle};

use super::{window::with_window, Dialogs, Filter, OpenFile, OpenFolder, SaveFile};

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
/// dialog is started on the window's own thread, which is where macOS
/// wants a sheet started; with no window any more, it is as cancelled.
fn ask<T, F>(dialog: AsyncFileDialog, start: fn(AsyncFileDialog) -> F) -> Task<T>
where
    T: Default + Send + 'static,
    F: Future<Output = T> + Send + 'static,
{
    with_window(move |window| start(dialog.set_parent(window))).then(|asked| match asked {
        Some(asked) => Task::future(asked),
        None => Task::done(T::default()),
    })
}

fn path(handle: Option<FileHandle>) -> Option<PathBuf> {
    handle.map(|handle| handle.path().to_owned())
}

// The system always has its dialogs: these never fail.
impl Dialogs for Native {
    // Each dialog is started where `ask` calls, on the window's thread,
    // and only awaited elsewhere.
    fn open(&self, request: OpenFile, many: bool) -> Task<AppResult<Vec<PathBuf>>> {
        let dialog = dialog(&request.title, &request.filters, request.folder);
        let picked: Task<Vec<PathBuf>> = if many {
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
        };
        picked.map(Ok)
    }

    fn folder(&self, request: OpenFolder) -> Task<AppResult<Option<PathBuf>>> {
        let dialog = dialog(&request.title, &[], request.folder).set_can_create_directories(true);
        let picked: Task<Option<PathBuf>> = ask(dialog, |dialog| {
            let asked = dialog.pick_folder();
            async move { path(asked.await) }
        });
        picked.map(Ok)
    }

    fn save(&self, request: SaveFile) -> Task<AppResult<Option<PathBuf>>> {
        let dialog = dialog(&request.title, &request.filters, request.folder)
            .set_file_name(request.name)
            .set_can_create_directories(true);
        let picked: Task<Option<PathBuf>> = ask(dialog, |dialog| {
            let asked = dialog.save_file();
            async move { path(asked.await) }
        });
        picked.map(Ok)
    }
}
