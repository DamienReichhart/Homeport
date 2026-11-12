//! Every widget of the kit on one page, to look at and to press:
//! `cargo run -p homeport-app --example widgets --features gallery`. Its tests draw it in
//! both themes and in Arabic (`tests/snapshots/widgets-*.png`).
//!
//! It is a developer's tool, not a page of the app: its own labels are
//! literals in English, while the widgets' text comes from the catalogue
//! as everywhere. It also shows how a page wires the kit: the state each
//! widget asks for, its messages, and Tab.

mod controls;
mod feedback;
#[cfg(test)]
mod stage_tests;
pub mod table;
#[cfg(test)]
mod tests;

use homeport_core::{
    error::{AppError, ErrorCode},
    messages as msg,
};
use iced::{
    widget::{container, rule, scrollable, text, text_editor},
    Length, Subscription, Task,
};

use super::{
    basic::button::{self, Variant},
    feedback::{
        confirm::{self, Ask, Outcome},
        confirm_dialog, copy,
        error_details::{self, Details, Effect},
        error_message::Event,
        toast::{self, Kind},
        About, Surfaced, Toasts,
    },
    focus,
    form::{select, Errors},
    line,
    overlay::{self, menu},
    stack, table as table_widget,
};
use crate::{
    i18n,
    ui::theme::{
        self, font,
        metrics::{space, text as size},
        Element, Theme,
    },
};

/// The options of the gallery's selects.
pub const ENGINES: [&str; 6] = [
    "Docker Engine",
    "containerd",
    "Incus",
    "Podman",
    "Kubernetes",
    "Nomad",
];

/// The part of the gallery a view shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Feedback,
    Controls,
    Table,
}

pub struct Gallery {
    pub theme: Theme,
    pub page: Page,
    pub toasts: Toasts,
    pub confirm: Option<(Ask, confirm::State)>,
    pub details: Option<Details>,
    pub menu: menu::State,
    pub popover: bool,
    pub select: select::State,
    pub combo: select::State,
    pub engine: Option<usize>,
    pub runtime: Option<usize>,
    pub tab: usize,
    pub layout: usize,
    pub bold: [bool; 3],
    pub name: String,
    pub secret: String,
    pub revealed: bool,
    pub port: String,
    pub notes: text_editor::Content,
    pub errors: Errors,
    pub checked: bool,
    pub table: table_widget::State<u32>,
    pub table_rows: Vec<table::Sample>,
    pub table_show: table::Show,
}

#[derive(Debug, Clone)]
pub enum Message {
    Theme(Theme),
    Language(&'static str),
    Page(Page),
    /// Tab, or Shift+Tab, that nothing captured.
    Tab(bool),
    Toast(toast::Message),
    Say(Kind),
    Fail,
    Failure(Event),
    Ask(bool),
    Confirm(confirm::Message),
    Details(error_details::Message),
    Menu(menu::Event),
    Popover(bool),
    Select(select::Message),
    Combo(select::Message),
    Tabs(usize),
    Layout(usize),
    Bold(usize),
    Name(String),
    Secret(String),
    Reveal,
    Port(String),
    Notes(text_editor::Action),
    Checked(bool),
    Chosen(&'static str),
    Table(table_widget::Message<u32>),
    TableRows(u32),
    TableShow(table::Show),
    Nothing,
}

impl Default for Gallery {
    fn default() -> Self {
        let mut errors = Errors::default();
        errors.set(
            "argument",
            AppError::validation(homeport_engine::messages::BUILD_ARG_NAME),
        );
        Self {
            theme: Theme::Light,
            page: Page::default(),
            toasts: Toasts::default(),
            confirm: None,
            details: None,
            menu: menu::State::default(),
            popover: false,
            select: select::State::default(),
            combo: select::State::default(),
            engine: Some(0),
            runtime: None,
            tab: 0,
            layout: 0,
            bold: [true, false, false],
            name: "TOKEN=abc".to_owned(),
            secret: "hunter2".to_owned(),
            revealed: false,
            port: "80a".to_owned(),
            notes: text_editor::Content::with_text("Two lines\nof notes."),
            errors,
            checked: true,
            table: table_widget::State::new()
                .selecting(table_widget::Mode::Multi)
                .sorted(table_widget::Sort::by(
                    "name",
                    table_widget::Order::Ascending,
                )),
            table_rows: table::samples(12),
            table_show: table::Show::Rows,
        }
    }
}

/// A failure with everything a failure can carry.
pub fn failure() -> AppError {
    AppError::new(
        ErrorCode::PermissionDenied,
        msg::FILE_FORBIDDEN.with("path", "/var/run/docker.sock"),
    )
    .with_hint(msg::FILE_FORBIDDEN_HINT)
    .with_command("sudo usermod -aG docker \"$USER\"")
    .with_detail("open /var/run/docker.sock: permission denied (os error 13)")
}

pub fn about() -> About {
    About {
        name: "Homeport".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
    }
}

/// The gallery on its page of controls.
#[cfg(test)]
fn controls_page() -> Gallery {
    Gallery {
        page: Page::Controls,
        ..Gallery::default()
    }
}

fn labels() -> Vec<String> {
    ENGINES.map(str::to_owned).to_vec()
}

impl Gallery {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Theme(theme) => self.theme = theme,
            Message::Language(tag) => {
                i18n::set_language(tag, &[]);
            }
            Message::Page(page) => self.page = page,
            Message::Tab(back) => return focus::traverse(back),
            Message::Toast(message) => {
                if let Some(surfaced) = self.toasts.update(message) {
                    self.details = Some(Details::new(surfaced));
                }
            }
            Message::Say(kind) => {
                self.toasts.say_more(
                    kind,
                    || "Started web".to_owned(),
                    || "On prod-eu-1.".to_owned(),
                );
            }
            Message::Fail => {
                self.toasts
                    .failure(Surfaced::new(failure(), "containers_act"));
            }
            Message::Failure(Event::Details) => {
                self.details = Some(Details::new(Surfaced::new(failure(), "containers_list")));
            }
            Message::Failure(Event::Retry) => {
                self.toasts.say(Kind::Info, || "Trying again…".to_owned());
            }
            Message::Failure(Event::Copy(command)) => {
                return iced::clipboard::write(command);
            }
            Message::Ask(typed) => {
                let ask = Ask::new(
                    "Remove the volume db-data?",
                    "Its data is deleted from prod-eu-1 and can't be brought back.",
                    "Remove",
                )
                .destructive();
                let ask = if typed { ask.typed("db-data") } else { ask };
                self.confirm = Some((ask, confirm::State::default()));
            }
            Message::Confirm(message) => {
                if let Some((ask, state)) = &mut self.confirm {
                    match state.update(message, ask) {
                        Outcome::Pending => {}
                        Outcome::Cancelled => self.confirm = None,
                        Outcome::Confirmed => {
                            self.confirm = None;
                            self.toasts
                                .say(Kind::Success, || "Removed db-data".to_owned());
                        }
                    }
                }
            }
            Message::Details(message) => {
                let Some(details) = &mut self.details else {
                    return Task::none();
                };
                match details.update(message, Some(&about())) {
                    Effect::None => {}
                    Effect::Close => self.details = None,
                    Effect::Copy(report) => {
                        return copy(report, Message::Details(error_details::Message::CopyShown));
                    }
                    Effect::OpenLogFolder => {
                        // The gallery has no log folder: it shows the failure.
                        let refused = AppError::new(
                            ErrorCode::Io,
                            msg::FOLDER_NOT_OPENED.with("path", "~/.local/share/homeport/logs"),
                        );
                        let opened = error_details::Message::LogFolderOpened(Err(refused));
                        return Task::done(Message::Details(opened));
                    }
                }
            }
            Message::Menu(event) => self.menu.update(event),
            Message::Popover(open) => self.popover = open,
            Message::Select(message) => {
                if let Some(index) = self.select.update(message, &labels()) {
                    self.engine = Some(index);
                }
            }
            Message::Combo(message) => {
                let stepped = matches!(message, select::Message::Step(_));
                if let Some(index) = self.combo.update(message, &labels()) {
                    self.runtime = Some(index);
                }
                if stepped {
                    return self.combo.reveal();
                }
            }
            Message::Tabs(tab) => self.tab = tab,
            Message::Layout(layout) => self.layout = layout,
            Message::Bold(index) => self.bold[index] = !self.bold[index],
            Message::Name(name) => {
                // Editing the field takes away what was wrong with it.
                self.errors.clear("argument");
                self.name = name;
            }
            Message::Secret(secret) => self.secret = secret,
            Message::Reveal => self.revealed = !self.revealed,
            Message::Port(port) => self.port = port,
            Message::Notes(action) => self.notes.perform(action),
            Message::Checked(checked) => self.checked = checked,
            Message::Chosen(what) => {
                self.toasts.say(Kind::Info, move || what.to_owned());
            }
            Message::Table(message) => {
                let opened = self.table.update(message, &self.table_rows, |row| row.id);
                if let Some(table_widget::Event::Opened(id)) = opened {
                    let name = self.table_rows[id as usize].name.clone();
                    self.toasts.say(Kind::Info, move || name.clone());
                }
            }
            Message::TableRows(count) => {
                self.table_rows = table::samples(count);
                self.table_show = table::Show::Rows;
            }
            Message::TableShow(show) => self.table_show = show,
            Message::Nothing => {}
        }
        Task::none()
    }

    /// The page, the dialog over it and the toasts over both.
    pub fn view(&self) -> Element<'_, Message> {
        let body = match self.page {
            Page::Feedback => self.feedback(),
            Page::Controls => self.controls(),
            Page::Table => self.tables(),
        };
        let page =
            stack(vec![self.switches(), rule::horizontal(1).into(), body]).spacing(space::S6);
        let page = container(scrollable(
            container(page).padding(space::S6).width(Length::Fill),
        ))
        .style(theme::container::page)
        .width(Length::Fill)
        .height(Length::Fill);
        let dialog = match (&self.details, &self.confirm) {
            (Some(details), _) => Some(details.view(Some(&about())).map(Message::Details)),
            (None, Some((ask, state))) => Some(confirm_dialog(ask, state).map(Message::Confirm)),
            (None, None) => None,
        };
        toast::over(
            overlay::over(page, dialog),
            self.toasts.view().map(|t| t.map(Message::Toast)),
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Tab moves focus when nothing (a dialog, a panel) kept it.
        let tab = focus::tab_presses().map(Message::Tab);
        Subscription::batch([tab, self.toasts.subscription().map(Message::Toast)])
    }

    fn switches(&self) -> Element<'_, Message> {
        let choice = |label: &'static str, message: Message, on: bool| -> Element<'_, Message> {
            button::text(label)
                .variant(if on {
                    Variant::Primary
                } else {
                    Variant::Outline
                })
                .on_press(message)
                .into()
        };
        let rtl = i18n::direction().is_rtl();
        line(vec![
            text("Homeport widgets")
                .size(size::XL)
                .font(font::SANS_BOLD)
                .into(),
            choice(
                "Feedback",
                Message::Page(Page::Feedback),
                self.page == Page::Feedback,
            ),
            choice(
                "Controls",
                Message::Page(Page::Controls),
                self.page == Page::Controls,
            ),
            choice(
                "Table",
                Message::Page(Page::Table),
                self.page == Page::Table,
            ),
            choice("Light", Message::Theme(Theme::Light), !self.theme.is_dark()),
            choice("Dark", Message::Theme(Theme::Dark), self.theme.is_dark()),
            choice("English", Message::Language("en"), !rtl),
            choice("Arabic", Message::Language("ar"), rtl),
        ])
        .spacing(space::S3)
        .into()
    }

    /// A titled part of the page.
    fn section<'a>(title: &'a str, body: Element<'a, Message>) -> Element<'a, Message> {
        let title = text(title).size(size::BASE).font(font::SANS_SEMIBOLD);
        stack(vec![title.into(), body]).spacing(space::S3).into()
    }
}

/// Opens the gallery in a window.
pub fn run() -> iced::Result {
    let app = iced::application(Gallery::default, Gallery::update, Gallery::view)
        .title("Homeport widgets")
        .theme(|gallery: &Gallery| gallery.theme)
        .subscription(Gallery::subscription)
        .default_font(font::DEFAULT)
        .window_size((1100.0, 900.0));
    font::files().fold(app, |app, font| app.font(font)).run()
}
