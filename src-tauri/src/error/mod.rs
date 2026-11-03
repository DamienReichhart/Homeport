//! Application-wide error type, serialized as `{ code, message, hint, detail }`
//! so the frontend (`src/shared/api/errors.ts`) can show every failure the same
//! way: what happened, what to do about it, and the raw details on demand.
//!
//! Writing an error:
//! - `message` says what happened in one sentence, without raw OS, library or
//!   Engine text. A Host is named by its name (remote ones as "prod (address)");
//!   "Docker Engine" means the software, and the machine is never "the server"
//!   (see `GLOSSARY.md`). British spelling, as `docs/BRAND.md` asks.
//! - `hint` says what to do, with copyable `commands` for the machine concerned;
//!   when there's nothing better, point to Details.
//! - `detail` keeps the raw cause (error chain, Engine response, stderr) for the
//!   Details dialog and bug reports.
//! - Classify in the backend: the frontend only displays errors. Failures from a
//!   transport travel inside `io::Error` and are recovered by `reach::classified_cause`.

pub mod compose;
pub mod engine;
pub mod reach;

use std::{error::Error as StdError, fmt::Display};

use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

/// Stable machine-readable identifier of a failure, one per case the user can act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// A failure that started in the frontend, outside any command.
    #[allow(dead_code)] // only the frontend produces it
    Unknown,
    // The app itself.
    Io,
    Json,
    Db,
    DbLocked,
    DbCorrupt,
    DbMigration,
    Tauri,
    Validation,
    NotFound,
    KeychainUnavailable,
    CredentialsLost,
    UpdateNetwork,
    UpdateSignatureInvalid,
    UpdatePermissionDenied,
    UpdateFailed,
    // Reaching a Host.
    HostNotConnected,
    EngineNotInstalled,
    EngineNotRunning,
    PermissionDenied,
    DnsFailed,
    ConnectionRefused,
    Timeout,
    NetworkUnreachable,
    ConnectionLost,
    TlsRequired,
    TlsNotSupported,
    TlsCertificateExpired,
    TlsCertificateNotYetValid,
    TlsUnknownCa,
    TlsNameMismatch,
    TlsClientRejected,
    TlsHandshakeFailed,
    TlsMaterialInvalid,
    SshAuthFailed,
    SshAgentUnavailable,
    SshKeyInvalid,
    SshFailed,
    RemoteEngineUnavailable,
    HostKeyUntrusted,
    HostKeyChanged,
    // Engine API responses.
    EngineError,
    BadRequest,
    Forbidden,
    Conflict,
    NameInUse,
    InUse,
    ContainerRunning,
    ContainerNotRunning,
    AlreadyDone,
    PortInUse,
    DiskFull,
    ImageNotFound,
    RegistryAuthRequired,
    RegistryRateLimited,
    RegistryUnreachable,
    ApiVersionUnsupported,
    // Compose.
    ComposeMissing,
    ComposeOutdated,
    ComposeSpawnFailed,
    ComposeInvalidDefinition,
    ComposeMissingVariable,
    ComposeFailed,
    // Files and exec inside containers.
    NoShell,
    PathNotFound,
    ContainerPermissionDenied,
    FileTooLarge,
}

/// What the user can do about a failure.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
pub struct Hint {
    pub text: String,
    /// Commands to run, each possibly spanning several lines. May be empty.
    pub commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type, thiserror::Error)]
#[error("{message}")]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    pub hint: Option<Hint>,
    /// The raw cause, for the Details dialog.
    pub detail: Option<String>,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            hint: None,
            detail: None,
        }
    }

    /// Bad input from the user; the message alone says what to fix.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Validation, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message)
    }

    pub fn with_hint(mut self, text: impl Into<String>) -> Self {
        self.hint.get_or_insert_with(Hint::default).text = text.into();
        self
    }

    /// Adds a command to the hint; call [`Self::with_hint`] first for its text.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.hint
            .get_or_insert_with(Hint::default)
            .commands
            .push(command.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Display) -> Self {
        self.detail = Some(detail.to_string());
        self
    }

    /// Uses `error` and every error it wraps as the detail.
    pub fn with_cause(self, error: &(dyn StdError + 'static)) -> Self {
        self.with_detail(chain(error))
    }

    pub fn is(&self, code: ErrorCode) -> bool {
        self.code == code
    }

    /// Writes the error to the app log, for failures that never reach a command's result.
    pub fn logged(self, context: &str) -> Self {
        log::warn!(
            "{context}: [{:?}] {}{}",
            self.code,
            self.message,
            self.detail
                .as_deref()
                .map(|d| format!(" ({d})"))
                .unwrap_or_default()
        );
        self
    }
}

/// `error: cause: cause…`, skipping causes already quoted by their parent.
pub fn chain(error: &(dyn StdError + 'static)) -> String {
    let mut text = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        let part = cause.to_string();
        if !text.contains(&part) {
            text.push_str(": ");
            text.push_str(&part);
        }
        source = cause.source();
    }
    text
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorCode::Io, "A file or system operation failed.").with_cause(&e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::new(ErrorCode::Json, "Homeport received data it couldn't read.").with_cause(&e)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        use rusqlite::ffi;
        let sqlite = match &e {
            rusqlite::Error::SqliteFailure(f, _) => Some(f.extended_code),
            _ => None,
        };
        let error = match sqlite {
            // Callers replace these with a message naming what clashed.
            Some(ffi::SQLITE_CONSTRAINT_UNIQUE) => {
                Self::new(ErrorCode::NameInUse, "That name is already used.")
            }
            Some(ffi::SQLITE_CONSTRAINT_FOREIGNKEY) => {
                Self::not_found("Something this refers to no longer exists.")
            }
            Some(code) if matches!(code & 0xff, ffi::SQLITE_BUSY | ffi::SQLITE_LOCKED) => {
                Self::new(ErrorCode::DbLocked, "Homeport's database is busy.")
                    .with_hint("Another Homeport may be running. Close it and try again.")
            }
            Some(code) if matches!(code & 0xff, ffi::SQLITE_CORRUPT | ffi::SQLITE_NOTADB) => {
                Self::new(ErrorCode::DbCorrupt, "Homeport's database is damaged.").with_hint(
                    "Quit Homeport, move homeport.db out of its data folder and start again. Hosts and stacks will need to be added again.",
                )
            }
            Some(code) if code & 0xff == ffi::SQLITE_FULL => {
                Self::new(ErrorCode::DiskFull, "The disk is full, so Homeport can't save.")
                    .with_hint("Free some disk space and try again.")
            }
            _ => Self::new(ErrorCode::Db, "Homeport's database failed."),
        };
        error.with_cause(&e)
    }
}

impl From<rusqlite_migration::Error> for AppError {
    fn from(e: rusqlite_migration::Error) -> Self {
        Self::new(
            ErrorCode::DbMigration,
            "Homeport couldn't upgrade its database.",
        )
        .with_cause(&e)
    }
}

impl AppError {
    pub fn credentials_lost() -> Self {
        Self::new(
            ErrorCode::CredentialsLost,
            "Stored credentials can't be decrypted.",
        )
        .with_hint("The key that protects them is gone from the OS keychain. Re-enter the credentials on the host page, or regenerate the key.")
    }

    pub fn keychain_unavailable(reason: impl Display) -> Self {
        let hint = if cfg!(target_os = "linux") {
            "Install and unlock a Secret Service keyring, such as GNOME Keyring or KWallet, then restart Homeport."
        } else if cfg!(target_os = "macos") {
            "Unlock the login keychain in Keychain Access, then restart Homeport."
        } else {
            "Make sure Windows Credential Manager is available, then restart Homeport."
        };
        Self::new(
            ErrorCode::KeychainUnavailable,
            "The OS keychain is unavailable, so secrets can't be stored.",
        )
        .with_hint(hint)
        .with_detail(reason)
    }
}

impl From<tauri::Error> for AppError {
    fn from(e: tauri::Error) -> Self {
        Self::new(ErrorCode::Tauri, "The app window failed.").with_cause(&e)
    }
}

impl From<bollard::errors::Error> for AppError {
    fn from(e: bollard::errors::Error) -> Self {
        engine::from_bollard(&e)
    }
}

impl From<tauri_plugin_updater::Error> for AppError {
    fn from(e: tauri_plugin_updater::Error) -> Self {
        use tauri_plugin_updater::Error as U;
        let error = match &e {
            U::Reqwest(_) | U::Network(_) | U::Http(_) => Self::new(
                ErrorCode::UpdateNetwork,
                "Homeport couldn't download the update.",
            )
            .with_hint("Check your internet connection, then try again."),
            U::ReleaseNotFound | U::TargetNotFound(_) | U::TargetsNotFound(_) => Self::new(
                ErrorCode::UpdateFailed,
                "No update is published for this system yet.",
            )
            .with_hint("Try again later, or download Homeport from its website."),
            U::Minisign(_) | U::SignedVersionMismatch { .. } => Self::new(
                ErrorCode::UpdateSignatureInvalid,
                "The update's signature doesn't match, so it wasn't installed.",
            )
            .with_hint("Download the new version from Homeport's website instead."),
            U::AuthenticationFailed => Self::new(
                ErrorCode::UpdatePermissionDenied,
                "Installing the update needs an administrator's approval, which was refused or cancelled.",
            )
            .with_hint("Try again and enter an administrator password when asked."),
            U::Io(io) if io.kind() == std::io::ErrorKind::PermissionDenied => Self::new(
                ErrorCode::UpdatePermissionDenied,
                "Homeport isn't allowed to replace its own files.",
            )
            .with_hint("Move the AppImage to a folder you own, or install Homeport from its package repository."),
            U::TempDirNotOnSameMountPoint => Self::new(
                ErrorCode::UpdatePermissionDenied,
                "Homeport can't replace itself where it's installed.",
            )
            .with_hint("Move Homeport to a folder on your main disk, or install it from its package repository."),
            _ => Self::new(ErrorCode::UpdateFailed, "The update failed."),
        };
        error.with_cause(&e)
    }
}

/// A file on this computer that the user picked (keys, compose files, downloads).
pub fn local_file(path: &std::path::Path, e: std::io::Error) -> AppError {
    let path = path.display();
    let error = match e.kind() {
        std::io::ErrorKind::NotFound => {
            AppError::new(ErrorCode::Io, format!("{path} doesn't exist."))
        }
        std::io::ErrorKind::PermissionDenied => AppError::new(
            ErrorCode::PermissionDenied,
            format!("Homeport isn't allowed to access {path}."),
        )
        .with_hint("Check the file's permissions, or pick another location."),
        std::io::ErrorKind::StorageFull => AppError::new(
            ErrorCode::DiskFull,
            format!("There's no space left to write {path}."),
        )
        .with_hint("Free some disk space, or pick another drive."),
        std::io::ErrorKind::IsADirectory => {
            AppError::validation(format!("{path} is a folder, not a file."))
        }
        std::io::ErrorKind::InvalidData => {
            AppError::validation(format!("{path} isn't a text file."))
        }
        _ => AppError::new(ErrorCode::Io, format!("Homeport couldn't access {path}.")),
    };
    error.with_cause(&e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_code_message_hint_and_detail() {
        let err = AppError::new(ErrorCode::PermissionDenied, "Access refused.")
            .with_hint("Add your user to the docker group.")
            .with_command("sudo usermod -aG docker \"$USER\"")
            .with_detail("Permission denied (os error 13)");
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({
                "code": "permission_denied",
                "message": "Access refused.",
                "hint": {
                    "text": "Add your user to the docker group.",
                    "commands": ["sudo usermod -aG docker \"$USER\""],
                },
                "detail": "Permission denied (os error 13)",
            })
        );
    }

    #[test]
    fn serializes_missing_hint_and_detail_as_null() {
        let json = serde_json::to_value(AppError::validation("Enter a name.")).unwrap();
        assert_eq!(json["code"], "validation");
        assert_eq!(json["hint"], serde_json::Value::Null);
        assert_eq!(json["detail"], serde_json::Value::Null);
    }

    #[test]
    fn io_errors_keep_the_os_text_as_detail() {
        let err = AppError::from(std::io::Error::other("disk full"));
        assert_eq!(err.code, ErrorCode::Io);
        assert!(!err.message.contains("disk full"));
        assert_eq!(err.detail.as_deref(), Some("disk full"));
    }

    #[test]
    fn local_files_name_the_path_and_the_problem() {
        let path = std::path::Path::new("/home/me/key.pem");
        let e = local_file(
            path,
            std::io::Error::from(std::io::ErrorKind::PermissionDenied),
        );
        assert_eq!(e.code, ErrorCode::PermissionDenied);
        assert_eq!(
            e.message,
            "Homeport isn't allowed to access /home/me/key.pem."
        );
        let e = local_file(path, std::io::Error::from(std::io::ErrorKind::NotFound));
        assert_eq!(e.message, "/home/me/key.pem doesn't exist.");
    }

    #[test]
    fn chains_causes_without_repeating_them() {
        #[derive(Debug, thiserror::Error)]
        #[error("connect failed")]
        struct Outer(#[source] std::io::Error);
        let e = Outer(std::io::Error::other("refused"));
        assert_eq!(chain(&e), "connect failed: refused");
    }
}
