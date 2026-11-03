//! The app shell's view of errors: everything from [`homeport_core::error`],
//! plus the failures of Tauri itself, which only the shell sees.

pub mod compose;

pub use homeport_core::error::*;

/// The webview, a window or an app path failed.
pub fn window(e: tauri::Error) -> AppError {
    AppError::new(ErrorCode::Tauri, "The app window failed.").with_cause(&e)
}

/// Checking for, downloading or installing an update failed.
pub fn update(e: tauri_plugin_updater::Error) -> AppError {
    use tauri_plugin_updater::Error as U;
    let error = match &e {
            U::Reqwest(_) | U::Network(_) | U::Http(_) => AppError::new(
                ErrorCode::UpdateNetwork,
                "Homeport couldn't download the update.",
            )
            .with_hint("Check your internet connection, then try again."),
            U::ReleaseNotFound | U::TargetNotFound(_) | U::TargetsNotFound(_) => AppError::new(
                ErrorCode::UpdateFailed,
                "No update is published for this system yet.",
            )
            .with_hint("Try again later, or download Homeport from its website."),
            U::Minisign(_) | U::SignedVersionMismatch { .. } => AppError::new(
                ErrorCode::UpdateSignatureInvalid,
                "The update's signature doesn't match, so it wasn't installed.",
            )
            .with_hint("Download the new version from Homeport's website instead."),
            U::AuthenticationFailed => AppError::new(
                ErrorCode::UpdatePermissionDenied,
                "Installing the update needs an administrator's approval, which was refused or cancelled.",
            )
            .with_hint("Try again and enter an administrator password when asked."),
            U::Io(io) if io.kind() == std::io::ErrorKind::PermissionDenied => AppError::new(
                ErrorCode::UpdatePermissionDenied,
                "Homeport isn't allowed to replace its own files.",
            )
            .with_hint("Move the AppImage to a folder you own, or install Homeport from its package repository."),
            U::TempDirNotOnSameMountPoint => AppError::new(
                ErrorCode::UpdatePermissionDenied,
                "Homeport can't replace itself where it's installed.",
            )
            .with_hint("Move Homeport to a folder on your main disk, or install it from its package repository."),
            _ => AppError::new(ErrorCode::UpdateFailed, "The update failed."),
        };
    error.with_cause(&e)
}
