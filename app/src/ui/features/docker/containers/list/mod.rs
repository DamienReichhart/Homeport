//! The Containers page (`ui/list/ContainersPage.tsx`): a Host's
//! containers nested under their Stacks, with their live CPU and memory,
//! the "newer image" marks, the search, the lifecycle of one container or
//! of a selection, Rename and Remove.
//!
//! A press on a row, Enter on it, or Logs and Open terminal in its menu
//! lead to the container's page (`../detail`). Export in its menu saves
//! the container's filesystem (`ui/domain/containers::actions::export`).
//!
//! What it leaves to other tickets: the Run dialog (#259), for which a
//! stand-in shows its title over "Nothing here.", and, in a container's
//! menu, Edit, Duplicate and Commit (#259), which are there and can't be
//! chosen.
//!
//! What differs from the React page, on purpose:
//!
//! - the columns sort (React's table had no sort); the containers of a
//!   Stack start by name, as React nested them;
//! - names are compared as the kit's tables compare text (whatever the
//!   case, digits by their value) where React used `localeCompare`: the
//!   same order for the names Docker allows, but for `web-10` after
//!   `web-2`;
//! - a selected container that left the list is let go of by the table,
//!   where React kept its id and sent it to the Engine; one the search
//!   hides stays selected and is acted on, as in React;
//! - a failed action is one toast, as React's, whose Details list every
//!   container that failed, where React's showed the first one's only;
//! - the health is a badge under the status and is not said in the status
//!   too (`Up 3 hours`, not `Up 3 hours (healthy)`), as on the Board, so
//!   the status reads whole in its column; React put the badge beside it
//!   and let the cell grow;
//! - what lacks room wraps onto the row's second line, then ends with an
//!   ellipsis and is whole under the pointer.

mod message;
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

/// The usage of the Host's running containers, while the page shows.
pub fn subscription(state: &State, ctx: &Context<'_>) -> Subscription<Message> {
    containers::usage(ctx.state, &state.host_id).map(Message::Usage)
}
