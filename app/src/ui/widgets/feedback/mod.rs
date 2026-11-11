//! Failures and feedback, as `src/shared/ui/feedback` showed them. One
//! error type everywhere: an `AppError` is shown by these widgets and never
//! as a bare string.
//!
//! - a failure that reaches the user becomes a [`Surfaced`], which logs it
//!   with the name of the use case that failed;
//! - in place: [`error_message`] (with Details and Retry); in passing:
//!   [`Toasts::failure`]; in full: the Details dialog ([`Details`]);
//! - before what can't be undone: [`confirm_dialog`];
//! - while it loads: [`spinner`] and [`skeleton`]; when there is nothing:
//!   [`empty_state`]; what state it is in: [`status_dot`].

pub mod confirm;
pub mod error_details;
pub mod error_message;
pub mod hint_view;
pub mod placeholder;
pub mod spinner;
pub mod surfaced;
pub mod toast;

pub use self::{
    confirm::{confirm_dialog, Ask},
    error_details::Details,
    error_message::error_message,
    hint_view::{copy, copy_button, hint_view},
    placeholder::{empty_state, skeleton, status_dot},
    spinner::{spinner, spinner_saying, turning},
    surfaced::{About, Surfaced},
    toast::Toasts,
};

#[cfg(test)]
mod tests;
