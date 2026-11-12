//! `Checkbox` as the React one that takes three values: off, on, and
//! `indeterminate` over a list of which only some rows are selected. It is
//! a [`Pressable`], so Tab reaches it and Space presses it, which iced's
//! own checkbox (two values, no keyboard) does not do.
//!
//! ```ignore
//! checkbox(Check::Mixed).on_press(Message::SelectAll(true))
//! ```

use iced::widget::{container, Space};

use crate::ui::{
    theme::{self, icon, metrics::control, Element, Icon},
    widgets::focus::{pressable, Pressable},
};

/// `size-3.5`: the mark inside the box.
const MARK: f32 = 14.0;

/// What a checkbox shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Check {
    Off,
    On,
    /// Some of what it stands for: the box stays empty and shows the mark,
    /// as the React one did.
    Mixed,
}

/// A checkbox showing `check`. Give it [`Pressable::on_press`] with the
/// message of the press: the page decides what the next value is.
pub fn checkbox<'a, Message: Clone + 'a>(check: Check) -> Pressable<'a, Message> {
    let mark: Element<'a, Message> = match check {
        Check::Off => Space::new().into(),
        Check::On | Check::Mixed => icon(Icon::Check).width(MARK).height(MARK).into(),
    };
    pressable(container(mark).center(control::CHECKBOX))
        .padding(0.0)
        .width(control::CHECKBOX)
        .height(control::CHECKBOX)
        .style(theme::button::part::checkbox(check == Check::On))
}
