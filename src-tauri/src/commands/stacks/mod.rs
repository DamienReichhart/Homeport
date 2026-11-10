//! Stacks on a Host: finding them, Managed, Linked and Synced Stacks'
//! definitions, deploying through the runtime, their deploy history and what
//! a redeploy would change.

pub mod deploy;
pub mod linked;
pub mod managed;
pub mod repositories;
pub mod synced;
pub(crate) mod target;

use tauri::State;

use homeport_engine::containers;
use homeport_stacks::{
    self as stacks,
    definition::{self, ComposeCheck},
    lifecycle::StackAction,
    runtime::{self, SourceKind},
    Stack, StackOwner,
};

use crate::{app::AppState, error::AppResult};

/// Every Stack on a Host: those running there plus undeployed Managed,
/// Linked and Synced Stacks. Stacks started from compose files on this machine are
/// recorded as Linked the first time they're seen.
#[tauri::command]
#[specta::specta]
pub async fn stacks_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Stack>> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let running = containers::list(&docker).await?;
    stacks::list(
        &host_id,
        &running,
        &state.stacks.managed,
        &state.stacks.linked,
        &state.stacks.synced.store,
        &state.stacks.linked_files,
    )
}

/// Checks a definition made of `files` (their YAML, merged in order) against
/// the rules of its kind of Stack, and of a Swarm's when `swarm`.
#[tauri::command]
#[specta::specta]
pub fn stacks_check(files: Vec<String>, kind: SourceKind, swarm: bool) -> ComposeCheck {
    let files: Vec<&str> = files.iter().map(String::as_str).collect();
    definition::check_for(&files, kind, swarm)
}

/// Whether deploying a Stack to a Swarm sends the docker CLI's own registry
/// logins (`--with-registry-auth`).
#[tauri::command]
#[specta::specta]
pub fn stacks_set_registry_auth(
    state: State<'_, AppState>,
    kind: SourceKind,
    id: String,
    on: bool,
) -> AppResult<bool> {
    state.stacks.set_registry_auth(kind, &id, on)
}

/// Sets the Compose profiles a Managed, Linked or Synced Stack deploys; they
/// apply from its next compose command.
#[tauri::command]
#[specta::specta]
pub fn stacks_set_profiles(
    state: State<'_, AppState>,
    kind: SourceKind,
    id: String,
    profiles: Vec<String>,
) -> AppResult<Vec<String>> {
    state.stacks.set_profiles(kind, &id, &profiles)
}

#[tauri::command]
#[specta::specta]
pub async fn stacks_act(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    action: StackAction,
) -> AppResult<()> {
    runtime::act(
        &target::of(&state, &StackOwner::from(&host_id))?,
        &name,
        action,
    )
    .await
}

/// Removes a Stack's containers and networks through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn stacks_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    runtime::remove(&target::of(&state, &StackOwner::from(&host_id))?, &name).await
}
