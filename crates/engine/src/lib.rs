//! Engine resources (containers, images, volumes, networks, …) as plain
//! functions over a connected [`bollard::Docker`] client, returning DTOs
//! shaped for the frontend.

pub mod containers;
pub mod images;
pub mod networks;
pub mod system;
pub mod volumes;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;

/// Compose labels that tie a container to its Stack.
pub const COMPOSE_PROJECT: &str = "com.docker.compose.project";
pub const COMPOSE_SERVICE: &str = "com.docker.compose.service";
