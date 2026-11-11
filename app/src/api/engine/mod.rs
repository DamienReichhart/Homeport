//! The Docker Engine of a Host: what runs on it, Podman's own API and Swarm
//! mode.

pub mod containers;
mod registries;

#[cfg(test)]
mod testing;
