//! What a page keeps of a JSON view: which objects and arrays are open,
//! the rows that show, what is searched and which button says "Copied".

use iced::{advanced::widget::Id, Task};

use super::document::Document;
use crate::ui::widgets::{feedback::Copied, form::Debounce, table};

/// A document of more nodes than this starts with only its root open
/// (its members show, closed): the rest opens as it is asked for.
pub const OPEN_WHOLE: usize = 2000;
/// How many nodes a search reads in one `update`: a large document is
/// read in as many turns as it takes, the window drawing in between.
pub const SLICE: usize = 20_000;

/// What can be copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Copy {
    /// The whole document, as the React view's button copied it.
    Document,
    /// Where a node is: `.ports[0].name`.
    Path(u32),
    /// A node and what it holds, on indented lines.
    Value(u32),
}

#[derive(Debug, Clone)]
pub enum Message {
    Table(table::Message<u32>),
    /// The search field changed.
    Typed(String),
    /// The typing paused: the ticket of the change it follows.
    Paused(u64),
    /// The search reads its next slice.
    Scan(u64),
    Next,
    Previous,
    Copy(Copy),
    /// A button has said "Copied" long enough.
    CopyShown(u64),
}

/// What the page is asked to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Put this on the clipboard (`ctx.copy`).
    Copy(String),
}

/// A search over the document, read a slice at a time.
#[derive(Debug, Default)]
pub(super) struct Search {
    pub field: Debounce,
    /// What is looked for, in lower case; empty when nothing is.
    pub query: String,
    /// The nodes that match, in document order.
    pub matches: Vec<u32>,
    /// How many nodes were read for this query.
    pub read: usize,
    /// Which match the view is on.
    pub current: Option<usize>,
    /// Counts the queries: a slice of an older one is dropped.
    pub run: u64,
}

pub struct State {
    pub(super) document: Document,
    /// Whether each node shows what it holds.
    pub(super) open: Vec<bool>,
    /// The nodes that show, in order: those no closed node holds.
    pub(super) visible: Vec<u32>,
    /// Counts the changes to `visible`, for the table.
    pub(super) revision: u64,
    pub(super) table: Box<table::State<u32>>,
    pub(super) search: Search,
    pub(super) copied: Copied<Copy>,
    pub(super) search_id: Id,
}

impl State {
    /// A view of `document`: whole if it is small, the root's members
    /// if not.
    pub fn new(document: Document) -> Self {
        let whole = document.len() <= OPEN_WHOLE;
        let open = document
            .nodes()
            .iter()
            .map(|node| whole || node.depth == 0)
            .collect();
        let mut state = Self {
            document,
            open,
            visible: vec![],
            revision: 0,
            table: Box::new(table::State::new().opening_on_press().row_height(28.0)),
            search: Search::default(),
            copied: Copied::default(),
            search_id: Id::unique(),
        };
        state.lay_out();
        state
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    /// What `/` on the page gives focus to (`form::focus_search`).
    pub fn search_id(&self) -> Id {
        self.search_id.clone()
    }

    /// How many rows show.
    pub fn rows(&self) -> usize {
        self.visible.len()
    }

    pub fn is_open(&self, node: usize) -> bool {
        self.open.get(node).copied().unwrap_or(false)
    }

    /// Which match the view is on and how many there are so far.
    pub fn matches(&self) -> (Option<usize>, usize) {
        (self.search.current, self.search.matches.len())
    }

    /// The node the search is on, which the view marks and scrolls to.
    pub fn marked(&self) -> Option<u32> {
        self.table.marked().copied()
    }

    /// How many nodes the search has read for what is typed.
    pub fn read(&self) -> usize {
        self.search.read
    }

    /// Whether the button of `what` says "Copied".
    pub fn is_copied(&self, what: Copy) -> bool {
        self.copied.is(&what)
    }

    /// Whether the search has more of the document to read.
    pub fn is_searching(&self) -> bool {
        !self.search.query.is_empty() && self.search.read < self.document.len()
    }

    /// Works out the rows that show: one pass over those, skipping what
    /// a closed node holds. Done when a node opens or closes, never for
    /// a draw.
    fn lay_out(&mut self) {
        self.visible.clear();
        let nodes = self.document.nodes();
        let mut at = 0;
        while let Some(node) = nodes.get(at) {
            // A document has fewer nodes than `document::MOST`.
            #[allow(clippy::cast_possible_truncation)]
            self.visible.push(at as u32);
            at = if node.kind.holds().is_some() && !self.open[at] {
                node.end as usize
            } else {
                at + 1
            };
        }
        self.revision = self.revision.wrapping_add(1);
    }

    /// Opens or closes an object or an array.
    pub fn toggle(&mut self, node: u32) {
        let at = node as usize;
        let holds = self
            .document
            .node(at)
            .and_then(|node| node.kind.holds())
            .unwrap_or(0);
        if holds == 0 {
            return;
        }
        self.open[at] = !self.open[at];
        self.lay_out();
    }

    /// Opens everything that holds `node`, so that it shows.
    fn reveal(&mut self, node: u32) {
        let line = self.document.lineage(node as usize);
        let mut changed = false;
        for holder in &line[..line.len().saturating_sub(1)] {
            changed |= !std::mem::replace(&mut self.open[*holder], true);
        }
        if changed {
            self.lay_out();
        }
    }

    /// Goes to the match `index`: it shows, and the table scrolls to it.
    fn go_to(&mut self, index: usize) {
        let Some(node) = self.search.matches.get(index).copied() else {
            return;
        };
        self.search.current = Some(index);
        self.reveal(node);
        self.table.mark(Some(node));
    }

    /// Starts looking for `query`.
    fn search_for(&mut self, query: &str) -> Task<Message> {
        let search = &mut self.search;
        search.query = query.trim().to_lowercase();
        search.matches.clear();
        search.read = 0;
        search.current = None;
        search.run += 1;
        self.table.mark(None);
        if self.search.query.is_empty() {
            return Task::none();
        }
        self.scan()
    }

    /// Reads the next slice of the document for the query, and asks for
    /// the one after if there is one.
    fn scan(&mut self) -> Task<Message> {
        let search = &mut self.search;
        let nodes = self.document.nodes();
        let end = (search.read + SLICE).min(nodes.len());
        for (at, node) in nodes[search.read..end].iter().enumerate() {
            let named = match &node.name {
                super::document::Name::Member(name) => name.to_lowercase().contains(&search.query),
                _ => false,
            };
            if named || node.text.to_lowercase().contains(&search.query) {
                #[allow(clippy::cast_possible_truncation)]
                search.matches.push((search.read + at) as u32);
            }
        }
        search.read = end;
        let (first, more) = (search.current.is_none(), end < nodes.len());
        let run = search.run;
        if first && !self.search.matches.is_empty() {
            self.go_to(0);
        }
        if more {
            Task::done(Message::Scan(run))
        } else {
            Task::none()
        }
    }

    fn step(&mut self, forward: bool) {
        let count = self.search.matches.len();
        if count == 0 {
            return;
        }
        let next = match (self.search.current, forward) {
            (None, true) => 0,
            (None, false) => count - 1,
            (Some(at), true) => (at + 1) % count,
            (Some(at), false) => (at + count - 1) % count,
        };
        self.go_to(next);
    }

    /// What `what` puts on the clipboard.
    pub fn text_of(&self, what: Copy) -> String {
        match what {
            Copy::Document => self.document.pretty(0),
            Copy::Path(node) => self.document.path(node as usize),
            Copy::Value(node) => self.document.pretty(node as usize),
        }
    }

    pub fn update(&mut self, message: Message) -> (Task<Message>, Option<Event>) {
        let task = match message {
            Message::Table(message) => {
                let opened = self.table.update(
                    message,
                    &super::rows(&self.visible, self.revision),
                    &super::columns::<Message>(&self.document, &self.open),
                );
                if let Some(table::Event::Opened(node)) = opened {
                    self.toggle(node);
                }
                Task::none()
            }
            Message::Typed(text) => self.search.field.input(text).map(Message::Paused),
            Message::Paused(ticket) => match self.search.field.settled(ticket) {
                Some(query) => {
                    let query = query.to_owned();
                    self.search_for(&query)
                }
                None => Task::none(),
            },
            Message::Scan(run) if run == self.search.run => self.scan(),
            Message::Scan(_) => Task::none(),
            Message::Next => {
                self.step(true);
                Task::none()
            }
            Message::Previous => {
                self.step(false);
                Task::none()
            }
            Message::Copy(what) => {
                let text = self.text_of(what);
                let shown = self.copied.start(what).map(Message::CopyShown);
                return (shown, Some(Event::Copy(text)));
            }
            Message::CopyShown(ticket) => {
                self.copied.finish(ticket);
                Task::none()
            }
        };
        (task, None)
    }
}
