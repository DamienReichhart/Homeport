//! What a viewer does with the lines it is given and with what the user
//! does to it. Nothing here waits: a search over more lines than a frame
//! can read, or a selection of more lines than a frame can copy, is a
//! task on another thread, whose answer comes back as a message.

use std::sync::Arc;

use iced::Task;

use super::{
    display::Pos,
    line::{split_stamp, Line, Marker},
    search::{self, Hit, Query, AT_ONCE, AT_ONCE_BYTES, SLICE},
    selection::{word_at, Mark, Selection},
    state::{Anchor, Download, Event, Message, Scroll, Sources, State},
};
use crate::ui::widgets::form;

/// A selection over up to this many lines, of up to this many bytes
/// together, is copied where it is.
const COPY_AT_ONCE: usize = 4_096;
const COPY_AT_ONCE_BYTES: usize = 1024 * 1024;

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
            // The timestamp is the stream's, not the program's: it is
            // taken off before the program's text is read, so nothing the
            // program writes (a carriage return) can reach it.
            let (stamp, message) = match split_stamp(&line.text).filter(|_| line.timestamped) {
                Some((stamp, message)) => (Some(stamp), message),
                None => (None, line.text.as_str()),
            };
            chunk.push(
                line.stream,
                source,
                stamp,
                |text, start, message_start, runs| {
                    parser.feed_message(message, text, start, message_start, runs);
                },
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
        // Nothing to find in it, but it shows while only what matches
        // does: a search that had read everything takes it as read.
        let end = self.buffer.range().end;
        if self.search.frontier + 1 == end && !self.search.busy {
            let shown = self.search.is_active().then_some(Hit {
                seq: end - 1,
                count: 0,
            });
            self.search.found(shown, end);
        }
    }

    /// Keeps the last `cap` lines from now on: the setting changed.
    pub fn set_cap(&mut self, cap: usize) {
        let dropped = self.buffer.set_cap(cap);
        self.dropped(dropped);
    }

    /// Drops every line, as the Clear button does. The stream goes on:
    /// its sources keep their colours and their place before the lines,
    /// but what a program left open (a colour, half a sequence) is
    /// forgotten with the lines it was in.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.sources.forget_styles();
        self.search.rescan(self.buffer.range().end);
        self.index.borrow_mut().clear();
        self.selection = None;
        self.follow = true;
        self.anchor = Anchor::default();
        self.across = 0;
        self.widest = 0;
        self.revision += 1;
    }

    /// Starts over for a stream that is opened again (other options,
    /// another container): no line, no source, nothing left of what the
    /// last stream wrote. What is searched and how the lines show stay.
    /// React reset its terminal the same way each time it opened one.
    pub fn reset(&mut self) {
        self.clear();
        self.sources = Sources::default();
    }

    /// The sources in the order their colours are given, for a stream
    /// that names them before its first line: [`State::with_sources`]
    /// after a [`State::reset`].
    pub fn set_sources<S: Into<Arc<str>>>(&mut self, labels: impl IntoIterator<Item = S>) {
        for label in labels {
            self.sources.id(&label.into());
        }
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
        self.index.borrow_mut().evict(first);
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
    /// they are few and short, else a slice of them on another thread.
    fn pump(&mut self) -> Task<Message> {
        let Some(matcher) = self.search.matcher.clone() else {
            return Task::none();
        };
        if self.search.busy || self.search.is_stopped() {
            return Task::none();
        }
        let range = self.buffer.range();
        let from = self.search.frontier.max(range.start);
        if from >= range.end {
            return Task::none();
        }
        let timestamps = self.timestamps;
        let few =
            range.end - from <= AT_ONCE && self.buffer.bytes(from..range.end) <= AT_ONCE_BYTES;
        if few {
            let found: Vec<Hit> = (from..range.end)
                .filter_map(|seq| {
                    let line = self.buffer.line(seq)?;
                    search::hit(&matcher, seq, &line, timestamps)
                })
                .collect();
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
                hits: hits.map(Arc::new),
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
                // Of another query, or of another viewer: not this one's.
                if generation != self.search.generation {
                    return quiet();
                }
                match hits {
                    Some(hits) => {
                        let first = self.buffer.range().start;
                        self.search.busy = false;
                        let kept = hits.iter().copied().filter(|hit| hit.seq >= first);
                        self.search.found(kept, to);
                    }
                    // The lines were not read: again, or the search
                    // says it stopped. The frontier stays before them.
                    None => {
                        if !self.search.lost() {
                            return quiet();
                        }
                    }
                }
                return (self.pump(), None);
            }
            Message::Compiled {
                generation,
                matcher,
            } => {
                if generation == self.search.generation && self.search.compiling {
                    self.search.compiled(matcher);
                    return (self.pump(), None);
                }
            }
            Message::DragPast { down, cell } => self.drag_past(down, cell),
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
        let first = self.buffer.range().start;
        if !self.search.restart(query, first) {
            return (self.pump(), None);
        }
        // A regular expression, or a long text: built on another thread.
        let generation = self.search.generation;
        let build = search::compile_elsewhere(self.search.query.clone());
        let task = Task::perform(build, move |matcher| Message::Compiled {
            generation,
            matcher,
        });
        (task, None)
    }

    /// The pointer is past the rows with the button held: the view goes
    /// a row that way, and the selection ends on the row that came in,
    /// in the same step.
    fn drag_past(&mut self, down: bool, cell: usize) {
        self.scroll(Scroll::Rows(if down { 1 } else { -1 }));
        let metrics = self.metrics;
        let top = self.window(metrics).top;
        let rows = i64::try_from(metrics.rows.saturating_sub(1)).unwrap_or(0);
        let at = if down {
            self.moved(top, rows, metrics.cols)
        } else {
            top
        };
        let last = self.count().saturating_sub(1);
        let at = Pos {
            index: at.index.min(last),
            ..at
        };
        let Some((seq, line)) = self
            .seq_at(at.index)
            .and_then(|seq| Some((seq, self.buffer.line(seq)?)))
        else {
            return;
        };
        let (from, to) = if self.wrap {
            self.starts(seq, &line, self.shape(), metrics.cols)
                .range(at.row)
        } else {
            (self.across, self.across + metrics.cols)
        };
        if let Some(selection) = &mut self.selection {
            selection.head = Mark::new(seq, (from + cell).min(to));
        }
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
            Scroll::To(share) if share >= 1.0 => {
                self.follow = true;
                return;
            }
            // Every line can be the first in view: the thumb's travel is
            // the lines down to where a view of the latest starts.
            Scroll::To(share) => Pos {
                index: (share.clamp(0.0, 1.0) * self.bottom(metrics).index as f32).round() as usize,
                row: 0,
            },
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
        if copyable.len() <= COPY_AT_ONCE && copyable.bytes() <= COPY_AT_ONCE_BYTES {
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
        let shape = self.shape();
        let start = self
            .matches_of(current.seq, &line)
            .get(current.nth as usize)
            .map_or(0, |(start, _)| *start as usize);
        let cell = self.shown_cell(current.seq, &line, shape, start);
        let row = if self.wrap {
            self.starts(current.seq, &line, shape, metrics.cols)
                .row_of(cell)
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
}
