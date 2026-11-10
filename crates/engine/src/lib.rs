//! Engine resources (containers, images, volumes, networks, …) as plain
//! functions over a connected [`bollard::Docker`] client, returning DTOs
//! shaped for the frontend.

pub mod auth;
pub mod build;
pub mod containers;
pub mod freshness;
pub mod images;
pub mod messages;
pub mod networks;
pub mod podman;
pub mod swarm;
pub mod system;
pub mod transfer;
pub mod volumes;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;

/// The client of a Host's Docker Engine, as the crates above name it.
pub use bollard::Docker;

/// Compose labels that tie a container to its Stack.
pub const COMPOSE_PROJECT: &str = "com.docker.compose.project";
pub const COMPOSE_SERVICE: &str = "com.docker.compose.service";
