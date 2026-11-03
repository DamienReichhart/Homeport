use std::path::{Path, PathBuf};

use tauri::State;

use homeport_engine::{
    containers::{self, Container, ContainerAction, Processes},
    files::{self, FileEntry},
};

use crate::{error::AppResult, state::AppState};

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

#[tauri::command]
#[specta::specta]
pub async fn files_list(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    path: String,
) -> AppResult<Vec<FileEntry>> {
    files::list(
        &state.hosts.registry.docker(&host_id)?,
        &container_id,
        &path,
    )
    .await
}

/// Saves a container path to `destination`; returns `true` when it was a
/// directory saved as a `.tar` archive.
#[tauri::command]
#[specta::specta]
pub async fn files_download(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    path: String,
    destination: String,
) -> AppResult<bool> {
    let docker = state.hosts.registry.docker(&host_id)?;
    files::download(&docker, &container_id, &path, Path::new(&destination)).await
}

#[tauri::command]
#[specta::specta]
pub async fn files_upload(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    directory: String,
    sources: Vec<String>,
) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let sources: Vec<PathBuf> = sources.into_iter().map(PathBuf::from).collect();
    files::upload(&docker, &container_id, &directory, &sources).await
}
