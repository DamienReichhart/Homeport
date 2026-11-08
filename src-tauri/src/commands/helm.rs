//! Helm releases on a Cluster, through the user's `helm` CLI and a private
//! proxy to the Cluster (ADR 0009).

use tauri::State;

use homeport_clusters::helm::{Helm, ReleaseDetails, ReleaseList, ReleaseRevision};

use crate::{app::AppState, error::AppResult};

/// helm, run against the connected Cluster `id`.
fn helm(state: &AppState, id: &str) -> AppResult<Helm> {
    Helm::on(state.clusters.access(id)?)
}

/// The releases in `namespace`, or every Namespace; when the credentials
/// can't list every Namespace, those of the Cluster's default one.
#[tauri::command]
#[specta::specta]
pub async fn clusters_helm_releases(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<ReleaseList> {
    let access = state.clusters.access(&id)?;
    let fallback = access
        .cluster
        .connection
        .namespace
        .clone()
        .unwrap_or_else(|| "default".into());
    Helm::on(access)?
        .releases(namespace.as_deref(), Some(&fallback))
        .await
}

/// Every revision of a release, oldest first.
#[tauri::command]
#[specta::specta]
pub async fn clusters_helm_history(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<Vec<ReleaseRevision>> {
    helm(&state, &id)?.history(&namespace, &name).await
}

/// The values, notes, manifest and resources of one revision of a release,
/// the current one without `revision`.
#[tauri::command]
#[specta::specta]
pub async fn clusters_helm_release_details(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
    revision: Option<u32>,
) -> AppResult<ReleaseDetails> {
    helm(&state, &id)?
        .details(&namespace, &name, revision)
        .await
}
