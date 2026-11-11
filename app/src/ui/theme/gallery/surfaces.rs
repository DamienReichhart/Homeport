//! The gallery's surfaces, badges, machine text and icons.

use iced::{
    alignment::Vertical,
    widget::{column, container, progress_bar, rule, text, Row},
    Color, Length,
};

use super::{muted, Gallery, Message, BLOCK, FIELD};
use crate::ui::theme::{
    self, font, icon,
    metrics::{control, space, text as size},
    Element, Icon, Theme, Tone,
};

impl Gallery {
    pub(super) fn surfaces(&self) -> Element<'_, Message> {
        let block = |name: &'static str, style: fn(&Theme) -> container::Style| {
            container(column![text(name).size(size::SM), muted("muted text")].spacing(space::S1))
                .padding(space::S4)
                .width(BLOCK)
                .style(style)
                .into()
        };
        let card = container(column![
            container(
                column![
                    text("Card").size(size::BASE).font(font::SANS_MEDIUM),
                    muted("A description")
                ]
                .spacing(space::S1)
            )
            .padding(space::S4),
            rule::horizontal(1),
            container(muted("Footer"))
                .padding(space::S4)
                .width(Length::Fill)
                .style(theme::container::card_footer),
        ])
        .width(FIELD)
        .style(theme::container::card);
        let sidebar = container(
            column![
                container(text("Containers").size(size::SM))
                    .padding([space::S1_5, space::S2])
                    .width(Length::Fill)
                    .style(theme::container::sidebar_accent),
                container(text("Images").size(size::SM)).padding([space::S1_5, space::S2]),
            ]
            .spacing(space::S1),
        )
        .padding(space::S2)
        .width(BLOCK)
        .style(theme::container::sidebar);
        let first = self.line(vec![
            card.into(),
            sidebar.into(),
            block("Popover", theme::container::popover),
            block("Bordered", theme::container::bordered),
        ]);
        let second = self.line(vec![
            block("Muted", theme::container::muted),
            block("Accent", theme::container::accent),
            container(text("Tooltip").size(size::XS))
                .padding([space::S1_5, space::S3])
                .style(theme::container::tooltip)
                .into(),
        ]);
        // A table's rows: at rest, under the pointer, selected.
        let row = |name: &'static str, hovered, selected| -> Element<'_, Message> {
            container(text(name).size(size::SM))
                .padding([space::S2, space::S3])
                .width(FIELD)
                .style(theme::container::table_row(hovered, selected))
                .into()
        };
        let rows = column![
            row("A row", false, false),
            row("A row under the pointer", true, false),
            row("A selected row", false, true),
        ];
        self.stack(vec![
            first.align_y(Vertical::Top).into(),
            second.into(),
            rows.into(),
        ])
        .into()
    }

    pub(super) fn feedback(&self) -> Element<'_, Message> {
        let badge = |name: &'static str, style: fn(&Theme) -> container::Style| {
            container(text(name).size(size::XS).font(font::SANS_MEDIUM))
                .padding([space::S0_5, space::S2])
                .style(style)
                .into()
        };
        let tones = [
            (Tone::Success, Icon::CircleCheck, "Running"),
            (Tone::Muted, Icon::CircleSlash, "Stopped"),
            (Tone::Info, Icon::LoaderCircle, "Connecting"),
            (Tone::Warning, Icon::TriangleAlert, "Restarting"),
            (Tone::Danger, Icon::CircleX, "Unreachable"),
        ];
        let status = tones.into_iter().map(|(tone, glyph, label)| {
            self.line(vec![
                icon(glyph).style(theme::svg::tone(tone)).into(),
                text(label)
                    .size(size::SM)
                    .style(theme::text::tone(tone))
                    .into(),
            ])
            .spacing(space::S1_5)
            .into()
        });
        let bar = |value, style: fn(&Theme) -> progress_bar::Style| {
            progress_bar(0.0..=100.0, value)
                .girth(control::PROGRESS)
                .length(160)
                .style(style)
                .into()
        };
        self.stack(vec![
            self.line(vec![
                badge("Default", theme::container::badge::primary),
                badge("Secondary", theme::container::badge::secondary),
                badge("Destructive", theme::container::badge::destructive),
                badge("Outline", theme::container::badge::outline),
                badge("Ghost", theme::container::badge::ghost),
                badge("Ghost, hovered", theme::container::badge::ghost_hovered),
                badge("Link", theme::container::badge::link),
            ])
            .into(),
            self.line(status.collect()).spacing(space::S5).into(),
            self.line(vec![
                bar(35.0, theme::progress_bar::progress),
                bar(82.0, theme::progress_bar::warning),
                bar(100.0, theme::progress_bar::destructive),
            ])
            .into(),
        ])
        .into()
    }

    pub(super) fn machine(&self) -> Element<'_, Message> {
        let t = self.theme.tokens();
        let mono = |label: &'static str, colour: Color| -> Element<'_, Message> {
            text(label)
                .font(font::MONO)
                .size(size::XS)
                .color(colour)
                .into()
        };
        let names = [
            "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
        ];
        let ansi = |from: usize| -> Element<'_, Message> {
            let cells = names
                .iter()
                .enumerate()
                .map(|(i, name)| mono(name, t.terminal.ansi[from + i]));
            Row::from_vec(cells.collect()).spacing(space::S3).into()
        };
        // Machine text reads left to right in a right-to-left layout too.
        let terminal = container(
            column![
                mono("$ docker ps --all", t.terminal.foreground),
                ansi(0),
                ansi(8)
            ]
            .spacing(space::S1),
        )
        .padding(space::S2)
        .style(theme::container::terminal);
        let syntax = [
            ("services", t.syntax.key),
            (":", t.syntax.punctuation),
            ("\"nginx\"", t.syntax.string),
            ("8080", t.syntax.number),
            ("true", t.syntax.atom),
        ];
        let mut code: Vec<Element<'_, Message>> = syntax
            .into_iter()
            .map(|(word, colour)| mono(word, colour))
            .collect();
        code.push(
            text("# a comment")
                .font(font::MONO_ITALIC)
                .size(size::XS)
                .color(t.syntax.comment)
                .into(),
        );
        let editor = container(Row::from_vec(code).spacing(space::S2))
            .padding(space::S2)
            .style(theme::container::card);
        self.stack(vec![terminal.into(), editor.into()]).into()
    }

    pub(super) fn icons(&self) -> Element<'_, Message> {
        let all = Icon::ALL.iter().map(|glyph| {
            icon(*glyph)
                .width(theme::metrics::icon::LG)
                .height(theme::metrics::icon::LG)
                .into()
        });
        Row::from_vec(self.direction.ordered(all.collect()))
            .spacing(space::S3)
            .wrap()
            .into()
    }
}
