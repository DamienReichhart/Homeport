//! Incus storage pools and their volumes.

use tauri::State;

use homeport_incus::storage::{self, IncusStoragePool, IncusVolume, VolumeRequest};

use crate::{app::AppState, error::AppResult};

/// Every storage pool, with how full it is.
#[tauri::command]
#[specta::specta]
pub async fn incus_storage_pools(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusStoragePool>> {
    storage::pools(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// The volumes of a pool in `project`, with what uses each.
#[tauri::command]
#[specta::specta]
pub async fn incus_storage_volumes(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    pool: String,
) -> AppResult<Vec<IncusVolume>> {
    storage::volumes(&state.hosts.registry.incus(&host_id)?, &project, &pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_volume_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    pool: String,
    request: VolumeRequest,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    storage::create_volume(&incus, &project, &pool, &request).await
}

/// Deletes a custom volume nothing uses.
#[tauri::command]
#[specta::specta]
pub async fn incus_volume_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    pool: String,
    name: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    storage::delete_volume(&incus, &project, &pool, &name).await
}
