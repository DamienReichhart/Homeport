//! Incus images: the Host's, a remote's catalogue, copies and aliases.

use tauri::{ipc::Channel, State};

use homeport_core::{pull::PullEvent, streams::StreamId};
use homeport_incus::images::{
    self,
    simplestreams::{self, RemoteImage},
    ImageCopy, IncusImage,
};

use crate::{app::AppState, error::AppResult};

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
