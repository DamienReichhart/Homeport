//! What is selected in a viewer: from one place in the text to another,
//! as in a terminal. A place is a line and a cell of what the line shows
//! (its timestamp and its source's label included), so a selection holds
//! while lines come and go around it.

/// A place in the text: before the cell `cell` of the line `seq`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mark {
    pub seq: u64,
    pub cell: usize,
}

impl Mark {
    pub const fn new(seq: u64, cell: usize) -> Self {
        Self { seq, cell }
    }
}

/// From where the selection started to where it was taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Mark,
    pub head: Mark,
}

impl Selection {
    pub const fn new(anchor: Mark, head: Mark) -> Self {
        Self { anchor, head }
    }

    /// Its two ends, the earlier first.
    pub fn ends(&self) -> (Mark, Mark) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    /// Nothing is between its ends.
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// The cells of the line `seq` it covers, from the first to before
    /// the last (`usize::MAX`: to the end of the line, and the line break
    /// with it).
    pub fn cells_of(&self, seq: u64) -> Option<(usize, usize)> {
        let (from, to) = self.ends();
        if seq < from.seq || seq > to.seq || self.is_empty() {
            return None;
        }
        let start = if seq == from.seq { from.cell } else { 0 };
        let end = if seq == to.seq { to.cell } else { usize::MAX };
        (start < end).then_some((start, end))
    }
}

/// The cells `from..to` of the word around the cell `cell` of `text`:
/// letters, digits and what holds a path or an address together. On
/// anything else, that one character.
pub fn word_at(text: &str, cell: usize) -> (usize, usize) {
    use super::line::cell_width;
    let is_word = |c: char| c.is_alphanumeric() || "_-./:@".contains(c);
    // Each character with the cell it starts at.
    let mut at = 0;
    let placed: Vec<(usize, char)> = text
        .chars()
        .map(|c| {
            let start = at;
            at += cell_width(c);
            (start, c)
        })
        .collect();
    let end_of = |index: usize| placed.get(index + 1).map_or(at, |(start, _)| *start);
    let Some(index) = placed
        .iter()
        .rposition(|(start, _)| *start <= cell)
        .filter(|_| cell < at)
    else {
        return (at, at);
    };
    if !is_word(placed[index].1) {
        return (placed[index].0, end_of(index));
    }
    let first = (0..=index)
        .rev()
        .take_while(|i| is_word(placed[*i].1))
        .last()
        .unwrap_or(index);
    let last = (index..placed.len())
        .take_while(|i| is_word(placed[*i].1))
        .last()
        .unwrap_or(index);
    (placed[first].0, end_of(last))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_selection_covers_its_lines_between_its_ends() {
        let selection = Selection::new(Mark::new(7, 4), Mark::new(5, 2));
        assert_eq!(selection.ends(), (Mark::new(5, 2), Mark::new(7, 4)));
        assert_eq!(selection.cells_of(4), None);
        assert_eq!(selection.cells_of(5), Some((2, usize::MAX)));
        assert_eq!(selection.cells_of(6), Some((0, usize::MAX)));
        assert_eq!(selection.cells_of(7), Some((0, 4)));
        assert_eq!(selection.cells_of(8), None);
        let one = Selection::new(Mark::new(3, 2), Mark::new(3, 9));
        assert_eq!(one.cells_of(3), Some((2, 9)));
        assert!(Selection::new(Mark::new(3, 2), Mark::new(3, 2)).is_empty());
        // Up to the start of a line: the line before, whole, and no more.
        let to_start = Selection::new(Mark::new(3, 0), Mark::new(4, 0));
        assert_eq!(to_start.cells_of(3), Some((0, usize::MAX)));
        assert_eq!(to_start.cells_of(4), None);
    }

    #[test]
    fn a_word_is_what_a_path_or_an_address_is_made_of() {
        let text = "GET /api/v1/users 200 from 10.0.0.7:8080";
        assert_eq!(word_at(text, 0), (0, 3));
        assert_eq!(word_at(text, 6), (4, 17));
        assert_eq!(word_at(text, 3), (3, 4), "a space is itself");
        assert_eq!(word_at(text, 30), (27, 40));
        assert_eq!(word_at(text, 99), (40, 40), "past the end: nothing");
        // Wide characters count their two cells.
        assert_eq!(word_at("日本 ok", 1), (0, 4));
        assert_eq!(word_at("日本 ok", 5), (5, 7));
    }
}
