//! The Docker Engine of a Host: what runs on it, Podman's own API and Swarm
//! mode.

pub mod containers;
pub mod exec;
pub mod files;
pub mod images;
pub mod networks;
mod registries;

#[cfg(test)]
mod testing;
