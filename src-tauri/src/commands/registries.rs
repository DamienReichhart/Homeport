//! Registry credentials, kept for every Host (Settings → Registries).

use tauri::State;

use homeport_registries::{login, Registry, RegistryDraft};

use crate::{app::AppState, error::AppResult};

/// Every stored login, never with its secret.
#[tauri::command]
#[specta::specta]
pub fn registries_list(state: State<'_, AppState>) -> AppResult<Vec<Registry>> {
    state.registries.list()
}

/// Adds a login (`id: null`) or changes one; `draft.secret: null` keeps the stored one.
#[tauri::command]
#[specta::specta]
pub fn registries_save(
    state: State<'_, AppState>,
    id: Option<String>,
    draft: RegistryDraft,
) -> AppResult<Registry> {
    state.registries.save(id.as_deref(), draft)
}

#[tauri::command]
#[specta::specta]
pub fn registries_remove(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.registries.remove(&id)
}

/// Checks a login from this machine before or after it's saved.
#[tauri::command]
#[specta::specta]
pub async fn registries_check(
    state: State<'_, AppState>,
    id: Option<String>,
    draft: RegistryDraft,
) -> AppResult<()> {
    let credentials = state.registries.resolve(id.as_deref(), draft)?;
    login::check(&credentials).await
}

/// The stored login an image reference would be pulled or pushed with, if any.
#[tauri::command]
#[specta::specta]
pub fn registries_login_for(
    state: State<'_, AppState>,
    reference: String,
) -> AppResult<Option<Registry>> {
    state.registries.login_for(&reference)
}
