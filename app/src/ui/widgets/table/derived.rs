//! What a view works out from the rows it is given: which of them show,
//! in what order, how many of those are selected, and which row is at the
//! top of the body. It is worked out in `view`, where the rows are, and
//! kept in the table's state for `update`, where the keyboard needs it.

use std::{cell::Ref, cmp::Ordering, hash::Hash};

use super::{
    sort::{self, Sort},
    state::{Place, State},
    window,
};

/// What a view works out from the rows it is given.
#[derive(Debug)]
pub struct Derived<K> {
    /// The rows that show, in the order they show, by their place in the
    /// slice the page gave.
    pub order: Vec<u32>,
    /// How long that slice was.
    pub total: usize,
    /// How many of the rows that show are selected.
    pub selected: usize,
    /// The row at the top of the body, to stay on it when rows are added
    /// or removed above.
    pub(super) top: Option<Place<K>>,
    pub(super) stamp: Option<Stamp>,
}

/// What a view's work depended on, for a page that says when its rows
/// change ([`super::Table::revision`]).
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Stamp {
    revision: u64,
    total: usize,
    sort: Sort,
    selection: u64,
}

/// Which of two rows comes first.
pub type Compare<'r, T> = dyn Fn(&T, &T) -> Ordering + 'r;

/// What `derive` reads the rows with.
pub struct Reading<'r, T, K> {
    pub key: &'r dyn Fn(&T) -> K,
    pub filter: Option<&'r dyn Fn(&T) -> bool>,
    pub compare: Option<&'r Compare<'r, T>>,
    pub revision: Option<u64>,
}

impl<K: Hash + Eq + Clone> State<K> {
    /// The rows that show and their order, worked out from the rows of
    /// this view. With a `revision`, the work is done again only when it,
    /// the number of rows, the sort or the selection changed; without one
    /// it is checked against the rows every time, which costs a pass over
    /// them when they are sorted or filtered.
    pub(super) fn derive<T>(&self, rows: &[T], reading: &Reading<'_, T, K>) -> Ref<'_, Derived<K>> {
        let stamp = reading.revision.map(|revision| Stamp {
            revision,
            total: rows.len(),
            sort: self.sort.clone(),
            selection: self.selected_at,
        });
        {
            let mut derived = self.derived.borrow_mut();
            if stamp.is_none() || derived.stamp != stamp {
                self.rederive(&mut derived, rows, reading);
                derived.stamp = stamp;
            }
        }
        self.derived.borrow()
    }

    fn rederive<T>(&self, derived: &mut Derived<K>, rows: &[T], reading: &Reading<'_, T, K>) {
        let before = derived.order.len();
        let whole = derived.total == rows.len() && before == rows.len();
        if reading.filter.is_some() || !whole {
            // Keeps the order of the rows that stay (a sorted list stays
            // nearly sorted) and adds the new ones after them.
            let passes = |index: usize| reading.filter.is_none_or(|passes| passes(&rows[index]));
            let mut seen = vec![false; rows.len()];
            derived.order.retain(|&index| {
                let index = index as usize;
                let kept = index < rows.len() && !seen[index] && passes(index);
                if kept {
                    seen[index] = true;
                }
                kept
            });
            // A list is never longer than `u32::MAX` rows.
            #[allow(clippy::cast_possible_truncation)]
            derived.order.extend(
                (0..rows.len())
                    .filter(|&index| !seen[index] && passes(index))
                    .map(|index| index as u32),
            );
        }
        derived.total = rows.len();
        match reading.compare {
            Some(compare) => sort::arrange(&mut derived.order, rows, compare),
            None => derived.order.sort_unstable(),
        }
        derived.selected = if self.selection.is_empty() {
            0
        } else {
            derived
                .order
                .iter()
                .filter(|&&index| {
                    self.selection
                        .contains(&(reading.key)(&rows[index as usize]))
                })
                .count()
        };
        self.stay_on_top(derived, rows, reading.key, before);
    }

    /// Keeps the row that was at the top of the body there when rows were
    /// added or removed above it.
    fn stay_on_top<T>(
        &self,
        derived: &mut Derived<K>,
        rows: &[T],
        key: &dyn Fn(&T) -> K,
        before: usize,
    ) {
        let count = derived.order.len();
        let key_at =
            |order: &[u32], index: usize| order.get(index).map(|&row| key(&rows[row as usize]));
        if let Some((top, was)) = derived.top.take() {
            if key_at(&derived.order, was).as_ref() != Some(&top) {
                let now = derived
                    .order
                    .iter()
                    .position(|&row| key(&rows[row as usize]) == top);
                if let Some(now) = now {
                    let moved = now.cast_signed() - was.cast_signed();
                    let grown = count.cast_signed() - before.cast_signed();
                    if window::follows(moved, grown) {
                        let offset = self.offset.get() + window::top_of(now, self.row_height)
                            - window::top_of(was, self.row_height);
                        self.offset.set(offset);
                    }
                }
            }
        }
        let offset = window::clamp(self.offset.get(), count, self.row_height, self.viewport);
        self.offset.set(offset);
        let first = window::row_at(0.0, offset, self.row_height, count);
        derived.top = first.and_then(|first| Some((key_at(&derived.order, first)?, first)));
    }
}
