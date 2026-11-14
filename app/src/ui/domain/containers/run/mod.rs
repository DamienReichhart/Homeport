//! The Run dialog (`ui/dialogs/RunContainerDialog.tsx` and `run/`):
//! create and start a container from an image, or from another
//! container's settings (Duplicate, Edit). What `@/features/containers`
//! exported to the Images and pods pages, so it is the domain's.
//!
//! - [`model`]: the form as typed, what can't be read in it, and the Run
//!   request it makes: the crate's `RunSpec`, as the Tauri command took;
//! - [`form`]: the fields as the page keeps them (Run, Duplicate and
//!   Edit show the same), drawn by `view`;
//! - [`dialog`]: the Run dialog, for Run and Duplicate.

pub mod dialog;
pub mod form;
pub mod model;
mod parts;
mod view;

pub use self::{
    dialog::{Dialog, Step},
    form::Form,
    view::fields,
};

#[cfg(all(test, unix))]
mod tests;
