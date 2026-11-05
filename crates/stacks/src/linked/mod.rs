//! Linked Stacks (ADR 0008): Stacks whose compose files are a folder on this
//! machine. Homeport keeps where they are, never a copy of them.
//!
//! - [`store`] keeps each Linked Stack's folder, files and fingerprint.
//! - [`detect`] tells when a Stack found running comes from files here.
//! - [`files`] fingerprints the files, to tell when they changed.

pub mod detect;
pub mod files;
pub mod store;

pub use self::{
    detect::FileCache,
    store::{LinkedStack, LinkedStore},
};
