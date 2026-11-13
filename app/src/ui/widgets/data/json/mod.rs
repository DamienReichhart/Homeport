//! `JsonView`: a JSON document as a tree. Objects and arrays open and
//! close, a row's menu copies where it is or what it holds, and a
//! search goes through what matches.
//!
//! ```ignore
//! // state, made once the document is read (in a task if it is large):
//! //   json::State::new(json::Document::of(&inspect)?)
//!
//! // in view:
//! json::View::new(&self.json, Message::Json).view()
//!
//! // in update:
//! Message::Json(message) => {
//!     let (task, event) = self.json.update(message);
//!     if let Some(json::Event::Copy(text)) = event { ctx.copy(text); }
//!     task.map(Message::Json)
//! }
//! ```
//!
//! - **A large document stays quick** because the view is the kit's
//!   table: only the rows in view are built, whatever the document
//!   holds. The rows that show are worked out when a node opens or
//!   closes (`state.rs`), and a document of more than 2,000 nodes
//!   starts with only its root open: its members show, closed.
//! - **The search** waits for a pause in the typing, then reads the
//!   document 20,000 nodes at an `update`, so the window draws while a
//!   large one is read. It looks in the names of members and in values
//!   as JSON writes them, whatever the case. Going to a match opens
//!   what holds it and scrolls to it.
//! - **The members of an object** stay in the order the document has
//!   them (`document.rs`).

pub mod document;
pub mod state;

use std::rc::Rc;

use iced::{
    keyboard::{key::Named, Key},
    widget::{container, text, Space},
    Length,
};

pub use self::{
    document::{Document, Invalid, Kind, Name, Node, Why},
    state::{Copy, Event, Message, State},
};
use crate::{
    i18n, t,
    ui::{
        theme::{
            self, font,
            metrics::{icon as glyph, space, text as size, ButtonSize},
            Element, Icon, Theme,
        },
        widgets::{
            basic::{
                button::{self, Variant},
                flow, technical, truncated,
            },
            feedback::copy_button,
            focus::key_area,
            form::search,
            line,
            overlay::{menu, tooltip, Side},
            stack, start,
            table::{Column, Rows, Table, Width},
        },
    },
};

/// How far a node is set in for each object or array that holds it,
/// and the deepest that still sets it further in: a document nested a
/// thousand deep keeps its names in view.
const INDENT: f32 = 16.0;
const DEEPEST_INDENT: u32 = 24;
/// How many characters of a value a row shows: a string of megabytes
/// is cut here, and whole on the clipboard.
pub const SHOWN: usize = 400;

pub(super) fn rows(visible: &[u32], revision: u64) -> Rows<'_, u32, u32> {
    Rows::new(visible, |node| *node).revision(revision)
}

/// The start of `text` a row shows: all of it, or its first [`SHOWN`]
/// characters and an ellipsis.
pub fn shown(text: &str) -> String {
    let cut = text.char_indices().nth(SHOWN).map(|(cut, _)| cut);
    let mut shown = String::with_capacity(cut.unwrap_or(text.len()) + 3);
    for c in text[..cut.unwrap_or(text.len())].chars() {
        if is_unseen(c) {
            // As JSON would escape it: what cannot be seen, or turns
            // the text around it the other way, is shown for what it
            // is, so one value cannot be made to look like another.
            for unit in c.encode_utf16(&mut [0; 2]) {
                shown.push_str(&format!("\\u{unit:04x}"));
            }
        } else {
            shown.push(c);
        }
    }
    if cut.is_some() {
        shown.push('…');
    }
    shown
}

/// Whether `c` draws nothing and may change how what is around it is
/// read: a control character, a mark of direction, a joiner or a space
/// of no width, a byte order mark.
fn is_unseen(c: char) -> bool {
    c.is_control()
        || matches!(
            c,
            '\u{061c}'
                | '\u{200b}'..='\u{200f}'
                | '\u{202a}'..='\u{202e}'
                | '\u{2060}'..='\u{2069}'
                | '\u{feff}'
        )
}

fn mono<'a>(content: String) -> crate::ui::widgets::basic::Truncated<'a> {
    // A name or a value is the document's: left to right, whatever
    // its letters.
    truncated(technical(&content))
        .size(size::XS)
        .font(font::MONO)
}

/// What a node is called in its row.
fn name_of(node: &Node) -> String {
    match &node.name {
        // Where the document is, as the paths write it.
        Name::Root => ".".to_owned(),
        Name::Member(name) => shown(name),
        Name::Item(index) => index.to_string(),
    }
}

/// What a node's row shows as its value: a scalar as JSON writes it,
/// an object or an array as its brackets around how many it holds.
fn value_of(node: &Node) -> String {
    match node.kind {
        Kind::Object(0) => "{}".to_owned(),
        Kind::Array(0) => "[]".to_owned(),
        Kind::Object(count) => format!("{{ {count} }}"),
        Kind::Array(count) => format!("[ {count} ]"),
        _ => shown(&node.text),
    }
}

fn colour_of(kind: Kind) -> fn(&Theme) -> text::Style {
    match kind {
        Kind::String => |theme| text::Style {
            color: Some(theme.tokens().syntax.string),
        },
        Kind::Number => |theme| text::Style {
            color: Some(theme.tokens().syntax.number),
        },
        Kind::Bool | Kind::Null => |theme| text::Style {
            color: Some(theme.tokens().syntax.atom),
        },
        Kind::Object(_) | Kind::Array(_) => theme::text::muted,
    }
}

pub(super) fn columns<'a, M: Clone + 'a>(
    document: &'a Document,
    open: &'a [bool],
) -> Vec<Column<'a, u32, M>> {
    vec![
        Column::new("key", t!("ui.json.key"), move |at: &u32| {
            built::count();
            let Some(node) = document.node(*at as usize) else {
                return Space::new().into();
            };
            let holds = node.kind.holds().unwrap_or(0) > 0;
            let chevron: Element<'a, M> = if holds {
                let icon = if open.get(*at as usize).copied().unwrap_or(false) {
                    Icon::ChevronDown
                } else {
                    // A closed node points the way the language reads.
                    i18n::direction().pick(Icon::ChevronRight, Icon::ChevronLeft)
                };
                theme::icon(icon)
                    .width(glyph::SM)
                    .height(glyph::SM)
                    .style(theme::svg::muted)
                    .into()
            } else {
                Space::new().width(glyph::SM).into()
            };
            // The tree is set in from where text starts, so it mirrors
            // with its column; a name itself reads as it is written.
            line(vec![
                // Depths that count are small.
                #[allow(clippy::cast_precision_loss)]
                Space::new()
                    .width(INDENT * node.depth.min(DEEPEST_INDENT) as f32)
                    .into(),
                chevron,
                Space::new().width(space::S1).into(),
                container(mono(name_of(node)).with_tip())
                    .width(Length::Fill)
                    .align_x(start())
                    .into(),
            ])
            .width(Length::Fill)
            .into()
        })
        .width(Width::Fill(2))
        .min_width(160.0)
        .hideable(false)
        .typed(move |at| document.node(*at as usize).map(name_of).unwrap_or_default()),
        Column::new("value", t!("ui.json.value"), move |at: &u32| {
            built::count();
            let Some(node) = document.node(*at as usize) else {
                return Space::new().into();
            };
            container(mono(value_of(node)).style(colour_of(node.kind)).with_tip())
                .width(Length::Fill)
                .align_x(start())
                .into()
        })
        .width(Width::Fill(3))
        .min_width(160.0)
        .hideable(false),
    ]
}

/// A view being described; [`View::view`] makes it an element.
pub struct View<'a, M> {
    state: &'a State,
    on_event: Rc<dyn Fn(Message) -> M + 'a>,
    max_height: f32,
}

impl<'a, M: Clone + 'a> View<'a, M> {
    pub fn new(state: &'a State, on_event: impl Fn(Message) -> M + 'a) -> Self {
        Self {
            state,
            on_event: Rc::new(on_event),
            max_height: 600.0,
        }
    }

    /// How tall the rows get before they scroll (`max-h-[70vh]`).
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// The search, where the matches are and the ways through them, and
    /// at the end of the line the button that copies the document.
    fn bar(&self) -> Element<'a, M> {
        let state = self.state;
        let on = self.on_event.clone();
        let typed = on.clone();
        let field = search(
            state.search_id(),
            &t!("ui.search"),
            state.search.field.text(),
            move |text| typed(Message::Typed(text)),
        );
        // Enter goes to the next match, with Shift to the one before.
        let stepped = on.clone();
        let field = key_area(field, |_, _| None).on_captured(move |key, modifiers| match key {
            Key::Named(Named::Enter) if modifiers.shift() => Some(stepped(Message::Previous)),
            Key::Named(Named::Enter) => Some(stepped(Message::Next)),
            _ => None,
        });
        let (current, count) = state.matches();
        let found = if state.search.query.is_empty() {
            String::new()
        } else if count == 0 && !state.is_searching() {
            t!("ui.log.noMatches")
        } else {
            t!(
                "ui.log.matches",
                current = current.map_or(0, |at| at + 1),
                count = count
            )
        };
        let step = |icon: Icon, says: String, message: Message| -> Element<'a, M> {
            let button = button::icon(icon)
                .variant(Variant::Ghost)
                .size(ButtonSize::Sm)
                .on_press_maybe((count > 0).then(|| on(message)));
            tooltip(button, says, Side::Top).into()
        };
        let search: Vec<Element<'a, M>> = vec![
            field.into(),
            text(found).size(size::XS).style(theme::text::muted).into(),
            step(
                Icon::ChevronUp,
                t!("containers.logs.previous"),
                Message::Previous,
            ),
            step(Icon::ChevronDown, t!("containers.logs.next"), Message::Next),
        ];
        let copy = copy_button(
            None,
            state.is_copied(Copy::Document),
            on(Message::Copy(Copy::Document)),
        );
        flow(vec![line(search).spacing(space::S1).into(), copy])
            .spacing(space::S2)
            .push_from(1)
            .into()
    }

    pub fn view(self) -> Element<'a, M> {
        let state = self.state;
        let bar = self.bar();
        let on = self.on_event.clone();
        let menu_of = self.on_event.clone();
        let table = Table::new(
            &state.table,
            columns(&state.document, &state.open),
            rows(&state.visible, state.revision),
            move |message| on(Message::Table(message)),
        )
        .row_menu(move |at: &u32| {
            vec![
                menu::Item::action(
                    t!("ui.json.copyPath"),
                    menu_of(Message::Copy(Copy::Path(*at))),
                )
                .glyph(Icon::Copy),
                menu::Item::action(
                    t!("ui.json.copyValue"),
                    menu_of(Message::Copy(Copy::Value(*at))),
                )
                .glyph(Icon::Copy),
            ]
        })
        .max_height(self.max_height)
        .view();
        stack(vec![bar, container(table).align_x(start()).into()])
            .spacing(space::S3)
            .into()
    }
}

/// How many cells this thread has built: a test holds a view of a
/// large document to a screen of them.
pub(super) mod built {
    #[cfg(test)]
    thread_local! {
        pub static CELLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    pub fn count() {
        #[cfg(test)]
        CELLS.with(|cells| cells.set(cells.get() + 1));
    }
}
