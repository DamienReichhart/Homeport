//! Keys of the Clusters and of what each one holds (`clusterKeys`,
//! `helmKeys` and `forwardKeys` in React). Everything read about one kind
//! of resource starts with [`kind`], so its watch refreshes it.

use homeport_kubernetes::resources::{ApiType, ResourceKind};

use crate::data::Key;

/// The Clusters, with their statuses.
pub fn list() -> Key {
    Key::new(["clusters"])
}

/// The kubeconfig contexts this machine offers to import.
pub fn kubeconfig() -> Key {
    list().with("kubeconfig")
}

/// Clusters running on this machine (kind, minikube…), added or not.
pub fn local_clusters() -> Key {
    Key::new(["local-clusters"])
}

/// Everything read from one Cluster.
pub fn scope(cluster_id: &str) -> Key {
    Key::new(["cluster", cluster_id])
}

/// Everything read about one kind of resource, as its watch refreshes it.
pub fn kind(cluster_id: &str, kind: ResourceKind) -> Key {
    scope(cluster_id).with(kind_part(kind))
}

/// The objects of a kind in `namespace`, or in every one.
pub fn objects(cluster_id: &str, of: ResourceKind, namespace: Option<&str>) -> Key {
    kind(cluster_id, of).with("list").with_optional(namespace)
}

/// One object of a kind.
pub fn object(cluster_id: &str, of: ResourceKind, namespace: Option<&str>, name: &str) -> Key {
    kind(cluster_id, of)
        .with("detail")
        .with_optional(namespace)
        .with(name)
}

/// An object's Events, refreshed with the Events' watch.
pub fn object_events(
    cluster_id: &str,
    of: ResourceKind,
    namespace: Option<&str>,
    name: &str,
) -> Key {
    kind(cluster_id, ResourceKind::Event)
        .with("of")
        .with(kind_part(of))
        .with_optional(namespace)
        .with(name)
}

/// Everything read about the objects of an API type, as the API resources
/// pages show them.
pub fn api(cluster_id: &str, api: &ApiType) -> Key {
    scope(cluster_id)
        .with("api")
        .with(&api.group)
        .with(&api.version)
        .with(&api.plural)
}

/// The API types the Cluster serves, as seen from `namespace`.
pub fn api_types(cluster_id: &str, namespace: Option<&str>) -> Key {
    scope(cluster_id).with("api-types").with_optional(namespace)
}

pub fn yaml(cluster_id: &str, of: ResourceKind, namespace: Option<&str>, name: &str) -> Key {
    kind(cluster_id, of)
        .with("yaml")
        .with_optional(namespace)
        .with(name)
}

/// The Overview's counts, in `namespace` or in every one.
pub fn summary(cluster_id: &str, namespace: Option<&str>) -> Key {
    scope(cluster_id).with("summary").with_optional(namespace)
}

/// The Cluster's RBAC, read-only.
pub fn rbac(cluster_id: &str) -> Key {
    scope(cluster_id).with("rbac")
}

/// The Cluster's Helm releases, and each one's history and details under
/// [`helm_release`].
pub fn helm(cluster_id: &str) -> Key {
    scope(cluster_id).with("helm")
}

pub fn helm_release(cluster_id: &str, namespace: &str, name: &str) -> Key {
    helm(cluster_id).with("release").with(namespace).with(name)
}

/// helm's own chart repositories on this machine, the same for every
/// Cluster.
pub fn helm_repositories() -> Key {
    Key::new(["helm-repositories"])
}

/// Every active port forward, whichever Cluster it reaches.
pub fn port_forwards() -> Key {
    Key::new(["port-forwards"])
}

/// A kind as the frontend named it (`statefulSet`), which is how it is
/// serialised.
fn kind_part(kind: ResourceKind) -> String {
    serde_json::to_value(kind)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| format!("{kind:?}"))
}
