//! The lines a viewer keeps: appended at the end, dropped from the start
//! past a cap, and never moved in between.
//!
//! They are kept in chunks of [`CHUNK`] lines, each one string with where
//! every line ends: a million lines are a thousand allocations, not a
//! million, and cost their text and sixteen bytes each. A line is known by
//! its sequence number, which never changes: appending touches the last
//! chunk only, dropping the oldest lets go of whole chunks from the front,
//! and neither looks at the lines in between.
//!
//! A chunk is shared ([`Arc`]): a [`Snapshot`] of a range of lines is a few
//! pointers, which a task takes to another thread to search or to save
//! while the viewer goes on appending.

use std::{collections::VecDeque, ops::Range, sync::Arc};

use super::{
    ansi::Run,
    line::{cells, has_wide, Marker, Stream},
};

/// The lines of a chunk.
pub const CHUNK: usize = 1024;

const NO_SOURCE: u16 = u16::MAX;
const MARKER: u8 = 1 << 2;
/// The line's text is ASCII: a cell is a byte.
const ASCII: u8 = 1 << 3;
/// A character of the line takes two cells: its rows are not a division.
const WIDE: u8 = 1 << 4;

/// What is kept of a line beside its text.
#[derive(Debug, Clone, Copy)]
struct Meta {
    /// Where its text ends in the chunk's, and its runs in the chunk's.
    end: u32,
    runs_end: u32,
    /// The cells of its message, the timestamp left out.
    cells: u32,
    source: u16,
    /// The bytes of its timestamp; none when 0.
    stamp: u8,
    flags: u8,
}

/// Up to [`CHUNK`] lines.
#[derive(Debug, Clone, Default)]
pub struct Chunk {
    text: String,
    meta: Vec<Meta>,
    runs: Vec<Run>,
    markers: Vec<(u16, Marker)>,
}

/// A line of the buffer, read in place.
#[derive(Debug, Clone, Copy)]
pub struct LineRef<'a> {
    /// The whole text, its timestamp included, without escape sequences.
    pub text: &'a str,
    /// Its runs that are not plain, by the bytes of `text`.
    pub runs: &'a [Run],
    /// The bytes of the timestamp `text` starts with; the message starts
    /// one further, after the space. `0`: no timestamp.
    pub stamp: usize,
    /// The cells of the message.
    pub cells: usize,
    /// Which source wrote it, by its number among the viewer's.
    pub source: Option<u16>,
    pub stream: Stream,
    /// Not a line of the log: something the viewer says.
    pub marker: Option<Marker>,
    /// Its text is ASCII: a cell is a byte.
    pub ascii: bool,
    /// One of its characters takes two cells.
    pub wide: bool,
}

impl<'a> LineRef<'a> {
    /// Where the message starts in `text`.
    pub fn message_start(&self) -> usize {
        if self.stamp == 0 {
            0
        } else {
            self.stamp + 1
        }
    }

    /// The text without its timestamp.
    pub fn message(&self) -> &'a str {
        &self.text[self.message_start()..]
    }

    pub fn timestamp(&self) -> Option<&'a str> {
        (self.stamp > 0).then(|| &self.text[..self.stamp])
    }
}

impl Chunk {
    fn len(&self) -> usize {
        self.meta.len()
    }

    fn line(&self, index: usize) -> LineRef<'_> {
        let meta = self.meta[index];
        let (start, runs_start) = match index.checked_sub(1) {
            Some(before) => (self.meta[before].end, self.meta[before].runs_end),
            None => (0, 0),
        };
        let marker = (meta.flags & MARKER != 0)
            .then(|| {
                self.markers
                    .iter()
                    .find(|(at, _)| usize::from(*at) == index)
            })
            .flatten()
            .map(|(_, marker)| *marker);
        LineRef {
            text: &self.text[start as usize..meta.end as usize],
            runs: &self.runs[runs_start as usize..meta.runs_end as usize],
            stamp: usize::from(meta.stamp),
            cells: meta.cells as usize,
            source: (meta.source != NO_SOURCE).then_some(meta.source),
            stream: Stream::from_bits(meta.flags),
            marker,
            ascii: meta.flags & ASCII != 0,
            wide: meta.flags & WIDE != 0,
        }
    }

    /// Adds a line: its timestamp as it came, when it has one, then its
    /// message, which `write` puts at the end of the string it is given.
    /// It is told where the line and the message start in that string,
    /// and puts the line's runs in the list.
    pub fn push(
        &mut self,
        stream: Stream,
        source: Option<u16>,
        stamp: Option<&str>,
        write: impl FnOnce(&mut String, usize, usize, &mut Vec<Run>),
    ) {
        let start = self.text.len();
        if let Some(stamp) = stamp {
            self.text.push_str(stamp);
            self.text.push(' ');
        }
        let message_start = self.text.len();
        write(&mut self.text, start, message_start, &mut self.runs);
        let message = &self.text[message_start..];
        let mut flags = stream.bits();
        if message.is_ascii() {
            flags |= ASCII;
        } else if has_wide(message) {
            flags |= WIDE;
        }
        let taken = cells(message);
        self.end_line(taken, source, stamp.map_or(0, str::len), flags);
    }

    /// The bytes of text of its lines `from..to`.
    fn span(&self, from: usize, to: usize) -> usize {
        let start = from
            .checked_sub(1)
            .map_or(0, |before| self.meta[before].end);
        let end = to.checked_sub(1).map_or(0, |last| self.meta[last].end);
        end.saturating_sub(start) as usize
    }

    /// The cells of the line added last, its timestamp included.
    pub fn last_width(&self) -> usize {
        self.meta.last().map_or(0, |meta| {
            let stamp = usize::from(meta.stamp);
            meta.cells as usize + if stamp > 0 { stamp + 1 } else { 0 }
        })
    }

    /// Adds something the viewer says.
    pub fn push_marker(&mut self, marker: Marker) {
        // A chunk holds 1024 lines.
        #[allow(clippy::cast_possible_truncation)]
        self.markers.push((self.len() as u16, marker));
        self.end_line(0, None, 0, MARKER | ASCII);
    }

    // A chunk's text is its lines': far below 4 GiB, as is a line's cells.
    #[allow(clippy::cast_possible_truncation)]
    fn end_line(&mut self, cells: usize, source: Option<u16>, stamp: usize, flags: u8) {
        self.meta.push(Meta {
            end: self.text.len() as u32,
            runs_end: self.runs.len() as u32,
            cells: cells as u32,
            source: source.unwrap_or(NO_SOURCE),
            stamp: stamp as u8,
            flags,
        });
    }
}

/// The lines kept, the oldest first.
#[derive(Debug, Clone)]
pub struct Buffer {
    chunks: VecDeque<Arc<Chunk>>,
    /// The sequence number of the first line of the first chunk.
    base: u64,
    /// Of the oldest line kept, and of the line the next one will be.
    first: u64,
    end: u64,
    cap: usize,
    /// The most chunks it ever held at once.
    #[cfg(test)]
    pub(super) most_chunks: usize,
}

impl Buffer {
    /// A buffer that keeps the last `cap` lines.
    pub fn new(cap: usize) -> Self {
        Self {
            chunks: VecDeque::new(),
            base: 0,
            first: 0,
            end: 0,
            cap: cap.max(1),
            #[cfg(test)]
            most_chunks: 0,
        }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    /// Keeps the last `cap` lines from now on. Returns how many were
    /// dropped for it.
    pub fn set_cap(&mut self, cap: usize) -> u64 {
        self.cap = cap.max(1);
        self.trim()
    }

    // A buffer holds what memory holds: its length fits a `usize`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn len(&self) -> usize {
        (self.end - self.first) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.end == self.first
    }

    /// The sequence numbers of the lines kept.
    pub fn range(&self) -> Range<u64> {
        self.first..self.end
    }

    #[allow(clippy::cast_possible_truncation)]
    fn place(&self, seq: u64) -> Option<(usize, usize)> {
        if !self.range().contains(&seq) {
            return None;
        }
        let offset = (seq - self.base) as usize;
        Some((offset / CHUNK, offset % CHUNK))
    }

    /// The line `seq`, if it is still kept.
    pub fn line(&self, seq: u64) -> Option<LineRef<'_>> {
        let (chunk, line) = self.place(seq)?;
        Some(self.chunks[chunk].line(line))
    }

    /// The last chunk, with room for a line.
    fn open(&mut self) -> &mut Chunk {
        if self.chunks.back().is_none_or(|last| last.len() >= CHUNK) {
            self.chunks.push_back(Arc::new(Chunk {
                meta: Vec::with_capacity(CHUNK),
                ..Chunk::default()
            }));
        }
        // Shared with a snapshot: this copies the one chunk, once.
        Arc::make_mut(self.chunks.back_mut().expect("a chunk was just added"))
    }

    /// Appends a line for each of `items`, which `write` adds to the chunk
    /// it is given (one [`Chunk::push`] or [`Chunk::push_marker`] each),
    /// then drops the oldest lines past the cap. Returns how many were
    /// dropped.
    pub fn append<T>(
        &mut self,
        items: impl IntoIterator<Item = T>,
        mut write: impl FnMut(&mut Chunk, T),
    ) -> u64 {
        let mut items = items.into_iter().peekable();
        let mut dropped = 0;
        while items.peek().is_some() {
            let chunk = self.open();
            let before = chunk.len();
            while chunk.len() < CHUNK {
                let Some(item) = items.next() else { break };
                let had = chunk.len();
                write(chunk, item);
                debug_assert_eq!(chunk.len(), had + 1, "one line an item");
            }
            let added = chunk.len() - before;
            self.end += added as u64;
            // As it goes: a batch larger than the cap is never held whole.
            dropped += self.trim();
            #[cfg(test)]
            {
                self.most_chunks = self.most_chunks.max(self.chunks.len());
            }
        }
        dropped
    }

    /// Drops the oldest lines past the cap.
    fn trim(&mut self) -> u64 {
        let dropped = (self.len().saturating_sub(self.cap)) as u64;
        self.first += dropped;
        while self.first - self.base >= CHUNK as u64 && !self.chunks.is_empty() {
            self.chunks.pop_front();
            self.base += CHUNK as u64;
        }
        dropped
    }

    /// How many bytes of text the lines `range` hold: what reading them
    /// costs, without reading them.
    pub fn bytes(&self, range: Range<u64>) -> usize {
        let from = range.start.max(self.first);
        let to = range.end.min(self.end);
        if from >= to {
            return 0;
        }
        let (Some((first, first_line)), Some((last, last_line))) =
            (self.place(from), self.place(to - 1))
        else {
            return 0;
        };
        (first..=last)
            .map(|at| {
                let chunk = &self.chunks[at];
                let start = if at == first { first_line } else { 0 };
                let end = if at == last {
                    last_line + 1
                } else {
                    chunk.len()
                };
                chunk.span(start, end)
            })
            .sum()
    }

    /// Drops every line. The numbering goes on: a line appended next is
    /// not mistaken for one that was there.
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.base = self.end;
        self.first = self.end;
    }

    /// The lines `range` (those of them still kept), to read elsewhere.
    pub fn snapshot(&self, range: Range<u64>) -> Snapshot {
        let from = range.start.max(self.first);
        let to = range.end.min(self.end).max(from);
        let (Some((first, _)), Some((last, _))) = (self.place(from), self.place(to.max(1) - 1))
        else {
            return Snapshot::default();
        };
        if from == to {
            return Snapshot::default();
        }
        Snapshot {
            chunks: self.chunks.range(first..=last).cloned().collect(),
            base: self.base + (first * CHUNK) as u64,
            from,
            to,
        }
    }
}

/// Some lines of a buffer as they were when it was taken: what a task
/// reads on another thread.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    chunks: Vec<Arc<Chunk>>,
    base: u64,
    from: u64,
    to: u64,
}

impl Snapshot {
    /// The sequence numbers of its lines.
    pub fn range(&self) -> Range<u64> {
        self.from..self.to
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn len(&self) -> usize {
        (self.to - self.from) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.from == self.to
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn line(&self, seq: u64) -> Option<LineRef<'_>> {
        if !self.range().contains(&seq) {
            return None;
        }
        let offset = (seq - self.base) as usize;
        Some(self.chunks[offset / CHUNK].line(offset % CHUNK))
    }

    /// The bytes of text of its lines: what reading it costs.
    #[allow(clippy::cast_possible_truncation)]
    pub fn bytes(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let place = |seq: u64| {
            let offset = (seq - self.base) as usize;
            (offset / CHUNK, offset % CHUNK)
        };
        let ((first, first_line), (last, last_line)) = (place(self.from), place(self.to - 1));
        (first..=last)
            .map(|at| {
                let chunk = &self.chunks[at];
                let start = if at == first { first_line } else { 0 };
                let end = if at == last {
                    last_line + 1
                } else {
                    chunk.len()
                };
                chunk.span(start, end)
            })
            .sum()
    }

    /// Its lines, the oldest first, each with its sequence number.
    pub fn lines(&self) -> impl Iterator<Item = (u64, LineRef<'_>)> {
        self.range()
            .filter_map(|seq| self.line(seq).map(|line| (seq, line)))
    }
}
