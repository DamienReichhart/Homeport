//! The rows in view, built from the lines: which cells of which line each
//! shows, shaped run by run. A row that was in the last layout and shows
//! the same cells is taken from it as it is; only what scrolled into view
//! is shaped.

use std::collections::HashMap;

use super::{shaped, Geometry, Raw};
use crate::{
    t,
    ui::widgets::log::{
        ansi::{Attrs, Style},
        buffer::LineRef,
        display::{Pos, Shape},
        index::{walked, Starts},
        line::{byte_at_cell, cells, Marker, Stream},
        state::{Metrics, State},
    },
};

/// What a piece of a row is drawn as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paint {
    /// Text of the line, in the style its program gave it.
    Text(Style),
    /// The time the line was written: dimmed.
    Stamp,
    /// The label of the source of this number, in its colour.
    Label(u16),
    /// Something the viewer says: dimmed.
    Marker,
}

/// Text of one style, at a cell of its row.
pub struct Piece {
    /// The cell of the row it starts at, and how many it takes.
    pub col: usize,
    pub cells: usize,
    pub paragraph: Raw,
    pub paint: Paint,
}

/// A row in view: some cells of a line.
pub struct Row {
    pub seq: u64,
    /// Which row of its line, and whether it is the last.
    pub part: usize,
    pub last: bool,
    /// The cells of what the line shows that are on this row.
    pub from: usize,
    pub to: usize,
    /// The state's revision the pieces were built at.
    revision: u64,
    pub pieces: Vec<Piece>,
    /// The matches of the search on it, as cells of the row, and whether
    /// each is the one the search stands on.
    pub hits: Vec<(usize, usize, bool)>,
    pub stream: Stream,
    pub marker: bool,
}

/// What a layout came to.
#[derive(Default)]
pub struct Laid {
    pub rows: Vec<Row>,
    pub top: Pos,
    pub at_end: bool,
    /// How many lines show, in view or not, and how many are in view.
    pub count: usize,
    pub lines: usize,
    pub metrics: Metrics,
}

/// What the viewer says for `marker`, in the app's language.
pub fn marker_text(marker: Marker) -> String {
    match marker {
        Marker::Ended => t!("containers.logs.end"),
        Marker::Skipped(count) => t!("ui.log.skipped", count = count),
    }
}

/// `label` padded to `width` cells, and the bar after it.
fn label_text(label: &str, width: usize) -> String {
    let mut text = String::with_capacity(width + 2);
    text.push_str(label);
    text.extend(std::iter::repeat_n(' ', width.saturating_sub(cells(label))));
    text.push_str(" |");
    text
}

/// The part of the cells `from..to` that is in `start..end`.
fn within(from: usize, to: usize, start: usize, end: usize) -> Option<(usize, usize)> {
    let (a, b) = (from.max(start), to.min(end));
    (a < b).then_some((a, b))
}

/// Some text of one look, at a cell of its row.
struct Segment {
    col: usize,
    cells: usize,
    text: String,
    paint: Paint,
}

/// `slice` cut where ASCII gives way to another script and back: each
/// piece is then placed at its own cell, so text after a character the
/// monospaced face lacks starts where its cell says, whatever a
/// fallback font made of that character's width.
fn by_script(slice: &str) -> impl Iterator<Item = &str> {
    let mut rest = slice;
    std::iter::from_fn(move || {
        let first = rest.chars().next()?;
        let ascii = first.is_ascii();
        let end = rest
            .char_indices()
            .find(|(_, c)| c.is_ascii() != ascii)
            .map_or(rest.len(), |(at, _)| at);
        let (piece, after) = rest.split_at(end);
        rest = after;
        Some(piece)
    })
}

/// What the cells `from..to` of the line `seq` show, look by look.
fn segments(
    state: &State,
    seq: u64,
    line: &LineRef<'_>,
    shape: Shape,
    from: usize,
    to: usize,
) -> Vec<Segment> {
    let mut all = vec![];
    if let Some(marker) = line.marker {
        let text = marker_text(marker);
        all.push(Segment {
            col: 0,
            cells: cells(&text),
            text,
            paint: Paint::Marker,
        });
        return all;
    }
    let stamp = shape.stamp_cells(line);
    let label = shape.label_cells(line);
    if let (Some(time), Some((a, b))) = (
        line.timestamp().filter(|_| stamp > 0),
        within(from, to, 0, stamp.saturating_sub(1)),
    ) {
        all.push(Segment {
            col: a - from,
            cells: b - a,
            text: time[a..b].to_owned(),
            paint: Paint::Stamp,
        });
    }
    if let (Some(source), Some((a, b))) = (
        line.source,
        within(from, to, stamp, (stamp + label).saturating_sub(1)),
    ) {
        let text = label_text(state.sources.label(source), shape.label_width);
        let (start, end) = (
            byte_at_cell(&text, a - stamp),
            byte_at_cell(&text, b - stamp),
        );
        all.push(Segment {
            col: a - from,
            cells: b - a,
            text: text[start..end].to_owned(),
            paint: Paint::Label(source),
        });
    }
    let prefix = stamp + label;
    let Some((a, b)) = within(from, to, prefix, prefix + line.cells) else {
        return all;
    };
    let message = line.message_start();
    let (first, first_cell) = state.byte_of_cell(seq, line, a - prefix);
    let (end, _) = state.byte_of_cell(seq, line, b - prefix);
    let (first, end) = (message + first, message + end);
    let mut col = prefix + first_cell - from.min(prefix + first_cell);
    let mut emit = |start: usize, end: usize, style: Style| {
        for piece in by_script(&line.text[start..end]) {
            let taken = cells(piece);
            all.push(Segment {
                col,
                cells: taken,
                text: piece.to_owned(),
                paint: Paint::Text(style),
            });
            col += taken;
        }
    };
    let mut done = first;
    let skipped = line.runs.partition_point(|run| run.end as usize <= first);
    for run in &line.runs[skipped..] {
        let (start, stop) = ((run.start as usize).max(first), (run.end as usize).min(end));
        if start >= stop {
            break;
        }
        if start > done {
            emit(done, start, Style::default());
        }
        emit(start, stop, run.style);
        done = stop;
    }
    if done < end {
        emit(done, end, Style::default());
    }
    all
}

/// The pieces of the cells `from..to` of what the line `seq` shows.
fn pieces(
    state: &State,
    seq: u64,
    line: &LineRef<'_>,
    shape: Shape,
    from: usize,
    to: usize,
) -> Vec<Piece> {
    segments(state, seq, line, shape, from, to)
        .into_iter()
        .map(|segment| {
            let italic =
                matches!(segment.paint, Paint::Text(style) if style.attrs.has(Attrs::ITALIC));
            Piece {
                col: segment.col,
                cells: segment.cells,
                paragraph: shaped(&segment.text, italic),
                paint: segment.paint,
            }
        })
        .collect()
}

/// The matches of the search between the cells `low` and `high` of what
/// the line `seq` shows, as cells. Only what is between them is walked,
/// once: the matches come in order.
fn hits(
    state: &State,
    seq: u64,
    line: &LineRef<'_>,
    shape: Shape,
    low: usize,
    high: usize,
) -> Vec<(usize, usize, bool)> {
    let all = state.matches_of(seq, line);
    if all.is_empty() {
        return vec![];
    }
    let current = state
        .search
        .current
        .filter(|current| current.seq == seq)
        .map(|current| current.nth as usize);
    let prefix = shape.prefix(line);
    let message = line.message_start();
    // The bytes of the line's text that are in view.
    let (low_byte, low_cell) = if low < prefix {
        (0, 0)
    } else {
        let (byte, cell) = state.byte_of_cell(seq, line, low - prefix);
        (message + byte, prefix + cell)
    };
    let high_byte = message + state.byte_of_cell(seq, line, high.saturating_sub(prefix)).0;
    let high_byte = if high <= prefix { high } else { high_byte };
    // Where the walk is: a byte of the message and the cell it shows at.
    let mut cursor = (low_byte.max(message), low_cell.max(prefix));
    let mut cell_of = |byte: usize| {
        if byte < message {
            return byte;
        }
        if line.ascii {
            return prefix + byte - message;
        }
        if byte > cursor.0 {
            walked(byte - cursor.0);
            cursor.1 += cells(&line.text[cursor.0..byte]);
            cursor.0 = byte;
        }
        cursor.1
    };
    let first = all.partition_point(|(_, end)| *end as usize <= low_byte);
    let mut found = vec![];
    for (nth, (start, end)) in all.iter().enumerate().skip(first) {
        let (start, end) = (*start as usize, *end as usize);
        if start >= high_byte {
            break;
        }
        let from = cell_of(start);
        let to = cell_of(end.min(high_byte.max(start)));
        found.push((from, to.max(from), current == Some(nth)));
    }
    found
}

/// Lays out what a viewer of this geometry shows, taking from `old` the
/// rows that are still the same.
pub fn lay(state: &State, old: Laid, geometry: &Geometry) -> Laid {
    let metrics = geometry.metrics;
    let window = state.window(metrics);
    let count = state.count();
    let shape = state.shape();
    let mut kept: HashMap<(u64, usize), Row> = old
        .rows
        .into_iter()
        .map(|row| ((row.seq, row.part), row))
        .collect();
    let mut rows = Vec::with_capacity(metrics.rows);
    let mut lines = 0;
    let mut pos = window.top;
    while rows.len() < metrics.rows && pos.index < count {
        let Some((seq, line)) = state
            .seq_at(pos.index)
            .and_then(|seq| Some((seq, state.buffer.line(seq)?)))
        else {
            break;
        };
        let total = shape.cells(&line);
        // The cells a row of the line shows: a screen's worth of a
        // wrapped line, or what is scrolled into view of a long one.
        let starts = state
            .wrap
            .then(|| state.starts(seq, &line, shape, metrics.cols));
        let parts = starts.as_ref().map_or(1, Starts::len);
        let range = |part: usize| match &starts {
            Some(starts) => starts.range(part),
            None => {
                let from = state.across.min(total);
                (from, (from + metrics.cols + 1).min(total))
            }
        };
        let first = pos.row.min(parts - 1);
        let end = (first + metrics.rows - rows.len()).min(parts);
        let found = hits(state, seq, &line, shape, range(first).0, range(end - 1).1);
        for part in first..end {
            let (from, to) = range(part);
            let reused = kept.remove(&(seq, part)).filter(|row| {
                row.from == from && row.to == to && row.revision == state.revision && !row.marker
            });
            let mut row = reused.unwrap_or_else(|| Row {
                seq,
                part,
                last: part + 1 == parts,
                from,
                to,
                revision: state.revision,
                pieces: pieces(state, seq, &line, shape, from, to),
                hits: vec![],
                stream: line.stream,
                marker: line.marker.is_some(),
            });
            row.hits = found
                .iter()
                .filter_map(|(start, stop, current)| {
                    let (a, b) = within(*start, *stop, from, to.max(from + 1))?;
                    Some((a - from, b - from, *current))
                })
                .collect();
            rows.push(row);
        }
        lines += 1;
        pos = Pos {
            index: pos.index + 1,
            row: 0,
        };
    }
    Laid {
        rows,
        top: window.top,
        at_end: window.at_end,
        count,
        lines,
        metrics,
    }
}

/// What `row` says, as text: its pieces, each at its cell.
pub fn say(state: &State, row: &Row, out: &mut String) {
    let Some(line) = state.buffer.line(row.seq) else {
        return;
    };
    let mut at = 0;
    for segment in segments(state, row.seq, &line, state.shape(), row.from, row.to) {
        out.extend(std::iter::repeat_n(' ', segment.col.saturating_sub(at)));
        out.push_str(&segment.text);
        at = segment.col + segment.cells;
    }
}
