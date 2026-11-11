//! Every style of the theme on one page, in light and dark, left to right
//! and right to left:
//! `cargo run -p homeport-app --example gallery --features gallery`.
//!
//! This is a developer's tool, not a page of the app, so its labels are
//! literals in English: the rule that every string is a Fluent message is
//! for what users see. It is built for tests and with the `gallery`
//! feature only, so none of it is in the app. Its tests hold it to
//! `tests/snapshots/theme-*.png`.

mod controls;
mod surfaces;
#[cfg(test)]
mod tests;

use crate::{
    i18n::Direction,
    ui::theme::{
        self, font, icon,
        metrics::{control, space, text as size, ButtonSize},
        Element, Icon, Theme,
    },
};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, rule, scrollable, text, text_editor, Column, Row, Text},
    Color, Length,
};

/// Opens the gallery in a window.
pub fn run() -> iced::Result {
    let app = iced::application(Gallery::default, Gallery::update, Gallery::view)
        .title("Homeport theme gallery")
        .theme(|gallery: &Gallery| gallery.theme)
        .default_font(font::DEFAULT)
        .window_size((1100.0, 900.0));
    font::files().fold(app, |app, font| app.font(font)).run()
}

pub struct Gallery {
    pub theme: Theme,
    pub direction: Direction,
    input: String,
    checked: bool,
    toggled: bool,
    picked: Option<&'static str>,
    radio: Option<u8>,
    slide: f32,
    notes: text_editor::Content,
    code: text_editor::Content,
    /// Whether the scripts the OS's fonts draw are shown: not in a
    /// snapshot, which must look the same on every machine.
    os_scripts: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
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

/// The gallery's own measures, on the spacing scale.
const LABEL: f32 = space::S6 * 4.0;
const FIELD: f32 = space::S8 * 7.0;
const BLOCK: f32 = space::S6 * 6.0;
const NOTES: (f32, f32) = (space::S8 * 10.0, space::S6 * 4.0);

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
    pub fn showing(theme: Theme, direction: Direction) -> Self {
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
            os_scripts: true,
        }
    }

    pub fn update(&mut self, message: Message) {
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

    pub fn view(&self) -> Element<'_, Message> {
        let mut sections = vec![self.switches(), self.section("Type", self.typography())];
        if self.os_scripts {
            sections.push(self.section("Other scripts (the OS's fonts)", self.scripts()));
        }
        sections.extend([
            self.section("Tokens", self.swatches()),
            self.section("Buttons", self.buttons()),
            self.section("Fields", self.fields()),
            self.section("Choices", self.choices()),
            self.section("Surfaces", self.surfaces()),
            self.section("Badges, status, progress", self.feedback()),
            self.section("Machine text", self.machine()),
            self.section("Icons", self.icons()),
        ]);
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
}

fn muted(label: &str) -> Text<'_, Theme> {
    text(label).size(size::XS).style(theme::text::muted)
}

/// A token's colour, edged so that the page's own colour still shows.
fn chip<'a>(colour: Color) -> Element<'a, Message> {
    container(text(""))
        .width(control::HEIGHT * 2.0)
        .height(control::HEIGHT)
        .style(theme::container::swatch(colour))
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

/// An icon before a label, with no colour of its own: it is drawn in the
/// label's, whatever the variant and whether the button is disabled.
fn labelled<'a>(
    glyph: Icon,
    label: &'a str,
    style: Variant,
    message: Option<Message>,
) -> Element<'a, Message> {
    let size = ButtonSize::Default;
    let content = Row::new()
        .push(icon(glyph).width(size.icon()).height(size.icon()))
        .push(text(label).size(size.text()).font(font::SANS_MEDIUM))
        .spacing(size.gap())
        .height(Length::Fill)
        .align_y(Vertical::Center);
    button(content)
        .height(size.height())
        .padding([0.0, size.padding_x()])
        .on_press_maybe(message)
        .style(style)
        .into()
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
