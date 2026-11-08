//! What Incus Hosts offer, through Incus's REST API (ADR 0021). Every
//! command about a project-scoped object names its project; a Host whose
//! Engine isn't Incus answers `EngineUnsupported`.

use tauri::{ipc::Channel, State};

use homeport_core::{
    host::{EngineKind, HostConnection},
    pull::PullEvent,
    streams::StreamId,
};
use homeport_hosts::service::{CredentialsInput, HostInput, HostView};
use homeport_incus::{
    images::{self, ImageCopy, IncusImage},
    instances::{self, IncusInstance, IncusInstanceDetail, InstanceAction},
    projects::{self, IncusProject},
    simplestreams::{self, RemoteImage},
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
