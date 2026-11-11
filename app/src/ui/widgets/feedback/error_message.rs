//! `ErrorMessage`: a failure shown in place. What happened, what to do, and
//! buttons for the raw details and (when the page can redo the work)
//! another try.

use homeport_core::error::AppError;
use iced::{
    alignment::Vertical,
    widget::{container, text},
    Length,
};

use super::hint_view::hint_view;
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font, icon,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            line, stack, start,
        },
    },
};

/// What the user asked of a failure shown in place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Open the Details dialog for it.
    Details,
    /// Do again what failed.
    Retry,
    /// Put this command of its hint on the clipboard.
    Copy(String),
}

/// `error` in place. `retry` adds the Retry button; the page maps each
/// [`Event`] to its own message.
pub fn error_message<'a, Message: Clone + 'a>(
    error: &AppError,
    retry: bool,
    on_event: impl Fn(Event) -> Message,
) -> Element<'a, Message> {
    let title = text(i18n::errors::message(error))
        .size(size::SM)
        .line_height(size::LEADING_SM)
        .font(font::SANS_MEDIUM)
        .style(theme::text::destructive)
        .width(Length::Fill)
        .align_x(start());
    let mut body: Vec<Element<'a, Message>> = vec![title.into()];
    if let Some(hint) = &error.hint {
        body.push(hint_view(hint, None, |index| {
            on_event(Event::Copy(hint.commands[index].clone()))
        }));
    }
    let small = |button: button::Button<'a, Message>| -> Element<'a, Message> {
        button.variant(Variant::Outline).size(ButtonSize::Sm).into()
    };
    let mut buttons = vec![small(
        button::text(t!("common.details")).on_press(on_event(Event::Details)),
    )];
    if retry {
        buttons.push(small(
            button::labelled(Icon::RotateCw, t!("common.retry")).on_press(on_event(Event::Retry)),
        ));
    }
    body.push(line(buttons).spacing(space::S2).into());
    let glyph = container(icon(Icon::CircleAlert).style(theme::svg::destructive))
        .padding([space::S0_5, 0.0]);
    container(
        line(vec![glyph.into(), stack(body).spacing(space::S2).into()])
            .spacing(space::S2)
            .align_y(Vertical::Top),
    )
    .padding([space::S2, space::S2_5])
    .width(Length::Fill)
    .style(theme::container::alert)
    .into()
}
