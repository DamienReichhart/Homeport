//! A container's page (`ui/detail/ContainerDetailPage.tsx`).

mod message;
mod state;
mod tabs;
mod update;
mod view;

#[cfg(test)]
mod tests;

use iced::Subscription;

use self::state::Tab;
pub use self::{
    message::Message,
    state::State,
    update::{open, refresh, request, update},
    view::{dialog, view},
};
use crate::ui::shell::Context;

/// What the tab that shows follows: the container's logs or its stats,
/// while it is there. A tab that is left, a page that is left and a
/// container that is gone let go of their stream, which cancels it.
pub fn subscription(state: &State, ctx: &Context<'_>) -> Subscription<Message> {
    let Some(container) = state.container(ctx) else {
        return Subscription::none();
    };
    match state.tab {
        Tab::Logs => Subscription::batch([
            state
                .logs
                .subscription(ctx.state, &state.host_id, &container.id)
                .map(Message::Logs),
            iced::window::resize_events().map(|(_, size)| Message::Resized(size)),
        ]),
        _ => Subscription::none(),
    }
}
