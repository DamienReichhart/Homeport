//! The Board (`src/features/board`): the containers of every connected
//! Host in one table, filtered by Host, state, Stack and search, sorted
//! by any column but the ports, with each Host's live usage and the
//! lifecycle of one container or of a selection that spans Hosts.
//!
//! Not behind a Host's gate: it shows every Host at once, names those it
//! can't show, and watches the Hosts that connect while it is open.

mod message;
mod model;
mod state;
mod update;
mod view;

#[cfg(test)]
mod tests;

use iced::Subscription;

pub use self::{
    message::Message,
    state::State,
    update::{open, refresh, request, update},
    view::{dialog, view},
};
use crate::ui::{domain::containers, shell::Context};

/// The usage of every connected Host the filter shows (`UsageFeed`): a
/// Host's stream stops when it disconnects or is filtered out.
pub fn subscription(state: &State, ctx: &Context<'_>) -> Subscription<Message> {
    let feeds = state
        .sources
        .iter()
        .filter(|source| state.filter.shows_host(&source.host_id))
        .map(|source| containers::usage(ctx.state, &source.host_id));
    Subscription::batch(feeds).map(Message::Usage)
}
