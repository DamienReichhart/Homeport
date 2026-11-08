//! containerd Hosts (ADR 0020): containerd's gRPC API over the Host's
//! dialer, for its namespaces, containers and tasks, images and snapshots,
//! and commands on the Host's machine for what the API leaves to its
//! clients, logs and exec.
//!
//! Every call but the namespaces' own names the namespace it's about
//! ([`client::call`]); the frontend remembers which one each Host shows.

pub mod api;
mod client;
pub mod containers;
mod error;
pub mod events;
pub mod exec;
pub mod introspection;
pub mod logs;
pub mod messages;
pub mod namespaces;
#[cfg(test)]
mod testing;

pub use client::{Containerd, MIN_VERSION, PULL_VERSION};
