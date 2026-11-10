//! Git repositories for Synced Stacks, kept for every Host
//! (Settings → Repositories, ADR 0014).

use tauri::State;

use homeport_git::{cli, RemoteRefs, Repository, RepositoryDraft};

use crate::{app::AppState, error::AppResult};

/// Every stored repository, never with its key or token.
#[tauri::command]
#[specta::specta]
pub fn repositories_list(state: State<'_, AppState>) -> AppResult<Vec<Repository>> {
    state.repositories.store.list()
}

/// Adds a repository, generating its deploy key when it uses one.
#[tauri::command]
#[specta::specta]
pub fn repositories_create(
    state: State<'_, AppState>,
    draft: RepositoryDraft,
) -> AppResult<Repository> {
    state.repositories.store.create(&draft)
}

/// Replaces a repository's deploy key with a new one.
#[tauri::command]
#[specta::specta]
pub fn repositories_rotate_key(state: State<'_, AppState>, id: String) -> AppResult<Repository> {
    state.repositories.store.rotate_key(&id)
}

/// Replaces a repository's user name and token.
#[tauri::command]
#[specta::specta]
pub fn repositories_set_token(
    state: State<'_, AppState>,
    id: String,
    username: String,
    token: String,
) -> AppResult<Repository> {
    state.repositories.store.set_token(&id, &username, &token)
}

/// Forgets the SSH server key trusted for a repository, so the next
/// command trusts the one the server presents.
#[tauri::command]
#[specta::specta]
pub fn repositories_reset_host_key(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Repository> {
    state.repositories.store.reset_host_key(&id)
}

/// Removes a repository and its clone; refused while a Synced Stack uses it.
#[tauri::command]
#[specta::specta]
pub fn repositories_remove(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.repositories.delete(&id)
}

/// "Test access": the repository's branches and tags, as this computer sees them.
#[tauri::command]
#[specta::specta]
pub async fn repositories_test(state: State<'_, AppState>, id: String) -> AppResult<RemoteRefs> {
    let git = cli::detect().await?;
    state.repositories.test(&git, &id).await
}
