//! The keys of everything the app caches, one module per family of the
//! React app's query keys: what was `hostKeys.containers(hostId)` is
//! `keys::host::containers(host_id)`, with the same parts in the same order.
//!
//! Every key of a Host starts with [`host::scope`], of a Cluster with
//! [`cluster::scope`] (a Nomad Cluster's with [`nomad::scope`]) and of a
//! Swarm with [`swarm::scope`], so one invalidation refreshes all of it. A
//! feature files what only it reads under one of these with
//! [`Key::with`](super::Key::with).

pub mod app;
pub mod cluster;
pub mod host;
pub mod nomad;
pub mod stacks;
pub mod swarm;

#[cfg(test)]
mod tests;
