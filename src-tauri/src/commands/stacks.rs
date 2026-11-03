//! Stacks on a Host, and deploying Managed Stacks.

use homeport_core::streams::StreamId;
use homeport_engine::containers;
use homeport_stacks::{
    self as stacks,
    compose::{self, ComposeAction, ComposeEvent, ComposeRun},
    definition::{self, ComposeCheck},
    lifecycle::{self, StackAction},
    store::ManagedStack,
    Stack,
};
use tauri::{ipc::Channel, State};

use crate::{error::AppResult, state::AppState};

/// Every Stack on a Host: those running there plus undeployed Managed Stacks.
#[tauri::command]
#[specta::specta]
pub async fn stacks_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Stack>> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let running = containers::list(&docker).await?;
    let managed: Vec<(String, String)> = state
        .stacks
        .list(&host_id)?
        .into_iter()
        .map(|s| (s.id, s.name))
        .collect();
    Ok(stacks::derive(&running, &managed))
}

#[tauri::command]
#[specta::specta]
pub fn stacks_get(state: State<'_, AppState>, id: String) -> AppResult<ManagedStack> {
    state.stacks.get(&id)
}

/// The decrypted environment of a Managed Stack, for editing.
#[tauri::command]
#[specta::specta]
pub fn stacks_get_env(state: State<'_, AppState>, id: String) -> AppResult<String> {
    state.stacks.env(&id)
}

#[tauri::command]
#[specta::specta]
pub fn stacks_check(compose: String) -> ComposeCheck {
    definition::check(&compose)
}

/// Creates a Managed Stack. Using the name of an External Stack adopts it.
#[tauri::command]
#[specta::specta]
pub fn stacks_create(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    compose: String,
    env: String,
) -> AppResult<ManagedStack> {
    state.hosts.store.get(&host_id)?;
    state.stacks.create(&host_id, &name, &compose, &env)
}

#[tauri::command]
#[specta::specta]
pub fn stacks_update(
    state: State<'_, AppState>,
    id: String,
    compose: String,
    env: String,
) -> AppResult<ManagedStack> {
    state.stacks.update(&id, &compose, &env)
}

/// Forgets a Managed Stack's definition; its containers keep running and it
/// becomes an External Stack.
#[tauri::command]
#[specta::specta]
pub fn stacks_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.stacks.delete(&id)
}

#[tauri::command]
#[specta::specta]
pub async fn stacks_act(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    action: StackAction,
) -> AppResult<()> {
    lifecycle::act(&state.hosts.registry.docker(&host_id)?, &name, action).await
}

/// Removes a Stack's containers and networks through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn stacks_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    lifecycle::remove(&state.hosts.registry.docker(&host_id)?, &name).await
}

/// Runs a compose command for a Managed Stack, streaming its output.
#[tauri::command]
#[specta::specta]
pub async fn stacks_compose_subscribe(
    state: State<'_, AppState>,
    id: String,
    action: ComposeAction,
    on_event: Channel<ComposeEvent>,
) -> AppResult<StreamId> {
    let stack = state.stacks.get(&id)?;
    let host = state.hosts.store.get(&stack.host_id)?;
    let dialer = state.hosts.registry.dialer(&stack.host_id)?;
    let cli = compose::detect().await?;
    let job = ComposeRun {
        cli,
        host_name: host.name,
        connection: host.connection,
        dialer,
        project: stack.name,
        env: state.stacks.env(&id)?,
        compose: stack.compose,
        action,
    };
    Ok(state
        .streams
        .spawn(compose::run(job, move |e| on_event.send(e).is_ok())))
}
