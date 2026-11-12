//! The order of the rows: which columns sort them, which way, and the
//! arrangement itself, which is stable (rows that compare equal keep the
//! order they were given in).

use std::cmp::Ordering;

use super::ColumnId;

/// The way a column sorts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub fn apply(self, ordering: Ordering) -> Ordering {
        match self {
            Self::Ascending => ordering,
            Self::Descending => ordering.reverse(),
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

/// What a press on a header leads to, as TanStack's: unsorted, then the
/// column's first way (ascending, or descending for a column of numbers),
/// then the other, then unsorted again.
pub fn next(current: Option<Order>, descending_first: bool) -> Option<Order> {
    let first = if descending_first {
        Order::Descending
    } else {
        Order::Ascending
    };
    match current {
        None => Some(first),
        Some(order) if order == first => Some(order.other()),
        Some(_) => None,
    }
}

/// The columns the rows are sorted by, the first one first.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sort(Vec<(ColumnId, Order)>);

impl Sort {
    /// Sorted by one column: `initialSorting`.
    pub fn by(column: ColumnId, order: Order) -> Self {
        Self(vec![(column, order)])
    }

    /// From what was kept: every column and its way, the first one first.
    pub fn from_keys(keys: impl IntoIterator<Item = (ColumnId, Order)>) -> Self {
        let mut sort = Self::default();
        for (column, order) in keys {
            if sort.of(column).is_none() {
                sort.0.push((column, order));
            }
        }
        sort
    }

    /// Every column the rows are sorted by, the first one first: what a
    /// page keeps to find the table as it was left.
    pub fn keys(&self) -> &[(ColumnId, Order)] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The way `column` sorts, if it does.
    pub fn of(&self, column: ColumnId) -> Option<Order> {
        self.0
            .iter()
            .find(|(id, _)| *id == column)
            .map(|(_, order)| *order)
    }

    /// A press on the header of `column`. Alone, the column becomes the
    /// only one that sorts; `additive` (Shift held) keeps the others and
    /// sorts by this one after them.
    pub fn toggle(&mut self, column: ColumnId, descending_first: bool, additive: bool) {
        let next = next(self.of(column), descending_first);
        if !additive {
            self.0.clear();
            self.0.extend(next.map(|order| (column, order)));
            return;
        }
        let at = self.0.iter().position(|(id, _)| *id == column);
        match (at, next) {
            (Some(at), Some(order)) => self.0[at].1 = order,
            (Some(at), None) => {
                self.0.remove(at);
            }
            (None, Some(order)) => self.0.push((column, order)),
            (None, None) => {}
        }
    }
}

/// Text as TanStack's automatic sort compares it (`alphanumeric`, which
/// is `text` when there is no digit): whatever the case, and a run of
/// digits by its value, so `host-2` comes before `host-10`.
pub fn natural(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.to_lowercase(), b.to_lowercase());
    let (mut left, mut right) = (chunks(&a), chunks(&b));
    loop {
        match (left.next(), right.next()) {
            (None, None) => return Ordering::Equal,
            // What has more to say comes after.
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(one), Some(other)) => {
                let ordering = match (number(one), number(other)) {
                    (Some(one), Some(other)) => one.cmp(&other),
                    // Text comes before a number.
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => one.cmp(other),
                };
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        }
    }
}

/// `text` cut into its runs of digits and what is between them.
fn chunks(text: &str) -> impl Iterator<Item = &str> {
    let mut rest = text;
    std::iter::from_fn(move || {
        let first = rest.chars().next()?;
        let digits = first.is_ascii_digit();
        let end = rest
            .find(|c: char| c.is_ascii_digit() != digits)
            .unwrap_or(rest.len());
        let (chunk, after) = rest.split_at(end);
        rest = after;
        Some(chunk)
    })
}

/// The value of a run of digits, however long: its length without the
/// zeros in front, then its digits.
fn number(chunk: &str) -> Option<(usize, &str)> {
    chunk.starts_with(|c: char| c.is_ascii_digit()).then(|| {
        let digits = chunk.trim_start_matches('0');
        (digits.len(), digits)
    })
}

/// Puts the rows `order` names in the order `compare` gives, rows that
/// compare equal by their place in `rows`. The result is the one stable
/// sort of them, whatever `order` held before; an `order` that was already
/// sorted (the usual case from one view to the next) costs one pass.
pub fn arrange<T>(order: &mut [u32], rows: &[T], compare: impl Fn(&T, &T) -> Ordering) {
    order.sort_by(|&a, &b| compare(&rows[a as usize], &rows[b as usize]).then(a.cmp(&b)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_header_cycles_through_both_ways_and_back_to_none() {
        assert_eq!(next(None, false), Some(Order::Ascending));
        assert_eq!(next(Some(Order::Ascending), false), Some(Order::Descending));
        assert_eq!(next(Some(Order::Descending), false), None);
        // A column of numbers starts with the largest.
        assert_eq!(next(None, true), Some(Order::Descending));
        assert_eq!(next(Some(Order::Descending), true), Some(Order::Ascending));
        assert_eq!(next(Some(Order::Ascending), true), None);
    }

    #[test]
    fn a_press_alone_sorts_by_that_column_only() {
        let mut sort = Sort::by("name", Order::Ascending);
        sort.toggle("size", true, false);
        assert_eq!(sort.keys(), [("size", Order::Descending)]);
        sort.toggle("size", true, false);
        assert_eq!(sort.keys(), [("size", Order::Ascending)]);
        sort.toggle("size", true, false);
        assert!(sort.is_empty());
    }

    #[test]
    fn a_press_with_shift_adds_the_column_after_the_others() {
        let mut sort = Sort::by("status", Order::Ascending);
        sort.toggle("name", false, true);
        assert_eq!(
            sort.keys(),
            [("status", Order::Ascending), ("name", Order::Ascending)]
        );
        sort.toggle("status", false, true);
        assert_eq!(sort.of("status"), Some(Order::Descending));
        assert_eq!(sort.keys()[0].0, "status", "it keeps its place");
        sort.toggle("status", false, true);
        assert_eq!(sort.keys(), [("name", Order::Ascending)]);
    }

    #[test]
    fn what_was_kept_names_a_column_once() {
        let sort = Sort::from_keys([
            ("name", Order::Ascending),
            ("name", Order::Descending),
            ("size", Order::Descending),
        ]);
        assert_eq!(
            sort.keys(),
            [("name", Order::Ascending), ("size", Order::Descending)]
        );
    }

    #[test]
    fn text_sorts_whatever_its_case_and_digits_by_their_value() {
        let mut names = vec![
            "host-10", "Host-2", "alpha", "host-1", "Beta", "host", "7", "host-02a",
        ];
        names.sort_by(|a, b| natural(a, b));
        assert_eq!(
            names,
            ["alpha", "Beta", "host", "host-1", "Host-2", "host-02a", "host-10", "7"]
        );
        assert_eq!(natural("Prod", "prod"), Ordering::Equal);
        assert_eq!(natural("a", "B"), Ordering::Less, "not by code point");
        assert_eq!(natural("v9", "v10"), Ordering::Less);
        // Longer than any integer type, and still by value.
        assert_eq!(
            natural("id-99999999999999999999999", "id-100000000000000000000000"),
            Ordering::Less
        );
        assert_eq!(natural("10.0.0.9:2376", "10.0.0.10:2376"), Ordering::Less);
    }

    #[test]
    fn rows_that_compare_equal_keep_their_order() {
        let rows = [("b", 1), ("a", 2), ("b", 3), ("a", 4), ("c", 5)];
        let mut order: Vec<u32> = (0..5).collect();
        arrange(&mut order, &rows, |a, b| a.0.cmp(b.0));
        let sorted: Vec<i32> = order.iter().map(|&i| rows[i as usize].1).collect();
        assert_eq!(sorted, [2, 4, 1, 3, 5]);
        // Descending reverses the keys, not the rows that share one.
        arrange(&mut order, &rows, |a, b| {
            Order::Descending.apply(a.0.cmp(b.0))
        });
        let sorted: Vec<i32> = order.iter().map(|&i| rows[i as usize].1).collect();
        assert_eq!(sorted, [5, 1, 3, 2, 4]);
    }

    #[test]
    fn the_arrangement_does_not_depend_on_where_it_started() {
        let rows = [3, 1, 2, 1, 3, 2];
        let mut from_start: Vec<u32> = (0..6).collect();
        let mut from_end: Vec<u32> = (0..6).rev().collect();
        arrange(&mut from_start, &rows, Ord::cmp);
        arrange(&mut from_end, &rows, Ord::cmp);
        assert_eq!(from_start, from_end);
        assert_eq!(from_start, [1, 3, 2, 5, 0, 4]);
    }
}
