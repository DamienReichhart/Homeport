//! The file dialogs of a Linux desktop, asked of its XDG desktop portal
//! (`org.freedesktop.portal.FileChooser`, over the session's D-Bus): the
//! desktop shows its own chooser (GNOME's, KDE's, GTK's), and the app
//! links no toolkit for it.

use std::path::{Path, PathBuf};

use ashpd::{
    desktop::file_chooser::{FileChooserProxy, FileFilter, OpenFileOptions, SaveFileOptions},
    zbus, FilePath, WindowIdentifier,
};
use iced::{futures::executor::block_on, Task};
use serde::{de::value::SeqDeserializer, Deserialize};

use super::{uri, window::with_window, Dialogs, Filter, OpenFile, OpenFolder, SaveFile};

/// The dialogs of the desktop the app runs on.
#[derive(Debug, Clone, Copy, Default)]
pub struct Native;

/// A dialog to ask the portal for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Ask {
    Open(OpenFile, bool),
    Folder(OpenFolder),
    Save(SaveFile),
}

/// `folder` as the portal takes a path: its bytes, ended by a NUL.
/// `None` for a path that holds one.
pub(super) fn folder_path(folder: &Path) -> Option<FilePath> {
    use std::os::unix::ffi::OsStrExt;
    let mut bytes = folder.as_os_str().as_bytes().to_vec();
    if bytes.contains(&0) {
        return None;
    }
    bytes.push(0);
    // `FilePath` is only made by reading one.
    let read = SeqDeserializer::<_, serde::de::value::Error>::new(bytes.into_iter());
    FilePath::deserialize(read).ok()
}

/// `filters` as the portal takes them: a name and its globs.
fn globs(filters: &[Filter]) -> Vec<FileFilter> {
    filters
        .iter()
        .map(|filter| {
            filter
                .extensions
                .iter()
                .fold(FileFilter::new(&filter.name), |globs, extension| {
                    if extension == "*" {
                        globs.glob("*")
                    } else {
                        globs.glob(&format!("*.{extension}"))
                    }
                })
        })
        .collect()
}

/// Asks the portal reached by `connection` for a dialog over `parent`,
/// and gives the files picked: none when the dialog was cancelled.
pub(super) async fn ask(
    connection: zbus::Connection,
    parent: Option<&WindowIdentifier>,
    ask: Ask,
) -> ashpd::Result<Vec<PathBuf>> {
    let chooser = FileChooserProxy::with_connection(connection).await?;
    let request = match ask {
        Ask::Open(request, many) => {
            let options = OpenFileOptions::default()
                .set_modal(true)
                .set_multiple(many)
                .set_filters(globs(&request.filters))
                .set_current_folder(request.folder.as_deref().and_then(folder_path));
            chooser.open_file(parent, &request.title, options).await
        }
        Ask::Folder(request) => {
            let options = OpenFileOptions::default()
                .set_modal(true)
                .set_directory(true)
                .set_current_folder(request.folder.as_deref().and_then(folder_path));
            chooser.open_file(parent, &request.title, options).await
        }
        Ask::Save(request) => {
            let options = SaveFileOptions::default()
                .set_modal(true)
                .set_current_name(request.name.as_str())
                .set_filters(globs(&request.filters))
                .set_current_folder(request.folder.as_deref().and_then(folder_path));
            chooser.save_file(parent, &request.title, options).await
        }
    };
    match request.and_then(|request| request.response()) {
        Ok(picked) => Ok(picked
            .uris()
            .iter()
            .filter_map(|picked| uri::path(picked.as_str()))
            .collect()),
        Err(ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) => Ok(vec![]),
        Err(e) => Err(e),
    }
}

/// Opens the dialog over the app's window and gives the files picked.
fn show(ask: Ask) -> Task<Vec<PathBuf>> {
    // The window is named to the portal on its own thread: its handle
    // can't leave it. On Wayland that asks the compositor for a name to
    // give the portal, which is kept until the dialog closes.
    let parent = with_window(|window| {
        let handle = window.window_handle().ok()?.as_raw();
        let display = window.display_handle().ok().map(|display| display.as_raw());
        block_on(WindowIdentifier::from_raw_handle(&handle, display.as_ref()))
    });
    parent.then(move |parent| {
        let ask = ask.clone();
        Task::future(async move {
            // No window any more: nothing to open a dialog over.
            let Some(parent) = parent else {
                return vec![];
            };
            let asked = async {
                let connection = zbus::Connection::session().await?;
                self::ask(connection, parent.as_ref(), ask).await
            };
            asked.await.unwrap_or_else(|e| {
                log::error!("the desktop's file dialog couldn't be opened: {e}");
                vec![]
            })
        })
    })
}

impl Dialogs for Native {
    fn open(&self, request: OpenFile, many: bool) -> Task<Vec<PathBuf>> {
        show(Ask::Open(request, many))
    }

    fn folder(&self, request: OpenFolder) -> Task<Option<PathBuf>> {
        show(Ask::Folder(request)).map(|picked| picked.into_iter().next())
    }

    fn save(&self, request: SaveFile) -> Task<Option<PathBuf>> {
        show(Ask::Save(request)).map(|picked| picked.into_iter().next())
    }
}
