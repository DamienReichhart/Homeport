//! What only Podman offers, through its own API: pods (containers run into
//! one go through `containers_run`). A Host that isn't Podman answers
//! `EngineUnsupported`.

use tauri::State;

use homeport_engine::podman::pods::{self, Pod, PodAction, PodDetail, PodSpec};

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn pods_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Pod>> {
    pods::list(&state.hosts.registry.podman(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn pods_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<PodDetail> {
    pods::inspect(&state.hosts.registry.podman(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn pods_act(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    action: PodAction,
) -> AppResult<()> {
    pods::act(&state.hosts.registry.podman(&host_id)?, &id, action).await
}

/// Removes a pod and its containers; returns how many containers went with it.
#[tauri::command]
#[specta::specta]
pub async fn pods_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    force: bool,
) -> AppResult<u32> {
    pods::remove(&state.hosts.registry.podman(&host_id)?, &id, force).await
}

/// Creates a pod; returns its id.
#[tauri::command]
#[specta::specta]
pub async fn pods_create(
    state: State<'_, AppState>,
    host_id: String,
    spec: PodSpec,
) -> AppResult<String> {
    pods::create(&state.hosts.registry.podman(&host_id)?, &spec).await
}
