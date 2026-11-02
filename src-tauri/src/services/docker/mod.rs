//! Engine resources (containers, images, volumes, networks, …) as plain
//! functions over a connected [`bollard::Docker`] client, returning DTOs
//! shaped for the frontend.

pub mod containers;
pub mod exec;
pub mod logs;
pub mod stats;

#[cfg(test)]
pub mod testing;

/// Compose labels that tie a container to its Stack.
pub const COMPOSE_PROJECT: &str = "com.docker.compose.project";
pub const COMPOSE_SERVICE: &str = "com.docker.compose.service";
