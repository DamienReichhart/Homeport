//! Every style of the theme on one page, in light and dark, left to right
//! and right to left: `cargo run -p homeport-app --example gallery`.
//!
//! This is a developer's tool, not a page of the app, so its labels are
//! literals in English: the rule that every string is a Fluent message is
//! for what users see. `app/tests/gallery.rs` draws it without a window.

use homeport_app::{
    i18n::Direction,
    ui::theme::{
        self,
        color::alpha,
        font, icon,
        metrics::{control, radius, space, text as size, ButtonSize},
        Element, Icon, Theme, Tokens, Tone,
    },
};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, checkbox, column, container, pick_list, progress_bar, radio, rule, scrollable,
        slider, text, text_editor, text_input, toggler, tooltip, Column, Row, Text,
    },
    Color, Length,
};

fn main() -> iced::Result {
    let app = iced::application(Gallery::default, Gallery::update, Gallery::view)
        .title("Homeport theme gallery")
        .theme(|gallery: &Gallery| gallery.theme)
        .default_font(font::DEFAULT)
        .window_size((1100.0, 900.0));
    font::files().fold(app, |app, font| app.font(font)).run()
}

pub(crate) struct Gallery {
    pub(crate) theme: Theme,
    pub(crate) direction: Direction,
    input: String,
    checked: bool,
    toggled: bool,
    picked: Option<&'static str>,
    radio: Option<u8>,
    slide: f32,
    notes: text_editor::Content,
    code: text_editor::Content,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Theme(Theme),
    Direction(Direction),
    Input(String),
    Checked(bool),
    Toggled(bool),
    Picked(&'static str),
    Radio(u8),
    Slide(f32),
    Notes(text_editor::Action),
    Code(text_editor::Action),
    Pressed,
}

const ENGINES: [&str; 3] = ["Docker Engine", "containerd", "Incus"];
const COMPOSE: &str =
    "services:\n  web:\n    image: nginx:1.27  # pinned\n    ports: [\"8080:80\"]";
const SCRIPTS: [(&str, &str); 5] = [
    ("Arabic", "الحاويات تعمل على هذا الخادم"),
    ("Hindi", "इस होस्ट पर कंटेनर चल रहे हैं"),
    ("Japanese", "このホストでコンテナが実行中です"),
    ("Korean", "이 호스트에서 컨테이너가 실행 중입니다"),
    ("Chinese", "容器正在此主机上运行"),
];

impl Default for Gallery {
    fn default() -> Self {
        Self::showing(Theme::Light, Direction::Ltr)
    }
}

type Variant = fn(&Theme, button::Status) -> button::Style;

impl Gallery {
    pub(crate) fn showing(theme: Theme, direction: Direction) -> Self {
        Self {
            theme,
            direction,
            input: "prod-eu-1".to_owned(),
            checked: true,
            toggled: true,
            picked: Some(ENGINES[0]),
            radio: Some(1),
            slide: 40.0,
            notes: text_editor::Content::with_text("Two lines\nof notes."),
            code: text_editor::Content::with_text(COMPOSE),
        }
    }

    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::Theme(theme) => self.theme = theme,
            Message::Direction(direction) => self.direction = direction,
            Message::Input(input) => self.input = input,
            Message::Checked(checked) => self.checked = checked,
            Message::Toggled(toggled) => self.toggled = toggled,
            Message::Picked(engine) => self.picked = Some(engine),
            Message::Radio(choice) => self.radio = Some(choice),
            Message::Slide(value) => self.slide = value,
            Message::Notes(action) => self.notes.perform(action),
            Message::Code(action) => self.code.perform(action),
            Message::Pressed => {}
        }
    }

    pub(crate) fn view(&self) -> Element<'_, Message> {
        let sections = vec![
            self.switches(),
            self.section("Type", self.typography()),
            self.section("Other scripts (the OS's fonts)", self.scripts()),
            self.section("Tokens", self.swatches()),
            self.section("Buttons", self.buttons()),
            self.section("Fields", self.fields()),
            self.section("Choices", self.choices()),
            self.section("Surfaces", self.surfaces()),
            self.section("Badges, status, progress", self.feedback()),
            self.section("Machine text", self.machine()),
            self.section("Icons", self.icons()),
        ];
        let page = self.stack(sections).spacing(space::S8);
        container(scrollable(
            container(page).padding(space::S6).width(Length::Fill),
        ))
        .style(theme::container::page)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    /// A row written start to end, drawn in the page's direction.
    fn line<'a>(&self, items: Vec<Element<'a, Message>>) -> Row<'a, Message, Theme> {
        Row::from_vec(self.direction.ordered(items))
            .spacing(space::S3)
            .align_y(Vertical::Center)
    }

    /// A column whose items start where text starts.
    fn stack<'a>(&self, items: Vec<Element<'a, Message>>) -> Column<'a, Message, Theme> {
        Column::from_vec(items)
            .spacing(space::S3)
            .width(Length::Fill)
            .align_x(self.direction.pick(Horizontal::Left, Horizontal::Right))
    }

    fn section<'a>(&self, title: &'a str, body: Element<'a, Message>) -> Element<'a, Message> {
        let title = text(title).size(size::BASE).font(font::SANS_SEMIBOLD);
        self.stack(vec![title.into(), rule::horizontal(1).into(), body])
            .into()
    }

    fn switches(&self) -> Element<'_, Message> {
        let choice = |label, message: Message, on: bool| {
            let style: Variant = if on {
                theme::button::primary
            } else {
                theme::button::outline
            };
            action(label, ButtonSize::Default, style, Some(message))
        };
        self.line(vec![
            text("Homeport theme")
                .size(size::XL)
                .font(font::SANS_BOLD)
                .into(),
            choice("Light", Message::Theme(Theme::Light), !self.theme.is_dark()),
            choice("Dark", Message::Theme(Theme::Dark), self.theme.is_dark()),
            choice(
                "Left to right",
                Message::Direction(Direction::Ltr),
                !self.direction.is_rtl(),
            ),
            choice(
                "Right to left",
                Message::Direction(Direction::Rtl),
                self.direction.is_rtl(),
            ),
        ])
        .into()
    }

    fn typography(&self) -> Element<'_, Message> {
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
                muted(name).width(96).into(),
                text(sample).font(face).size(px).into(),
            ])
            .into()
        });
        self.stack(lines.collect()).into()
    }

    fn scripts(&self) -> Element<'_, Message> {
        let lines = SCRIPTS.into_iter().map(|(name, sample)| {
            self.line(vec![
                muted(name).width(96).into(),
                text(sample).size(size::BASE).into(),
            ])
            .into()
        });
        self.stack(lines.collect()).into()
    }

    fn swatches(&self) -> Element<'_, Message> {
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
        let swatches = named.into_iter().map(|(name, colour)| {
            column![chip(colour, t), muted(name)]
                .spacing(space::S1)
                .into()
        });
        Row::from_vec(self.direction.ordered(swatches.collect()))
            .spacing(space::S3)
            .wrap()
            .into()
    }

    fn buttons(&self) -> Element<'_, Message> {
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
        self.stack(rows.chain([icons.into()]).collect()).into()
    }

    fn fields(&self) -> Element<'_, Message> {
        let field = |placeholder, value| {
            text_input(placeholder, value)
                .size(size::SM)
                .padding([space::S1, control::PADDING_X])
                .width(220)
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
            .width(220);
        let code = text_editor(&self.code)
            .on_action(Message::Code)
            .font(font::MONO)
            .size(size::XS)
            .line_height(size::LEADING_MACHINE)
            .height(96)
            .width(320)
            .style(theme::text_editor::code);
        let editors = self.line(vec![select.into(), notes.into(), code.into()]);
        self.stack(vec![inputs.into(), editors.align_y(Vertical::Top).into()])
            .into()
    }

    fn choices(&self) -> Element<'_, Message> {
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
                .width(220)
                .into(),
        ])
        .into()
    }

    fn surfaces(&self) -> Element<'_, Message> {
        let block = |name: &'static str, style: fn(&Theme) -> container::Style| {
            container(column![text(name).size(size::SM), muted("muted text")].spacing(space::S1))
                .padding(space::S4)
                .width(150)
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
        .width(220)
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
        .width(150)
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
        self.stack(vec![first.align_y(Vertical::Top).into(), second.into()])
            .into()
    }

    fn feedback(&self) -> Element<'_, Message> {
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

    fn machine(&self) -> Element<'_, Message> {
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

    fn icons(&self) -> Element<'_, Message> {
        let all = Icon::ALL
            .iter()
            .map(|glyph| icon(*glyph).width(20).height(20).into());
        Row::from_vec(self.direction.ordered(all.collect()))
            .spacing(space::S3)
            .wrap()
            .into()
    }
}

fn muted(label: &str) -> Text<'_, Theme> {
    text(label).size(size::XS).style(theme::text::muted)
}

/// A token's colour, edged so that the page's own colour still shows.
fn chip<'a>(colour: Color, tokens: &Tokens) -> Element<'a, Message> {
    let edge = alpha(tokens.foreground, 0.2);
    container(text(""))
        .width(64)
        .height(32)
        .style(move |_: &Theme| container::Style {
            background: Some(colour.into()),
            border: iced::Border {
                color: edge,
                width: 1.0,
                radius: radius::LG.into(),
            },
            ..container::Style::default()
        })
        .into()
}

/// A button of the React `Button`'s measures; without a message, disabled.
fn action<'a>(
    label: &'a str,
    size: ButtonSize,
    style: Variant,
    message: Option<Message>,
) -> Element<'a, Message> {
    let label = text(label)
        .size(size.text())
        .font(font::SANS_MEDIUM)
        .height(Length::Fill)
        .align_y(Vertical::Center);
    let button = button(label)
        .height(size.height())
        .padding([0.0, size.padding_x()])
        .on_press_maybe(message);
    match size {
        ButtonSize::Default | ButtonSize::Lg => button.style(style).into(),
        ButtonSize::Xs | ButtonSize::Sm => button.style(theme::button::small(style)).into(),
    }
}

fn icon_button<'a>(glyph: Icon, style: Variant) -> Element<'a, Message> {
    button(container(icon(glyph)).center(Length::Fill))
        .width(control::HEIGHT)
        .height(control::HEIGHT)
        .padding(0)
        .style(style)
        .on_press(Message::Pressed)
        .into()
}
