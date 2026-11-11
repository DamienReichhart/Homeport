//! The gallery's type, tokens, buttons, fields and choices.

use iced::{
    alignment::Vertical,
    widget::{
        checkbox, column, container, pick_list, radio, slider, text, text_editor, text_input,
        toggler, tooltip, Row,
    },
};

use super::{
    action, chip, icon_button, labelled, muted, Gallery, Message, Variant, ENGINES, FIELD, LABEL,
    NOTES, SCRIPTS,
};
use crate::ui::theme::{
    self, font,
    metrics::{control, space, text as size, ButtonSize},
    Element, Icon,
};

impl Gallery {
    pub(super) fn typography(&self) -> Element<'_, Message> {
        let sample = "Every Docker Engine, on one board.";
        let weights = [
            ("400 body", font::SANS, size::SM),
            ("500 labels", font::SANS_MEDIUM, size::SM),
            ("600 titles", font::SANS_SEMIBOLD, size::BASE),
            ("700 display", font::SANS_BOLD, size::XXL),
            ("mono", font::MONO, size::XS),
            ("mono italic", font::MONO_ITALIC, size::XS),
        ];
        let lines = weights.into_iter().map(|(name, face, px)| {
            self.line(vec![
                muted(name).width(LABEL).into(),
                text(sample).font(face).size(px).into(),
            ])
            .into()
        });
        self.stack(lines.collect()).into()
    }

    pub(super) fn scripts(&self) -> Element<'_, Message> {
        let lines = SCRIPTS.into_iter().map(|(name, sample)| {
            self.line(vec![
                muted(name).width(LABEL).into(),
                text(sample).size(size::BASE).into(),
            ])
            .into()
        });
        self.stack(lines.collect()).into()
    }

    pub(super) fn swatches(&self) -> Element<'_, Message> {
        let t = self.theme.tokens();
        let named = [
            ("background", t.background),
            ("card", t.card),
            ("popover", t.popover),
            ("primary", t.primary),
            ("secondary", t.secondary),
            ("muted", t.muted),
            ("accent", t.accent),
            ("muted-fg", t.muted_foreground),
            ("destructive", t.destructive),
            ("warning", t.warning),
            ("border", t.border),
            ("input", t.input),
            ("ring", t.ring),
            ("chart-4", t.chart[3]),
            ("sidebar", t.sidebar.background),
            ("sidebar-accent", t.sidebar.accent),
        ];
        let swatches = named
            .into_iter()
            .map(|(name, colour)| column![chip(colour), muted(name)].spacing(space::S1).into());
        Row::from_vec(self.direction.ordered(swatches.collect()))
            .spacing(space::S3)
            .wrap()
            .into()
    }

    pub(super) fn buttons(&self) -> Element<'_, Message> {
        let variants: [(&str, Variant); 6] = [
            ("Primary", theme::button::primary),
            ("Secondary", theme::button::secondary),
            ("Outline", theme::button::outline),
            ("Ghost", theme::button::ghost),
            ("Destructive", theme::button::destructive),
            ("Link", theme::button::link),
        ];
        let rows = variants.into_iter().map(|(name, style)| {
            self.line(vec![
                action(name, ButtonSize::Default, style, Some(Message::Pressed)),
                action("Large", ButtonSize::Lg, style, Some(Message::Pressed)),
                action("Small", ButtonSize::Sm, style, Some(Message::Pressed)),
                action("Tiny", ButtonSize::Xs, style, Some(Message::Pressed)),
                action("Disabled", ButtonSize::Default, style, None),
            ])
            .into()
        });
        let icons = self.line(vec![
            icon_button(Icon::Play, theme::button::ghost),
            icon_button(Icon::RefreshCw, theme::button::outline),
            icon_button(Icon::Settings, theme::button::secondary),
            tooltip(
                icon_button(Icon::Info, theme::button::ghost),
                container(text("A tooltip").size(size::XS)).padding([space::S1_5, space::S3]),
                tooltip::Position::Bottom,
            )
            .style(theme::container::tooltip)
            .into(),
        ]);
        // An icon takes the colour of its button's text: Paper on the
        // primary one, red in the destructive ones, faded when disabled.
        let pressed = Some(Message::Pressed);
        let with_icons = self.line(vec![
            labelled(Icon::Play, "Start", theme::button::primary, pressed.clone()),
            labelled(
                Icon::Trash,
                "Remove",
                theme::button::destructive,
                pressed.clone(),
            ),
            labelled(
                Icon::Trash,
                "Remove",
                theme::button::destructive_solid,
                pressed,
            ),
            labelled(Icon::Play, "Start", theme::button::primary, None),
            labelled(Icon::Trash, "Remove", theme::button::destructive, None),
            labelled(Icon::RefreshCw, "Refresh", theme::button::outline, None),
        ]);
        self.stack(rows.chain([icons.into(), with_icons.into()]).collect())
            .into()
    }

    pub(super) fn fields(&self) -> Element<'_, Message> {
        let field = |placeholder, value| {
            text_input(placeholder, value)
                .size(size::SM)
                .padding([space::S1, control::PADDING_X])
                .width(FIELD)
        };
        let inputs = self.line(vec![
            field("Host name", &self.input)
                .on_input(Message::Input)
                .into(),
            field("Placeholder", "").on_input(Message::Input).into(),
            field("", "not-a-port")
                .on_input(Message::Input)
                .style(theme::text_input::invalid)
                .into(),
            field("Disabled", "").into(),
        ]);
        let select = pick_list(ENGINES, self.picked, Message::Picked)
            .placeholder("Engine")
            .text_size(size::SM)
            .padding([space::S1_5, control::PADDING_X]);
        let notes = text_editor(&self.notes)
            .on_action(Message::Notes)
            .size(size::SM)
            .height(control::TEXTAREA_MIN)
            .width(FIELD);
        let code = text_editor(&self.code)
            .on_action(Message::Code)
            .font(font::MONO)
            .size(size::XS)
            .line_height(size::LEADING_MACHINE)
            .height(NOTES.1)
            .width(NOTES.0)
            .style(theme::text_editor::code);
        let editors = self.line(vec![select.into(), notes.into(), code.into()]);
        self.stack(vec![inputs.into(), editors.align_y(Vertical::Top).into()])
            .into()
    }

    pub(super) fn choices(&self) -> Element<'_, Message> {
        let check = |label, on| {
            checkbox(on)
                .label(label)
                .size(control::CHECKBOX)
                .text_size(size::SM)
        };
        let switch = |label, on| {
            toggler(on)
                .label(label)
                .size(control::SWITCH_HEIGHT)
                .text_size(size::SM)
        };
        let radios = (1..=2).map(|n| {
            radio(format!("Choice {n}"), n, self.radio, Message::Radio)
                .size(control::CHECKBOX)
                .text_size(size::SM)
                .into()
        });
        self.stack(vec![
            self.line(vec![
                check("Checked", self.checked)
                    .on_toggle(Message::Checked)
                    .into(),
                check("Unchecked", !self.checked)
                    .on_toggle(|on| Message::Checked(!on))
                    .into(),
                check("Disabled", true).into(),
                check("Disabled", false).into(),
            ])
            .into(),
            self.line(vec![
                switch("On", self.toggled)
                    .on_toggle(Message::Toggled)
                    .into(),
                switch("Off", !self.toggled)
                    .on_toggle(|on| Message::Toggled(!on))
                    .into(),
                switch("Disabled", true).into(),
                switch("Disabled", false).into(),
            ])
            .into(),
            self.line(radios.collect()).into(),
            slider(0.0..=100.0, self.slide, Message::Slide)
                .width(FIELD)
                .into(),
        ])
        .into()
    }
}
