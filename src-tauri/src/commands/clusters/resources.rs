//! What runs on a Cluster, read-only: workloads, Pods, services, config, nodes
//! and events, and the stream that says when they change.

use tauri::{ipc::Channel, State};

use homeport_clusters::resources::{
    self, ClusterNodeDetail, ClusterNodeView, ClusterServiceDetail, ClusterServiceView,
    ClusterSummary, ConfigMapDetail, ConfigMapView, EventView, IngressDetail, IngressView,
    NamespaceView, PodDetail, PodView, ResourceKind, RevealedValue, SecretDetail, SecretView,
    WorkloadDetail, WorkloadKind, WorkloadView,
};
use homeport_core::streams::StreamId;

use super::on;
use crate::{app::AppState, error::AppResult};

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

#[tauri::command]
#[specta::specta]
pub async fn clusters_services_list(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<ClusterServiceView>> {
    on(&state, &id)?.services(namespace.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_service_get(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<ClusterServiceDetail> {
    on(&state, &id)?.service(&namespace, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_ingresses_list(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<IngressView>> {
    on(&state, &id)?.ingresses(namespace.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_ingress_get(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<IngressDetail> {
    on(&state, &id)?.ingress(&namespace, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_config_maps_list(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<ConfigMapView>> {
    on(&state, &id)?.config_maps(namespace.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_config_map_get(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<ConfigMapDetail> {
    on(&state, &id)?.config_map(&namespace, &name).await
}

/// Secrets with their keys and sizes, never their values.
#[tauri::command]
#[specta::specta]
pub async fn clusters_secrets_list(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<SecretView>> {
    on(&state, &id)?.secrets(namespace.as_deref()).await
}

/// A Secret's keys, labels and annotations, never its values.
#[tauri::command]
#[specta::specta]
pub async fn clusters_secret_get(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<SecretDetail> {
    on(&state, &id)?.secret(&namespace, &name).await
}

/// One decoded value of a Secret, read when the user reveals it; neither
/// cached nor logged.
#[tauri::command]
#[specta::specta]
pub async fn clusters_secret_reveal(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
    key: String,
) -> AppResult<RevealedValue> {
    on(&state, &id)?.reveal(&namespace, &name, &key).await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_nodes_list(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Vec<ClusterNodeView>> {
    on(&state, &id)?.nodes().await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_node_get(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> AppResult<ClusterNodeDetail> {
    on(&state, &id)?.node(&name).await
}

/// The Events in `namespace` (every one without), warnings first.
#[tauri::command]
#[specta::specta]
pub async fn clusters_events_list(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<EventView>> {
    on(&state, &id)?.events(namespace.as_deref()).await
}

/// The Cluster `id` at a glance (Nodes, Pods by phase, workloads short of
/// Pods, the latest warnings), for the Overview; `namespace` (every one
/// without) for what lives in Namespaces.
#[tauri::command]
#[specta::specta]
pub async fn clusters_summary(
    state: State<'_, AppState>,
    id: String,
    namespace: Option<String>,
) -> AppResult<ClusterSummary> {
    Ok(on(&state, &id)?.summary(namespace.as_deref()).await)
}
