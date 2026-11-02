//! Application-wide error type. Serialized as `{ code, message }` so the
//! frontend (`src/shared/api/errors.ts`) can handle failures uniformly.

use serde::{ser::SerializeStruct, Serialize, Serializer};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("settings I/O error: {0}")]
    SettingsIo(#[from] std::io::Error),

    #[error("settings are malformed: {0}")]
    SettingsFormat(#[from] serde_json::Error),

    #[error("{0}")]
    Tauri(#[from] tauri::Error),
}

impl AppError {
    /// Stable machine-readable identifier consumed by the frontend.
    pub fn code(&self) -> &'static str {
        match self {
            Self::SettingsIo(_) => "settings_io",
            Self::SettingsFormat(_) => "settings_format",
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
        let err = AppError::SettingsIo(std::io::Error::other("disk full"));
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["code"], "settings_io");
        assert!(json["message"].as_str().unwrap().contains("disk full"));
    }
}
