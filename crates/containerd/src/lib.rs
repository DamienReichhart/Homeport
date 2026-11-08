//! containerd Hosts (ADR 0020): containerd's gRPC API over the Host's
//! dialer, for its namespaces, containers and tasks, images and snapshots,
//! and commands on the Host's machine for what the API leaves to its
//! clients, logs and exec.
//!
//! Every call but the namespaces' own names the namespace it's about
//! ([`client::call`]); the frontend remembers which one each Host shows.

pub mod api;
mod client;
mod error;
pub mod events;
pub mod introspection;
pub mod messages;
pub mod namespaces;

pub use client::{Containerd, MIN_VERSION, PULL_VERSION};
