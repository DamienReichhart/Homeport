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

/// Which copy button says "Copied", if any: for a view with several, or
/// one that may be pressed again before it has finished saying so.
///
/// ```ignore
/// Message::Copy(what) => {
///     ctx.copy(text_of(what));
///     self.copied.start(what).map(Message::CopyShown)
/// }
/// Message::CopyShown(ticket) => self.copied.finish(ticket),
/// // in view: copy_button(None, self.copied.is(&what), Message::Copy(what))
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Copied<T = ()> {
    shown: Option<T>,
    /// Counts the copies: only the last one's timer ends what shows.
    copies: u64,
}

impl<T> Default for Copied<T> {
    fn default() -> Self {
        Self {
            shown: None,
            copies: 0,
        }
    }
}

impl<T: PartialEq> Copied<T> {
    /// `what` was copied: its button says so from now, until the task's
    /// ticket is given to [`Copied::finish`]. The clipboard is the
    /// caller's to write.
    pub fn start(&mut self, what: T) -> Task<u64> {
        self.shown = Some(what);
        self.copies += 1;
        let ticket = self.copies;
        Task::perform(tokio::time::sleep(COPIED_FOR), move |()| ticket)
    }

    /// The time of the copy `ticket` is over. A button pressed again
    /// since goes on saying "Copied" until its own time is.
    pub fn finish(&mut self, ticket: u64) {
        if ticket == self.copies {
            self.shown = None;
        }
    }

    /// Whether the button of `what` says "Copied".
    pub fn is(&self, what: &T) -> bool {
        self.shown.as_ref() == Some(what)
    }
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
            .push(
                container(code)
                    .padding([space::S1, 0.0])
                    .width(Length::Fill),
            )
            .push(button)
            .spacing(space::S2)
            .width(Length::Fill)
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

#[cfg(test)]
mod copied {
    use super::{Copied, COPIED_FOR};
    use crate::test_support::tasks::messages;

    /// A button pressed again while it says "Copied" goes on saying so
    /// until its own time is over, not the first press's.
    #[tokio::test(start_paused = true)]
    async fn the_last_copy_s_timer_is_the_one_that_ends_it() {
        let mut copied = Copied::default();
        assert!(!copied.is(&"path"));
        let first = copied.start("path");
        assert!(copied.is(&"path") && !copied.is(&"value"));
        let second = copied.start("value");
        assert!(copied.is(&"value") && !copied.is(&"path"));

        let started = tokio::time::Instant::now();
        let first = messages(first).await;
        assert_eq!(
            started.elapsed(),
            COPIED_FOR,
            "it says so for a second and a half"
        );
        copied.finish(first[0]);
        assert!(copied.is(&"value"), "the first press's timer ends nothing");
        copied.finish(messages(second).await[0]);
        assert!(!copied.is(&"value"));
        // A ticket that was never given ends nothing either.
        let _ = copied.start("path");
        copied.finish(99);
        assert!(copied.is(&"path"));
    }
}
