//! What Incus Hosts offer, through Incus's REST API (ADR 0021). Every
//! command about a project-scoped object names its project; a Host whose
//! Engine isn't Incus answers `EngineUnsupported`.

use tauri::{ipc::Channel, State};

use homeport_core::{
    host::{EngineKind, HostConnection},
    pull::PullEvent,
    streams::StreamId,
    terminal::ExecEvent,
};
use homeport_hosts::service::{CredentialsInput, HostInput, HostView};
use homeport_incus::{
    images::{self, ImageCopy, IncusImage},
    instances::{self, IncusInstance, IncusInstanceDetail, InstanceAction, InstanceSpec},
    networks::{self, BridgeRequest, IncusNetwork, IncusNetworkState},
    profiles::{self, IncusProfile, IncusProfileDetail},
    projects::{self, IncusProject},
    simplestreams::{self, RemoteImage},
    snapshots::{self, IncusSnapshot, SnapshotRequest},
    storage::{self, IncusStoragePool, IncusVolume, VolumeRequest},
    terminal::{self, IncusExecOptions, IncusTerminal, SessionId},
    trust::{self, TrustToken, TrustTokenSummary},
};

use crate::{app::AppState, error::AppResult};

/// What a pasted trust token says, for the Host form to show before adding.
#[tauri::command]
#[specta::specta]
pub fn incus_trust_token_read(token: String) -> AppResult<TrustTokenSummary> {
    Ok(TrustToken::parse(&token)?.summary())
}

/// Adds an Incus Host over HTTPS with a trust token from
/// `incus config trust add`: Homeport makes a client certificate, registers
/// it with the token, and keeps the server's certificate pinned (ADR 0021).
#[tauri::command]
#[specta::specta]
pub async fn incus_trust_add(
    state: State<'_, AppState>,
    name: String,
    token: String,
) -> AppResult<HostView> {
    let token = TrustToken::parse(&token)?;
    let name = if name.trim().is_empty() {
        token.client_name.clone()
    } else {
        name.trim().to_owned()
    };
    let trusted = trust::join(&name, &token).await?;
    state.hosts.create(HostInput {
        name,
        connection: HostConnection::Tls {
            address: trusted.address,
        },
        engine: EngineKind::Incus,
        credentials: Some(CredentialsInput::Tls {
            ca_pem: trusted.server_pem,
            cert_pem: trusted.client.cert_pem,
            key_pem: trusted.client.key_pem,
        }),
        bastion_keys: vec![],
    })
}

/// The projects the Host's connection may use, with what each holds.
#[tauri::command]
#[specta::specta]
pub async fn incus_projects(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<IncusProject>> {
    projects::list(&state.hosts.registry.incus(&host_id)?).await
}

/// The Instances of `project`, with their state.
#[tauri::command]
#[specta::specta]
pub async fn incus_instances(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusInstance>> {
    instances::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// One Instance with its configuration, devices and snapshots' names.
#[tauri::command]
#[specta::specta]
pub async fn incus_instance(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusInstanceDetail> {
    instances::get(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// Starts, stops, restarts, freezes or unfreezes an Instance; `force` stops
/// or restarts it without waiting for its system to shut down.
#[tauri::command]
#[specta::specta]
pub async fn incus_instance_act(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    action: InstanceAction,
    force: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::act(&incus, &project, &name, action, force).await
}

/// Makes an Instance from an image, streaming the image download's
/// progress until it's made (and started, if asked).
#[tauri::command]
#[specta::specta]
pub async fn incus_instance_create_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    spec: InstanceSpec,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::validate_name(&spec.name)?;
    Ok(state.streams.spawn(async move {
        instances::create(&incus, &project, &spec, move |e| on_event.send(e).is_ok()).await;
    }))
}

/// Deletes an Instance with its snapshots; `stop_first` stops a running one.
#[tauri::command]
#[specta::specta]
pub async fn incus_instance_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    stop_first: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    instances::delete(&incus, &project, &name, stop_first).await
}

/// The images of `project`, newest first.
#[tauri::command]
#[specta::specta]
pub async fn incus_images(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusImage>> {
    images::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// What `server` (a simplestreams image server) publishes for the Host's
/// architecture, read from this computer and kept a few minutes.
#[tauri::command]
#[specta::specta]
pub async fn incus_remote_images(
    state: State<'_, AppState>,
    host_id: String,
    server: String,
) -> AppResult<Vec<RemoteImage>> {
    let incus = state.hosts.registry.incus(&host_id)?;
    simplestreams::catalogue(&server, &incus.server().arch).await
}

/// Copies an image from a remote server into `project`, streaming the
/// download's progress.
#[tauri::command]
#[specta::specta]
pub async fn incus_image_copy_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    copy: ImageCopy,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    Ok(state.streams.spawn(async move {
        images::copy(&incus, &project, &copy, move |e| on_event.send(e).is_ok()).await;
    }))
}

/// Fetches a newer build of an image from where it came from.
#[tauri::command]
#[specta::specta]
pub async fn incus_image_refresh(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    fingerprint: String,
) -> AppResult<()> {
    images::refresh(
        &state.hosts.registry.incus(&host_id)?,
        &project,
        &fingerprint,
    )
    .await
}

/// Deletes an image; Instances made from it keep their own copy.
#[tauri::command]
#[specta::specta]
pub async fn incus_image_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    fingerprint: String,
) -> AppResult<()> {
    images::delete(
        &state.hosts.registry.incus(&host_id)?,
        &project,
        &fingerprint,
    )
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_image_alias_add(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    fingerprint: String,
    name: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    images::add_alias(&incus, &project, &fingerprint, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_image_alias_remove(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    images::remove_alias(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// The profiles of `project`, with the Instances using each.
#[tauri::command]
#[specta::specta]
pub async fn incus_profiles(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusProfile>> {
    profiles::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// One profile, with its YAML and the ETag to save it with.
#[tauri::command]
#[specta::specta]
pub async fn incus_profile(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusProfileDetail> {
    profiles::get(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// Saves a profile's YAML; refused if it changed since `etag` was read.
#[tauri::command]
#[specta::specta]
pub async fn incus_profile_update(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    yaml: String,
    etag: Option<String>,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    profiles::update(&incus, &project, &name, &yaml, etag.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_profile_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    description: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    profiles::create(&incus, &project, &name, &description).await
}

/// Deletes an unused profile other than `default`.
#[tauri::command]
#[specta::specta]
pub async fn incus_profile_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    profiles::delete(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// Runs a command (a shell by default) in a running Instance, with a
/// terminal, through Incus's exec websockets.
#[tauri::command]
#[specta::specta]
pub async fn incus_exec_start(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    options: IncusExecOptions,
    on_event: Channel<ExecEvent>,
) -> AppResult<SessionId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    let (session, output) = IncusTerminal::exec(&incus, &project, &name, &options).await?;
    session.resize(options.cols, options.rows).await.ok();
    Ok(state
        .terminals
        .incus
        .open(session, output, move |e| on_event.send(e).is_ok()))
}

/// Attaches to a running Instance's text console.
#[tauri::command]
#[specta::specta]
pub async fn incus_console_start(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    cols: u16,
    rows: u16,
    on_event: Channel<ExecEvent>,
) -> AppResult<SessionId> {
    let incus = state.hosts.registry.incus(&host_id)?;
    let (session, output) = IncusTerminal::console(&incus, &project, &name, cols, rows).await?;
    Ok(state
        .terminals
        .incus
        .open(session, output, move |e| on_event.send(e).is_ok()))
}

/// What an Instance printed on its console, if Incus kept it.
#[tauri::command]
#[specta::specta]
pub async fn incus_console_log(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<String> {
    let incus = state.hosts.registry.incus(&host_id)?;
    Ok(terminal::console_log(&incus, &project, &name).await)
}

#[tauri::command]
#[specta::specta]
pub async fn incus_terminal_input(
    state: State<'_, AppState>,
    session: SessionId,
    data: String,
) -> AppResult<()> {
    state
        .terminals
        .incus
        .get(session)?
        .write(data.as_bytes())
        .await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_terminal_resize(
    state: State<'_, AppState>,
    session: SessionId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state.terminals.incus.get(session)?.resize(cols, rows).await
}

/// Closes a terminal: hangs up an exec's command, detaches from a console.
#[tauri::command]
#[specta::specta]
pub async fn incus_terminal_close(state: State<'_, AppState>, session: SessionId) -> AppResult<()> {
    state.terminals.incus.close(session);
    Ok(())
}

/// The snapshots of an Instance, oldest first.
#[tauri::command]
#[specta::specta]
pub async fn incus_snapshots(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
) -> AppResult<Vec<IncusSnapshot>> {
    snapshots::list(&state.hosts.registry.incus(&host_id)?, &project, &instance).await
}

/// Takes a snapshot of an Instance, stateful and expiring if asked.
#[tauri::command]
#[specta::specta]
pub async fn incus_snapshot_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
    request: SnapshotRequest,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::create(&incus, &project, &instance, &request).await
}

/// Restores an Instance to one of its snapshots; what changed since is lost.
#[tauri::command]
#[specta::specta]
pub async fn incus_snapshot_restore(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
    snapshot: String,
    stateful: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::restore(&incus, &project, &instance, &snapshot, stateful).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_snapshot_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
    snapshot: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::delete(&incus, &project, &instance, &snapshot).await
}

/// Every storage pool, with how full it is.
#[tauri::command]
#[specta::specta]
pub async fn incus_storage_pools(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusStoragePool>> {
    storage::pools(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// The volumes of a pool in `project`, with what uses each.
#[tauri::command]
#[specta::specta]
pub async fn incus_storage_volumes(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    pool: String,
) -> AppResult<Vec<IncusVolume>> {
    storage::volumes(&state.hosts.registry.incus(&host_id)?, &project, &pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_volume_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    pool: String,
    request: VolumeRequest,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    storage::create_volume(&incus, &project, &pool, &request).await
}

/// Deletes a custom volume nothing uses.
#[tauri::command]
#[specta::specta]
pub async fn incus_volume_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    pool: String,
    name: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    storage::delete_volume(&incus, &project, &pool, &name).await
}

/// The networks `project` sees, managed ones first.
#[tauri::command]
#[specta::specta]
pub async fn incus_networks(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusNetwork>> {
    networks::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// A network's addresses on the host and its DHCP leases.
#[tauri::command]
#[specta::specta]
pub async fn incus_network_state(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusNetworkState> {
    networks::state(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_bridge_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    request: BridgeRequest,
) -> AppResult<()> {
    networks::create_bridge(&state.hosts.registry.incus(&host_id)?, &project, &request).await
}

/// Deletes a managed network nothing uses.
#[tauri::command]
#[specta::specta]
pub async fn incus_network_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    networks::delete(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}
