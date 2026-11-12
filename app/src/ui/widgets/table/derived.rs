//! What a table works out from the rows it is given: which of them show,
//! in what order, under which group, how many of those are selected, and
//! which is at the top of the body. It is worked out where the rows are
//! (in `view`, and again in `update` before the keyboard goes through
//! them) and kept in the table's state.
//!
//! What points at a row lives here too (the selection, the row the
//! keyboard is on, where a range starts, the open menu), because a list
//! that is read again without a row must let go of it.

use std::{
    cell::Ref,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

use super::{
    column::Column,
    selection::Selection,
    sort::{self, Order, Sort},
    state::State,
    window,
};
use crate::ui::widgets::overlay::menu;

type Key<'r, T, K> = Box<dyn Fn(&T) -> K + 'r>;
type Filter<'r, T> = Box<dyn Fn(&T) -> bool + 'r>;
type GroupOf<'r, T> = Box<dyn Fn(&T) -> Option<String> + 'r>;

/// The rows of a table and how to read them: what the view and `update`
/// are both given.
pub struct Rows<'r, T, K> {
    pub(super) rows: &'r [T],
    pub(super) key: Key<'r, T, K>,
    pub(super) filter: Option<Filter<'r, T>>,
    pub(super) group: Option<GroupOf<'r, T>>,
    pub(super) revision: Option<u64>,
}

impl<'r, T, K> Rows<'r, T, K> {
    /// `rows`, each known by its `key`.
    pub fn new(rows: &'r [T], key: impl Fn(&T) -> K + 'r) -> Self {
        Self {
            rows,
            key: Box::new(key),
            filter: None,
            group: None,
            revision: None,
        }
    }

    /// Only the rows `passes` lets through show: the search of a page.
    #[must_use]
    pub fn filter(mut self, passes: impl Fn(&T) -> bool + 'r) -> Self {
        self.filter = Some(Box::new(passes));
        self
    }

    /// Rows that `group` names are nested under a header of that name,
    /// the groups in the order of their names and before the rows of no
    /// group: containers under their Stack.
    #[must_use]
    pub fn group_by(mut self, group: impl Fn(&T) -> Option<String> + 'r) -> Self {
        self.group = Some(Box::new(group));
        self
    }

    /// A number the page changes whenever the rows, or what the filter
    /// looks for, change. With it, a view that changes nothing sorts and
    /// filters nothing; without it, every view checks the order against
    /// the rows, which is a pass over a sorted or filtered list.
    #[must_use]
    pub fn revision(mut self, revision: u64) -> Self {
        self.revision = Some(revision);
        self
    }
}

/// What shows on a line of the body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    /// A row, by its place in the slice the page gave.
    Row(u32),
    /// The header of a group, by its place among the groups.
    Group(u32),
}

/// What something of the table points at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target<K> {
    Row(K),
    Group(String),
}

/// A line the table points at, and where it was last seen.
pub type Place<K> = (Target<K>, usize);

/// Rows under one header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub name: String,
    /// Its rows, in the order they show.
    pub rows: Vec<u32>,
    pub collapsed: bool,
    /// How many of them are selected.
    pub selected: usize,
}

#[derive(Debug)]
pub struct Derived<K> {
    /// The rows that pass the filter, in the order of the sort.
    pub order: Vec<u32>,
    /// The lines of the body when rows are grouped; `order` otherwise.
    pub entries: Option<Vec<Entry>>,
    pub groups: Vec<Group>,
    /// The lines from this one on are rows of no group.
    pub alone_from: usize,
    /// How long the slice the page gave was.
    pub total: usize,
    /// How many of `order` are selected.
    pub selected: usize,
    pub selection: Selection<K>,
    /// The line the keyboard is on.
    pub focused: Option<Place<K>>,
    /// The row a Shift range starts at.
    pub anchor: Option<(K, usize)>,
    /// The row whose menu is open.
    pub menu: Option<(K, usize, menu::State)>,
    /// The line at the top of the body, to stay on it when rows are added
    /// or removed above.
    pub(super) top: Option<Place<K>>,
    pub(super) stamp: Option<Stamp>,
}

impl<K> Default for Derived<K> {
    fn default() -> Self {
        Self {
            order: vec![],
            entries: None,
            groups: vec![],
            alone_from: 0,
            total: 0,
            selected: 0,
            selection: Selection::default(),
            focused: None,
            anchor: None,
            menu: None,
            top: None,
            stamp: None,
        }
    }
}

impl<K> Derived<K> {
    /// How many lines the body has.
    pub fn len(&self) -> usize {
        self.entries.as_ref().map_or(self.order.len(), Vec::len)
    }

    pub fn entry(&self, index: usize) -> Option<Entry> {
        match &self.entries {
            Some(entries) => entries.get(index).copied(),
            None => self.order.get(index).map(|&row| Entry::Row(row)),
        }
    }
}

/// What a view's work depended on, for a page that says when its rows
/// change ([`Rows::revision`]).
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Stamp {
    revision: u64,
    total: usize,
    sort: Sort,
    /// When the selection and the collapsed groups last changed.
    marks: u64,
}

/// Which of two rows comes first.
pub type Compare<'r, T> = dyn Fn(&T, &T) -> Ordering + 'r;

/// The comparison the sort asks for, from the columns that have one.
pub(super) fn comparison<'c, T, Message>(
    sort: &Sort,
    columns: &'c [Column<'_, T, Message>],
) -> Option<impl Fn(&T, &T) -> Ordering + 'c> {
    let by: Vec<(&'c Compare<'c, T>, Order)> = sort
        .keys()
        .iter()
        .filter_map(|(id, order)| {
            let column = columns.iter().find(|column| column.id == *id)?;
            Some((column.compare.as_deref()?, *order))
        })
        .collect();
    let sorts = !by.is_empty();
    let compare = move |a: &T, b: &T| {
        by.iter().fold(Ordering::Equal, |so_far, (compare, order)| {
            so_far.then_with(|| order.apply(compare(a, b)))
        })
    };
    sorts.then_some(compare)
}

impl<K: Hash + Eq + Clone> State<K> {
    /// Works out what shows from `rows`, and gives it. With a revision,
    /// the work is done again only when it, the number of rows, the sort,
    /// the selection or the collapsed groups changed.
    pub(super) fn derive<T>(
        &self,
        rows: &Rows<'_, T, K>,
        compare: Option<&Compare<'_, T>>,
    ) -> Ref<'_, Derived<K>> {
        self.rederive_if_stale(rows, compare);
        self.derived.borrow()
    }

    pub(super) fn rederive_if_stale<T>(
        &self,
        rows: &Rows<'_, T, K>,
        compare: Option<&Compare<'_, T>>,
    ) {
        let stamp = rows.revision.map(|revision| Stamp {
            revision,
            total: rows.rows.len(),
            sort: self.sort.clone(),
            marks: self.marked_at.get(),
        });
        let mut derived = self.derived.borrow_mut();
        if stamp.is_none() || derived.stamp != stamp {
            self.rederive(&mut derived, rows, compare);
            // What was let go of counts as a change of the marks.
            derived.stamp = stamp.map(|stamp| Stamp {
                marks: self.marked_at.get(),
                ..stamp
            });
        } else if derived.top.is_none() {
            // Scrolled since: the same lines, another one on top.
            self.note_top(&mut derived, rows);
        }
        self.reveal_marked(&mut derived, rows);
    }

    fn rederive<T>(
        &self,
        derived: &mut Derived<K>,
        rows: &Rows<'_, T, K>,
        compare: Option<&Compare<'_, T>>,
    ) {
        let list = rows.rows;
        let before = derived.len();
        let whole = derived.total == list.len() && derived.order.len() == list.len();
        if rows.filter.is_some() || !whole {
            // Keeps the order of the rows that stay (a sorted list stays
            // nearly sorted) and adds the new ones after them.
            let passes = |index: usize| {
                rows.filter
                    .as_ref()
                    .is_none_or(|passes| passes(&list[index]))
            };
            let mut seen = vec![false; list.len()];
            derived.order.retain(|&index| {
                let index = index as usize;
                let kept = index < list.len() && !seen[index] && passes(index);
                if kept {
                    seen[index] = true;
                }
                kept
            });
            // A list is never longer than `u32::MAX` rows.
            #[allow(clippy::cast_possible_truncation)]
            derived.order.extend(
                (0..list.len())
                    .filter(|&index| !seen[index] && passes(index))
                    .map(|index| index as u32),
            );
        }
        derived.total = list.len();
        match compare {
            Some(compare) => sort::arrange(&mut derived.order, list, compare),
            None => derived.order.sort_unstable(),
        }
        self.group(derived, rows);
        self.let_go(derived, rows);
        self.stay_on_top(derived, rows, before);
    }

    /// Nests the rows under their groups, the groups by name.
    fn group<T>(&self, derived: &mut Derived<K>, rows: &Rows<'_, T, K>) {
        let Some(group_of) = &rows.group else {
            derived.entries = None;
            derived.groups.clear();
            derived.alone_from = 0;
            return;
        };
        let mut groups: Vec<Group> = vec![];
        let mut by_name: HashMap<String, usize> = HashMap::new();
        let mut alone = vec![];
        for &row in &derived.order {
            match group_of(&rows.rows[row as usize]) {
                Some(name) => {
                    let at = *by_name.entry(name).or_insert_with_key(|name| {
                        groups.push(Group {
                            name: name.clone(),
                            rows: vec![],
                            collapsed: self.collapsed.contains(name),
                            selected: 0,
                        });
                        groups.len() - 1
                    });
                    groups[at].rows.push(row);
                }
                None => alone.push(row),
            }
        }
        groups.sort_by(|a, b| sort::natural(&a.name, &b.name).then_with(|| a.name.cmp(&b.name)));
        let mut entries = Vec::with_capacity(derived.order.len() + groups.len());
        for (at, group) in groups.iter().enumerate() {
            // There are far fewer groups than `u32::MAX`.
            #[allow(clippy::cast_possible_truncation)]
            entries.push(Entry::Group(at as u32));
            if !group.collapsed {
                entries.extend(group.rows.iter().map(|&row| Entry::Row(row)));
            }
        }
        derived.alone_from = entries.len();
        entries.extend(alone.into_iter().map(Entry::Row));
        derived.groups = groups;
        derived.entries = Some(entries);
    }

    /// Counts the selected rows, and lets go of what points at a row that
    /// is no longer there: a selected row that left the list, the focused
    /// line, the start of a range and the open menu when their row doesn't
    /// show any more.
    fn let_go<T>(&self, derived: &mut Derived<K>, rows: &Rows<'_, T, K>) {
        let key = |row: u32| (rows.key)(&rows.rows[row as usize]);
        if derived.selection.is_empty() {
            derived.selected = 0;
        } else {
            let selected = |row: &u32| derived.selection.contains(&key(*row));
            derived.selected = derived.order.iter().filter(|row| selected(row)).count();
            let counts: Vec<usize> = derived
                .groups
                .iter()
                .map(|group| group.rows.iter().filter(|row| selected(row)).count())
                .collect();
            // Every row of the list, shown or filtered out: a selected
            // row that is only filtered out stays selected.
            let there = if derived.order.len() == rows.rows.len() {
                derived.selected
            } else {
                rows.rows
                    .iter()
                    .filter(|row| derived.selection.contains(&(rows.key)(row)))
                    .count()
            };
            if there < derived.selection.len() {
                let present: HashSet<K> = rows.rows.iter().map(|row| (rows.key)(row)).collect();
                derived.selection.retain(|key| present.contains(key));
                self.marked_at.set(self.marked_at.get() + 1);
            }
            for (group, count) in derived.groups.iter_mut().zip(counts) {
                group.selected = count;
            }
        }
        if let Some(focused) = derived.focused.take() {
            derived.focused = self.find(derived, rows, &focused).map(|at| (focused.0, at));
        }
        if let Some((anchor, was)) = derived.anchor.take() {
            derived.anchor = self
                .find(derived, rows, &(Target::Row(anchor.clone()), was))
                .map(|at| (anchor, at));
        }
        if let Some((of, was, menu)) = derived.menu.take() {
            derived.menu = self
                .find(derived, rows, &(Target::Row(of.clone()), was))
                .map(|at| (of, at, menu));
        }
    }

    /// Whether the line `index` is `target`.
    fn is_at<T>(
        derived: &Derived<K>,
        rows: &Rows<'_, T, K>,
        target: &Target<K>,
        index: usize,
    ) -> bool {
        match (derived.entry(index), target) {
            (Some(Entry::Row(row)), Target::Row(key)) => {
                (rows.key)(&rows.rows[row as usize]) == *key
            }
            (Some(Entry::Group(group)), Target::Group(name)) => {
                derived.groups[group as usize].name == *name
            }
            _ => false,
        }
    }

    /// Where `place` is among the lines now: where it was, or wherever it
    /// went.
    pub(super) fn find<T>(
        &self,
        derived: &Derived<K>,
        rows: &Rows<'_, T, K>,
        place: &Place<K>,
    ) -> Option<usize> {
        if Self::is_at(derived, rows, &place.0, place.1) {
            return Some(place.1);
        }
        (0..derived.len()).find(|&index| Self::is_at(derived, rows, &place.0, index))
    }

    /// What the line `index` is.
    pub(super) fn target<T>(
        derived: &Derived<K>,
        rows: &Rows<'_, T, K>,
        index: usize,
    ) -> Option<Target<K>> {
        Some(match derived.entry(index)? {
            Entry::Row(row) => Target::Row((rows.key)(&rows.rows[row as usize])),
            Entry::Group(group) => Target::Group(derived.groups[group as usize].name.clone()),
        })
    }

    /// Keeps the line that was at the top of the body there when rows
    /// were added or removed above it.
    fn stay_on_top<T>(&self, derived: &mut Derived<K>, rows: &Rows<'_, T, K>, before: usize) {
        let count = derived.len();
        if let Some(top) = derived.top.take() {
            if !Self::is_at(derived, rows, &top.0, top.1) {
                if let Some(now) = self.find(derived, rows, &top) {
                    let moved = now.cast_signed() - top.1.cast_signed();
                    let grown = count.cast_signed() - before.cast_signed();
                    if window::follows(moved, grown) {
                        let offset = self.offset.get() + window::top_of(now, self.row_height)
                            - window::top_of(top.1, self.row_height);
                        self.offset.set(offset);
                    }
                }
            }
        }
        self.note_top(derived, rows);
    }

    /// Remembers which line is at the top of the body, the offset kept
    /// within what the lines allow.
    fn note_top<T>(&self, derived: &mut Derived<K>, rows: &Rows<'_, T, K>) {
        let count = derived.len();
        let offset = window::clamp(self.offset.get(), count, self.row_height, self.viewport);
        self.offset.set(offset);
        derived.top = window::row_at(0.0, offset, self.row_height, count)
            .and_then(|first| Some((Self::target(derived, rows, first)?, first)));
    }

    /// Scrolls to the row the page marked, once.
    fn reveal_marked<T>(&self, derived: &mut Derived<K>, rows: &Rows<'_, T, K>) {
        if !self.reveal_marked.get() {
            return;
        }
        let Some(marked) = &self.marked else {
            return;
        };
        // A row that is not there yet (the list is still being read) is
        // waited for.
        let Some(at) = self.find(derived, rows, &(Target::Row(marked.clone()), 0)) else {
            return;
        };
        self.reveal_marked.set(false);
        let offset = window::reveal(at, self.offset.get(), self.viewport, self.row_height);
        if (offset - self.offset.get()).abs() > f32::EPSILON {
            self.offset.set(offset);
            self.note_top(derived, rows);
        }
    }
}
