//! Linked Stacks (ADR 0008): Stacks whose compose files are a folder on this
//! machine. Homeport keeps where they are, never a copy of them.
//!
//! - [`store`] keeps each Linked Stack's folder, files and fingerprint.
//! - [`detect`] tells when a Stack found running comes from files here.
//! - [`files`] reads and writes the files, and fingerprints them to tell when
//!   they changed.
//! - [`folder`] tells which files Compose would use in a folder.

pub mod detect;
pub mod files;
pub mod folder;
pub mod store;

pub use self::{
    detect::FileCache,
    store::{LinkedStack, LinkedStore},
};
