//! Linked Stacks (ADR 0008): Stacks whose compose files are a folder on this
//! machine. Homeport keeps where they are, never a copy of them.
//!
//! - [`store`] keeps each Linked Stack's folder, files and fingerprint.

pub mod store;

pub use self::store::{LinkedStack, LinkedStore};
