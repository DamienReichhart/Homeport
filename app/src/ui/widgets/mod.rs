//! The kit: what every page is built from, with no knowledge of Hosts or
//! containers. `docs/agents/widgets.md` lists each React primitive and what
//! stands for it here.
//!
//! - [`focus`]: keyboard focus, which iced 0.14 leaves to the app;
//! - [`overlay`]: what floats (dialogs, popovers, menus, tooltips), placed
//!   and kept inside the window in one place;
//! - [`basic`]: buttons, badges, cards, separators, progress, scroll areas;
//! - [`form`]: fields and the controls that go in them;
//! - [`feedback`]: failures, toasts, confirmations, loading and empty states.
//!
//! A row is written start to end and drawn in the language's direction:
//! build rows with [`line`] and align with [`start`] and [`end`], and a
//! page mirrors in Arabic without knowing it.

pub mod basic;
pub mod feedback;
pub mod focus;
pub mod form;
pub mod gallery;
pub mod overlay;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Row},
    Length,
};

use crate::{
    i18n,
    ui::theme::{Element, Theme},
};

/// A row of `items` written start to end, centred on its line.
pub fn line<'a, Message>(items: Vec<Element<'a, Message>>) -> Row<'a, Message, Theme> {
    Row::from_vec(i18n::direction().ordered(items)).align_y(Vertical::Center)
}

/// A column as wide as it may be, whose items start where text starts.
pub fn stack<'a, Message>(items: Vec<Element<'a, Message>>) -> Column<'a, Message, Theme> {
    Column::from_vec(items).width(Length::Fill).align_x(start())
}

/// The side text starts on.
pub fn start() -> Horizontal {
    i18n::direction().pick(Horizontal::Left, Horizontal::Right)
}

/// The side text ends on.
pub fn end() -> Horizontal {
    i18n::direction().pick(Horizontal::Right, Horizontal::Left)
}
