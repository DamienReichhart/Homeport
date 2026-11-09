//! Nomad Clusters: adding, editing and testing one. What runs on them is
//! read through the same Cluster registry as Kubernetes Clusters.

use tauri::State;

use homeport_clusters::{nomad::NomadForm, service::ClusterView};
use homeport_nomad::NomadAgent;

use crate::{app::AppState, error::AppResult};

/// Adds a Nomad Cluster, its ACL token and TLS key sealed, and starts
/// connecting to it.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_add(
    state: State<'_, AppState>,
    form: NomadForm,
) -> AppResult<ClusterView> {
    state.clusters.add_nomad(&form)
}

/// Changes a Nomad Cluster's name, address and credentials (those left
/// out are kept), then reconnects.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_update(
    state: State<'_, AppState>,
    id: String,
    form: NomadForm,
) -> AppResult<ClusterView> {
    state.clusters.update_nomad(&id, &form)
}

/// Connects once to the Nomad Cluster the form describes, without saving
/// it. With `id`, the credentials left out are that Cluster's.
#[tauri::command]
#[specta::specta]
pub async fn clusters_nomad_test(
    state: State<'_, AppState>,
    form: NomadForm,
    id: Option<String>,
) -> AppResult<NomadAgent> {
    state.clusters.test_nomad(&form, id.as_deref()).await
}
