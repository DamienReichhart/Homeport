//! Images, volumes, networks and Engine-wide operations.

use tauri::State;

use crate::{
    error::AppResult,
    services::docker::{
        images::{self, Image, ImageLayer, PruneReport},
        networks::{self, NetworkInput, NetworkView},
        run::{self, RunSpec},
        system::{self, DiskUsage, EngineInfo, PruneSelection, SystemPruneReport},
        volumes::{self, VolumeInput, VolumeView},
    },
    state::AppState,
};

macro_rules! docker {
    ($state:expr, $host:expr) => {
        $state.hosts.registry.docker(&$host)?
    };
}

#[tauri::command]
#[specta::specta]
pub async fn images_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Image>> {
    images::list(&docker!(state, host_id)).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    images::inspect(&docker!(state, host_id), &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_history(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<Vec<ImageLayer>> {
    images::history(&docker!(state, host_id), &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_tag(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    target: String,
) -> AppResult<()> {
    images::tag(&docker!(state, host_id), &id, &target).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    force: bool,
) -> AppResult<Vec<String>> {
    images::remove(&docker!(state, host_id), &id, force).await
}

#[tauri::command]
#[specta::specta]
pub async fn images_prune(
    state: State<'_, AppState>,
    host_id: String,
    all: bool,
) -> AppResult<PruneReport> {
    images::prune(&docker!(state, host_id), all).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_list(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<VolumeView>> {
    volumes::list(&docker!(state, host_id)).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_inspect(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<serde_json::Value> {
    volumes::inspect(&docker!(state, host_id), &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_create(
    state: State<'_, AppState>,
    host_id: String,
    input: VolumeInput,
) -> AppResult<String> {
    volumes::create(&docker!(state, host_id), input).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    force: bool,
) -> AppResult<()> {
    volumes::remove(&docker!(state, host_id), &name, force).await
}

#[tauri::command]
#[specta::specta]
pub async fn volumes_prune(
    state: State<'_, AppState>,
    host_id: String,
    all: bool,
) -> AppResult<PruneReport> {
    volumes::prune(&docker!(state, host_id), all).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_list(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<Vec<NetworkView>> {
    networks::list(&docker!(state, host_id)).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    networks::inspect(&docker!(state, host_id), &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_create(
    state: State<'_, AppState>,
    host_id: String,
    input: NetworkInput,
) -> AppResult<String> {
    networks::create(&docker!(state, host_id), input).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<()> {
    networks::remove(&docker!(state, host_id), &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_connect(
    state: State<'_, AppState>,
    host_id: String,
    network: String,
    container: String,
) -> AppResult<()> {
    networks::connect(&docker!(state, host_id), &network, &container).await
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
    networks::disconnect(&docker!(state, host_id), &network, &container, force).await
}

#[tauri::command]
#[specta::specta]
pub async fn networks_prune(state: State<'_, AppState>, host_id: String) -> AppResult<PruneReport> {
    networks::prune(&docker!(state, host_id)).await
}

#[tauri::command]
#[specta::specta]
pub async fn system_info(state: State<'_, AppState>, host_id: String) -> AppResult<EngineInfo> {
    system::info(&docker!(state, host_id)).await
}

#[tauri::command]
#[specta::specta]
pub async fn system_disk_usage(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<DiskUsage> {
    system::disk_usage(&docker!(state, host_id)).await
}

#[tauri::command]
#[specta::specta]
pub async fn system_prune(
    state: State<'_, AppState>,
    host_id: String,
    selection: PruneSelection,
) -> AppResult<SystemPruneReport> {
    system::prune(&docker!(state, host_id), selection).await
}

/// Creates and starts a container; returns its id.
#[tauri::command]
#[specta::specta]
pub async fn containers_run(
    state: State<'_, AppState>,
    host_id: String,
    spec: RunSpec,
) -> AppResult<String> {
    run::run(&docker!(state, host_id), spec).await
}
