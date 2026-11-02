//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use crate::services::settings::SettingsService;

pub struct AppState {
    pub settings: SettingsService,
}
