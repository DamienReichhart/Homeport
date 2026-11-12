//! Finding one's way in a line: which byte a cell is, which cell a byte
//! is, where its rows start and where its matches are.
//!
//! In a line of ASCII a cell is a byte and all of it is arithmetic. In
//! any other line the characters before a place have to be walked. For a
//! short line that walk is nothing; for a long one (a megabyte of JSON
//! with one accent in it) it is done once: the line gets marks every
//! [`STEP`] cells, the starts of its rows when a wide character keeps
//! them from being a division, and the list of its matches. A layout
//! then walks only what it shows, however long the line and however many
//! its matches.

use std::{collections::HashMap, rc::Rc};

use super::{
    buffer::LineRef,
    display::Shape,
    line::{cells, widths},
    search,
    state::State,
};

/// A line of up to this many bytes is walked each time: no marks.
pub const SHORT: usize = 1024;
/// The cells between two marks of a long line.
pub const STEP: usize = 128;
/// How many long lines are remembered: more than a viewer shows.
const LINES: usize = 64;

/// A byte of a message and the cell the character there starts at.
type Mark = (u32, u32);
/// The bytes of a line's text a match covers.
pub type Match = (u32, u32);

#[derive(Default)]
struct Entry {
    marks: Option<Rc<[Mark]>>,
    /// For these columns and this many cells before the message.
    rows: Option<(usize, usize, Rc<[u32]>)>,
    /// For the search of this generation.
    matches: Option<(u64, Rc<[Match]>)>,
}

/// What is remembered of the long lines in view.
#[derive(Default)]
pub struct Cache {
    lines: HashMap<u64, Entry>,
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cache({} lines)", self.lines.len())
    }
}

impl Cache {
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Lets go of the lines before `first`, which the buffer dropped.
    pub fn evict(&mut self, first: u64) {
        self.lines.retain(|seq, _| *seq >= first);
    }

    fn entry(&mut self, seq: u64) -> &mut Entry {
        if self.lines.len() >= LINES && !self.lines.contains_key(&seq) {
            self.lines.clear();
        }
        self.lines.entry(seq).or_default()
    }
}

#[cfg(test)]
thread_local! {
    /// How many characters this thread walked to find its way in lines:
    /// what a test counts to hold a layout to what it shows.
    pub static WALKED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg_attr(not(test), allow(unused_variables, clippy::missing_const_for_fn))]
pub(super) fn walked(characters: usize) {
    #[cfg(test)]
    WALKED.with(|count| count.set(count.get() + characters));
}

/// Where the rows of a line start, among the cells it shows.
#[derive(Debug, Clone)]
pub enum Starts {
    /// Every `cols` cells: no wide character to keep whole.
    Even { cols: usize, total: usize },
    /// Where each row starts, and how many cells the line shows.
    Listed(Rc<[u32]>, usize),
}

impl Starts {
    pub fn len(&self) -> usize {
        match self {
            Self::Even { cols, total } => (*total).max(1).div_ceil((*cols).max(1)),
            Self::Listed(starts, _) => starts.len(),
        }
    }

    /// The cells the row `part` shows, from the first to before the last.
    pub fn range(&self, part: usize) -> (usize, usize) {
        match self {
            Self::Even { cols, total } => {
                ((part * cols).min(*total), ((part + 1) * cols).min(*total))
            }
            Self::Listed(starts, total) => {
                let from = starts.get(part).map_or(*total, |start| *start as usize);
                let to = starts.get(part + 1).map_or(*total, |start| *start as usize);
                (from, to)
            }
        }
    }

    /// The row the cell `cell` is on.
    pub fn row_of(&self, cell: usize) -> usize {
        match self {
            Self::Even { cols, .. } => cell / (*cols).max(1),
            Self::Listed(starts, _) => starts
                .partition_point(|start| *start as usize <= cell)
                .saturating_sub(1),
        }
    }
}

/// Where the rows of a message start when `prefix` cells stand before it
/// and a row holds `cols`: a character that would not fit whole at the
/// end of a row starts the next one.
// A line's cells fit a `u32`, as its bytes do.
#[allow(clippy::cast_possible_truncation)]
fn starts_of(message: &str, prefix: usize, cols: usize, mut each: impl FnMut(u32)) {
    let cols = cols.max(1);
    let mut start = 0;
    each(0);
    while prefix > start + cols {
        start += cols;
        each(start as u32);
    }
    let mut cell = prefix;
    let mut count = 0;
    for placed in widths(message) {
        count += 1;
        if placed.width > 0 && cell + placed.width > start + cols && cell > start {
            start = cell;
            each(start as u32);
        }
        cell += placed.width;
    }
    walked(count);
}

impl State {
    /// The marks of the long message of the line `seq`.
    #[allow(clippy::cast_possible_truncation)]
    fn marks(&self, seq: u64, message: &str) -> Rc<[Mark]> {
        let mut cache = self.index.borrow_mut();
        let entry = cache.entry(seq);
        if let Some(marks) = &entry.marks {
            return marks.clone();
        }
        let mut marks = vec![];
        let (mut cell, mut next, mut count) = (0, 0, 0);
        for placed in widths(message) {
            count += 1;
            if cell >= next {
                marks.push((placed.byte as u32, cell as u32));
                next += STEP;
            }
            cell += placed.width;
        }
        walked(count);
        if marks.is_empty() {
            marks.push((0, 0));
        }
        let marks: Rc<[Mark]> = marks.into();
        entry.marks = Some(marks.clone());
        marks
    }

    /// The byte of the message of the line `seq` at which its cell
    /// `cell` is, and the cell that character starts at. Past the end:
    /// the message's length and cells.
    pub(super) fn byte_of_cell(&self, seq: u64, line: &LineRef<'_>, cell: usize) -> (usize, usize) {
        let message = line.message();
        if line.ascii {
            let byte = cell.min(message.len());
            return (byte, byte);
        }
        let (mut byte, mut at) = if message.len() <= SHORT {
            (0, 0)
        } else {
            let marks = self.marks(seq, message);
            let mut k = (cell / STEP).min(marks.len() - 1);
            if marks[k].1 as usize > cell {
                k = k.saturating_sub(1);
            }
            (marks[k].0 as usize, marks[k].1 as usize)
        };
        let mut count = 0;
        for placed in widths(&message[byte..]) {
            count += 1;
            if placed.width > 0 && at + placed.width > cell {
                walked(count);
                return (byte + placed.byte, at);
            }
            at += placed.width;
        }
        walked(count);
        byte = message.len();
        (byte, at)
    }

    /// The cell of the message of the line `seq` at which its byte
    /// `byte` is.
    pub(super) fn cell_of_byte(&self, seq: u64, line: &LineRef<'_>, byte: usize) -> usize {
        let message = line.message();
        let byte = byte.min(message.len());
        if line.ascii {
            return byte;
        }
        let (from, at) = if message.len() <= SHORT {
            (0, 0)
        } else {
            let marks = self.marks(seq, message);
            let k = marks
                .partition_point(|mark| mark.0 as usize <= byte)
                .saturating_sub(1);
            (marks[k].0 as usize, marks[k].1 as usize)
        };
        walked(byte - from);
        at + cells(&message[from..byte])
    }

    /// Where the rows of the line `seq` start in `cols` cells.
    pub(super) fn starts(&self, seq: u64, line: &LineRef<'_>, shape: Shape, cols: usize) -> Starts {
        let prefix = shape.prefix(line);
        let total = prefix + line.cells;
        if !line.wide || line.marker.is_some() {
            return Starts::Even { cols, total };
        }
        let message = line.message();
        let list = |message: &str| -> Rc<[u32]> {
            let mut starts = vec![];
            starts_of(message, prefix, cols, |start| starts.push(start));
            starts.into()
        };
        if message.len() <= SHORT {
            return Starts::Listed(list(message), total);
        }
        let mut cache = self.index.borrow_mut();
        let entry = cache.entry(seq);
        let starts = match &entry.rows {
            Some((for_cols, for_prefix, starts)) if (*for_cols, *for_prefix) == (cols, prefix) => {
                starts.clone()
            }
            _ => {
                let starts = list(message);
                entry.rows = Some((cols, prefix, starts.clone()));
                starts
            }
        };
        Starts::Listed(starts, total)
    }

    /// How many rows the line `seq` takes in `cols` cells.
    pub(super) fn row_count(
        &self,
        seq: u64,
        line: &LineRef<'_>,
        shape: Shape,
        cols: usize,
    ) -> usize {
        if line.wide && line.marker.is_none() && line.message().len() <= SHORT {
            // Counted without a list: this is asked for every line in view.
            let mut count = 0;
            starts_of(line.message(), shape.prefix(line), cols, |_| count += 1);
            return count;
        }
        self.starts(seq, line, shape, cols).len()
    }

    /// The matches of the search in the line `seq`, in order, as bytes
    /// of its whole text.
    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn matches_of(&self, seq: u64, line: &LineRef<'_>) -> Rc<[Match]> {
        let Some(matcher) = &self.search.matcher else {
            return Rc::from([]);
        };
        let find = || -> Rc<[Match]> {
            search::matches(matcher, line, self.timestamps)
                .map(|(start, end)| (start as u32, end as u32))
                .collect()
        };
        if line.text.len() <= SHORT {
            return find();
        }
        let generation = self.search.generation;
        let mut cache = self.index.borrow_mut();
        let entry = cache.entry(seq);
        match &entry.matches {
            Some((of, matches)) if *of == generation => matches.clone(),
            _ => {
                let matches = find();
                entry.matches = Some((generation, matches.clone()));
                matches
            }
        }
    }

    /// Among what the line `seq` shows, the cell at which the byte
    /// `byte` of its whole text is.
    pub(super) fn shown_cell(
        &self,
        seq: u64,
        line: &LineRef<'_>,
        shape: Shape,
        byte: usize,
    ) -> usize {
        let message = line.message_start();
        if byte < message {
            // In the timestamp, which is ASCII and shows from the start.
            return byte;
        }
        shape.prefix(line) + self.cell_of_byte(seq, line, byte - message)
    }
}
