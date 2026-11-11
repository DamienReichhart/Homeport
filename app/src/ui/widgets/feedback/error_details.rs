//! The Details dialog: everything known about a failure. Its message and
//! hint in the app's language, the code, the use case and the time, the raw
//! detail as the system wrote it, and two buttons: open the log folder, and
//! copy a report (in English) for a bug report.
//!
//! The window keeps one [`Details`] while the dialog is open and gives it
//! the dialog's messages. What it can't do itself comes back as an
//! [`Effect`]: the kit knows no use case and no clipboard.

use homeport_core::error::AppError;
use iced::{
    widget::{container, scrollable, text, Column},
    Length,
};

use super::{
    hint_view::{copy_button, hint_view},
    surfaced::{code_name, About, Surfaced},
};
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            basic::button::{self, Variant},
            line,
            overlay::{dialog, modal, modal::WIDTH_WIDE, Modal},
            paragraph, stack, start,
        },
    },
};

/// `max-h-64`: the raw detail scrolls past this.
const DETAIL_HEIGHT: f32 = 256.0;

/// What a copy button of the dialog copies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// The whole report.
    Report,
    /// This command of the hint.
    Command(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Close,
    Copy(Target),
    /// The button has said "Copied" long enough.
    CopyShown,
    OpenLogFolder,
    /// How opening the log folder went.
    LogFolderOpened(Result<(), AppError>),
}

/// What the window does for the dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    None,
    /// Drop the dialog.
    Close,
    /// Put this on the clipboard (`hint_view::copy`, then `CopyShown`).
    Copy(String),
    /// Open the log folder, then send `LogFolderOpened`.
    OpenLogFolder,
}

/// The open Details dialog.
#[derive(Debug, Clone, PartialEq)]
pub struct Details {
    pub shown: Surfaced,
    copied: Option<Target>,
    /// Why the log folder didn't open, shown under the detail.
    log_folder: Option<AppError>,
}

impl Details {
    pub fn new(shown: Surfaced) -> Self {
        Self {
            shown,
            copied: None,
            log_folder: None,
        }
    }

    pub fn update(&mut self, message: Message, about: Option<&About>) -> Effect {
        match message {
            Message::Close => Effect::Close,
            Message::Copy(target) => {
                let text = match target {
                    Target::Report => Some(self.shown.report(about)),
                    Target::Command(index) => self
                        .shown
                        .error
                        .hint
                        .as_ref()
                        .and_then(|hint| hint.commands.get(index).cloned()),
                };
                let Some(text) = text else {
                    return Effect::None;
                };
                self.copied = Some(target);
                Effect::Copy(text)
            }
            Message::CopyShown => {
                self.copied = None;
                Effect::None
            }
            Message::OpenLogFolder => Effect::OpenLogFolder,
            Message::LogFolderOpened(result) => {
                self.log_folder = result.err();
                Effect::None
            }
        }
    }

    pub fn view(&self, about: Option<&About>) -> Modal<'_, Message> {
        let error = &self.shown.error;
        let mut content =
            dialog(t!("ui.errorDetails.title")).push(paragraph(i18n::errors::message(error)));
        if let Some(hint) = &error.hint {
            let copied = match self.copied {
                Some(Target::Command(index)) => Some(index),
                _ => None,
            };
            content = content.push(hint_view(hint, copied, |index| {
                Message::Copy(Target::Command(index))
            }));
        }
        content = content.push(self.facts(about));
        if let Some(detail) = &error.detail {
            content = content.push(raw_detail(detail));
        }
        if let Some(failed) = &self.log_folder {
            content = content
                .push(paragraph(i18n::errors::message(failed)).style(theme::text::destructive));
        }
        let open = button::labelled(Icon::FolderOpen, t!("ui.errorDetails.openLogFolder"))
            .variant(Variant::Outline)
            .on_press(Message::OpenLogFolder);
        let copy = copy_button(
            Some(t!("ui.errorDetails.copyReport")),
            self.copied == Some(Target::Report),
            Message::Copy(Target::Report),
        );
        modal(content.footer(vec![open.into(), copy]))
            .width(WIDTH_WIDE)
            .on_close(Message::Close)
            .close_on_scrim()
    }

    /// The code, the use case, the time and the app, as a list of terms.
    fn facts<'a>(&self, about: Option<&About>) -> Element<'a, Message> {
        let mut facts = vec![(t!("ui.errorDetails.code"), code_name(self.shown.error.code))];
        facts.extend(
            self.shown
                .command
                .map(|command| (t!("ui.errorDetails.command"), command.to_owned())),
        );
        facts.push((t!("ui.errorDetails.when"), when(self.shown.at.as_second())));
        facts.extend(about.map(|app| {
            (
                t!("ui.errorDetails.app"),
                format!("{} {} · {} {}", app.name, app.version, app.os, app.arch),
            )
        }));
        let mut terms = Column::new().spacing(space::S1).align_x(start());
        let mut values = Column::new()
            .spacing(space::S1)
            .width(Length::Fill)
            .align_x(start());
        for (term, value) in facts {
            terms = terms.push(
                text(term)
                    .size(size::SM)
                    .line_height(size::LEADING_SM)
                    .style(theme::text::muted),
            );
            // The same line height as the term, so the two columns line up.
            values = values.push(
                text(value)
                    .font(font::MONO)
                    .size(size::XS)
                    .line_height(iced::Pixels(size::SM * size::LEADING_SM)),
            );
        }
        line(vec![terms.into(), values.into()])
            .spacing(space::S4)
            .into()
    }
}

/// The time of a failure. On this computer's clock; tests read UTC, so a
/// snapshot is the same wherever it is drawn.
fn when(unix_seconds: i64) -> String {
    #[cfg(test)]
    {
        i18n::current()
            .formats
            .date_time(unix_seconds, &jiff::tz::TimeZone::UTC)
    }
    #[cfg(not(test))]
    {
        i18n::format::date_time(unix_seconds)
    }
}

/// The raw cause, as the system, the Engine or the library wrote it.
fn raw_detail<'a>(detail: &str) -> Element<'a, Message> {
    let note = format!(
        "{} · {}",
        t!("ui.errorDetails.rawDetail"),
        t!("ui.errorDetails.rawDetailNote")
    );
    let raw = text(detail.to_owned())
        .font(font::MONO)
        .size(size::XS)
        .line_height(size::LEADING_XS)
        .width(Length::Fill);
    // Machine text: it starts on the left in every language.
    let block = container(scrollable(container(raw).padding(space::S3)))
        .max_height(DETAIL_HEIGHT)
        .width(Length::Fill)
        .style(theme::container::muted);
    stack(vec![
        paragraph(note).style(theme::text::muted).into(),
        block.into(),
    ])
    .spacing(space::S1)
    .into()
}
