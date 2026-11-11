//! The gallery's first page: failures, toasts, confirmations, and what
//! stands in for content that loads or isn't there.

use iced::widget::text;

use super::{failure, Gallery, Message};
use crate::ui::{
    theme::{
        self,
        metrics::{space, text as size},
        Element, Icon, Tone,
    },
    widgets::{
        basic::button::{self, Variant},
        feedback::{
            empty_state, error_message, skeleton, spinner, spinner_saying, status_dot, toast::Kind,
        },
        line, stack,
    },
};

impl Gallery {
    pub(super) fn feedback(&self) -> Element<'_, Message> {
        stack(vec![
            Self::section("Error in place", self.errors_in_place()),
            Self::section("Toasts and dialogs", Self::openers()),
            Self::section("Loading", Self::loading()),
            Self::section("Status", Self::statuses()),
            Self::section("Empty state", Self::empty()),
        ])
        .spacing(space::S6)
        .into()
    }

    pub(super) fn errors_in_place(&self) -> Element<'_, Message> {
        let plain = homeport_core::error::AppError::not_found(
            homeport_core::messages::FILE_MISSING.with("path", "compose.yaml"),
        );
        stack(vec![
            error_message(&failure(), true, Message::Failure),
            error_message(&plain, false, Message::Failure),
        ])
        .spacing(space::S3)
        .into()
    }

    pub(super) fn openers() -> Element<'static, Message> {
        let open = |label: &'static str, message: Message| -> Element<'static, Message> {
            button::text(label)
                .variant(Variant::Outline)
                .on_press(message)
                .into()
        };
        line(vec![
            open("Success toast", Message::Say(Kind::Success)),
            open("Info toast", Message::Say(Kind::Info)),
            open("Warning toast", Message::Say(Kind::Warning)),
            open("Failure toast", Message::Fail),
            open("Confirm", Message::Ask(false)),
            open("Confirm by typing", Message::Ask(true)),
        ])
        .spacing(space::S3)
        .into()
    }

    pub(super) fn loading() -> Element<'static, Message> {
        let lines = stack(vec![
            skeleton(240, 16),
            skeleton(180, 16),
            skeleton(210, 16),
        ])
        .spacing(space::S2)
        .width(260);
        line(vec![
            spinner(),
            spinner_saying("Pulling nginx:1.27…"),
            lines.into(),
        ])
        .spacing(space::S8)
        .into()
    }

    pub(super) fn statuses() -> Element<'static, Message> {
        let tones = [
            (Tone::Success, "Running"),
            (Tone::Muted, "Stopped"),
            (Tone::Info, "Connecting"),
            (Tone::Warning, "Restarting"),
            (Tone::Danger, "Unreachable"),
        ];
        let dots = tones.into_iter().map(|(tone, label)| {
            line(vec![
                status_dot(tone),
                text(label)
                    .size(size::SM)
                    .style(theme::text::tone(tone))
                    .into(),
            ])
            .spacing(space::S1_5)
            .into()
        });
        line(dots.collect()).spacing(space::S5).into()
    }

    pub(super) fn empty() -> Element<'static, Message> {
        empty_state("No containers yet")
            .icon(Icon::Box)
            .description(
                "Containers of this Host show here as soon as one exists. \
                 Run one from an image to start.",
            )
            .action(button::text("Run a container").on_press(Message::Nothing))
            .view()
    }
}
