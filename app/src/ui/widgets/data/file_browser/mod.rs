//! `FileBrowser`: a folder's listing under the path that leads to it.
//! A press on a folder opens it, a press on a part of the path goes
//! back there, and an entry's menu downloads it.
//!
//! The browser lists nothing and saves nothing itself: the page reads
//! the folder, gives the entries, and does what the [`Event`]s ask.
//!
//! ```ignore
//! // state: file_browser::State, made once
//!
//! // in view: `entries` is None while the folder is read
//! FileBrowser::new(&self.files, &self.path, entries, Message::Files)
//!     .failed(failure)
//!     .uploads(true)
//!     .view()
//!
//! // in update:
//! Message::Files(message) => match self.files.update(message, entries) {
//!     Some(Event::Navigate(path)) => { /* read that folder */ }
//!     Some(Event::Refresh) => { /* read this one again */ }
//!     Some(Event::Download(entry)) => { /* the save dialog, then the transfer */ }
//!     Some(Event::Upload) => { /* the open dialog, then the transfer into the folder shown */ }
//!     Some(Event::Failure(event)) => { /* Details, or the hint's command to copy */ }
//!     None => {}
//! }
//! ```

pub mod model;

use std::rc::Rc;

use homeport_core::error::AppError;
use iced::{
    alignment::Vertical,
    widget::{container, text, Row},
    Length,
};

pub use self::model::{crumbs, Entry, Kind};
use crate::{
    i18n::format,
    t,
    ui::{
        theme::{
            self, font,
            metrics::{icon as glyph, space, text as size, ButtonSize},
            Element, Icon,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                flow, technical, truncated,
            },
            feedback::{error_message, error_message::Event as Failure},
            focus::pressable,
            line,
            overlay::menu,
            stack, start,
            table::{self, sort, Align, Column, Rows, Table, Width},
        },
    },
};

/// What the browser's own controls send; the page hands each to
/// [`State::update`].
#[derive(Debug, Clone)]
pub enum Message {
    Table(table::Message<String>),
    /// A part of the path was pressed.
    Navigate(String),
    Refresh,
    Upload,
    /// "Download" in the menu of the entry at this path.
    Download(String),
    Failure(Failure),
}

/// What the page is asked to do.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Show this folder.
    Navigate(String),
    /// Read the folder shown again.
    Refresh,
    /// Save this entry somewhere.
    Download(Entry),
    /// Put files in the folder shown.
    Upload,
    /// What the failure's own buttons ask for; Retry is [`Event::Refresh`].
    Failure(Failure),
}

/// What a page keeps of a browser: how its table is sorted and scrolled.
#[derive(Debug)]
pub struct State {
    table: Box<table::State<String>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            // A press opens a folder, as `onRowClick` did.
            table: Box::new(table::State::new().opening_on_press()),
        }
    }
}

type Send<'a, M> = Rc<dyn Fn(Message) -> M + 'a>;

fn rows(entries: &[Entry]) -> Rows<'_, Entry, String> {
    Rows::new(entries, |entry| entry.path.clone())
}

fn icon_of(kind: Kind) -> Icon {
    match kind {
        Kind::Directory => Icon::Folder,
        Kind::Symlink => Icon::Link,
        Kind::File | Kind::Other => Icon::File,
    }
}

fn columns<'a, M: Clone + 'a>() -> Vec<Column<'a, Entry, M>> {
    vec![
        Column::new("name", t!("ui.files.column.name"), |entry: &Entry| {
            line(vec![
                theme::icon(icon_of(entry.kind))
                    .width(glyph::MD)
                    .height(glyph::MD)
                    .style(theme::svg::muted)
                    .into(),
                // A name is a path's: it reads left to right, cut with
                // an ellipsis and whole under the pointer.
                container(truncated(technical(&entry.name)).with_tip())
                    .width(Length::Fill)
                    .align_x(start())
                    .into(),
            ])
            .spacing(space::S2)
            .width(Length::Fill)
            .into()
        })
        .width(Width::Fill(3))
        .min_width(160.0)
        .hideable(false)
        .sort_by(|a, b| sort::natural(&a.name, &b.name))
        .typed(|entry| entry.name.clone()),
        Column::new("size", t!("ui.files.column.size"), |entry: &Entry| {
            // A folder's size is its entry's, which says nothing.
            let size = if entry.kind == Kind::File {
                // A file's size is far below 2^53 bytes.
                #[allow(clippy::cast_precision_loss)]
                format::bytes(entry.size as f64)
            } else {
                String::new()
            };
            table::text(size).into()
        })
        .width(Width::Fixed(table::width_for("1023.9 كيلوبايت")))
        .align(Align::End)
        .sort_by(|a, b| a.size.cmp(&b.size))
        .descending_first(),
        Column::new(
            "modified",
            t!("ui.files.column.modified"),
            |entry: &Entry| table::text(format::date_time(entry.modified)).into(),
        )
        .width(Width::Fill(2))
        .min_width(170.0)
        .sort_by(|a, b| a.modified.cmp(&b.modified))
        .descending_first(),
        Column::new("mode", t!("ui.files.column.mode"), |entry: &Entry| {
            text(entry.mode.clone())
                .size(size::XS)
                .font(font::MONO)
                .wrapping(text::Wrapping::None)
                .into()
        })
        .width(Width::Fixed(112.0))
        .sort_by(|a, b| a.mode.cmp(&b.mode)),
    ]
}

impl State {
    /// Takes a message of the browser, given the entries it shows
    /// (none while the folder is read).
    pub fn update(&mut self, message: Message, entries: Option<&[Entry]>) -> Option<Event> {
        let entries = entries.unwrap_or_default();
        match message {
            Message::Table(message) => {
                let opened = self
                    .table
                    .update(message, &rows(entries), &columns::<Message>());
                let Some(table::Event::Opened(path)) = opened else {
                    return None;
                };
                // Only a folder opens: a file is downloaded from its menu.
                entries
                    .iter()
                    .find(|entry| entry.path == path && entry.kind == Kind::Directory)
                    .map(|entry| Event::Navigate(entry.path.clone()))
            }
            Message::Navigate(path) => Some(Event::Navigate(path)),
            Message::Refresh | Message::Failure(Failure::Retry) => Some(Event::Refresh),
            Message::Upload => Some(Event::Upload),
            Message::Download(path) => entries
                .iter()
                .find(|entry| entry.path == path)
                .cloned()
                .map(Event::Download),
            Message::Failure(event) => Some(Event::Failure(event)),
        }
    }
}

/// A browser being described; [`FileBrowser::view`] makes it an element.
pub struct FileBrowser<'a, M> {
    state: &'a State,
    path: &'a str,
    entries: Option<&'a [Entry]>,
    on_event: Send<'a, M>,
    failed: Option<&'a AppError>,
    uploads: bool,
    actions: Vec<Element<'a, M>>,
}

impl<'a, M: Clone + 'a> FileBrowser<'a, M> {
    /// The folder at `path`: its `entries`, or none while it is read.
    pub fn new(
        state: &'a State,
        path: &'a str,
        entries: Option<&'a [Entry]>,
        on_event: impl Fn(Message) -> M + 'a,
    ) -> Self {
        Self {
            state,
            path,
            entries,
            on_event: Rc::new(on_event),
            failed: None,
            uploads: false,
            actions: vec![],
        }
    }

    /// The folder could not be read: said in place of its entries, with
    /// a Retry that asks for [`Event::Refresh`].
    #[must_use]
    pub fn failed(mut self, failure: Option<&'a AppError>) -> Self {
        self.failed = failure;
        self
    }

    /// Whether files can be put in the folder: the Upload button.
    #[must_use]
    pub fn uploads(mut self, uploads: bool) -> Self {
        self.uploads = uploads;
        self
    }

    /// What the page adds after Refresh and Upload.
    #[must_use]
    pub fn actions(mut self, actions: Vec<Element<'a, M>>) -> Self {
        self.actions = actions;
        self
    }

    /// The path, each folder on the way a button. A path reads left to
    /// right in every language, so its parts are never mirrored.
    fn path(&self) -> Element<'a, M> {
        let parts = crumbs(self.path)
            .into_iter()
            .enumerate()
            .flat_map(|(index, (label, to))| {
                // The separator: the root is one itself.
                let slash = (index > 1).then(|| {
                    text("/")
                        .size(size::SM)
                        .font(font::MONO)
                        .style(theme::text::muted)
                        .into()
                });
                let part = pressable(text(technical(&label)).size(size::SM).font(font::MONO))
                    .padding([0.0, space::S1])
                    .style(theme::button::part::bare)
                    .on_press((self.on_event)(Message::Navigate(to)));
                slash.into_iter().chain([part.into()])
            });
        Row::with_children(parts)
            .spacing(space::S0_5)
            .align_y(Vertical::Center)
            .wrap()
            .into()
    }

    pub fn view(self) -> Element<'a, M> {
        let on = self.on_event.clone();
        let mut bar: Vec<Element<'a, M>> = vec![
            self.path(),
            button::labelled(Icon::RefreshCw, t!("common.refresh"))
                .variant(Variant::Outline)
                .size(ButtonSize::Sm)
                .on_press(on(Message::Refresh))
                .into(),
        ];
        if self.uploads {
            bar.push(
                button::labelled(Icon::Upload, t!("ui.files.upload"))
                    .variant(Variant::Outline)
                    .size(ButtonSize::Sm)
                    .on_press(on(Message::Upload))
                    .into(),
            );
        }
        bar.extend(self.actions);
        let bar = flow(bar).spacing(space::S2).push_from(1);

        let sent = on.clone();
        let failed = self
            .failed
            .map(|error| error_message(error, true, |event| sent(Message::Failure(event))));
        let sent = on.clone();
        let menu_of = on.clone();
        let entries = self.entries.unwrap_or_default();
        let table = Table::new(
            &self.state.table,
            columns(),
            rows(entries),
            move |message| sent(Message::Table(message)),
        )
        .row_menu(move |entry: &Entry| {
            vec![menu::Item::action(
                t!("ui.files.downloadOne", name = entry.name.as_str()),
                menu_of(Message::Download(entry.path.clone())),
            )
            .glyph(Icon::Download)]
        })
        .loading(self.entries.is_none() && self.failed.is_none())
        .failed(failed)
        .empty_label(t!("ui.files.empty"))
        .view();
        stack(vec![bar.into(), table]).spacing(space::S3).into()
    }
}
