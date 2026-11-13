//! The folders the app keeps on this computer, where the Tauri app kept
//! them: an install that moves to the iced app finds its database and its
//! logs where they were.

use std::path::PathBuf;

use crate::app::setup::IDENTIFIER;

/// Where the app's log files are: `app_log_dir` of the Tauri app.
/// `~/Library/Logs/com.homeport.app` on macOS, `com.homeport.app/logs`
/// under the local data folder elsewhere (`~/.local/share`,
/// `%LOCALAPPDATA%`). The logging layer (#254) writes there.
pub fn log_dir() -> PathBuf {
    if cfg!(target_os = "macos") {
        dirs::home_dir()
            .unwrap_or_default()
            .join("Library/Logs")
            .join(IDENTIFIER)
    } else {
        dirs::data_local_dir()
            .unwrap_or_default()
            .join(IDENTIFIER)
            .join("logs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_logs_are_where_the_tauri_app_wrote_them() {
        let dir = log_dir();
        let expected = if cfg!(target_os = "macos") {
            "Library/Logs/com.homeport.app"
        } else {
            "com.homeport.app/logs"
        };
        assert!(dir.ends_with(expected), "{}", dir.display());
        assert!(dir.is_absolute(), "{}", dir.display());
    }
}
