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
    // A comparison is made a hundred thousand times a view: nothing is
    // allocated, and plain ASCII (most names) is walked byte by byte.
    if a.is_ascii() && b.is_ascii() {
        return natural_ascii(a.as_bytes(), b.as_bytes());
    }
    natural_chunks(a, b)
}

/// [`natural`] for any text: chunk by chunk, as TanStack wrote it.
pub(super) fn natural_chunks(a: &str, b: &str) -> Ordering {
    let (mut left, mut right) = (chunks(a), chunks(b));
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
                    (None, None) => lowered(one).cmp(lowered(other)),
                };
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        }
    }
}

/// [`natural`] over ASCII: the same order, without cutting into chunks.
fn natural_ascii(a: &[u8], b: &[u8]) -> Ordering {
    let (mut i, mut j) = (0, 0);
    // Whether both sides are at the start of a chunk.
    let mut fresh = true;
    loop {
        let (Some(&one), Some(&other)) = (a.get(i), b.get(j)) else {
            // What has more to say comes after.
            return (a.len() - i).cmp(&(b.len() - j));
        };
        match (one.is_ascii_digit(), other.is_ascii_digit()) {
            (true, true) => {
                let run = |text: &[u8], from: usize| {
                    let end = text[from..]
                        .iter()
                        .position(|byte| !byte.is_ascii_digit())
                        .map_or(text.len(), |length| from + length);
                    let zeros = text[from..end].iter().take_while(|byte| **byte == b'0');
                    (from + zeros.count(), end)
                };
                let ((from_a, end_a), (from_b, end_b)) = (run(a, i), run(b, j));
                let (digits_a, digits_b) = (&a[from_a..end_a], &b[from_b..end_b]);
                let ordering = digits_a
                    .len()
                    .cmp(&digits_b.len())
                    .then_with(|| digits_a.cmp(digits_b));
                if ordering != Ordering::Equal {
                    return ordering;
                }
                (i, j, fresh) = (end_a, end_b, true);
            }
            // A chunk of text against a number: the text first. In the
            // middle of two chunks of text, the one that ends is the
            // shorter, and comes first.
            (false, true) if fresh => return Ordering::Less,
            (true, false) if fresh => return Ordering::Greater,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            (false, false) => {
                let ordering = one.to_ascii_lowercase().cmp(&other.to_ascii_lowercase());
                if ordering != Ordering::Equal {
                    return ordering;
                }
                (i, j, fresh) = (i + 1, j + 1, false);
            }
        }
    }
}

/// The letters of `text`, in lower case.
fn lowered(text: &str) -> impl Iterator<Item = char> + '_ {
    text.chars().flat_map(char::to_lowercase)
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
