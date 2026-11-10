//! The user's preferences.

use std::collections::BTreeMap;

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

/// Hands over the active language's templates (`native.*` and `errors.*`,
/// by key) for the text the OS draws: the tray menu and notifications.
/// An empty map means English (ADR 0011).
#[tauri::command]
#[specta::specta]
pub fn set_native_text(app: AppHandle, templates: BTreeMap<String, String>) {
    homeport_core::error::set_translations(templates);
    crate::app::desktop::tray::refresh(&app);
}
