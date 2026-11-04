//! Volumes on a Host.

use tauri::State;

use homeport_engine::{
    images::PruneReport,
    volumes::{self, VolumeInput, VolumeView},
};

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn volumes_list(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<VolumeView>> {
    volumes::list(&state.hosts.registry.docker(&host_id)?).await
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
