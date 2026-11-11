//! `HintView`: what to do about a failure. The advice, then each command
//! ready to copy. Commands are never translated, and read left to right in
//! every language.

use std::time::Duration;

use homeport_core::error::Hint;
use iced::{
    alignment::Vertical,
    widget::{container, text, Row},
    Length, Task,
};

use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font,
            metrics::{space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            paragraph, stack,
        },
    },
};

/// How long a copy button says "Copied".
pub const COPIED_FOR: Duration = Duration::from_millis(1500);

/// `CopyButton`: "Copy" (or `label`), and "Copied" with a check while
/// `copied`.
pub fn copy_button<'a, Message: Clone + 'a>(
    label: Option<String>,
    copied: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let (glyph, label) = if copied {
        (Icon::Check, t!("ui.copied"))
    } else {
        (Icon::Copy, label.unwrap_or_else(|| t!("ui.copy")))
    };
    button::labelled(glyph, label)
        .variant(Variant::Outline)
        .size(ButtonSize::Sm)
        .on_press(on_press)
        .into()
}

/// Puts `text` on the clipboard, then sends `done` once the button has
/// said "Copied" long enough.
pub fn copy<Message: Send + 'static>(text: String, done: Message) -> Task<Message> {
    iced::clipboard::write::<Message>(text)
        .chain(Task::perform(tokio::time::sleep(COPIED_FOR), move |()| {
            done
        }))
}

/// The hint's advice and its commands; `copied` is the command whose button
/// says so, and `on_copy` gets the index of the command to copy.
pub fn hint_view<'a, Message: Clone + 'a>(
    hint: &Hint,
    copied: Option<usize>,
    on_copy: impl Fn(usize) -> Message,
) -> Element<'a, Message> {
    let advice = i18n::errors::hint(hint);
    let mut parts: Vec<Element<'a, Message>> = vec![];
    if !advice.is_empty() {
        parts.push(paragraph(advice).into());
    }
    for (index, command) in hint.commands.iter().enumerate() {
        let code = text(command.clone())
            .font(font::MONO)
            .size(size::XS)
            .line_height(size::LEADING_XS)
            .width(Length::Fill);
        let button = copy_button(None, copied == Some(index), on_copy(index));
        // A command is machine text: its line starts on the left in every
        // language, with its button after it.
        let row = Row::new()
            .push(container(code).padding([space::S1, 0.0]))
            .push(button)
            .spacing(space::S2)
            .align_y(Vertical::Top);
        parts.push(
            container(row)
                .padding(iced::Padding {
                    top: space::S1_5,
                    right: space::S1_5,
                    bottom: space::S1_5,
                    left: space::S3,
                })
                .width(Length::Fill)
                .style(theme::container::muted)
                .into(),
        );
    }
    stack(parts).spacing(space::S2).into()
}
