//! What floats over the page. Two things only:
//!
//! - [`anchored`]: a panel beside what it belongs to. [`popover`],
//!   [`tooltip`], the dropdown [`menu`] and the select's list are all one,
//!   so placement and the window's edges are solved once ([`position`]);
//! - [`modal`]: a dialog over a scrim, with the keyboard to itself;
//!   [`dialog`] lays out its title, body and footer.
//!
//! A page never writes an overlay of its own.

pub mod anchored;
pub mod dialog;
pub mod menu;
pub mod modal;
mod panel;
pub mod position;

use std::time::Duration;

use iced::widget::{container, text};

use crate::{
    i18n,
    ui::theme::{
        self,
        metrics::{space, text as size},
        Element,
    },
};

pub use self::{
    anchored::{anchored, hovered, Anchored},
    dialog::{dialog, Dialog},
    modal::{modal, over, Modal},
    position::{Align, Placement, Side},
};

/// `w-72`: a popover's width.
pub const POPOVER_WIDTH: f32 = 288.0;

/// How long the pointer rests on something before its tooltip shows. The
/// React app showed it at once; a short rest keeps a sweep of the pointer
/// across a toolbar quiet. Keyboard focus shows it at once.
pub const TOOLTIP_DELAY: Duration = Duration::from_millis(300);

/// `Popover`: `content` in a panel under `anchor` while `open`, lined up
/// where text starts. Escape and a press outside send `on_dismiss`.
pub fn popover<'a, Message: Clone + 'a>(
    anchor: impl Into<Element<'a, Message>>,
    open: bool,
    content: impl FnOnce() -> Element<'a, Message>,
    on_dismiss: Message,
) -> Anchored<'a, Message> {
    let panel = open.then(|| {
        container(content())
            .padding(space::S2_5)
            .width(POPOVER_WIDTH)
            .style(theme::container::popover)
            .into()
    });
    let placement = Placement::new(Side::Bottom, Align::start(i18n::direction()));
    anchored(anchor, panel, placement).on_dismiss(on_dismiss)
}

/// `WithShortcut`: a tooltip that says what `anchor` does and the keys
/// that do the same (`["N"]`, `["Ctrl", "K"]`).
pub fn tooltip_with_keys<'a, Message: Clone + 'a>(
    anchor: impl Into<Element<'a, Message>>,
    label: impl Into<String>,
    keys: &[String],
    side: Side,
) -> Anchored<'a, Message> {
    let says = crate::ui::widgets::line(vec![
        text(label.into()).size(size::XS).into(),
        crate::ui::widgets::basic::keys(keys),
    ])
    .spacing(space::S2);
    let panel = container(says)
        .padding([space::S1_5, space::S3])
        .style(theme::container::tooltip);
    hovered(
        anchor,
        panel,
        Placement::new(side, Align::Center),
        TOOLTIP_DELAY,
    )
}

/// `Tooltip`: `label` over `anchor` while the pointer rests on it or it has
/// keyboard focus.
pub fn tooltip<'a, Message: Clone + 'a>(
    anchor: impl Into<Element<'a, Message>>,
    label: impl Into<String>,
    side: Side,
) -> Anchored<'a, Message> {
    let panel = container(text(label.into()).size(size::XS))
        .padding([space::S1_5, space::S3])
        .max_width(320)
        .style(theme::container::tooltip);
    hovered(
        anchor,
        panel,
        Placement::new(side, Align::Center),
        TOOLTIP_DELAY,
    )
}
