//! A container's page (`ui/detail/ContainerDetailPage.tsx`).

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
use crate::ui::shell::Context;

pub fn subscription(_state: &State, _ctx: &Context<'_>) -> Subscription<Message> {
    Subscription::none()
}
