use serde::Serialize;
use tauri::{AppHandle, State};

use crate::{
    error::{AppError, AppResult},
    services::stacks::compose::{self, ComposeCli},
    state::AppState,
};

#[derive(Debug, Serialize, specta::Type)]
pub struct AppInfo {
    name: String,
    version: String,
    os: &'static str,
    arch: &'static str,
}

#[tauri::command]
#[specta::specta]
pub fn get_app_info(app: AppHandle) -> AppInfo {
    let info = app.package_info();
    AppInfo {
        name: info.name.clone(),
        version: info.version.to_string(),
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
    }
}

/// What this machine can do; features degrade gracefully when something is missing.
#[derive(Debug, Serialize, specta::Type)]
pub struct Capabilities {
    /// An OS keychain is reachable, so SSH, TLS and stack `.env` secrets can be stored.
    keychain: bool,
    /// The compose CLI used to deploy Managed Stacks, if installed.
    compose: Option<ComposeCli>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_capabilities(state: State<'_, AppState>) -> Result<Capabilities, ()> {
    Ok(Capabilities {
        keychain: state.secrets.is_available(),
        compose: compose::detect().await,
    })
}

/// Largest file `read_text_file` accepts; certificates and keys are far smaller.
const MAX_TEXT_FILE: u64 = 1024 * 1024;

/// Reads a file the user picked in an open dialog (certificates, keys, compose files).
#[tauri::command]
#[specta::specta]
pub fn read_text_file(path: String) -> AppResult<String> {
    if std::fs::metadata(&path)?.len() > MAX_TEXT_FILE {
        return Err(AppError::Validation("That file is too large.".into()));
    }
    Ok(std::fs::read_to_string(path)?)
}

/// Writes text to a path the user picked in a save dialog (e.g. downloaded logs).
#[tauri::command]
#[specta::specta]
pub fn write_text_file(path: String, contents: String) -> AppResult<()> {
    Ok(std::fs::write(path, contents)?)
}
