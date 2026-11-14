//! The Run dialog (`ui/dialogs/RunContainerDialog.tsx` and `run/`):
//! create and start a container from an image, or from another
//! container's settings (Duplicate, Edit). What `@/features/containers`
//! exported to the Images and pods pages, so it is the domain's.
//!
//! - [`model`]: the form as typed, what can't be read in it, and the Run
//!   request it makes: the crate's `RunSpec`, as the Tauri command took.

pub mod model;
