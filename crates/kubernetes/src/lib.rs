//! The Kubernetes API of a Cluster, through kube-rs (ADR 0018).
//!
//! - [`model`] is what this crate needs to know of a Cluster: its name, how
//!   its API server is reached and the credentials its kubeconfig gives.
//! - [`kubeconfig`] reads kubeconfigs into them; [`detect`] finds the
//!   clusters that run on this computer.
//! - [`client`] builds a kube-rs client in memory; [`classify`] explains its failures.
//! - [`resources`] is what runs on a Cluster; [`apply`], [`logs`], [`terminal`]
//!   and [`forward`] act on it.
//! - [`metrics`] reads Pods' and Nodes' CPU and memory from the Metrics API
//!   metrics-server serves, polled while a page is open.
//! - [`rbac`] reads RBAC objects and computes what a subject may do.
//! - [`proxy`] lets a CLI reach a Cluster with no credential on disk, which
//!   [`helm`] runs through (ADR 0009).
//!
//! Homeport installs nothing on a Cluster: it only talks to the Kubernetes
//! API the kubeconfig already reaches.

pub mod apply;
pub mod classify;
pub mod client;
pub mod detect;
pub mod discovery;
pub mod exec;
pub mod forward;
pub mod helm;
pub mod kubeconfig;
pub mod logs;
pub mod messages;
pub mod metrics;
pub mod model;
pub mod proxy;
pub mod rbac;
pub mod resources;
pub mod terminal;

pub use model::{
    AuthMethod, ExecConfig, KubeCluster, KubeconfigSource, KubernetesConnection, KubernetesSecrets,
};
