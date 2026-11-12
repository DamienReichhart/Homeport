//! A container as every list shows it (the Containers page, the Board,
//! and the lists of a Stack or a pod when their tickets come): what
//! `@/features/containers` exported to the other features.
//!
//! - [`model`]: its tone, its ports, its Stack, the search and its usage;
//! - [`actions`]: its start and stop button, its menu, and the dialogs
//!   and confirmations they open (`ContainerRowMenu`).

pub mod actions;
pub mod model;

pub use self::actions::{Actions, Bulk, Target};
