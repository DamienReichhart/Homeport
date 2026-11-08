//! What containerd Hosts offer, through containerd's gRPC API (ADR 0020).
//! Every command but the namespaces' own names the namespace it's about;
//! a Host whose Engine isn't containerd answers `EngineUnsupported`.

use tauri::{ipc::Channel, State};

use homeport_containerd::{
    containers::{self, ContainerdContainer, ContainerdContainerDetail, TaskAction},
    logs::{self, ContainerdLogSource},
    namespaces::{self, ContainerdNamespace},
};
use homeport_core::{
    logs::{LogEvent, LogOptions},
    streams::StreamId,
};

use crate::{app::AppState, error::AppResult};

/// Every namespace of the Host, with what each holds.
#[tauri::command]
#[specta::specta]
pub async fn containerd_namespaces(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<ContainerdNamespace>> {
    namespaces::list(&state.hosts.registry.containerd(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn containerd_namespace_create(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    namespaces::create(&state.hosts.registry.containerd(&host_id)?, &name).await
}

/// Removes an empty namespace; one that holds anything is refused.
#[tauri::command]
#[specta::specta]
pub async fn containerd_namespace_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    namespaces::remove(&state.hosts.registry.containerd(&host_id)?, &name).await
}

/// The containers of `namespace`, each with its task.
#[tauri::command]
#[specta::specta]
pub async fn containerd_containers(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
) -> AppResult<Vec<ContainerdContainer>> {
    containers::list(&state.hosts.registry.containerd(&host_id)?, &namespace).await
}

#[tauri::command]
#[specta::specta]
pub async fn containerd_container_inspect(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    id: String,
) -> AppResult<ContainerdContainerDetail> {
    containers::inspect(&state.hosts.registry.containerd(&host_id)?, &namespace, &id).await
}

/// Starts, stops (SIGKILL after 10 s), pauses or resumes a container's task.
#[tauri::command]
#[specta::specta]
pub async fn containerd_container_act(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    id: String,
    action: TaskAction,
) -> AppResult<()> {
    containers::act(
        &state.hosts.registry.containerd(&host_id)?,
        &namespace,
        &id,
        action,
    )
    .await
}

/// Sends a signal, by name or number, to a container's task.
#[tauri::command]
#[specta::specta]
pub async fn containerd_container_kill(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    id: String,
    signal: String,
) -> AppResult<()> {
    let client = state.hosts.registry.containerd(&host_id)?;
    containers::kill_named(&client, &namespace, &id, &signal).await
}

/// Removes a container and, unless `keep_snapshot`, its snapshot; `force`
/// kills a running task first.
#[tauri::command]
#[specta::specta]
pub async fn containerd_container_remove(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    id: String,
    force: bool,
    keep_snapshot: bool,
) -> AppResult<()> {
    let client = state.hosts.registry.containerd(&host_id)?;
    containers::remove(&client, &namespace, &id, force, keep_snapshot).await
}

/// Where a container's output was kept, if anywhere, so the Logs tab can
/// explain an empty one.
#[tauri::command]
#[specta::specta]
pub async fn containerd_logs_source(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    id: String,
) -> AppResult<ContainerdLogSource> {
    logs::locate(&state.hosts.registry.containerd(&host_id)?, &namespace, &id).await
}

/// Streams a container's logs from the file its creator wrote, read with
/// `tail` on the Host's machine (ADR 0020).
#[tauri::command]
#[specta::specta]
pub async fn containerd_logs_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    id: String,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let client = state.hosts.registry.containerd(&host_id)?;
    Ok(state
        .streams
        .spawn(logs::stream(client, namespace, id, options, move |e| {
            on_event.send(e).is_ok()
        })))
}
