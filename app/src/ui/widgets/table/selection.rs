//! The selected rows, by key: a row stays selected when the list is
//! sorted, filtered or read again, and one that left the list is no longer
//! counted once the page prunes it.

use std::{collections::HashSet, hash::Hash, ops::RangeInclusive};

/// What a table lets the user select.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Nothing: rows only open.
    #[default]
    None,
    /// One row at a time.
    Single,
    /// Any rows: checkboxes, Ctrl or Cmd to add one, Shift for a range.
    Multi,
}

/// What the checkbox over the rows shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extent {
    None,
    Some,
    All,
}

/// The keys of the selected rows.
#[derive(Debug, Clone)]
pub struct Selection<K> {
    keys: HashSet<K>,
}

impl<K> Default for Selection<K> {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }
}

impl<K: Hash + Eq + Clone> Selection<K> {
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.keys.contains(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }

    /// A plain click: this row and no other.
    pub fn only(&mut self, key: K) {
        self.keys.clear();
        self.keys.insert(key);
    }

    /// A checkbox, Ctrl or Cmd with a click, Space: this row in or out.
    /// Returns whether it is selected now.
    pub fn toggle(&mut self, key: K) -> bool {
        if self.keys.remove(&key) {
            false
        } else {
            self.keys.insert(key);
            true
        }
    }

    pub fn set(&mut self, key: K, selected: bool) {
        if selected {
            self.keys.insert(key);
        } else {
            self.keys.remove(&key);
        }
    }

    /// Shift with a click or an arrow: these rows and no other.
    pub fn replace(&mut self, keys: impl IntoIterator<Item = K>) {
        self.keys.clear();
        self.keys.extend(keys);
    }

    /// Select all over a filtered list: these rows too, the others as
    /// they were.
    pub fn extend(&mut self, keys: impl IntoIterator<Item = K>) {
        self.keys.extend(keys);
    }

    /// Deselect all over a filtered list: these rows out, the others as
    /// they were.
    pub fn remove(&mut self, keys: impl IntoIterator<Item = K>) {
        for key in keys {
            self.keys.remove(&key);
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }

    /// Forgets the rows that are no longer in the list.
    pub fn retain(&mut self, is_there: impl Fn(&K) -> bool) {
        self.keys.retain(|key| is_there(key));
    }

    /// How much of the `shown` rows is selected, `selected` of them being.
    pub fn extent(selected: usize, shown: usize) -> Extent {
        match (selected, shown) {
            (0, _) | (_, 0) => Extent::None,
            (selected, shown) if selected >= shown => Extent::All,
            _ => Extent::Some,
        }
    }
}

/// The rows a Shift selects: from where the selection started (`anchor`)
/// to the row reached (`to`), both included, whichever comes first.
pub fn range(anchor: usize, to: usize) -> RangeInclusive<usize> {
    anchor.min(to)..=anchor.max(to)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(selection: &Selection<&'static str>) -> Vec<&'static str> {
        let mut keys: Vec<_> = selection.keys().copied().collect();
        keys.sort_unstable();
        keys
    }

    #[test]
    fn a_click_selects_one_row_and_a_toggle_adds_or_removes() {
        let mut selection = Selection::default();
        selection.only("a");
        selection.only("b");
        assert_eq!(keys(&selection), ["b"]);
        assert!(selection.toggle("c"));
        assert_eq!(keys(&selection), ["b", "c"]);
        assert!(!selection.toggle("b"));
        assert_eq!(keys(&selection), ["c"]);
        selection.set("c", false);
        assert!(selection.is_empty());
    }

    #[test]
    fn a_range_runs_from_the_anchor_to_the_row_reached_either_way() {
        assert_eq!(range(2, 5), 2..=5);
        assert_eq!(range(5, 2), 2..=5);
        assert_eq!(range(3, 3), 3..=3);
        let rows = ["a", "b", "c", "d", "e", "f"];
        let mut selection = Selection::default();
        selection.only("x");
        selection.replace(range(4, 1).map(|index| rows[index]));
        assert_eq!(keys(&selection), ["b", "c", "d", "e"]);
        // Shift again from the same anchor shrinks the range.
        selection.replace(range(4, 3).map(|index| rows[index]));
        assert_eq!(keys(&selection), ["d", "e"]);
    }

    #[test]
    fn the_selection_is_by_key_so_it_outlives_the_order_of_the_rows() {
        let mut selection = Selection::default();
        selection.replace(["web", "db"]);
        // The list is read again, in another order, with a row gone.
        let now = ["cache", "web"];
        assert!(selection.contains(&"web"));
        assert_eq!(now.iter().filter(|key| selection.contains(key)).count(), 1);
        selection.retain(|key| now.contains(key));
        assert_eq!(keys(&selection), ["web"]);
    }

    #[test]
    fn the_checkbox_over_the_rows_says_none_some_or_all() {
        type Keys = Selection<u8>;
        assert_eq!(Keys::extent(0, 4), Extent::None);
        assert_eq!(Keys::extent(2, 4), Extent::Some);
        assert_eq!(Keys::extent(4, 4), Extent::All);
        assert_eq!(Keys::extent(0, 0), Extent::None, "an empty list");
    }
}
