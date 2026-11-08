//! What containerd Hosts offer, through containerd's gRPC API (ADR 0020).
//! Every command but the namespaces' own names the namespace it's about;
//! a Host whose Engine isn't containerd answers `EngineUnsupported`.

use tauri::{ipc::Channel, State};

use homeport_containerd::{
    containers::{self, ContainerdContainer, ContainerdContainerDetail, TaskAction},
    exec::{self, ContainerdExecOptions, SessionId},
    images::{self, ContainerdImage, PullCredentials},
    logs::{self, ContainerdLogSource},
    namespaces::{self, ContainerdNamespace},
    snapshots::{self, ContainerdSnapshots},
};
use homeport_core::{
    logs::{LogEvent, LogOptions},
    pull::PullEvent,
    streams::StreamId,
    terminal::ExecEvent,
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

/// Opens a terminal in a container's running task, through FIFOs on the
/// Host's machine (ADR 0020).
#[tauri::command]
#[specta::specta]
pub async fn containerd_exec_start(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    id: String,
    options: ContainerdExecOptions,
    on_event: Channel<ExecEvent>,
) -> AppResult<SessionId> {
    let client = state.hosts.registry.containerd(&host_id)?;
    let (session, output) = exec::start(&client, &namespace, &id, options).await?;
    Ok(state
        .containerd_exec
        .open(session, output, move |e| on_event.send(e).is_ok()))
}

#[tauri::command]
#[specta::specta]
pub async fn containerd_exec_input(
    state: State<'_, AppState>,
    session: SessionId,
    data: String,
) -> AppResult<()> {
    state
        .containerd_exec
        .get(session)?
        .write(data.as_bytes())
        .await
}

#[tauri::command]
#[specta::specta]
pub async fn containerd_exec_resize(
    state: State<'_, AppState>,
    session: SessionId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state.containerd_exec.get(session)?.resize(cols, rows).await
}

/// Closes a terminal, killing its process; async, since that needs the runtime.
#[tauri::command]
#[specta::specta]
pub async fn containerd_exec_close(
    state: State<'_, AppState>,
    session: SessionId,
) -> AppResult<()> {
    state.containerd_exec.close(session);
    Ok(())
}

/// The images of `namespace`, with their size for the Host's platform.
#[tauri::command]
#[specta::specta]
pub async fn containerd_images(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
) -> AppResult<Vec<ContainerdImage>> {
    images::list(&state.hosts.registry.containerd(&host_id)?, &namespace).await
}

/// Removes an image no container of the namespace was made from.
#[tauri::command]
#[specta::specta]
pub async fn containerd_image_remove(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    name: String,
) -> AppResult<()> {
    images::remove(
        &state.hosts.registry.containerd(&host_id)?,
        &namespace,
        &name,
    )
    .await
}

/// Pulls an image through containerd's Transfer service with the stored
/// login for its registry, streaming progress.
#[tauri::command]
#[specta::specta]
pub async fn containerd_images_pull_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    reference: String,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let client = state.hosts.registry.containerd(&host_id)?;
    let qualified = images::qualify(&reference);
    let credentials = state
        .registries
        .credentials_for(&qualified)?
        .map(|c| PullCredentials {
            username: c.username,
            secret: c.secret,
        });
    let explained = super::registries::explainer(&state, &qualified);
    Ok(state.streams.spawn(images::pull(
        client,
        namespace,
        reference,
        credentials,
        move |e| {
            let e = match e {
                PullEvent::Error { error } => PullEvent::Error {
                    error: explained(error),
                },
                e => e,
            };
            on_event.send(e).is_ok()
        },
    )))
}

/// The snapshots of a snapshotter (the Host's default when `None`) in
/// `namespace`, largest first, each with what uses it.
#[tauri::command]
#[specta::specta]
pub async fn containerd_snapshots(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    snapshotter: Option<String>,
) -> AppResult<ContainerdSnapshots> {
    let client = state.hosts.registry.containerd(&host_id)?;
    snapshots::list(&client, &namespace, snapshotter.as_deref()).await
}

/// Removes a snapshot nothing uses and nothing is built on.
#[tauri::command]
#[specta::specta]
pub async fn containerd_snapshot_remove(
    state: State<'_, AppState>,
    host_id: String,
    namespace: String,
    snapshotter: String,
    key: String,
) -> AppResult<()> {
    let client = state.hosts.registry.containerd(&host_id)?;
    snapshots::remove(&client, &namespace, &snapshotter, &key).await
}
