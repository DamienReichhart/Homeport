//! The Docker Engine of a Host: what runs on it, Podman's own API and Swarm
//! mode.

pub mod containers;
pub mod exec;
pub mod files;
mod registries;

#[cfg(test)]
mod testing;
