use serde::Serialize;
use tauri::AppHandle;

/// Mirrors `AppInfo` in `src/features/home/types.ts`.
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
