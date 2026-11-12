//! What a table remembers from one view to the next, and what the user
//! does to it. Everything about a row is kept by its key, so the list can
//! be read again, sorted or filtered under a selection, a focused row and a
//! scroll position that stay where they were.

use std::{
    cell::{Cell, Ref, RefCell},
    collections::HashSet,
    hash::Hash,
};

use iced::keyboard::Modifiers;

use super::{
    derived::{Derived, Target},
    selection::{Mode, Selection},
    sort::Sort,
    ColumnId,
};
use crate::ui::widgets::overlay::menu;

/// `h-10` and its hairline: the header.
pub const HEADER_HEIGHT: f32 = 41.0;
/// `p-2` around a line of `text-sm`, and its hairline: a row.
pub const ROW_HEIGHT: f32 = 37.0;
/// How tall the rows are assumed to show before the table was laid out.
const ASSUMED_VIEWPORT: f32 = 720.0;

/// Where the keyboard takes the focused row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
}

/// What the user did to a table. The page hands each to [`State::update`].
#[derive(Debug, Clone, PartialEq)]
pub enum Message<K> {
    /// The rows were scrolled to this offset.
    Scrolled(f32),
    /// The columns were scrolled sideways to this offset.
    ScrolledAcross(f32),
    /// The table was laid out with this much room for its rows.
    Viewport(f32),
    /// A header was pressed (`additive`: with Shift).
    Sort {
        column: ColumnId,
        descending_first: bool,
        additive: bool,
    },
    /// The edge of a header is being dragged.
    Resize { column: ColumnId, width: f32 },
    /// The edge was let go.
    Resized,
    /// The edge of a header was pressed twice: the column goes back to
    /// the width it had before anyone resized it.
    ResetWidth(ColumnId),
    /// A column is hidden or shown again.
    Hide { column: ColumnId, hidden: bool },
    /// A row was pressed with the pointer.
    Pressed {
        key: K,
        index: usize,
        modifiers: Modifiers,
    },
    /// A row was pressed twice.
    Opened(K),
    /// A row's checkbox.
    Toggle { key: K, index: usize },
    /// The checkbox over the rows.
    ToggleAll(bool),
    /// The header of a group was pressed: it opens or closes.
    GroupPressed { name: String, index: usize },
    /// A group is closed or opened: the arrows across on its header.
    Collapse { name: String, collapsed: bool },
    /// The checkbox of a group.
    SelectGroup { name: String, selected: bool },
    /// An arrow, Home, End, Page Up or Page Down (`extend`: with Shift).
    Move { to: Motion, extend: bool },
    /// Space on the focused line.
    ToggleFocused,
    /// Enter on the focused line.
    OpenFocused,
    /// Ctrl or Cmd with A.
    SelectAll,
    /// Escape.
    Clear,
    /// Letters typed while the rows have focus: everything typed so far.
    Typed(String),
    /// The menu key or Shift+F10 on the focused row, or the secondary
    /// button on a row.
    MenuFor(Option<(K, usize)>),
    /// A row's menu.
    Menu(K, menu::Event),
}

/// What a message led to that the page may care about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event<K> {
    /// A row was opened: Enter, a double press, or a press when the table
    /// [opens on a press](State::opening_on_press).
    Opened(K),
    /// The selection changed: read it with [`State::selection`].
    Selected,
    /// The sort, a column's width or the hidden columns changed: what a
    /// page may keep ([`State::sort`], [`State::widths`], [`State::hidden`]).
    Arranged,
}

/// A table's state: the page keeps one per table.
#[derive(Debug)]
pub struct State<K> {
    pub(super) sort: Sort,
    pub(super) widths: Vec<(ColumnId, f32)>,
    pub(super) hidden: Vec<ColumnId>,
    pub(super) mode: Mode,
    pub(super) opens_on_press: bool,
    /// A header goes from one way to the other, never to "unsorted".
    pub(super) always_sorted: bool,
    /// The groups whose rows don't show.
    pub(super) collapsed: HashSet<String>,
    /// The one row the page marks (`selectedId`), and whether the table
    /// still has to scroll to it.
    pub(super) marked: Option<K>,
    pub(super) reveal_marked: Cell<bool>,
    /// Bumped whenever the selection or the collapsed groups change.
    pub(super) marked_at: Cell<u64>,
    /// How much room the rows had at the last layout.
    pub(super) viewport: f32,
    pub(super) row_height: f32,
    pub(super) offset: Cell<f32>,
    pub(super) across: f32,
    /// What the rows of the last view came to, and what points at them.
    pub(super) derived: RefCell<Derived<K>>,
}

impl<K> Default for State<K> {
    fn default() -> Self {
        Self {
            sort: Sort::default(),
            widths: vec![],
            hidden: vec![],
            mode: Mode::None,
            opens_on_press: false,
            always_sorted: false,
            collapsed: HashSet::new(),
            marked: None,
            reveal_marked: Cell::new(false),
            marked_at: Cell::new(0),
            viewport: ASSUMED_VIEWPORT,
            row_height: ROW_HEIGHT,
            offset: Cell::new(0.0),
            across: 0.0,
            derived: RefCell::new(Derived::default()),
        }
    }
}

impl<K: Hash + Eq + Clone> State<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// What the user may select; nothing until told.
    #[must_use]
    pub fn selecting(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// A plain press on a row opens it, as the React table's `onRowClick`
    /// did; Ctrl, Cmd and Shift still select. Otherwise a press selects and
    /// a double press opens.
    #[must_use]
    pub fn opening_on_press(mut self) -> Self {
        self.opens_on_press = true;
        self
    }

    /// A press on a header goes from one way to the other and back, never
    /// to "unsorted": a table that was always sorted by one column in
    /// React (the Board), rather than TanStack's three states.
    #[must_use]
    pub fn always_sorted(mut self) -> Self {
        self.always_sorted = true;
        self
    }

    /// The order the table starts in: `initialSorting`, or what the page
    /// kept.
    #[must_use]
    pub fn sorted(mut self, sort: Sort) -> Self {
        self.sort = sort;
        self
    }

    /// The widths the user gave to columns, as the page kept them.
    #[must_use]
    pub fn with_widths(mut self, widths: impl IntoIterator<Item = (ColumnId, f32)>) -> Self {
        self.widths = widths.into_iter().collect();
        self
    }

    /// The columns the user hid, as the page kept them.
    #[must_use]
    pub fn with_hidden(mut self, hidden: impl IntoIterator<Item = ColumnId>) -> Self {
        self.hidden = hidden.into_iter().collect();
        self
    }

    /// Rows taller than a line of text (the Board's two lines).
    #[must_use]
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height.max(1.0);
        self
    }

    pub fn sort(&self) -> &Sort {
        &self.sort
    }

    /// The columns the user resized, and how wide.
    pub fn widths(&self) -> &[(ColumnId, f32)] {
        &self.widths
    }

    /// The columns the user hid.
    pub fn hidden(&self) -> &[ColumnId] {
        &self.hidden
    }

    pub fn is_hidden(&self, column: ColumnId) -> bool {
        self.hidden.contains(&column)
    }

    /// The selected rows, by key. A row that left the list is no longer
    /// in it once the table was drawn without it.
    pub fn selection(&self) -> Ref<'_, Selection<K>> {
        Ref::map(self.derived.borrow(), |derived| &derived.selection)
    }

    /// The key of the row the keyboard is on.
    pub fn focused(&self) -> Option<K> {
        match &self.derived.borrow().focused {
            Some((Target::Row(key), _)) => Some(key.clone()),
            _ => None,
        }
    }

    /// The group whose header the keyboard is on.
    pub fn focused_group(&self) -> Option<String> {
        match &self.derived.borrow().focused {
            Some((Target::Group(name), _)) => Some(name.clone()),
            _ => None,
        }
    }

    pub fn is_collapsed(&self, group: &str) -> bool {
        self.collapsed.contains(group)
    }

    /// How far the rows are scrolled.
    pub fn offset(&self) -> f32 {
        self.offset.get()
    }

    /// How far the columns are scrolled sideways.
    pub fn across(&self) -> f32 {
        self.across
    }

    /// The one row the page marks, as the React table's `selectedId`: it
    /// is drawn selected, and the table scrolls to it the next time it is
    /// drawn (a row that was just made, the one a search led to). `None`
    /// takes the mark away.
    pub fn mark(&mut self, key: Option<K>) {
        self.reveal_marked.set(key.is_some());
        self.marked = key;
    }

    pub fn marked(&self) -> Option<&K> {
        self.marked.as_ref()
    }

    /// Deselects everything: after the selected rows were removed.
    pub fn deselect(&mut self) {
        self.derived.get_mut().selection.clear();
        self.touch();
    }

    /// Hides `column` or shows it again: what a menu of the columns does.
    pub fn set_hidden(&mut self, column: ColumnId, hidden: bool) {
        self.hidden.retain(|id| *id != column);
        if hidden {
            self.hidden.push(column);
        }
    }

    /// The selection or the collapsed groups changed.
    pub(super) fn touch(&self) {
        self.marked_at.set(self.marked_at.get() + 1);
    }

    /// The width the user gave to `column`, if any.
    pub(super) fn width_of(&self, column: ColumnId) -> Option<f32> {
        self.widths
            .iter()
            .find(|(id, _)| *id == column)
            .map(|(_, width)| *width)
    }
}
