//! The app log: what Homeport writes, and where (`open_log_folder` shows it).

use tauri::{plugin::TauriPlugin, Runtime};

/// Size at which the log file is rotated.
const LOG_FILE_BYTES: u128 = 5 * 1024 * 1024;
/// Rotated log files kept next to the current one.
const LOG_FILES_KEPT: usize = 5;

pub fn plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::default()
        .level(log::LevelFilter::Info)
        // Keep up to 5 files of 5 MB each, named by local time.
        .max_file_size(LOG_FILE_BYTES)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(LOG_FILES_KEPT))
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        // bollard and hyper log every request at debug/trace level.
        .level_for("bollard", log::LevelFilter::Warn)
        .level_for("hyper_util", log::LevelFilter::Warn)
        .level_for("russh", log::LevelFilter::Warn)
        .build()
}
