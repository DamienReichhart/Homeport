//! Clusters: the Kubernetes APIs Homeport connects to, each supervised
//! like a Host.

use tauri::{ipc::Channel, State};

use homeport_clusters::resources::{
    self, EventView, NamespaceView, On, PodDetail, PodView, ResourceKind, WorkloadDetail,
    WorkloadKind, WorkloadView,
};
use homeport_clusters::{
    detect::{self, LocalClusters},
    kubeconfig::{ContextChoice, KubeContexts},
    service::ClusterView,
};
use homeport_core::{host::HostConnection, streams::StreamId};
use homeport_engine::containers;

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
    let found = detect::detect(&state.clusters.store.list()?).await;
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
fn on(state: &AppState, id: &str) -> AppResult<On> {
    let (client, cluster) = state.clusters.registry.client(id)?;
    Ok(On { client, cluster })
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_namespaces(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Vec<NamespaceView>> {
    on(&state, &id)?.namespaces().await
}

/// The workloads of `kind` in `namespace`, or in every Namespace without one.
#[tauri::command]
#[specta::specta]
pub async fn clusters_workloads_list(
    state: State<'_, AppState>,
    id: String,
    kind: WorkloadKind,
    namespace: Option<String>,
) -> AppResult<Vec<WorkloadView>> {
    on(&state, &id)?.workloads(kind, namespace.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_workload_get(
    state: State<'_, AppState>,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
) -> AppResult<WorkloadDetail> {
    on(&state, &id)?.workload(kind, &namespace, &name).await
}

/// The Jobs a CronJob started, newest first.
#[tauri::command]
#[specta::specta]
pub async fn clusters_cron_job_jobs(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<Vec<WorkloadView>> {
    on(&state, &id)?.jobs_of(&namespace, &name).await
}

/// The Pods in `namespace` (every one without), those matching a label
/// `selector` or on a `node` when given.
#[tauri::command]
#[specta::specta]
pub async fn clusters_pods_list(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
    selector: Option<String>,
    node: Option<String>,
) -> AppResult<Vec<PodView>> {
    on(&state, &id)?
        .pods(namespace.as_deref(), selector.as_deref(), node.as_deref())
        .await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_pod_get(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<PodDetail> {
    on(&state, &id)?.pod(&namespace, &name).await
}

/// The Events about one object, warnings first.
#[tauri::command]
#[specta::specta]
pub async fn clusters_object_events(
    state: State<'_, AppState>,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
) -> AppResult<Vec<EventView>> {
    on(&state, &id)?
        .object_events(kind, namespace.as_deref(), &name)
        .await
}

/// An object as read-only YAML, without managedFields or Secret values.
#[tauri::command]
#[specta::specta]
pub async fn clusters_resource_yaml(
    state: State<'_, AppState>,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
) -> AppResult<String> {
    on(&state, &id)?
        .yaml(kind, namespace.as_deref(), &name)
        .await
}

/// Told when the resources a list shows changed.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesChanged {
    pub kind: ResourceKind,
}

/// Watches the resources of `kind` in `namespace` (every one without) and
/// says when they change, until cancelled.
#[tauri::command]
#[specta::specta]
pub async fn clusters_watch_subscribe(
    state: State<'_, AppState>,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    on_event: Channel<ResourcesChanged>,
) -> AppResult<StreamId> {
    let (client, _) = state.clusters.registry.client(&id)?;
    Ok(state
        .streams
        .spawn(resources::watch(client, kind, namespace, move || {
            on_event.send(ResourcesChanged { kind }).is_ok()
        })))
}
