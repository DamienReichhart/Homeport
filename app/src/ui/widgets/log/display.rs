//! Which lines show, and what each shows. The view is anchored to a line,
//! never to a distance in pixels: the lines above it can be dropped and
//! lines can be added below it without it moving, and a million lines are
//! no further away than ten.
//!
//! Wrapped, a line is as many rows as its cells need (the font is
//! monospaced, so that is a division, not a measure): only the lines in
//! view are ever looked at, whatever the width of the viewer.

use std::sync::Arc;

use super::{
    buffer::{LineRef, Snapshot},
    line::{byte_at_cell, cells},
    selection::Selection,
    state::{Anchor, Metrics, State},
};

/// A row of a line among those that show: where a view starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Pos {
    /// The line's place among the lines that show.
    pub index: usize,
    /// Which of its rows.
    pub row: usize,
}

/// What a view shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub top: Pos,
    /// The latest line shows: nothing is below.
    pub at_end: bool,
}

/// What stands before the message of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    pub timestamps: bool,
    /// The cells of the longest label; the labels are padded to it.
    pub label_width: usize,
}

impl Shape {
    /// The cells of the timestamp and the space after it.
    pub fn stamp_cells(&self, line: &LineRef<'_>) -> usize {
        if self.timestamps && line.stamp > 0 {
            line.stamp + 1
        } else {
            0
        }
    }

    /// The cells of `label |` and the space after it.
    pub fn label_cells(&self, line: &LineRef<'_>) -> usize {
        if line.source.is_some() {
            self.label_width + 3
        } else {
            0
        }
    }

    /// Where the message starts.
    pub fn prefix(&self, line: &LineRef<'_>) -> usize {
        self.stamp_cells(line) + self.label_cells(line)
    }

    /// The cells of everything the line shows.
    pub fn cells(&self, line: &LineRef<'_>) -> usize {
        self.prefix(line) + line.cells
    }

    /// Everything the line shows, as text: what a selection copies.
    pub fn write(&self, line: &LineRef<'_>, label: &str, out: &mut String) {
        if let Some(stamp) = line.timestamp().filter(|_| self.timestamps) {
            out.push_str(stamp);
            out.push(' ');
        }
        if line.source.is_some() {
            out.push_str(label);
            let padding = self.label_width.saturating_sub(cells(label));
            out.extend(std::iter::repeat_n(' ', padding));
            out.push_str(" | ");
        }
        out.push_str(line.message());
    }
}

impl State {
    pub(super) fn shape(&self) -> Shape {
        Shape {
            timestamps: self.timestamps,
            label_width: self.sources.width,
        }
    }

    /// Whether only the lines that match show.
    pub(super) fn filtered(&self) -> bool {
        self.filter && self.search.is_active()
    }

    /// How many lines show.
    pub(super) fn count(&self) -> usize {
        if self.filtered() {
            self.search.hits.len()
        } else {
            self.buffer.len()
        }
    }

    /// The line at `index` among those that show.
    pub(super) fn seq_at(&self, index: usize) -> Option<u64> {
        if self.filtered() {
            self.search.hits.get(index).map(|hit| hit.seq)
        } else {
            let seq = self.buffer.range().start + index as u64;
            self.buffer.range().contains(&seq).then_some(seq)
        }
    }

    /// Where the line `seq` is among those that show, or where it would be.
    // A buffer's length fits a `usize`.
    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn index_of(&self, seq: u64) -> Result<usize, usize> {
        if self.filtered() {
            return self.search.position(seq);
        }
        let range = self.buffer.range();
        if seq < range.start {
            Err(0)
        } else if seq >= range.end {
            Err(self.buffer.len())
        } else {
            Ok((seq - range.start) as usize)
        }
    }

    /// How many rows the line at `index` takes in `cols` cells.
    pub(super) fn rows_of(&self, index: usize, cols: usize) -> usize {
        if !self.wrap {
            return 1;
        }
        let Some((seq, line)) = self
            .seq_at(index)
            .and_then(|seq| Some((seq, self.buffer.line(seq)?)))
        else {
            return 1;
        };
        self.row_count(seq, &line, self.shape(), cols)
    }

    /// Where a view that shows the latest line starts.
    pub(super) fn bottom(&self, metrics: Metrics) -> Pos {
        let count = self.count();
        if !self.wrap {
            return Pos {
                index: count.saturating_sub(metrics.rows),
                row: 0,
            };
        }
        let mut room = metrics.rows.max(1);
        for index in (0..count).rev() {
            let rows = self.rows_of(index, metrics.cols);
            if rows >= room {
                return Pos {
                    index,
                    row: rows - room,
                };
            }
            room -= rows;
        }
        Pos::default()
    }

    /// The row `by` rows after `from` (before, when negative), no further
    /// than the first row and the end of the lines.
    pub(super) fn moved(&self, from: Pos, by: i64, cols: usize) -> Pos {
        let count = self.count();
        let distance = usize::try_from(by.unsigned_abs()).unwrap_or(usize::MAX);
        if !self.wrap {
            let index = if by < 0 {
                from.index.saturating_sub(distance)
            } else {
                from.index.saturating_add(distance).min(count)
            };
            return Pos { index, row: 0 };
        }
        let mut at = from;
        let mut left = distance;
        if by < 0 {
            while left > 0 {
                if at.row >= left {
                    at.row -= left;
                    break;
                }
                if at.index == 0 {
                    return Pos::default();
                }
                left -= at.row + 1;
                at.index -= 1;
                at.row = self.rows_of(at.index, cols) - 1;
            }
        } else {
            while left > 0 && at.index < count {
                let rows = self.rows_of(at.index, cols);
                if at.row + left < rows {
                    at.row += left;
                    break;
                }
                left -= rows - at.row;
                at.index += 1;
                at.row = 0;
            }
        }
        at
    }

    /// What a viewer laid out as `metrics` shows: the latest lines while
    /// it follows, else from its anchor, and never past the latest line.
    pub(super) fn window(&self, metrics: Metrics) -> Window {
        let bottom = self.bottom(metrics);
        let top = if self.follow {
            bottom
        } else {
            let anchored = match self.index_of(self.anchor.seq) {
                Ok(index) => Pos {
                    index,
                    row: self
                        .anchor
                        .row
                        .min(self.rows_of(index, metrics.cols).saturating_sub(1)),
                },
                // The line is gone, or hidden: the one that took its place.
                Err(index) => Pos { index, row: 0 },
            };
            anchored.min(bottom)
        };
        Window {
            top,
            at_end: top == bottom,
        }
    }

    /// The anchor that keeps a view at `pos`.
    pub(super) fn anchored(&self, pos: Pos) -> Anchor {
        Anchor {
            seq: self
                .seq_at(pos.index)
                .unwrap_or_else(|| self.buffer.range().end),
            row: pos.row,
        }
    }

    /// How far the lines scroll sideways, in cells.
    pub(super) fn across_max(&self) -> usize {
        if self.wrap {
            return 0;
        }
        let labels = if self.sources.labels.is_empty() {
            0
        } else {
            self.sources.width + 3
        };
        (self.widest + labels).saturating_sub(self.metrics.cols)
    }

    /// The selection, with what it takes to read its text elsewhere.
    pub(super) fn copyable(&self) -> Option<Copyable> {
        let selection = self.selection.filter(|selection| !selection.is_empty())?;
        let (from, to) = selection.ends();
        let range = from.seq..to.seq.saturating_add(1);
        let only = self.filtered().then(|| {
            let first = self.search.position(range.start).unwrap_or_else(|at| at);
            let hits = self.search.hits.from(first);
            hits.map(|hit| hit.seq)
                .take_while(|seq| *seq < range.end)
                .collect()
        });
        Some(Copyable {
            snapshot: self.buffer.snapshot(range),
            only,
            selection,
            shape: self.shape(),
            labels: self.sources.labels.clone().into(),
        })
    }
}

/// A selection and the lines it is over, as they were when it was copied.
#[derive(Debug, Clone)]
pub struct Copyable {
    snapshot: Snapshot,
    /// The lines that show, when not all do.
    only: Option<Vec<u64>>,
    selection: Selection,
    shape: Shape,
    labels: Arc<[Arc<str>]>,
}

impl Copyable {
    /// The bytes of the lines it is over: what reading it costs.
    pub fn bytes(&self) -> usize {
        self.snapshot.bytes()
    }

    /// How many lines it reads.
    pub fn len(&self) -> usize {
        self.only.as_ref().map_or(self.snapshot.len(), Vec::len)
    }

    /// The selected text: each line as it shows, cut at the selection's
    /// ends, one under the other. A line wrapped onto several rows is one
    /// line.
    pub fn text(&self) -> String {
        let mut text = String::new();
        let mut shown = String::new();
        let mut add = |seq: u64, line: LineRef<'_>| {
            let (Some((from, to)), None) = (self.selection.cells_of(seq), line.marker) else {
                return;
            };
            shown.clear();
            let label = line
                .source
                .and_then(|id| self.labels.get(usize::from(id)))
                .map_or("", |label| &**label);
            self.shape.write(&line, label, &mut shown);
            let start = byte_at_cell(&shown, from);
            let end = if to == usize::MAX {
                shown.len()
            } else {
                byte_at_cell(&shown, to)
            };
            text.push_str(&shown[start..end.max(start)]);
            if to == usize::MAX {
                text.push('\n');
            }
        };
        match &self.only {
            Some(seqs) => {
                for seq in seqs {
                    if let Some(line) = self.snapshot.line(*seq) {
                        add(*seq, line);
                    }
                }
            }
            None => self.snapshot.lines().for_each(|(seq, line)| add(seq, line)),
        }
        text
    }
}
