//! Clusters: the Kubernetes APIs Homeport connects to, each supervised
//! like a Host.

use tauri::State;

use homeport_clusters::service::ClusterView;

use crate::{app::AppState, error::AppResult};

/// Every Cluster with its connection's status, by name.
#[tauri::command]
#[specta::specta]
pub fn clusters_list(state: State<'_, AppState>) -> AppResult<Vec<ClusterView>> {
    state.clusters.list()
}

#[tauri::command]
#[specta::specta]
pub fn clusters_get(state: State<'_, AppState>, id: String) -> AppResult<ClusterView> {
    state.clusters.get(&id)
}

/// Renames a Cluster.
#[tauri::command]
#[specta::specta]
pub fn clusters_update(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> AppResult<ClusterView> {
    state.clusters.rename(&id, &name)
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_set_enabled(
    state: State<'_, AppState>,
    id: String,
    enabled: bool,
) -> AppResult<ClusterView> {
    state.clusters.set_enabled(&id, enabled)
}

/// Forgets a Cluster and its credentials; nothing changes on the Cluster.
#[tauri::command]
#[specta::specta]
pub fn clusters_remove(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.clusters.remove(&id)
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_reconnect(state: State<'_, AppState>, id: String) -> AppResult<ClusterView> {
    state.clusters.reconnect(&id)
}
