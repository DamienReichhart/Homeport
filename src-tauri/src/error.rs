//! Application-wide error type. Serialized as `{ code, message }` so the
//! frontend (`src/shared/api/errors.ts`) can handle failures uniformly.

use serde::{ser::SerializeStruct, Serialize, Serializer};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("malformed JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("database migration failed: {0}")]
    Migration(#[from] rusqlite_migration::Error),

    #[allow(dead_code)] // raised by the host and stack stores
    #[error("the OS keychain is unavailable, so secrets can't be stored: {0}")]
    KeychainUnavailable(String),

    #[allow(dead_code)]
    #[error("stored credentials can't be decrypted; re-enter them or regenerate the key")]
    CredentialsLost,

    #[error("{0}")]
    Tauri(#[from] tauri::Error),
}

impl AppError {
    /// Stable machine-readable identifier consumed by the frontend.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Io(_) => "io",
            Self::Json(_) => "json",
            Self::Db(_) => "db",
            Self::Migration(_) => "db_migration",
            Self::KeychainUnavailable(_) => "keychain_unavailable",
            Self::CredentialsLost => "credentials_lost",
            Self::Tauri(_) => "tauri",
        }
    }
}

/// Wire shape of [`AppError`]; mirrors `AppErrorPayload` in `src/shared/api/errors.ts`.
#[derive(Serialize, specta::Type)]
#[serde(rename = "AppError")]
#[allow(dead_code)]
struct AppErrorPayload {
    code: String,
    message: String,
}

impl specta::Type for AppError {
    fn definition(types: &mut specta::Types) -> specta::datatype::DataType {
        AppErrorPayload::definition(types)
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("AppError", 2)?;
        s.serialize_field("code", self.code())?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_code_and_message() {
        let err = AppError::Io(std::io::Error::other("disk full"));
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["code"], "io");
        assert!(json["message"].as_str().unwrap().contains("disk full"));
    }
}
