use homeport_settings::Settings;
use tauri::State;

use crate::{error::AppResult, state::AppState};

#[tauri::command]
#[specta::specta]
pub fn get_settings(state: State<'_, AppState>) -> AppResult<Settings> {
    state.settings.get()
}

#[tauri::command]
#[specta::specta]
pub fn update_settings(state: State<'_, AppState>, settings: Settings) -> AppResult<Settings> {
    state.settings.update(settings)
}
