//! Clusters: the Kubernetes APIs Homeport connects to, each supervised
//! like a Host.

pub mod actions;
pub mod api;
pub mod forwards;
pub mod helm;
pub mod nomad;
pub mod rbac;
pub mod resources;
pub mod streams;

use tauri::State;

use homeport_clusters::service::ClusterView;
use homeport_core::host::HostConnection;
use homeport_engine::containers;
use homeport_kubernetes::resources::On;
use homeport_kubernetes::{
    detect::{self, LocalClusters},
    kubeconfig::{ContextChoice, KubeContexts},
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

/// The local clusters (Docker Desktop, OrbStack, Rancher Desktop, kind,
/// k3d, minikube) this machine's kubeconfig reaches, each probed, and the
/// kind and k3d clusters running on a connected local Host that it doesn't.
#[tauri::command]
#[specta::specta]
pub async fn clusters_detect_local(state: State<'_, AppState>) -> AppResult<LocalClusters> {
    let found = detect::detect(&state.clusters.kubernetes()?).await;
    let mut labels = vec![];
    for host in state.hosts.store.list()? {
        if !matches!(host.connection, HostConnection::Local { .. }) {
            continue;
        }
        let Ok(docker) = state.hosts.registry.docker(&host.id) else {
            continue;
        };
        match containers::list(&docker).await {
            Ok(list) => labels.extend(list.into_iter().map(|c| c.labels)),
            Err(e) => log::debug!("couldn't list the containers of {}: {e}", host.name),
        }
    }
    let missing = detect::missing(&labels, &detect::local_contexts());
    Ok(LocalClusters { found, missing })
}

// What runs on a Cluster (read-only).

/// The connected Cluster `id`.
pub(crate) fn on(state: &AppState, id: &str) -> AppResult<On> {
    let (client, cluster) = state.clusters.registry.client(id)?;
    Ok(On { client, cluster })
}
