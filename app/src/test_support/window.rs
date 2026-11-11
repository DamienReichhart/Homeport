//! A view without a window, for a test that presses and types: what
//! Testing Library's `render` and `userEvent` were.
//!
//! ```ignore
//! let messages = drive((400.0, 300.0), || page.view(), |window| {
//!     window.click("Remove").unwrap();
//!     window.tap_key(key::Named::Enter);
//! });
//! for message in messages {
//!     page.update(message);
//! }
//! ```

use iced::Size;
use iced_test::Simulator;

use super::snapshot::settings;
use crate::ui::theme::{Element, Theme};

/// A view laid out as in a window of some size.
pub type Window<'a, Message> = Simulator<'a, Message, Theme, iced::Renderer>;

pub fn window<'a, Message: 'a>(
    size: impl Into<Size>,
    view: impl Into<Element<'a, Message>>,
) -> Window<'a, Message> {
    Simulator::with_size(settings(), size, view)
}

/// Lays `view` out at `size`, lets `act` use it, and returns the messages
/// that produced, in order, to give to `update`.
pub fn drive<'a, Message: 'a>(
    size: impl Into<Size>,
    view: impl FnOnce() -> Element<'a, Message>,
    act: impl FnOnce(&mut Window<'a, Message>),
) -> Vec<Message> {
    let mut window = window(size, view());
    act(&mut window);
    window.into_messages().collect()
}
