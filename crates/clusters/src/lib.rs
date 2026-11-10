//! Clusters: the Kubernetes and Nomad APIs Homeport connects to (see `GLOSSARY.md`).
//!
//! - [`model`] is what a Cluster is: its clear connection and its sealed credentials.
//! - [`store`] persists them, their credentials sealed (ADR 0002).
//! - [`registry`] keeps one supervised connection per Cluster and reports its
//!   status through a [`registry::ClusterNotifier`].
//! - [`client`] builds a kube-rs client in memory; [`classify`] explains its failures.
//! - [`nomad`] turns the Nomad form into a connection and a `homeport-nomad` client.
//! - [`metrics`] reads Pods' and Nodes' CPU and memory from the Metrics API
//!   metrics-server serves, polled while a page is open.
//! - [`service`] holds the use cases the app exposes.
//! - [`rbac`] reads RBAC objects and computes what a subject may do.
//! - [`proxy`] lets a CLI reach a Cluster with no credential on disk, which
//!   [`helm`] runs through (ADR 0009).
//!
//! Homeport installs nothing on a Cluster: it only talks to the Kubernetes
//! API the kubeconfig already reaches, or the Nomad API at the address given.

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
pub mod nomad;
pub mod proxy;
pub mod rbac;
pub mod registry;
pub mod resources;
pub mod service;
pub mod store;
pub mod terminal;

pub use model::{
    AuthMethod, Cluster, ClusterConnection, ClusterId, ClusterKind, ClusterSecrets,
    KubernetesConnection, KubernetesSecrets, NomadConnection, NomadSecrets,
};
