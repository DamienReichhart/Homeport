//! Snapshots of Incus Instances.

use tauri::State;

use homeport_incus::instances::snapshots::{self, IncusSnapshot, SnapshotRequest};

use crate::{app::AppState, error::AppResult};

/// The snapshots of an Instance, oldest first.
#[tauri::command]
#[specta::specta]
pub async fn incus_snapshots(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
) -> AppResult<Vec<IncusSnapshot>> {
    snapshots::list(&state.hosts.registry.incus(&host_id)?, &project, &instance).await
}

/// Takes a snapshot of an Instance, stateful and expiring if asked.
#[tauri::command]
#[specta::specta]
pub async fn incus_snapshot_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
    request: SnapshotRequest,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::create(&incus, &project, &instance, &request).await
}

/// Restores an Instance to one of its snapshots; what changed since is lost.
#[tauri::command]
#[specta::specta]
pub async fn incus_snapshot_restore(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
    snapshot: String,
    stateful: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::restore(&incus, &project, &instance, &snapshot, stateful).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_snapshot_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    instance: String,
    snapshot: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::delete(&incus, &project, &instance, &snapshot).await
}
