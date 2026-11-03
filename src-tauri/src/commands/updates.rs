//! Checks for a new version and, where Homeport can, installs it. The manifest
//! (`updates/latest.json` on the website) and the public key that every update
//! must be signed with are set in `tauri.conf.json > plugins > updater`.

use serde::Serialize;
use tauri::{ipc::Channel, AppHandle, State};
use tauri_plugin_updater::UpdaterExt;

use crate::{
    error::{self, AppError, AppResult},
    services::updates::{update_method, Bundle, UpdateMethod},
    state::AppState,
};

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheck {
    current_version: String,
    /// The newer version, if there is one.
    available: Option<AvailableUpdate>,
    method: UpdateMethod,
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AvailableUpdate {
    version: String,
    /// Markdown release notes.
    notes: Option<String>,
    /// RFC 3339 publication date.
    date: Option<String>,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgress {
    downloaded: u64,
    /// Total size, when the server sends it.
    total: Option<u64>,
}

fn method() -> UpdateMethod {
    use tauri::utils::{config::BundleType, platform::bundle_type};
    let bundle = match bundle_type() {
        Some(BundleType::AppImage) => Bundle::AppImage,
        Some(BundleType::Deb) => Bundle::Deb,
        Some(BundleType::Rpm) => Bundle::Rpm,
        Some(BundleType::Msi) => Bundle::Msi,
        Some(BundleType::Nsis) => Bundle::Nsis,
        // macOS reports `App` for every build, release or not.
        Some(BundleType::App) if !cfg!(debug_assertions) => Bundle::App,
        _ => Bundle::Unknown,
    };
    let os_release = std::fs::read_to_string("/etc/os-release").ok();
    update_method(bundle, os_release.as_deref())
}

/// Asks the update server for a newer version and remembers it for [`updates_install`].
#[tauri::command]
#[specta::specta]
pub async fn updates_check(app: AppHandle, state: State<'_, AppState>) -> AppResult<UpdateCheck> {
    let update = app
        .updater()
        .map_err(error::update)?
        .check()
        .await
        .map_err(error::update)?;
    let available = update.as_ref().map(|u| AvailableUpdate {
        version: u.version.clone(),
        notes: u.body.clone(),
        date: u.raw_json["pub_date"].as_str().map(str::to_owned),
    });
    *state.update.lock().unwrap() = update;
    Ok(UpdateCheck {
        current_version: app.package_info().version.to_string(),
        available,
        method: method(),
    })
}

/// Downloads the update found by the last [`updates_check`], verifies its
/// signature, installs it and restarts Homeport. Only for installs Homeport can
/// update itself; package-managed installs are refused.
#[tauri::command]
#[specta::specta]
pub async fn updates_install(
    app: AppHandle,
    state: State<'_, AppState>,
    on_progress: Channel<UpdateProgress>,
) -> AppResult<()> {
    if method() != UpdateMethod::InApp {
        return Err(AppError::validation(
            "This copy of Homeport is updated by your package manager.",
        ));
    }
    let Some(update) = state.update.lock().unwrap().clone() else {
        return Err(AppError::not_found("No update is waiting to be installed."));
    };
    let mut downloaded = 0u64;
    update
        .download_and_install(
            |chunk, total| {
                downloaded += chunk as u64;
                let _ = on_progress.send(UpdateProgress { downloaded, total });
            },
            || {},
        )
        .await
        .map_err(error::update)?;
    log::info!("installed Homeport {}, restarting", update.version);
    app.restart()
}
