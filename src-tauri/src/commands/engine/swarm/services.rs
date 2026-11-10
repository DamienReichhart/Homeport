//! A Swarm's services and their tasks.

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::{
    containers::logs::{LogEvent, LogOptions},
    swarm::{
        services::{self, ServiceView},
        spec::{self, ServiceChange, ServiceSpecInput},
        tasks::{self, TaskFilter, TaskView},
    },
};

use crate::{app::AppState, error::AppResult};

/// Every service of a Swarm, with its running and desired tasks.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<ServiceView>> {
    services::list(&state.hosts.registry.manager(&swarm_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_services_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    services::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Sets a replicated service's replicas, from the service as read at `version`.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_scale(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    version: u64,
    replicas: u64,
) -> AppResult<()> {
    services::scale(
        &state.hosts.registry.manager(&swarm_id)?,
        &id,
        version,
        replicas,
    )
    .await
}

/// Puts back a service's previous spec.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_rollback(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::rollback(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_services_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::remove(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Streams the logs of every task of a service, each line labelled with its
/// task and node.
#[tauri::command]
#[specta::specta]
pub async fn swarm_service_logs_subscribe(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.manager(&swarm_id)?;
    Ok(state
        .streams
        .spawn(services::stream_logs(docker, id, options, move |e| {
            on_event.send(e).is_ok()
        })))
}

/// The tasks of a Swarm, of one of its services or of one of its nodes,
/// newest first, each with the Host its node is, if it's one.
#[tauri::command]
#[specta::specta]
pub async fn swarm_tasks_list(
    state: State<'_, AppState>,
    swarm_id: String,
    filter: TaskFilter,
) -> AppResult<Vec<SwarmTask>> {
    let tasks = tasks::list(&state.hosts.registry.manager(&swarm_id)?, &filter).await?;
    let hosts = state.hosts.registry.node_hosts();
    Ok(tasks
        .into_iter()
        .map(|task| SwarmTask {
            host_id: task.node_id.as_ref().and_then(|n| hosts.get(n).cloned()),
            task,
        })
        .collect())
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_tasks_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    tasks::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// A task of a Swarm, and the Host its node is when Homeport connects to it.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmTask {
    #[serde(flatten)]
    pub task: TaskView,
    pub host_id: Option<String>,
}

/// Creates a service from the Create service form, returning its ID.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_create(
    state: State<'_, AppState>,
    swarm_id: String,
    input: ServiceSpecInput,
) -> AppResult<String> {
    spec::create(&state.hosts.registry.manager(&swarm_id)?, &input).await
}

/// Changes a service's image, environment or replicas, from the service as
/// read at `version`; the Swarm rolls its tasks over.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_update(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    version: u64,
    change: ServiceChange,
) -> AppResult<()> {
    spec::update(
        &state.hosts.registry.manager(&swarm_id)?,
        &id,
        version,
        &change,
    )
    .await
}
