//! The Board (`src/features/board`): the containers of every connected
//! Host in one table, filtered by Host, state, Stack and search, sorted
//! by any column but the ports, with each Host's live usage and the
//! lifecycle of one container or of a selection that spans Hosts.
//!
//! What differs from the React page, on purpose:
//!
//! - the selection's buttons stand in the page's header, not after the
//!   filters: the filters fill their line at the window's first size, and
//!   React's wrapped under them and moved the rows;
//! - the notice of a Host whose list failed has a Retry;
//! - the Hosts' menu closes when a Host is chosen (React's stayed open);
//! - ties and the order under a column's sort are by Host then by name as
//!   the kit's tables compare text, where React used `localeCompare`;
//! - a confirmation spells out at most eight names a Host, then "…";
//! - at the window's first size the ten columns are at their least and
//!   their text is cut with an ellipsis (React's table scrolled sideways).
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
