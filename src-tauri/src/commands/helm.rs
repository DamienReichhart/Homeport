//! Helm releases on a Cluster, through the user's `helm` CLI and a private
//! proxy to the Cluster (ADR 0009).

use tauri::{ipc::Channel, State};

use homeport_clusters::{
    helm::{ChartRepository, ChartVersion, Helm, ReleaseDetails, ReleaseList, ReleaseRevision},
    proxy::CliEvent,
};
use homeport_core::streams::StreamId;

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

/// The chart repositories helm knows on this machine.
#[tauri::command]
#[specta::specta]
pub async fn helm_repos_list() -> AppResult<Vec<ChartRepository>> {
    Helm::local()?.repositories().await
}

/// Adds a public chart repository to helm's list.
#[tauri::command]
#[specta::specta]
pub async fn helm_repos_add(name: String, url: String) -> AppResult<()> {
    Helm::local()?.add_repository(&name, &url).await
}

/// Downloads the indexes of `names`, or of every repository, streaming
/// helm's output.
#[tauri::command]
#[specta::specta]
pub async fn helm_repos_update_subscribe(
    state: State<'_, AppState>,
    names: Vec<String>,
    on_event: Channel<CliEvent>,
) -> AppResult<StreamId> {
    let helm = Helm::local()?;
    Ok(state
        .streams
        .spawn(helm.update_repositories(names, move |e| on_event.send(e).is_ok())))
}

#[tauri::command]
#[specta::specta]
pub async fn helm_repos_remove(name: String) -> AppResult<()> {
    Helm::local()?.remove_repository(&name).await
}

/// Charts in the repositories matching `query`, at their latest version or
/// every version.
#[tauri::command]
#[specta::specta]
pub async fn helm_charts_search(query: String, versions: bool) -> AppResult<Vec<ChartVersion>> {
    Helm::local()?.search_charts(&query, versions).await
}
