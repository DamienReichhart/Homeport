//! A container's page (`ui/detail/ContainerDetailPage.tsx`).

mod message;
mod model;
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
    let stream = match state.tab {
        Tab::Logs => state
            .logs
            .subscription(ctx.state, &state.host_id, &container.id)
            .map(Message::Logs),
        // Only a running container has stats (`useStats`'s `running`).
        Tab::Stats if container.state == "running" => state
            .stats
            .subscription(ctx.state, &state.host_id, &container.id)
            .map(Message::Stats),
        _ => Subscription::none(),
    };
    // The window's size lays the page out: how tall the logs are, how
    // many charts a row holds.
    let resized = iced::window::resize_events().map(|(_, size)| Message::Resized(size));
    Subscription::batch([stream, resized])
}
