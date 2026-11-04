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
//!   transport travel inside `io::Error` and are recovered by [`reach::classified_cause`].

mod code;
#[cfg(feature = "bollard")]
pub mod engine;
mod local_file;
pub mod reach;
mod secrets;
mod text;

use std::{error::Error as StdError, fmt::Display};

use serde::Serialize;

pub use code::ErrorCode;
pub use local_file::local_file;
pub use text::{arguments, render, Message, Param, Params, Text};

pub type AppResult<T> = Result<T, AppError>;

/// What the user can do about a failure.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
pub struct Hint {
    /// The catalogue key of `text`; empty for text not in the catalogue.
    pub key: String,
    pub params: Params,
    /// In English, for the log and when the key is unknown.
    pub text: String,
    /// Commands to run, each possibly spanning several lines. May be empty.
    /// Never translated.
    pub commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type, thiserror::Error)]
#[error("{message}")]
pub struct AppError {
    /// What failed, for logic: `e.is(ErrorCode::PortInUse)`.
    pub code: ErrorCode,
    /// The catalogue key of `message` (`errors.*`), which the frontend
    /// renders with `params`; empty for text not in the catalogue.
    pub key: String,
    pub params: Params,
    /// In English, for the app log and when the key is unknown.
    pub message: String,
    // Boxed to keep `AppResult` small.
    pub hint: Option<Box<Hint>>,
    /// The raw cause, for the Details dialog. Never translated.
    pub detail: Option<String>,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<Text>) -> Self {
        let text = message.into();
        Self {
            code,
            message: text.english(),
            key: text.key,
            params: text.params,
            hint: None,
            detail: None,
        }
    }

    /// Bad input from the user; the message alone says what to fix.
    pub fn validation(message: impl Into<Text>) -> Self {
        Self::new(ErrorCode::Validation, message)
    }

    pub fn not_found(message: impl Into<Text>) -> Self {
        Self::new(ErrorCode::NotFound, message)
    }

    pub fn with_hint(mut self, text: impl Into<Text>) -> Self {
        let text = text.into();
        let hint = self.hint.get_or_insert_with(Box::default);
        hint.text = text.english();
        hint.key = text.key;
        hint.params = text.params;
        self
    }

    /// Replaces the message, keeping the code, hint and detail.
    pub fn with_message(mut self, message: impl Into<Text>) -> Self {
        let text = message.into();
        self.message = text.english();
        self.key = text.key;
        self.params = text.params;
        self
    }

    /// Adds a command to the hint; call [`Self::with_hint`] first for its text.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.hint
            .get_or_insert_with(Box::default)
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
                "key": "",
                "params": {},
                "message": "Access refused.",
                "hint": {
                    "key": "",
                    "params": {},
                    "text": "Add your user to the docker group.",
                    "commands": ["sudo usermod -aG docker \"$USER\""],
                },
                "detail": "Permission denied (os error 13)",
            })
        );
    }

    #[allow(unreachable_pub)]
    mod sample {
        crate::messages! {
            BUSY = "errors.test.busy" => "Port {port} is busy.";
            STOP = "errors.test.stop" => "Stop what uses {port}.";
        }
    }

    #[test]
    fn keyed_errors_carry_their_key_and_params() {
        use sample::{ALL, BUSY, STOP};
        let err = AppError::new(ErrorCode::PortInUse, BUSY.with("port", 80))
            .with_hint(STOP.with("port", 80));
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["key"], "errors.test.busy");
        assert_eq!(json["params"]["port"], 80);
        assert_eq!(json["message"], "Port 80 is busy.");
        assert_eq!(json["hint"]["key"], "errors.test.stop");
        assert_eq!(json["hint"]["text"], "Stop what uses 80.");
        assert_eq!(ALL.len(), 2);
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
    fn chains_causes_without_repeating_them() {
        #[derive(Debug, thiserror::Error)]
        #[error("connect failed")]
        struct Outer(#[source] std::io::Error);
        let e = Outer(std::io::Error::other("refused"));
        assert_eq!(chain(&e), "connect failed: refused");
    }
}
