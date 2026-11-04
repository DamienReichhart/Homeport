//! The app shell's view of errors: everything from [`homeport_core::error`],
//! plus the failures of Tauri itself, which only the shell sees.

pub use homeport_core::error::*;

use crate::messages as msg;

/// The webview, a window or an app path failed.
pub fn window(e: tauri::Error) -> AppError {
    AppError::new(ErrorCode::Tauri, msg::WINDOW_FAILED).with_cause(&e)
}

/// Checking for, downloading or installing an update failed.
pub fn update(e: tauri_plugin_updater::Error) -> AppError {
    use tauri_plugin_updater::Error as U;
    let error = match &e {
        U::Reqwest(_) | U::Network(_) | U::Http(_) => {
            AppError::new(ErrorCode::UpdateNetwork, msg::UPDATE_DOWNLOAD)
                .with_hint(msg::UPDATE_DOWNLOAD_HINT)
        }
        U::ReleaseNotFound | U::TargetNotFound(_) | U::TargetsNotFound(_) => {
            AppError::new(ErrorCode::UpdateFailed, msg::UPDATE_NONE_PUBLISHED)
                .with_hint(msg::UPDATE_NONE_PUBLISHED_HINT)
        }
        U::Minisign(_) | U::SignedVersionMismatch { .. } => {
            AppError::new(ErrorCode::UpdateSignatureInvalid, msg::UPDATE_SIGNATURE)
                .with_hint(msg::UPDATE_SIGNATURE_HINT)
        }
        U::AuthenticationFailed => {
            AppError::new(ErrorCode::UpdatePermissionDenied, msg::UPDATE_ADMIN_REFUSED)
                .with_hint(msg::UPDATE_ADMIN_REFUSED_HINT)
        }
        U::Io(io) if io.kind() == std::io::ErrorKind::PermissionDenied => {
            AppError::new(ErrorCode::UpdatePermissionDenied, msg::UPDATE_NOT_ALLOWED)
                .with_hint(msg::UPDATE_NOT_ALLOWED_HINT)
        }
        U::TempDirNotOnSameMountPoint => {
            AppError::new(ErrorCode::UpdatePermissionDenied, msg::UPDATE_WRONG_DISK)
                .with_hint(msg::UPDATE_WRONG_DISK_HINT)
        }
        _ => AppError::new(ErrorCode::UpdateFailed, msg::UPDATE_FAILED),
    };
    error.with_cause(&e)
}
