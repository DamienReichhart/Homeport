//! Registry credentials, kept for every Host (Settings → Registries).

use tauri::State;

use homeport_core::error::AppError;
use homeport_engine::auth::RegistryAuth;
use homeport_registries::{hints, login, Credentials, Registry, RegistryDraft};

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

/// The stored login for the registry of `reference`, as the Engine takes it.
pub(crate) fn auth_for(state: &AppState, reference: &str) -> AppResult<Option<RegistryAuth>> {
    Ok(state.registries.credentials_for(reference)?.map(to_auth))
}

/// Rewrites a registry refusal for `reference` to point at the login that
/// would fix it.
pub(crate) fn explain(state: &AppState, reference: &str, error: AppError) -> AppError {
    explainer(state, reference)(error)
}

/// [`explain`] for a stream that outlives the command, which can't keep `state`.
pub(crate) fn explainer(
    state: &AppState,
    reference: &str,
) -> impl Fn(AppError) -> AppError + Send + Sync + 'static {
    let login = state.registries.login_for(reference).ok().flatten();
    let reference = reference.to_owned();
    move |error| hints::explain(error, &reference, login.as_ref())
}

fn to_auth(c: Credentials) -> RegistryAuth {
    RegistryAuth {
        address: c.address,
        username: c.username,
        secret: c.secret,
    }
}
