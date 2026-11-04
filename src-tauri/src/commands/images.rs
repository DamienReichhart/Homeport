//! Images on a Host.

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::images::{self, Image, ImageLayer, PruneReport, PullEvent};

use crate::{app::AppState, commands::registries, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn images_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Image>> {
    images::list(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    images::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_history(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<Vec<ImageLayer>> {
    images::history(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_tag(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    target: String,
) -> AppResult<()> {
    images::tag(&state.hosts.registry.docker(&host_id)?, &id, &target).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    force: bool,
) -> AppResult<Vec<String>> {
    images::remove(&state.hosts.registry.docker(&host_id)?, &id, force).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_prune(
    state: State<'_, AppState>,
    host_id: String,
    all: bool,
) -> AppResult<PruneReport> {
    images::prune(&state.hosts.registry.docker(&host_id)?, all).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_pull_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    reference: String,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let auth = registries::auth_for(&state, &reference)?;
    let explained = registries::explainer(&state, &reference);
    Ok(state
        .streams
        .spawn(images::pull(docker, reference, auth, move |e| {
            let e = match e {
                PullEvent::Error { error } => PullEvent::Error {
                    error: explained(error),
                },
                e => e,
            };
            on_event.send(e).is_ok()
        })))
}
