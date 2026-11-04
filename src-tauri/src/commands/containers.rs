//! Containers on a Host: their lifecycle, and what runs inside one.

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::containers::{
    self,
    logs::{self, LogEvent, LogOptions, LogSource},
    run::{self, RunSpec},
    stats::{self, StatsSample},
    Container, ContainerAction, Processes,
};

use crate::{app::AppState, commands::registries, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn containers_list(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<Container>> {
    containers::list(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    containers::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_act(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    action: ContainerAction,
) -> AppResult<()> {
    containers::act(&state.hosts.registry.docker(&host_id)?, &id, action).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    force: bool,
    volumes: bool,
) -> AppResult<()> {
    containers::remove(&state.hosts.registry.docker(&host_id)?, &id, force, volumes).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_rename(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    name: String,
) -> AppResult<()> {
    containers::rename(&state.hosts.registry.docker(&host_id)?, &id, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_processes(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<Processes> {
    containers::processes(&state.hosts.registry.docker(&host_id)?, &id).await
}

/// Creates and starts a container; returns its id.
#[tauri::command]
#[specta::specta]
pub async fn containers_run(
    state: State<'_, AppState>,
    host_id: String,
    spec: RunSpec,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let image = spec.image.trim().to_owned();
    let auth = registries::auth_for(&state, &image)?;
    run::run(&docker, spec, auth)
        .await
        .map_err(|e| registries::explain(&state, &image, e))
}

/// The Run dialog's form for an existing container, for Duplicate and Edit.
#[tauri::command]
#[specta::specta]
pub async fn containers_run_spec(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<RunSpec> {
    run::spec_for(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn logs_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    sources: Vec<LogSource>,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(logs::stream(docker, sources, options, move |e| {
            on_event.send(e).is_ok()
        })))
}

#[tauri::command]
#[specta::specta]
pub async fn stats_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    on_sample: Channel<StatsSample>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(stats::stream(docker, container_id, move |s| {
            on_sample.send(s).is_ok()
        })))
}
