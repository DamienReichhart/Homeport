use serde::Serialize;
use tauri::{AppHandle, State};

use crate::{
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
