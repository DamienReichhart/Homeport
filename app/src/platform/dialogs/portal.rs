//! The file dialogs of a Linux desktop, asked of its XDG desktop portal
//! (`org.freedesktop.portal.FileChooser`, over the session's D-Bus): the
//! desktop shows its own chooser (GNOME's, KDE's, GTK's), and the app
//! links no toolkit for it.

use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::pin,
    time::Duration,
};

use ashpd::{
    desktop::file_chooser::{FileChooserProxy, FileFilter, OpenFileOptions, SaveFileOptions},
    zbus, FilePath, WindowIdentifier,
};
use homeport_core::error::AppResult;
use iced::{
    futures::{
        executor::block_on,
        future::{select, Either},
        StreamExt,
    },
    Task,
};
use serde::{de::value::SeqDeserializer, Deserialize};

use super::{
    unavailable, uri, window::with_window, Dialogs, Filter, OpenFile, OpenFolder, SaveFile,
};

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

/// The name the desktop's portal answers to on the session's bus.
const PORTAL: &str = "org.freedesktop.portal.Desktop";

/// Asks the portal reached by `connection` for a dialog over `parent`,
/// and gives the files picked: none when the dialog was cancelled. No
/// portal, a portal without a file chooser, one that fails, and one that
/// leaves the bus while its dialog is open are all failures, never a
/// cancel and never a wait without end.
pub(super) async fn ask(
    connection: zbus::Connection,
    parent: Option<&WindowIdentifier>,
    ask: Ask,
) -> AppResult<Vec<PathBuf>> {
    // Told when the portal's name changes hands: with no new owner, it
    // died, and its answer will never come.
    let bus = zbus::fdo::DBusProxy::new(&connection)
        .await
        .map_err(unavailable)?;
    let mut owners = bus
        .receive_name_owner_changed_with_args(&[(0, PORTAL)])
        .await
        .map_err(unavailable)?;
    let gone = async {
        while let Some(change) = owners.next().await {
            if change
                .args()
                .is_ok_and(|change| change.new_owner().is_none())
            {
                break;
            }
        }
        // The stream ends with the bus itself.
    };
    // Is anything there, and awake? A session with no portal says so at
    // once (and one that starts on demand is started); a portal that
    // hangs is given up on after the connection's own timeout.
    connection
        .call_method(
            Some(PORTAL),
            "/org/freedesktop/portal/desktop",
            Some("org.freedesktop.DBus.Peer"),
            "Ping",
            &(),
        )
        .await
        .map_err(unavailable)?;
    let answered = answer(connection.clone(), parent, ask);
    match select(pin!(answered), pin!(gone)).await {
        Either::Left((Ok(picked), _)) => Ok(picked),
        Either::Left((Err(e), _)) => Err(unavailable(e)),
        Either::Right(((), _)) => Err(unavailable(PORTAL)),
    }
}

/// The portal's answer to a dialog: the files picked, none for a cancel.
async fn answer(
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

/// [`ask`] on the bus `connect` reaches: the session's, for the app. A
/// session with no bus has no portal either.
pub(super) async fn over(
    connect: impl Future<Output = zbus::Result<zbus::Connection>>,
    parent: Option<&WindowIdentifier>,
    ask: Ask,
) -> AppResult<Vec<PathBuf>> {
    let connection = connect.await.map_err(unavailable)?;
    self::ask(connection, parent, ask).await
}

/// How long the portal has to say it is there. The dialog itself may
/// stay open as long as the user likes: this is only for the call made
/// before it, which a portal that hangs would otherwise never answer.
pub(super) const CALL_TIMEOUT: Duration = Duration::from_secs(25);

/// A connection to the session's bus.
async fn session() -> zbus::Result<zbus::Connection> {
    zbus::connection::Builder::session()?
        .method_timeout(CALL_TIMEOUT)
        .build()
        .await
}

/// Opens the dialog over the app's window and gives the files picked.
fn show(ask: Ask) -> Task<AppResult<Vec<PathBuf>>> {
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
                return Ok(vec![]);
            };
            over(session(), parent.as_ref(), ask).await
        })
    })
}

fn first(picked: AppResult<Vec<PathBuf>>) -> AppResult<Option<PathBuf>> {
    picked.map(|picked| picked.into_iter().next())
}

impl Dialogs for Native {
    fn open(&self, request: OpenFile, many: bool) -> Task<AppResult<Vec<PathBuf>>> {
        show(Ask::Open(request, many))
    }

    fn folder(&self, request: OpenFolder) -> Task<AppResult<Option<PathBuf>>> {
        show(Ask::Folder(request)).map(first)
    }

    fn save(&self, request: SaveFile) -> Task<AppResult<Option<PathBuf>>> {
        show(Ask::Save(request)).map(first)
    }
}
