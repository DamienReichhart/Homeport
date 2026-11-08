//! Clusters: the Kubernetes APIs Homeport connects to, each supervised
//! like a Host.

use tauri::State;

use homeport_clusters::{
    kubeconfig::{ContextChoice, KubeContexts},
    service::ClusterView,
};

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

/// The contexts of the pasted kubeconfig `text`, or of this machine's
/// (`KUBECONFIG`, else `~/.kube/config`). A pasted one stays in memory.
#[tauri::command]
#[specta::specta]
pub fn clusters_kubeconfig_contexts(
    state: State<'_, AppState>,
    text: Option<String>,
) -> AppResult<KubeContexts> {
    state.clusters.kubeconfig_contexts(text.as_deref())
}

/// Adds a Cluster for each context picked, its credentials sealed.
#[tauri::command]
#[specta::specta]
pub async fn clusters_import(
    state: State<'_, AppState>,
    contexts: Vec<ContextChoice>,
    text: Option<String>,
) -> AppResult<Vec<ClusterView>> {
    state.clusters.import(&contexts, text.as_deref())
}

/// Reads a Cluster's context again from its kubeconfig and reconnects.
#[tauri::command]
#[specta::specta]
pub async fn clusters_refresh(
    state: State<'_, AppState>,
    id: String,
    text: Option<String>,
) -> AppResult<ClusterView> {
    state.clusters.refresh(&id, text.as_deref())
}
