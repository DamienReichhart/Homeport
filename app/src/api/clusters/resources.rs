//! What runs on a Cluster, read-only: workloads, Pods, services, config, nodes
//! and events, and the stream that says when they change.

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_kubernetes::resources::{
    self, ClusterNodeDetail, ClusterNodeView, ClusterServiceDetail, ClusterServiceView,
    ClusterSummary, ConfigMapDetail, ConfigMapView, EventView, IngressDetail, IngressView,
    NamespaceView, PodDetail, PodView, ResourceKind, RevealedValue, SecretDetail, SecretView,
    WorkloadDetail, WorkloadKind, WorkloadView,
};

use super::on;
use crate::app::AppState;

pub async fn clusters_namespaces(state: &AppState, id: String) -> AppResult<Vec<NamespaceView>> {
    on(state, &id)?.namespaces().await
}

/// The workloads of `kind` in `namespace`, or in every Namespace without one.
pub async fn clusters_workloads_list(
    state: &AppState,
    id: String,
    kind: WorkloadKind,
    namespace: Option<String>,
) -> AppResult<Vec<WorkloadView>> {
    on(state, &id)?.workloads(kind, namespace.as_deref()).await
}

pub async fn clusters_workload_get(
    state: &AppState,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
) -> AppResult<WorkloadDetail> {
    on(state, &id)?.workload(kind, &namespace, &name).await
}

/// The Jobs a CronJob started, newest first.
pub async fn clusters_cron_job_jobs(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<Vec<WorkloadView>> {
    on(state, &id)?.jobs_of(&namespace, &name).await
}

/// The Pods in `namespace` (every one without), those matching a label
/// `selector` or on a `node` when given.
pub async fn clusters_pods_list(
    state: &AppState,
    id: String,
    namespace: Option<String>,
    selector: Option<String>,
    node: Option<String>,
) -> AppResult<Vec<PodView>> {
    on(state, &id)?
        .pods(namespace.as_deref(), selector.as_deref(), node.as_deref())
        .await
}

pub async fn clusters_pod_get(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<PodDetail> {
    on(state, &id)?.pod(&namespace, &name).await
}

/// The Events about one object, warnings first.
pub async fn clusters_object_events(
    state: &AppState,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
) -> AppResult<Vec<EventView>> {
    on(state, &id)?
        .object_events(kind, namespace.as_deref(), &name)
        .await
}

/// An object as read-only YAML, without managedFields or Secret values.
pub async fn clusters_resource_yaml(
    state: &AppState,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
) -> AppResult<String> {
    on(state, &id)?
        .yaml(kind, namespace.as_deref(), &name)
        .await
}

/// Told when the resources a list shows changed.
#[derive(Debug, Clone)]
pub struct ResourcesChanged {
    pub kind: ResourceKind,
}

/// Watches the resources of `kind` in `namespace` (every one without) and
/// says when they change, until cancelled.
pub async fn clusters_watch_subscribe(
    state: &AppState,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    on_event: impl Fn(ResourcesChanged) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let (client, _) = state.clusters.registry.client(&id)?;
    Ok(state
        .streams
        .spawn(resources::watch(client, kind, namespace, move || {
            on_event(ResourcesChanged { kind })
        })))
}

pub async fn clusters_services_list(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<ClusterServiceView>> {
    on(state, &id)?.services(namespace.as_deref()).await
}

pub async fn clusters_service_get(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<ClusterServiceDetail> {
    on(state, &id)?.service(&namespace, &name).await
}

pub async fn clusters_ingresses_list(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<IngressView>> {
    on(state, &id)?.ingresses(namespace.as_deref()).await
}

pub async fn clusters_ingress_get(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<IngressDetail> {
    on(state, &id)?.ingress(&namespace, &name).await
}

pub async fn clusters_config_maps_list(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<ConfigMapView>> {
    on(state, &id)?.config_maps(namespace.as_deref()).await
}

pub async fn clusters_config_map_get(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<ConfigMapDetail> {
    on(state, &id)?.config_map(&namespace, &name).await
}

/// Secrets with their keys and sizes, never their values.
pub async fn clusters_secrets_list(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<SecretView>> {
    on(state, &id)?.secrets(namespace.as_deref()).await
}

/// A Secret's keys, labels and annotations, never its values.
pub async fn clusters_secret_get(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<SecretDetail> {
    on(state, &id)?.secret(&namespace, &name).await
}

/// One decoded value of a Secret, read when the user reveals it; neither
/// cached nor logged.
pub async fn clusters_secret_reveal(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
    key: String,
) -> AppResult<RevealedValue> {
    on(state, &id)?.reveal(&namespace, &name, &key).await
}

pub async fn clusters_nodes_list(state: &AppState, id: String) -> AppResult<Vec<ClusterNodeView>> {
    on(state, &id)?.nodes().await
}

pub async fn clusters_node_get(
    state: &AppState,
    id: String,
    name: String,
) -> AppResult<ClusterNodeDetail> {
    on(state, &id)?.node(&name).await
}

/// The Events in `namespace` (every one without), warnings first.
pub async fn clusters_events_list(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<Vec<EventView>> {
    on(state, &id)?.events(namespace.as_deref()).await
}

/// The Cluster `id` at a glance (Nodes, Pods by phase, workloads short of
/// Pods, the latest warnings), for the Overview; `namespace` (every one
/// without) for what lives in Namespaces.
pub async fn clusters_summary(
    state: &AppState,
    id: String,
    namespace: Option<String>,
) -> AppResult<ClusterSummary> {
    Ok(on(state, &id)?.summary(namespace.as_deref()).await)
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::super::tests::{failed, gone, kubernetes, unknown};
    use super::*;
    use crate::app::testing::app;

    fn ns() -> String {
        "default".into()
    }

    fn name() -> String {
        "web".into()
    }

    #[tokio::test]
    async fn an_unknown_cluster_has_nothing_to_list() {
        let app = app();
        let s = &app.state;
        let kind = WorkloadKind::Deployment;
        assert!(gone(clusters_namespaces(s, unknown()).await));
        assert!(gone(
            clusters_workloads_list(s, unknown(), kind, None).await
        ));
        assert!(gone(
            clusters_pods_list(s, unknown(), None, None, None).await
        ));
        assert!(gone(clusters_services_list(s, unknown(), None).await));
        assert!(gone(clusters_ingresses_list(s, unknown(), None).await));
        assert!(gone(clusters_config_maps_list(s, unknown(), None).await));
        assert!(gone(clusters_secrets_list(s, unknown(), None).await));
        assert!(gone(clusters_nodes_list(s, unknown()).await));
        assert!(gone(clusters_events_list(s, unknown(), None).await));
        assert!(gone(clusters_summary(s, unknown(), None).await));
        assert!(gone(
            clusters_cron_job_jobs(s, unknown(), ns(), name()).await
        ));
    }

    #[tokio::test]
    async fn an_unknown_cluster_has_no_object_to_show() {
        let app = app();
        let s = &app.state;
        let pod = ResourceKind::Pod;
        let kind = WorkloadKind::Deployment;
        assert!(gone(
            clusters_workload_get(s, unknown(), kind, ns(), name()).await
        ));
        assert!(gone(clusters_pod_get(s, unknown(), ns(), name()).await));
        assert!(gone(
            clusters_object_events(s, unknown(), pod, None, name()).await
        ));
        assert!(gone(
            clusters_resource_yaml(s, unknown(), pod, None, name()).await
        ));
        assert!(gone(clusters_service_get(s, unknown(), ns(), name()).await));
        assert!(gone(clusters_ingress_get(s, unknown(), ns(), name()).await));
        assert!(gone(
            clusters_config_map_get(s, unknown(), ns(), name()).await
        ));
        assert!(gone(clusters_secret_get(s, unknown(), ns(), name()).await));
        assert!(gone(
            clusters_secret_reveal(s, unknown(), ns(), name(), "key".into()).await
        ));
        assert!(gone(clusters_node_get(s, unknown(), name()).await));
        let watch = clusters_watch_subscribe(s, unknown(), pod, None, |_| {
            panic!("an unknown Cluster's resources changed")
        });
        assert!(gone(watch.await));
    }

    #[tokio::test]
    async fn a_cluster_that_is_not_connected_says_so() {
        let app = app();
        let id = kubernetes(&app, "dev").await.cluster.id;
        let not_connected = ErrorCode::ClusterNotConnected;
        assert!(failed(
            clusters_namespaces(&app.state, id.clone()).await,
            not_connected
        ));
        assert!(failed(
            clusters_summary(&app.state, id, None).await,
            not_connected
        ));
    }
}
