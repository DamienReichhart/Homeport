//! Clusters: the Kubernetes APIs Homeport connects to (see `GLOSSARY.md`).
//!
//! - [`model`] is what a Cluster is: its clear connection and its sealed credentials.
//! - [`store`] persists them, their credentials sealed (ADR 0002).
//! - [`registry`] keeps one supervised connection per Cluster and reports its
//!   status through a [`registry::ClusterNotifier`].
//! - [`client`] builds a kube-rs client in memory; [`classify`] explains its failures.
//! - [`service`] holds the use cases the app exposes.
//!
//! Homeport installs nothing on a Cluster: it only talks to the Kubernetes
//! API the kubeconfig already reaches.

pub mod classify;
pub mod client;
pub mod detect;
pub mod exec;
pub mod kubeconfig;
pub mod logs;
pub mod messages;
pub mod model;
pub mod registry;
pub mod resources;
pub mod service;
pub mod store;
pub mod terminal;

pub use model::{AuthMethod, Cluster, ClusterConnection, ClusterId, ClusterKind, ClusterSecrets};
