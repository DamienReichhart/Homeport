//! What a table remembers from one view to the next, and what the user
//! does to it. Everything about a row is kept by its key, so the list can
//! be read again, sorted or filtered under a selection, a focused row and a
//! scroll position that stay where they were.

use std::{
    cell::{Cell, RefCell},
    hash::Hash,
};

use iced::keyboard::Modifiers;

use super::{
    derived::Derived,
    selection::{self, Mode, Selection},
    sort::Sort,
    window, ColumnId,
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
    /// An arrow, Home, End, Page Up or Page Down (`extend`: with Shift).
    Move { to: Motion, extend: bool },
    /// Space on the focused row.
    ToggleFocused,
    /// Enter on the focused row.
    OpenFocused,
    /// Ctrl or Cmd with A.
    SelectAll,
    /// Escape.
    Clear,
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
    /// The sort or a column's width changed: what a page may keep
    /// ([`State::sort`], [`State::widths`]).
    Arranged,
}

/// A row the table points at: its key, and where it was last seen.
pub(super) type Place<K> = (K, usize);

/// A table's state: the page keeps one per table.
#[derive(Debug)]
pub struct State<K> {
    pub(super) sort: Sort,
    pub(super) widths: Vec<(ColumnId, f32)>,
    pub(super) selection: Selection<K>,
    /// Bumped whenever the selection changes.
    pub(super) selected_at: u64,
    pub(super) mode: Mode,
    pub(super) opens_on_press: bool,
    /// The row the keyboard is on.
    pub(super) focused: Option<Place<K>>,
    /// Where a Shift range starts.
    anchor: Option<Place<K>>,
    pub(super) menu: Option<(K, menu::State)>,
    /// How much room the rows had at the last layout.
    pub(super) viewport: f32,
    pub(super) row_height: f32,
    pub(super) offset: Cell<f32>,
    pub(super) derived: RefCell<Derived<K>>,
}

impl<K> Default for State<K> {
    fn default() -> Self {
        Self {
            sort: Sort::default(),
            widths: vec![],
            selection: Selection::default(),
            selected_at: 0,
            mode: Mode::None,
            opens_on_press: false,
            focused: None,
            anchor: None,
            menu: None,
            viewport: ASSUMED_VIEWPORT,
            row_height: ROW_HEIGHT,
            offset: Cell::new(0.0),
            derived: RefCell::new(Derived {
                order: vec![],
                total: 0,
                selected: 0,
                top: None,
                stamp: None,
            }),
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

    pub fn selection(&self) -> &Selection<K> {
        &self.selection
    }

    /// The key of the row the keyboard is on.
    pub fn focused(&self) -> Option<&K> {
        self.focused.as_ref().map(|(key, _)| key)
    }

    /// How far the rows are scrolled.
    pub fn offset(&self) -> f32 {
        self.offset.get()
    }

    /// Deselects everything: after the selected rows were removed.
    pub fn deselect(&mut self) {
        self.selection.clear();
        self.selected_at += 1;
    }

    /// Forgets the selected rows that are no longer in the list.
    pub fn prune(&mut self, is_there: impl Fn(&K) -> bool) {
        self.selection.retain(is_there);
        self.selected_at += 1;
    }

    /// The width the user gave to `column`, if any.
    pub(super) fn width_of(&self, column: ColumnId) -> Option<f32> {
        self.widths
            .iter()
            .find(|(id, _)| *id == column)
            .map(|(_, width)| *width)
    }

    pub(super) fn menu_of(&self, key: &K) -> Option<&menu::State> {
        self.menu
            .as_ref()
            .filter(|(open, _)| open == key)
            .map(|(_, state)| state)
    }

    fn scroll_to(&mut self, offset: f32) {
        let count = self.derived.borrow().order.len();
        self.offset
            .set(window::clamp(offset, count, self.row_height, self.viewport));
        // Scrolled on purpose: the next view says which row is on top.
        self.derived.get_mut().top = None;
    }

    /// Where `place` is now among the rows that show.
    fn locate<T>(&self, place: &Place<K>, rows: &[T], key: &impl Fn(&T) -> K) -> Option<usize> {
        let derived = self.derived.borrow();
        let at = |index: usize| {
            derived
                .order
                .get(index)
                .is_some_and(|&row| key(&rows[row as usize]) == place.0)
        };
        if at(place.1) {
            return Some(place.1);
        }
        (0..derived.order.len()).find(|&index| at(index))
    }

    fn key_at<T>(&self, index: usize, rows: &[T], key: &impl Fn(&T) -> K) -> Option<K> {
        let derived = self.derived.borrow();
        derived
            .order
            .get(index)
            .map(|&row| key(&rows[row as usize]))
    }

    fn changed(&mut self) -> Option<Event<K>> {
        self.selected_at += 1;
        Some(Event::Selected)
    }

    /// Selects from the anchor to the row `to`, both included.
    fn select_range<T>(
        &mut self,
        to: usize,
        rows: &[T],
        key: &impl Fn(&T) -> K,
    ) -> Option<Event<K>> {
        let anchor = self
            .anchor
            .as_ref()
            .and_then(|anchor| self.locate(anchor, rows, key))
            .unwrap_or(to);
        if self.anchor.is_none() {
            self.anchor = self.key_at(to, rows, key).map(|key| (key, to));
        }
        let keys: Vec<K> = selection::range(anchor, to)
            .filter_map(|index| self.key_at(index, rows, key))
            .collect();
        self.selection.replace(keys);
        self.changed()
    }

    fn moved<T>(
        &mut self,
        to: Motion,
        extend: bool,
        rows: &[T],
        key: &impl Fn(&T) -> K,
    ) -> Option<Event<K>> {
        let count = self.derived.borrow().order.len();
        let last = count.checked_sub(1)?;
        let from = self
            .focused
            .as_ref()
            .and_then(|focused| self.locate(focused, rows, key));
        let page = window::page(self.viewport, self.row_height);
        let target = match (to, from) {
            (Motion::Home, _) | (Motion::Down | Motion::PageDown, None) => 0,
            (Motion::End, _) | (Motion::Up | Motion::PageUp, None) => last,
            (Motion::Up, Some(at)) => at.saturating_sub(1),
            (Motion::Down, Some(at)) => (at + 1).min(last),
            (Motion::PageUp, Some(at)) => at.saturating_sub(page),
            (Motion::PageDown, Some(at)) => (at + page).min(last),
        };
        let reached = (self.key_at(target, rows, key)?, target);
        if extend && self.mode == Mode::Multi && self.anchor.is_none() {
            // The range starts where the keyboard was.
            self.anchor = match (&self.focused, from) {
                (Some((key, _)), Some(at)) => Some((key.clone(), at)),
                _ => Some(reached.clone()),
            };
        }
        self.focused = Some(reached.clone());
        let offset = window::reveal(target, self.offset.get(), self.viewport, self.row_height);
        self.scroll_to(offset);
        if extend && self.mode == Mode::Multi {
            self.select_range(target, rows, key)
        } else {
            self.anchor = Some(reached);
            None
        }
    }

    fn pressed<T>(
        &mut self,
        place: Place<K>,
        modifiers: Modifiers,
        rows: &[T],
        key: &impl Fn(&T) -> K,
    ) -> Option<Event<K>> {
        self.focused = Some(place.clone());
        match self.mode {
            Mode::Multi if modifiers.shift() => self.select_range(place.1, rows, key),
            Mode::Multi | Mode::Single if modifiers.command() => {
                let selected = self.selection.contains(&place.0);
                if self.mode == Mode::Single {
                    self.selection.clear();
                }
                self.selection.set(place.0.clone(), !selected);
                self.anchor = Some(place);
                self.changed()
            }
            _ if self.opens_on_press => Some(Event::Opened(place.0)),
            Mode::None => None,
            Mode::Single | Mode::Multi => {
                self.selection.only(place.0.clone());
                self.anchor = Some(place);
                self.changed()
            }
        }
    }

    /// Takes in what the user did. `rows` and `key` are the ones the view
    /// was given: the keyboard goes through them.
    pub fn update<T>(
        &mut self,
        message: Message<K>,
        rows: &[T],
        key: impl Fn(&T) -> K,
    ) -> Option<Event<K>> {
        if self.derived.borrow().total != rows.len() {
            // The list changed since the last view: until the next one,
            // the rows are taken in the order they come in.
            let derived = self.derived.get_mut();
            // A list is never longer than `u32::MAX` rows.
            #[allow(clippy::cast_possible_truncation)]
            let order = (0..rows.len() as u32).collect();
            derived.order = order;
            derived.total = rows.len();
            derived.stamp = None;
        }
        match message {
            Message::Scrolled(offset) => {
                self.scroll_to(offset);
                None
            }
            Message::Viewport(height) => {
                self.viewport = height.max(0.0);
                let offset = self.offset.get();
                self.scroll_to(offset);
                None
            }
            Message::Sort {
                column,
                descending_first,
                additive,
            } => {
                self.sort.toggle(column, descending_first, additive);
                // Another order: the view stays where it is scrolled.
                self.derived.get_mut().top = None;
                Some(Event::Arranged)
            }
            Message::Resize { column, width } => {
                match self.widths.iter_mut().find(|(id, _)| *id == column) {
                    Some(kept) => kept.1 = width,
                    None => self.widths.push((column, width)),
                }
                None
            }
            Message::Resized => Some(Event::Arranged),
            Message::Pressed {
                key: pressed,
                index,
                modifiers,
            } => self.pressed((pressed, index), modifiers, rows, &key),
            Message::Opened(opened) => (!self.opens_on_press).then_some(Event::Opened(opened)),
            Message::Toggle {
                key: toggled,
                index,
            } => {
                if self.mode == Mode::Single && !self.selection.contains(&toggled) {
                    self.selection.clear();
                }
                self.selection.toggle(toggled.clone());
                self.anchor = Some((toggled, index));
                self.changed()
            }
            Message::ToggleAll(on) => {
                if on && self.mode == Mode::Multi {
                    let count = self.derived.borrow().order.len();
                    let keys: Vec<K> = (0..count)
                        .filter_map(|index| self.key_at(index, rows, &key))
                        .collect();
                    self.selection.replace(keys);
                } else {
                    self.selection.clear();
                }
                self.changed()
            }
            Message::Move { to, extend } => self.moved(to, extend, rows, &key),
            Message::ToggleFocused => {
                let focused = self.focused.clone()?;
                let index = self.locate(&focused, rows, &key)?;
                if self.mode == Mode::None {
                    return None;
                }
                self.update(
                    Message::Toggle {
                        key: focused.0,
                        index,
                    },
                    rows,
                    key,
                )
            }
            Message::OpenFocused => {
                let focused = self.focused.clone()?;
                self.locate(&focused, rows, &key)?;
                Some(Event::Opened(focused.0))
            }
            Message::SelectAll => {
                if self.mode != Mode::Multi {
                    return None;
                }
                self.update(Message::ToggleAll(true), rows, key)
            }
            Message::Clear => {
                if self.selection.is_empty() {
                    return None;
                }
                self.selection.clear();
                self.changed()
            }
            Message::MenuFor(place) => {
                let place = place.or_else(|| self.focused.clone())?;
                let mut menu = menu::State::default();
                menu.update(menu::Event::Toggle);
                self.focused = Some(place.clone());
                self.menu = Some((place.0, menu));
                None
            }
            Message::Menu(of, event) => {
                let mut menu = self
                    .menu
                    .take()
                    .filter(|(open, _)| *open == of)
                    .map(|(_, menu)| menu)
                    .unwrap_or_default();
                menu.update(event);
                if menu.is_open() {
                    self.menu = Some((of, menu));
                }
                None
            }
        }
    }
}
