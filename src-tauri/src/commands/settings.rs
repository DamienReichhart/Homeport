//! The user's preferences.

use tauri::{AppHandle, State};

use homeport_settings::Settings;

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub fn get_settings(state: State<'_, AppState>) -> AppResult<Settings> {
    state.settings.get()
}

#[tauri::command]
#[specta::specta]
pub fn update_settings(
    app: AppHandle,
    state: State<'_, AppState>,
    settings: Settings,
) -> AppResult<Settings> {
    let previous = state.settings.get()?;
    let updated = state.settings.update(settings)?;
    if updated.keep_events_days < previous.keep_events_days {
        crate::app::history::prune(&app);
    }
    Ok(updated)
}
