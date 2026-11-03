//! Networks on a Host.

use tauri::State;

use homeport_engine::{
    images::PruneReport,
    networks::{self, NetworkInput, NetworkView},
};

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn networks_list(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<NetworkView>> {
    networks::list(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    networks::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_create(
    state: State<'_, AppState>,
    host_id: String,
    input: NetworkInput,
) -> AppResult<String> {
    networks::create(&state.hosts.registry.docker(&host_id)?, input).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<()> {
    networks::remove(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_connect(
    state: State<'_, AppState>,
    host_id: String,
    network: String,
    container: String,
) -> AppResult<()> {
    networks::connect(
        &state.hosts.registry.docker(&host_id)?,
        &network,
        &container,
    )
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_disconnect(
    state: State<'_, AppState>,
    host_id: String,
    network: String,
    container: String,
    force: bool,
) -> AppResult<()> {
    networks::disconnect(
        &state.hosts.registry.docker(&host_id)?,
        &network,
        &container,
        force,
    )
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_prune(state: State<'_, AppState>, host_id: String) -> AppResult<PruneReport> {
    networks::prune(&state.hosts.registry.docker(&host_id)?).await
}
