//! Volumes on a Host.

use std::{collections::HashMap, path::Path};

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::{
    containers::files::FileEntry,
    images::{self, PruneReport, PullEvent},
    volumes::{
        self, browse,
        helper::{self, HelperImage},
        VolumeInput, VolumeView,
    },
};

use crate::{app::AppState, commands::registries, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn volumes_list(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<VolumeView>> {
    volumes::list(&state.hosts.registry.docker(&host_id)?).await
}

/// Each volume's size in bytes, `null` when its driver doesn't report one.
/// Apart from the list, because the Engine walks every volume to answer.
#[tauri::command]
#[specta::specta]
pub async fn volumes_sizes(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<HashMap<String, Option<u64>>> {
    volumes::sizes(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_inspect(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<serde_json::Value> {
    volumes::inspect(&state.hosts.registry.docker(&host_id)?, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_create(
    state: State<'_, AppState>,
    host_id: String,
    input: VolumeInput,
) -> AppResult<String> {
    volumes::create(&state.hosts.registry.docker(&host_id)?, input).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    force: bool,
) -> AppResult<()> {
    volumes::remove(&state.hosts.registry.docker(&host_id)?, &name, force).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_prune(
    state: State<'_, AppState>,
    host_id: String,
    all: bool,
) -> AppResult<PruneReport> {
    volumes::prune(&state.hosts.registry.docker(&host_id)?, all).await
}

/// The image of the helper container that opens volumes (ADR 0015), and
/// whether the Host has it.
#[tauri::command]
#[specta::specta]
pub async fn volumes_helper_image(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<HelperImage> {
    helper::image(&state.hosts.registry.docker(&host_id)?).await
}

/// Pulls the helper's image onto a Host, once the user agreed, streaming
/// progress. A failure says how to get the image there by hand.
#[tauri::command]
#[specta::specta]
pub async fn volumes_helper_pull_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let auth = registries::auth_for(&state, helper::IMAGE)?;
    Ok(state.streams.spawn(images::pull(
        docker,
        helper::IMAGE.into(),
        auth,
        move |event| {
            on_event
                .send(match event {
                    PullEvent::Error { error } => PullEvent::Error {
                        error: helper::image_unavailable(Some(&error)),
                    },
                    e => e,
                })
                .is_ok()
        },
    )))
}

/// Opens `volume` read-only in a helper; returns the session's id. With
/// `pulled_image`, ending the session also removes the image Homeport pulled.
#[tauri::command]
#[specta::specta]
pub async fn volumes_browse_start(
    state: State<'_, AppState>,
    host_id: String,
    volume: String,
    pulled_image: bool,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    state
        .volume_browser
        .start(&docker, &volume, pulled_image)
        .await
}

/// Lists a folder of a browsed volume; paths are relative to the volume.
#[tauri::command]
#[specta::specta]
pub async fn volumes_browse_list(
    state: State<'_, AppState>,
    host_id: String,
    session: String,
    path: String,
) -> AppResult<Vec<FileEntry>> {
    browse::list(&state.hosts.registry.docker(&host_id)?, &session, &path).await
}

/// Saves a file, or a folder as a `.tar`, of a browsed volume to `destination`.
#[tauri::command]
#[specta::specta]
pub async fn volumes_browse_download(
    state: State<'_, AppState>,
    host_id: String,
    session: String,
    path: String,
    destination: String,
) -> AppResult<bool> {
    let docker = state.hosts.registry.docker(&host_id)?;
    browse::download(&docker, &session, &path, Path::new(&destination)).await
}

/// Ends a browse session, removing its helper.
#[tauri::command]
#[specta::specta]
pub async fn volumes_browse_end(state: State<'_, AppState>, session: String) -> AppResult<()> {
    state.volume_browser.end(&session).await;
    Ok(())
}
