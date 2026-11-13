//! The OS's file dialogs: a file or several to open, a folder, a place to
//! save. What `src/shared/lib/dialogs.ts` asked of `tauri-plugin-dialog`.
//!
//! A dialog is asked for with a request (its title, its filters, where it
//! starts) and answers with the paths picked: `None`, or none, when it is
//! cancelled, which is an answer and never a failure. A dialog that
//! couldn't be shown at all is one ([`unavailable`]): on a Linux desktop
//! with no portal, pressing Download must not look like a cancel. The
//! dialogs themselves are behind [`Dialogs`], so a test answers in their
//! place.

#[cfg(target_os = "linux")]
mod portal;
#[cfg(all(test, target_os = "linux"))]
mod portal_tests;
#[cfg(not(target_os = "linux"))]
mod system;
#[cfg(target_os = "linux")]
mod uri;
mod window;

use std::{fmt::Display, path::PathBuf};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    messages as msg,
};
use iced::Task;

#[cfg(target_os = "linux")]
pub use self::portal::Native;
#[cfg(not(target_os = "linux"))]
pub use self::system::Native;
use super::names;

/// The kind of files a dialog shows: a name and the extensions it stands
/// for (`Compose files`, `yml` and `yaml`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    pub name: String,
    pub extensions: Vec<String>,
}

impl Filter {
    /// `extensions` are taken as they are usually written: `tar`, `.tar`
    /// and `*.tar` are the same, case aside; one given twice counts once,
    /// and `*` stands for every file.
    pub fn new(name: impl Into<String>, extensions: &[&str]) -> Self {
        let mut kept: Vec<String> = vec![];
        for extension in extensions {
            let extension = extension
                .trim()
                .trim_start_matches("*.")
                .trim_start_matches('.')
                .to_lowercase();
            if !extension.is_empty() && !kept.contains(&extension) {
                kept.push(extension);
            }
        }
        Self {
            name: name.into(),
            extensions: kept,
        }
    }

    /// Whether the filter lets nothing through: it is then left out.
    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }
}

/// A file, or several, to open.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpenFile {
    pub title: String,
    pub filters: Vec<Filter>,
    /// The folder the dialog starts in.
    pub folder: Option<PathBuf>,
}

impl OpenFile {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn filter(mut self, filter: Filter) -> Self {
        if !filter.is_empty() {
            self.filters.push(filter);
        }
        self
    }

    #[must_use]
    pub fn starting_in(mut self, folder: impl Into<PathBuf>) -> Self {
        self.folder = Some(folder.into());
        self
    }
}

/// A folder to pick.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpenFolder {
    pub title: String,
    pub folder: Option<PathBuf>,
}

impl OpenFolder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            folder: None,
        }
    }

    #[must_use]
    pub fn starting_in(mut self, folder: impl Into<PathBuf>) -> Self {
        self.folder = Some(folder.into());
        self
    }
}

/// A place to save a file.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveFile {
    pub title: String,
    /// The name the dialog suggests, already safe on every system.
    pub name: String,
    pub filters: Vec<Filter>,
    pub folder: Option<PathBuf>,
}

impl SaveFile {
    /// A save dialog that suggests `name`, made safe first
    /// ([`names::suggested`]): a container's name or a path inside a
    /// volume can be handed as it is.
    pub fn new(title: impl Into<String>, name: &str) -> Self {
        Self {
            title: title.into(),
            name: names::suggested(name, "file"),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn filter(mut self, filter: Filter) -> Self {
        if !filter.is_empty() {
            self.filters.push(filter);
        }
        self
    }

    #[must_use]
    pub fn starting_in(mut self, folder: impl Into<PathBuf>) -> Self {
        self.folder = Some(folder.into());
        self
    }
}

/// The desktop has nothing to show a file dialog with, or what it has
/// failed: `cause` is what it said, for the Details.
pub fn unavailable(cause: impl Display) -> AppError {
    AppError::new(ErrorCode::Io, msg::DIALOG_UNAVAILABLE)
        .with_hint(msg::DIALOG_UNAVAILABLE_HINT)
        .with_detail(cause)
}

/// What shows the dialogs: the OS ([`Native`]), or a test's answers.
///
/// Each returns a [`Task`], not a future: the OS's dialog is opened over
/// the app's window, which only the window's own thread can name. The
/// task always ends with one answer: what was picked, nothing when the
/// dialog was cancelled or the window went away, or why no dialog showed.
pub trait Dialogs: Send + Sync {
    /// The files picked, one at most unless `many`; none when cancelled.
    fn open(&self, request: OpenFile, many: bool) -> Task<AppResult<Vec<PathBuf>>>;
    /// The folder picked; `None` when cancelled.
    fn folder(&self, request: OpenFolder) -> Task<AppResult<Option<PathBuf>>>;
    /// Where to save; `None` when cancelled. The OS has already asked
    /// before a file that exists is given.
    fn save(&self, request: SaveFile) -> Task<AppResult<Option<PathBuf>>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extensions_are_read_however_they_are_written() {
        let filter = Filter::new("Archives", &["tar", ".TAR", "*.tar.gz", " tgz ", "", "."]);
        assert_eq!(filter.name, "Archives");
        assert_eq!(filter.extensions, ["tar", "tar.gz", "tgz"]);
        assert_eq!(Filter::new("All files", &["*"]).extensions, ["*"]);
    }

    #[test]
    fn a_filter_that_lets_nothing_through_is_left_out() {
        let empty = Filter::new("Nothing", &["", " . "]);
        assert!(empty.is_empty());
        let yaml = Filter::new("YAML", &["yml", "yaml"]);
        let open = OpenFile::new("Open")
            .filter(empty.clone())
            .filter(yaml.clone());
        assert_eq!(open.filters, std::slice::from_ref(&yaml));
        let save = SaveFile::new("Save", "a.yml")
            .filter(empty)
            .filter(yaml.clone());
        assert_eq!(save.filters, [yaml]);
    }

    #[test]
    fn a_save_dialog_suggests_a_name_every_system_takes() {
        assert_eq!(SaveFile::new("Save", "logs.txt").name, "logs.txt");
        assert_eq!(
            SaveFile::new("Save", "etc/nginx:conf").name,
            "etc_nginx_conf"
        );
        assert_eq!(SaveFile::new("Save", "con.tar").name, "_con.tar");
        assert_eq!(SaveFile::new("Save", "").name, "file");
    }

    #[test]
    fn a_request_keeps_where_it_starts() {
        let open = OpenFile::new("Open").starting_in("/srv/stacks/shop");
        assert_eq!(
            open.folder.as_deref(),
            Some(std::path::Path::new("/srv/stacks/shop"))
        );
        assert_eq!(OpenFolder::new("Pick").title, "Pick");
        assert_eq!(OpenFolder::new("Pick").folder, None);
    }
}
