//! What containerd Hosts offer, through containerd's gRPC API (ADR 0020).
//! Every command but the namespaces' own names the namespace it's about;
//! a Host whose Engine isn't containerd answers `EngineUnsupported`.

use homeport_containerd::{
    containers::{
        self,
        exec::{self, ContainerdExecOptions, SessionId},
        logs::{self, ContainerdLogSource},
        ContainerdContainer, ContainerdContainerDetail, TaskAction,
    },
    images::{self, ContainerdImage, PullCredentials},
    namespaces::{self, ContainerdNamespace},
    snapshots::{self, ContainerdSnapshots},
};
use homeport_core::{
    error::{AppError, AppResult},
    logs::{LogEvent, LogOptions},
    pull::PullEvent,
    streams::StreamId,
    terminal::ExecEvent,
};
use homeport_registries::hints;

use crate::app::AppState;

/// Every namespace of the Host, with what each holds.
pub async fn containerd_namespaces(
    state: &AppState,
    host_id: String,
) -> AppResult<Vec<ContainerdNamespace>> {
    namespaces::list(&state.hosts.registry.containerd(&host_id)?).await
}

pub async fn containerd_namespace_create(
    state: &AppState,
    host_id: String,
    name: String,
) -> AppResult<()> {
    namespaces::create(&state.hosts.registry.containerd(&host_id)?, &name).await
}

/// Removes an empty namespace; one that holds anything is refused.
pub async fn containerd_namespace_remove(
    state: &AppState,
    host_id: String,
    name: String,
) -> AppResult<()> {
    namespaces::remove(&state.hosts.registry.containerd(&host_id)?, &name).await
}

/// The containers of `namespace`, each with its task.
pub async fn containerd_containers(
    state: &AppState,
    host_id: String,
    namespace: String,
) -> AppResult<Vec<ContainerdContainer>> {
    containers::list(&state.hosts.registry.containerd(&host_id)?, &namespace).await
}

pub async fn containerd_container_inspect(
    state: &AppState,
    host_id: String,
    namespace: String,
    id: String,
) -> AppResult<ContainerdContainerDetail> {
    containers::inspect(&state.hosts.registry.containerd(&host_id)?, &namespace, &id).await
}

/// Starts, stops (SIGKILL after 10 s), pauses or resumes a container's task.
pub async fn containerd_container_act(
    state: &AppState,
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
pub async fn containerd_container_kill(
    state: &AppState,
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
pub async fn containerd_container_remove(
    state: &AppState,
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
pub async fn containerd_logs_source(
    state: &AppState,
    host_id: String,
    namespace: String,
    id: String,
) -> AppResult<ContainerdLogSource> {
    logs::locate(&state.hosts.registry.containerd(&host_id)?, &namespace, &id).await
}

/// Streams a container's logs from the file its creator wrote, read with
/// `tail` on the Host's machine (ADR 0020).
pub async fn containerd_logs_subscribe(
    state: &AppState,
    host_id: String,
    namespace: String,
    id: String,
    options: LogOptions,
    on_event: impl Fn(LogEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let client = state.hosts.registry.containerd(&host_id)?;
    Ok(state
        .streams
        .spawn(logs::stream(client, namespace, id, options, move |e| {
            on_event(e)
        })))
}

/// Opens a terminal in a container's running task, through FIFOs on the
/// Host's machine (ADR 0020).
pub async fn containerd_exec_start(
    state: &AppState,
    host_id: String,
    namespace: String,
    id: String,
    options: ContainerdExecOptions,
    on_event: impl Fn(ExecEvent) -> bool + Send + Sync + 'static,
) -> AppResult<SessionId> {
    let client = state.hosts.registry.containerd(&host_id)?;
    let (session, output) = exec::start(&client, &namespace, &id, options).await?;
    Ok(state.terminals.containerd.open(session, output, on_event))
}

pub async fn containerd_exec_input(
    state: &AppState,
    session: SessionId,
    data: String,
) -> AppResult<()> {
    state
        .terminals
        .containerd
        .get(session)?
        .write(data.as_bytes())
        .await
}

pub async fn containerd_exec_resize(
    state: &AppState,
    session: SessionId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state
        .terminals
        .containerd
        .get(session)?
        .resize(cols, rows)
        .await
}

/// Closes a terminal, killing its process; async, since that needs the runtime.
pub async fn containerd_exec_close(state: &AppState, session: SessionId) -> AppResult<()> {
    state.terminals.containerd.close(session);
    Ok(())
}

/// The images of `namespace`, with their size for the Host's platform.
pub async fn containerd_images(
    state: &AppState,
    host_id: String,
    namespace: String,
) -> AppResult<Vec<ContainerdImage>> {
    images::list(&state.hosts.registry.containerd(&host_id)?, &namespace).await
}

/// Removes an image no container of the namespace was made from.
pub async fn containerd_image_remove(
    state: &AppState,
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
pub async fn containerd_images_pull_subscribe(
    state: &AppState,
    host_id: String,
    namespace: String,
    reference: String,
    on_event: impl Fn(PullEvent) -> bool + Send + Sync + 'static,
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
    let explained = explainer(state, &qualified);
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
            on_event(e)
        },
    )))
}

/// Rewrites a registry refusal for `reference` to point at the login that
/// would fix it, for a stream that outlives the call and can't keep `state`.
fn explainer(
    state: &AppState,
    reference: &str,
) -> impl Fn(AppError) -> AppError + Send + Sync + 'static {
    let login = state.registries.login_for(reference).ok().flatten();
    let reference = reference.to_owned();
    move |error| hints::explain(error, &reference, login.as_ref())
}

/// The snapshots of a snapshotter (the Host's default when `None`) in
/// `namespace`, largest first, each with what uses it.
pub async fn containerd_snapshots(
    state: &AppState,
    host_id: String,
    namespace: String,
    snapshotter: Option<String>,
) -> AppResult<ContainerdSnapshots> {
    let client = state.hosts.registry.containerd(&host_id)?;
    snapshots::list(&client, &namespace, snapshotter.as_deref()).await
}

/// Removes a snapshot nothing uses and nothing is built on.
pub async fn containerd_snapshot_remove(
    state: &AppState,
    host_id: String,
    namespace: String,
    snapshotter: String,
    key: String,
) -> AppResult<()> {
    let client = state.hosts.registry.containerd(&host_id)?;
    snapshots::remove(&client, &namespace, &snapshotter, &key).await
}

#[cfg(test)]
mod tests;
