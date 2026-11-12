//! What several features show of one domain: a container's tone, its
//! ports, its menu and what the menu does, the "newer image" mark, the pod
//! a container is in. React had these in `core/host` and in the public
//! API of a feature (`@/features/containers`); a feature here never uses
//! another, so what two of them share lives under the features and over
//! the shell, the kit and the data layer.
//!
//! Nothing here is a page: the features build theirs from it.

pub mod containers;
pub mod freshness;
pub mod pods;
