//! The folders the app keeps on this computer, where the Tauri app kept
//! them: an install that moves to the iced app finds its database and its
//! logs where they were.

use std::path::{Path, PathBuf};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    messages as msg,
};

use super::opener::Os;
use crate::app::setup::IDENTIFIER;

/// Where the app's log files are: `app_log_dir` of the Tauri app. The
/// logging layer (#254) writes there. A computer that names no home is
/// a failure, as it was for the Tauri app: never a folder made wherever
/// the app happened to be started from.
pub fn log_dir() -> AppResult<PathBuf> {
    log_dir_on(
        Os::current(),
        dirs::home_dir().as_deref(),
        dirs::data_local_dir().as_deref(),
    )
}

/// [`log_dir`] on `os`, given what it says of this user's folders:
/// `Library/Logs/com.homeport.app` under `home` on macOS, and
/// `com.homeport.app/logs` under the local data folder elsewhere
/// (`~/.local/share` or `$XDG_DATA_HOME`, `%LOCALAPPDATA%`).
pub fn log_dir_on(os: Os, home: Option<&Path>, local_data: Option<&Path>) -> AppResult<PathBuf> {
    let (under, logs) = match os {
        Os::Macos => (home, Path::new("Library/Logs").join(IDENTIFIER)),
        Os::Linux | Os::Windows => (local_data, Path::new(IDENTIFIER).join("logs")),
    };
    match under {
        // A folder from "here" is no user's folder.
        Some(under) if under.is_absolute() => Ok(under.join(logs)),
        _ => Err(AppError::new(ErrorCode::Io, msg::NO_APP_FOLDER)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A system, its home, its local data folder, and where the logs are.
    type Case = (
        Os,
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
    );

    #[test]
    fn the_logs_are_where_the_tauri_app_wrote_them_on_each_system() {
        let cases: [Case; 9] = [
            (
                Os::Linux,
                Some("/home/ada"),
                Some("/home/ada/.local/share"),
                Some("/home/ada/.local/share/com.homeport.app/logs"),
            ),
            // `$XDG_DATA_HOME` set: the OS's answer is the one that counts.
            (
                Os::Linux,
                Some("/home/ada"),
                Some("/data/ada"),
                Some("/data/ada/com.homeport.app/logs"),
            ),
            (
                Os::Macos,
                Some("/Users/ada"),
                Some("/Users/ada/Library/Application Support"),
                Some("/Users/ada/Library/Logs/com.homeport.app"),
            ),
            (
                Os::Windows,
                None,
                Some("/c/Users/ada/AppData/Local"),
                Some("/c/Users/ada/AppData/Local/com.homeport.app/logs"),
            ),
            // No home, no folder: never a path from wherever the app runs.
            (Os::Linux, Some("/home/ada"), None, None),
            (Os::Linux, None, None, None),
            (
                Os::Macos,
                None,
                Some("/Users/ada/Library/Application Support"),
                None,
            ),
            (Os::Windows, Some("/c/Users/ada"), None, None),
            (Os::Linux, Some("/home/ada"), Some("share"), None),
        ];
        for (os, home, local_data, expected) in cases {
            let found = log_dir_on(os, home.map(Path::new), local_data.map(Path::new));
            match expected {
                Some(expected) => assert_eq!(found.as_deref(), Ok(Path::new(expected)), "{os:?}"),
                None => {
                    let failed = found.unwrap_err();
                    assert_eq!(failed.key, "errors.file.noAppFolder", "{os:?} {home:?}");
                }
            }
        }
    }

    #[test]
    fn this_computer_s_log_folder_is_whole_or_a_failure() {
        // Whatever this machine says: never a relative path.
        if let Ok(dir) = log_dir() {
            assert!(dir.is_absolute(), "{}", dir.display());
            assert!(dir.to_string_lossy().contains("com.homeport.app"));
        }
    }
}
