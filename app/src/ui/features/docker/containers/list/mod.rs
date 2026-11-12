//! The Containers page (`ui/list/ContainersPage.tsx`): a Host's
//! containers nested under their Stacks, with their live CPU and memory,
//! the "newer image" marks, the search, the lifecycle of one container or
//! of a selection, Rename and Remove.
//!
//! What it leaves to other tickets: the Run dialog (#259) and, in a
//! container's menu, Edit, Duplicate, Commit and Export (#258, #259).

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
