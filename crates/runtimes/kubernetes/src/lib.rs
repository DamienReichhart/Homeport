//! The Kubernetes API of a Cluster, through kube-rs (ADR 0018).
//!
//! - [`model`] is what this crate needs to know of a Cluster: its name, how
//!   its API server is reached and the credentials its kubeconfig gives.
//! - [`kubeconfig`] reads kubeconfigs into them, and finds the clusters that
//!   run on this computer.
//! - [`api`] builds a kube-rs client in memory, runs exec credential plugins
//!   and explains the API's failures.
//! - [`resources`] is what runs on a Cluster; [`apply`] changes it and
//!   [`streams`] holds a Pod's logs, terminals and port forwards.
//! - [`metrics`] reads Pods' and Nodes' CPU and memory from the Metrics API
//!   metrics-server serves, polled while a page is open.
//! - [`rbac`] reads RBAC objects and computes what a subject may do.
//! - [`proxy`] lets a CLI reach a Cluster with no credential on disk, which
//!   [`helm`] runs through (ADR 0009).
//!
//! Homeport installs nothing on a Cluster: it only talks to the Kubernetes
//! API the kubeconfig already reaches.

pub mod api;
pub mod apply;
pub mod helm;
pub mod kubeconfig;
pub mod messages;
pub mod metrics;
pub mod model;
pub mod proxy;
pub mod rbac;
pub mod resources;
pub mod streams;

pub use model::{
    AuthMethod, ExecConfig, KubeCluster, KubeconfigSource, KubernetesConnection, KubernetesSecrets,
};
