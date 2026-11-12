//! What a viewer does with the lines it is given and with what the user
//! does to it. Nothing here waits: a search over more lines than a frame
//! can read, or a selection of more lines than a frame can copy, is a
//! task on another thread, whose answer comes back as a message.

use std::sync::Arc;

use iced::Task;

use super::{
    buffer::LineRef,
    display::Pos,
    line::{byte_at_cell, cells, Line, Marker},
    search::{self, Hit, Query, AT_ONCE, SLICE},
    selection::{word_at, Mark, Selection},
    state::{Anchor, Download, Event, Message, Scroll, State},
};
use crate::ui::widgets::form;

/// A selection over up to this many lines is copied where it is.
const COPY_AT_ONCE: usize = 4_096;

/// What [`State::update`] gives back: what to run, mapped to the page's
/// message, and what the page acts on.
pub type Outcome = (Task<Message>, Option<Event>);

fn quiet() -> Outcome {
    (Task::none(), None)
}

impl State {
    /// Appends lines, the oldest first, and drops the oldest kept past
    /// the cap. What it returns reads the new lines for the search when
    /// there are too many for now: map it to the page's message.
    pub fn extend(&mut self, lines: impl IntoIterator<Item = Line>) -> Task<Message> {
        let sources = &mut self.sources;
        let width = sources.width;
        let mut widest = self.widest;
        let dropped = self.buffer.append(lines, |chunk, line: Line| {
            let source = line.source.as_ref().map(|label| sources.id(label));
            let parser = sources.parser(source);
            chunk.push(
                line.stream,
                source,
                line.timestamped,
                |text, start, runs| parser.feed(&line.text, text, start, runs),
            );
            widest = widest.max(chunk.last_width());
        });
        self.widest = widest;
        if self.sources.width != width {
            // The labels are padded to the longest: every row moves.
            self.revision += 1;
        }
        self.dropped(dropped);
        self.pump()
    }

    /// Appends one line.
    pub fn push(&mut self, line: Line) -> Task<Message> {
        self.extend([line])
    }

    /// The source said it has no more: "— end of logs —".
    pub fn end(&mut self) {
        self.mark(Marker::Ended);
    }

    /// The window was behind and `count` lines were never shown
    /// (`Update::Batch::dropped`): the viewer says so where they would be.
    pub fn skipped(&mut self, count: u64) {
        if count > 0 {
            self.mark(Marker::Skipped(count));
        }
    }

    fn mark(&mut self, marker: Marker) {
        let dropped = self
            .buffer
            .append([marker], |chunk, marker| chunk.push_marker(marker));
        self.dropped(dropped);
        // Nothing to find in it: a search that had read everything still has.
        let end = self.buffer.range().end;
        if self.search.frontier + 1 == end {
            self.search.frontier = end;
        }
    }

    /// Keeps the last `cap` lines from now on: the setting changed.
    pub fn set_cap(&mut self, cap: usize) {
        let dropped = self.buffer.set_cap(cap);
        self.dropped(dropped);
    }

    /// Drops every line, as the Clear button does.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.search.rescan(self.buffer.range().end);
        self.selection = None;
        self.follow = true;
        self.anchor = Anchor::default();
        self.across = 0;
        self.widest = 0;
        self.revision += 1;
    }

    /// The buffer dropped its `count` oldest lines: what pointed at them
    /// lets go.
    fn dropped(&mut self, count: u64) {
        if count == 0 {
            return;
        }
        let first = self.buffer.range().start;
        self.search.evict(first);
        if !self.follow && self.anchor.seq < first {
            // What the view was on is gone: it stays on the oldest line.
            self.anchor = Anchor { seq: first, row: 0 };
        }
        self.selection = self.selection.and_then(|selection| {
            let (from, to) = selection.ends();
            if to.seq < first {
                return None;
            }
            let from = from.max(Mark::new(first, 0));
            Some(if selection.anchor <= selection.head {
                Selection::new(from, to)
            } else {
                Selection::new(to, from)
            })
        });
    }

    /// Reads for the search the lines it has not read yet: at once when
    /// they are few, else a slice of them on another thread.
    fn pump(&mut self) -> Task<Message> {
        let Some(matcher) = self.search.matcher.clone() else {
            return Task::none();
        };
        if self.search.busy {
            return Task::none();
        }
        let range = self.buffer.range();
        let from = self.search.frontier.max(range.start);
        if from >= range.end {
            return Task::none();
        }
        let timestamps = self.timestamps;
        if range.end - from <= AT_ONCE {
            let found = (from..range.end).filter_map(|seq| {
                let line = self.buffer.line(seq)?;
                let count = search::matches(&matcher, &line, timestamps).count();
                // A line holds fewer matches than it has bytes.
                #[allow(clippy::cast_possible_truncation)]
                (count > 0).then_some(Hit {
                    seq,
                    count: count as u32,
                })
            });
            let found: Vec<Hit> = found.collect();
            self.search.found(found, range.end);
            return Task::none();
        }
        let to = (from + SLICE).min(range.end);
        let snapshot = self.buffer.snapshot(from..to);
        let generation = self.search.generation;
        self.search.busy = true;
        Task::perform(
            search::scan_elsewhere(snapshot, matcher, timestamps),
            move |hits| Message::Scanned {
                generation,
                to,
                hits: Arc::new(hits),
            },
        )
    }

    /// Takes a message of the viewer.
    pub fn update(&mut self, message: Message) -> Outcome {
        match message {
            Message::Viewport(metrics) => {
                self.metrics = metrics;
                self.across = self.across.min(self.across_max());
            }
            Message::Scroll(scroll) => self.scroll(scroll),
            Message::Press { at, extend } => {
                self.selection = Some(match self.selection.filter(|_| extend) {
                    Some(selection) => Selection::new(selection.anchor, at),
                    None => Selection::new(at, at),
                });
            }
            Message::Drag(to) => {
                if let Some(selection) = &mut self.selection {
                    selection.head = to;
                }
            }
            Message::SelectWord(at) => {
                if let Some(text) = self.shown(at.seq) {
                    let (from, to) = word_at(&text, at.cell);
                    self.selection = Some(Selection::new(
                        Mark::new(at.seq, from),
                        Mark::new(at.seq, to),
                    ));
                }
            }
            Message::SelectLine(seq) => {
                self.selection = Some(Selection::new(
                    Mark::new(seq, 0),
                    Mark::new(seq, usize::MAX),
                ));
            }
            Message::SelectAll => {
                let last = self.count().checked_sub(1);
                self.selection = self
                    .seq_at(0)
                    .zip(last.and_then(|last| self.seq_at(last)))
                    .map(|(first, last)| {
                        Selection::new(Mark::new(first, 0), Mark::new(last, usize::MAX))
                    });
            }
            Message::Deselect => self.selection = None,
            Message::Copy => return self.copy(),
            Message::Copied(text) => {
                return (Task::none(), Some(Event::Copy(Arc::unwrap_or_clone(text))));
            }
            Message::Wrap(wrap) => {
                // The line at the top stays there.
                let top = self.window(self.metrics).top;
                self.anchor = self.anchored(Pos { row: 0, ..top });
                self.wrap = wrap;
                self.across = 0;
                self.revision += 1;
            }
            Message::Timestamps(timestamps) => {
                self.timestamps = timestamps;
                self.revision += 1;
                // The timestamps are searched only while they show.
                self.search.rescan(self.buffer.range().start);
                return (self.pump(), Some(Event::Timestamps(timestamps)));
            }
            Message::Follow(follow) => {
                if !follow {
                    self.anchor = self.anchored(self.window(self.metrics).top);
                }
                self.follow = follow;
            }
            Message::Query(text) => {
                return self.look_for(Query {
                    text,
                    ..self.search.query.clone()
                });
            }
            Message::Regex(regex) => {
                return self.look_for(Query {
                    regex,
                    ..self.search.query.clone()
                });
            }
            Message::Case(case) => {
                return self.look_for(Query {
                    case,
                    ..self.search.query.clone()
                });
            }
            Message::Filter(filter) => {
                // The view stays on its line, or on the next one that shows.
                self.anchor = self.anchored(self.window(self.metrics).top);
                self.filter = filter;
            }
            Message::Next => self.step(false),
            Message::Previous => self.step(true),
            Message::Scanned {
                generation,
                to,
                hits,
            } => {
                if generation == self.search.generation {
                    let first = self.buffer.range().start;
                    self.search.busy = false;
                    let kept = hits.iter().copied().filter(|hit| hit.seq >= first);
                    self.search.found(kept, to);
                    return (self.pump(), None);
                }
            }
            Message::Clear => {
                self.clear();
                return (Task::none(), Some(Event::Cleared));
            }
            Message::Download => {
                let download = Download {
                    snapshot: self.buffer.snapshot(self.buffer.range()),
                    labels: self.sources.labels.clone().into(),
                    timestamps: self.timestamps,
                };
                return (Task::none(), Some(Event::Download(download)));
            }
            Message::FocusSearch => return (form::focus_search(self.search_id()), None),
        }
        quiet()
    }

    fn look_for(&mut self, query: Query) -> Outcome {
        if self.filtered() {
            self.anchor = self.anchored(self.window(self.metrics).top);
        }
        self.search.restart(query, self.buffer.range().start);
        (self.pump(), None)
    }

    fn scroll(&mut self, scroll: Scroll) {
        let metrics = self.metrics;
        let top = self.window(metrics).top;
        // Lines and rows are counted well within what an `f32` and an
        // `i64` hold.
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_possible_wrap
        )]
        let target = match scroll {
            Scroll::Rows(rows) => self.moved(top, rows, metrics.cols),
            Scroll::Pages(pages) => {
                let page = metrics.rows.saturating_sub(1).max(1) as i64;
                self.moved(top, pages.saturating_mul(page), metrics.cols)
            }
            Scroll::Start => Pos::default(),
            Scroll::End => {
                self.follow = true;
                return;
            }
            Scroll::To(share) if share >= 0.999 => {
                self.follow = true;
                return;
            }
            Scroll::To(share) => {
                let most = self.count().saturating_sub(metrics.rows);
                Pos {
                    index: (share.clamp(0.0, 1.0) * most as f32).round() as usize,
                    row: 0,
                }
            }
            Scroll::Across(by) => {
                let by_cells = usize::try_from(by.unsigned_abs()).unwrap_or(usize::MAX);
                self.across = if by < 0 {
                    self.across.saturating_sub(by_cells)
                } else {
                    self.across.saturating_add(by_cells).min(self.across_max())
                };
                return;
            }
            Scroll::AcrossTo(share) => {
                let most = self.across_max();
                self.across = (share.clamp(0.0, 1.0) * most as f32).round() as usize;
                return;
            }
        };
        // Back on the latest line, the view follows again.
        self.follow = target >= self.bottom(metrics);
        if !self.follow {
            self.anchor = self.anchored(target);
        }
    }

    /// What the line `seq` shows, as text.
    fn shown(&self, seq: u64) -> Option<String> {
        let line = self.buffer.line(seq)?;
        let mut text = String::new();
        let label = line.source.map_or("", |id| self.sources.label(id));
        self.shape().write(&line, label, &mut text);
        Some(text)
    }

    fn copy(&mut self) -> Outcome {
        let Some(copyable) = self.copyable() else {
            return quiet();
        };
        if copyable.len() <= COPY_AT_ONCE {
            return (Task::none(), Some(Event::Copy(copyable.text())));
        }
        let read = async move {
            match tokio::runtime::Handle::try_current() {
                Ok(runtime) => {
                    let work = runtime.spawn_blocking(move || copyable.text());
                    work.await.unwrap_or_default()
                }
                Err(_) => copyable.text(),
            }
        };
        let task = Task::perform(read, |text| Message::Copied(Arc::new(text)));
        (task, None)
    }

    /// Goes to the next match (the one before, `back`) and shows it.
    fn step(&mut self, back: bool) {
        let metrics = self.metrics;
        let top = self.window(metrics).top;
        // From what shows: the first line going on, the last going back.
        let from = if back {
            let last = (top.index + metrics.rows)
                .min(self.count())
                .saturating_sub(1);
            self.seq_at(last)
        } else {
            self.seq_at(top.index)
        };
        let from = from.unwrap_or_else(|| self.buffer.range().start);
        let Some(current) = self.search.step(back, from) else {
            return;
        };
        let (Ok(index), Some(line)) = (self.index_of(current.seq), self.buffer.line(current.seq))
        else {
            return;
        };
        let cell = self.cell_of_match(&line, current.nth as usize);
        let row = if self.wrap {
            cell / metrics.cols.max(1)
        } else {
            0
        };
        let at = Pos { index, row };
        // Already in view: nothing moves.
        let below = self.moved(
            top,
            i64::try_from(metrics.rows).unwrap_or(i64::MAX),
            metrics.cols,
        );
        if at < top || at >= below {
            let third = i64::try_from(metrics.rows / 3).unwrap_or(0);
            let target = self.moved(at, -third, metrics.cols);
            self.follow = false;
            self.anchor = self.anchored(target.min(self.bottom(metrics)));
        }
        if !self.wrap && (cell < self.across || cell >= self.across + metrics.cols) {
            self.across = cell.saturating_sub(metrics.cols / 4).min(self.across_max());
        }
    }

    /// The cell at which the `nth` match of a line starts, among what
    /// the line shows.
    fn cell_of_match(&self, line: &LineRef<'_>, nth: usize) -> usize {
        let Some(matcher) = &self.search.matcher else {
            return 0;
        };
        let start = search::matches(matcher, line, self.timestamps)
            .nth(nth)
            .map_or(0, |(start, _)| start);
        self.shape().cell_of_byte(line, start)
    }
}

impl super::display::Shape {
    /// The cell, among what the line shows, at which the byte `byte` of
    /// its text is.
    pub fn cell_of_byte(&self, line: &LineRef<'_>, byte: usize) -> usize {
        let message = line.message_start();
        if byte < message {
            // In the timestamp, which is ASCII and shows from the start.
            return byte;
        }
        self.prefix(line) + cells(&line.text[message..byte.min(line.text.len())])
    }

    /// The byte of the line's text at which the cell `cell` of its
    /// message starts.
    pub fn byte_of_message_cell(line: &LineRef<'_>, cell: usize) -> usize {
        line.message_start() + byte_at_cell(line.message(), cell)
    }
}
