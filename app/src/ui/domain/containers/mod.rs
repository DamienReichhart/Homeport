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

use std::sync::Arc;

use iced::Subscription;

use crate::{
    app::AppState,
    data::streams::{self, shapes, Update},
    ui::widgets::feedback::Surfaced,
};

/// The live CPU and memory of a Host's running containers (`useUsage`):
/// each report, with the Host's id, while a page's `subscription` returns
/// it. A stream that could not start is logged and reports nothing.
pub fn usage(state: &Arc<AppState>, host_id: &str) -> Subscription<(String, model::Usage)> {
    streams::subscription(state, host_id.to_owned(), |source| {
        shapes::usage(source.state.clone(), source.params.clone())
    })
    .with(host_id.to_owned())
    .filter_map(|(host_id, update)| {
        if let Update::Failed(error) = &update {
            drop(Surfaced::new(error.clone(), "usage_subscribe"));
        }
        model::latest_usage(update).map(|usage| (host_id, usage))
    })
}
