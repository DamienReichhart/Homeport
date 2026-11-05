//! Stacks on a Host, and deploying Managed Stacks.

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::containers;
use homeport_stacks::{
    self as stacks,
    compose::{self, ComposeAction, ComposeEvent},
    definition::{self, ComposeCheck},
    lifecycle::StackAction,
    runtime::{self, ComposeJob, SourceKind, StackSource, StackTarget},
    store::ManagedStack,
    Stack,
};

use crate::{app::AppState, commands::registries, error::AppResult};

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
    definition::check(&compose, SourceKind::Managed)
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
    runtime::act(&target(&state, &host_id)?, &name, action).await
}

/// Removes a Stack's containers and networks through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn stacks_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    runtime::remove(&target(&state, &host_id)?, &name).await
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
    let job = ComposeJob {
        cli: compose::detect().await?,
        source: StackSource::Managed {
            env: state.stacks.env(&id)?,
            compose: stack.compose,
        },
        target: target(&state, &stack.host_id)?,
        logins: registries::logins(&state)?,
        project: stack.name,
        action,
    };
    Ok(state
        .streams
        .spawn(runtime::run(job, move |e| on_event.send(e).is_ok())))
}

/// Where the Stacks of `host_id` run: its Compose projects.
fn target(state: &AppState, host_id: &str) -> AppResult<StackTarget> {
    let host = state.hosts.store.get(host_id)?;
    Ok(StackTarget::Host {
        name: host.name,
        connection: host.connection,
        dialer: state.hosts.registry.dialer(host_id)?,
        docker: state.hosts.registry.docker(host_id)?,
    })
}
