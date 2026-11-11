//! What stopped Homeport from starting, kept for the window to show.
//!
//! When the database can't be opened, nothing that needs it starts: no
//! Hosts, no tray, no polling. The window opens on the failure alone, with
//! the database's path and a way to its folder, instead of the app exiting
//! with its reason only in the log.

use std::path::PathBuf;

use serde::Serialize;

use homeport_core::error::AppError;

/// The database couldn't be opened.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupProblem {
    pub error: AppError,
    /// Where the database is, for the user to find and keep.
    pub database_path: String,
}

/// Kept in every run: `None` once the app has started normally.
#[derive(Debug, Default)]
pub struct Startup(pub Option<StartupProblem>);

impl Startup {
    pub fn failed(error: AppError, database: PathBuf) -> Self {
        Self(Some(StartupProblem {
            error,
            database_path: database.to_string_lossy().into_owned(),
        }))
    }
}
