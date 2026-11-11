//! The Docker Engine of a Host: what runs on it, Podman's own API and Swarm
//! mode.

pub mod containers;
pub mod exec;
pub mod files;
pub mod images;
pub mod networks;
pub mod podman;
mod registries;
pub mod system;
pub mod volumes;

#[cfg(test)]
mod testing;
