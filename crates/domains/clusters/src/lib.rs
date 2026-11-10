//! Clusters: the Kubernetes and Nomad APIs Homeport connects to (see `GLOSSARY.md`).
//!
//! - [`model`] is what a Cluster is: its clear connection and its sealed credentials.
//! - [`store`] persists them, their credentials sealed (ADR 0002).
//! - [`registry`] keeps one supervised connection per Cluster and reports its
//!   status through a [`registry::ClusterNotifier`].
//! - [`nomad`] turns the Nomad form into a connection and a `homeport-nomad` client.
//! - [`service`] holds the use cases the app exposes.
//!
//! What a Cluster's API offers is in the crate of its kind:
//! `homeport-kubernetes` and `homeport-nomad`.

pub mod messages;
pub mod model;
pub mod nomad;
pub mod registry;
pub mod service;
pub mod store;

pub use homeport_kubernetes::{AuthMethod, KubernetesConnection, KubernetesSecrets};
pub use model::{
    Cluster, ClusterConnection, ClusterId, ClusterKind, ClusterSecrets, NomadConnection,
    NomadSecrets,
};
