//! Registry credentials, kept for every Host (Settings → Registries).

use tauri::State;

use homeport_core::error::AppError;
use homeport_engine::auth::{Logins, RegistryAuth};
use homeport_registries::{
    hints,
    hub::{self, HubTagPage},
    login, Credentials, Registry, RegistryDraft,
};

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

/// A page of a Docker Hub repository's tags, newest first, with the stored
/// Docker Hub login when there is one.
#[tauri::command]
#[specta::specta]
pub async fn registries_hub_tags(
    state: State<'_, AppState>,
    repository: String,
    page: u32,
    filter: String,
) -> AppResult<HubTagPage> {
    let login = state
        .registries
        .credentials(homeport_registries::DOCKER_HUB)
        .unwrap_or_else(|e| {
            e.logged("Docker Hub login for listing tags");
            None
        });
    hub::tags(&repository, page, &filter, login.as_ref()).await
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

/// Every stored login, and the rule that matches them to image references,
/// for runs that pull several images (Stack deploys, builds).
pub(crate) fn logins(state: &AppState) -> AppResult<Logins> {
    Ok(Logins::new(
        state
            .registries
            .all_credentials()?
            .into_iter()
            .map(to_auth)
            .collect(),
        homeport_registries::registry_of,
    ))
}

/// Rewrites a registry refusal for `reference` to point at the login that
/// would fix it.
pub(crate) fn explain(state: &AppState, reference: &str, error: AppError) -> AppError {
    explainer(state, reference)(error)
}

/// [`explainer`] for a push.
pub(crate) fn push_explainer(
    state: &AppState,
    reference: &str,
) -> impl Fn(AppError) -> AppError + Send + Sync + 'static {
    let login = state.registries.login_for(reference).ok().flatten();
    let reference = reference.to_owned();
    move |error| hints::explain_push(error, &reference, login.as_ref())
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
