use tauri::State;

use crate::{
    error::AppResult,
    services::docker::containers::{self, Container, ContainerAction, Processes},
    state::AppState,
};

#[tauri::command]
#[specta::specta]
pub async fn containers_list(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<Container>> {
    containers::list(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    containers::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_act(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    action: ContainerAction,
) -> AppResult<()> {
    containers::act(&state.hosts.registry.docker(&host_id)?, &id, action).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    force: bool,
    volumes: bool,
) -> AppResult<()> {
    containers::remove(&state.hosts.registry.docker(&host_id)?, &id, force, volumes).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_rename(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    name: String,
) -> AppResult<()> {
    containers::rename(&state.hosts.registry.docker(&host_id)?, &id, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn containers_processes(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<Processes> {
    containers::processes(&state.hosts.registry.docker(&host_id)?, &id).await
}
