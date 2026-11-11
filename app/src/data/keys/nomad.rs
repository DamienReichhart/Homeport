//! Keys of what Homeport reads from a Nomad Cluster, under one root per
//! Cluster (`nomadKeys` in React).

use crate::data::Key;

/// Everything read from one Nomad Cluster.
pub fn scope(cluster_id: &str) -> Key {
    Key::new(["nomad", cluster_id])
}

/// The jobs of `namespace`, or of every one.
pub fn jobs(cluster_id: &str, namespace: Option<&str>) -> Key {
    scope(cluster_id).with("jobs").with_optional(namespace)
}

pub fn job(cluster_id: &str, namespace: &str, job_id: &str) -> Key {
    scope(cluster_id).with("job").with(namespace).with(job_id)
}

pub fn allocation(cluster_id: &str, alloc_id: &str) -> Key {
    scope(cluster_id).with("allocation").with(alloc_id)
}

pub fn nodes(cluster_id: &str) -> Key {
    scope(cluster_id).with("nodes")
}

pub fn node(cluster_id: &str, node_id: &str) -> Key {
    scope(cluster_id).with("node").with(node_id)
}

pub fn namespaces(cluster_id: &str) -> Key {
    scope(cluster_id).with("namespaces")
}
