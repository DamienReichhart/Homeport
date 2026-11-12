//! What a log viewer remembers from one view to the next: its lines, how
//! they are shown, where the view is among them, what is searched and
//! what is selected. The page keeps one per viewer.

use std::{collections::HashMap, sync::Arc};

use iced::advanced::widget::Id;

use super::{
    ansi::Parser,
    buffer::{Buffer, Snapshot},
    search::{Hit, Query, Search},
    selection::{Mark, Selection},
};

/// How many lines a viewer keeps unless told: what the React viewer kept
/// for a download.
pub const DEFAULT_CAP: usize = 100_000;

/// A line of the view and one of its rows on screen: where the view
/// starts while it does not follow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Anchor {
    pub seq: u64,
    /// Which row of the line, when it is wrapped onto several.
    pub row: usize,
}

/// How much of the lines the viewer shows, as it was last laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metrics {
    /// The cells of a row of text.
    pub cols: usize,
    /// The rows that show whole.
    pub rows: usize,
}

impl Default for Metrics {
    fn default() -> Self {
        // Before the first layout: a terminal's.
        Self { cols: 80, rows: 24 }
    }
}

/// Where the view goes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scroll {
    /// This many rows down (up when negative).
    Rows(i64),
    /// This many screens down (up when negative).
    Pages(i64),
    /// To the oldest line.
    Start,
    /// To the latest line, which is followed again.
    End,
    /// The scrollbar's thumb was dragged this far, from 0 to 1.
    To(f32),
    /// This many cells towards the end of the lines (the start when
    /// negative): the lines are not wrapped.
    Across(i64),
    /// The thumb under the lines was dragged this far, from 0 to 1.
    AcrossTo(f32),
}

/// What the user did to a viewer, and what its tasks found. The page
/// hands each to [`State::update`].
#[derive(Debug, Clone)]
pub enum Message {
    /// The viewer was laid out with this much room.
    Viewport(Metrics),
    Scroll(Scroll),
    /// A press on the text (`extend`: with Shift, from where the
    /// selection started).
    Press {
        at: Mark,
        extend: bool,
    },
    /// The pointer moved with the button held.
    Drag(Mark),
    /// Two presses on a word, three on a line.
    SelectWord(Mark),
    SelectLine(u64),
    SelectAll,
    Deselect,
    /// Ctrl or Cmd + C, or the Copy button.
    Copy,
    /// The selection's text, built on another thread.
    Copied(Arc<String>),
    Wrap(bool),
    Timestamps(bool),
    Follow(bool),
    Query(String),
    Regex(bool),
    Case(bool),
    /// Only the lines that match show.
    Filter(bool),
    Next,
    Previous,
    /// What a task found in the lines up to `to`.
    Scanned {
        generation: u64,
        to: u64,
        hits: Arc<Vec<Hit>>,
    },
    Clear,
    Download,
    /// Ctrl or Cmd + F in the viewer.
    FocusSearch,
}

/// What a message led to that the page acts on.
#[derive(Debug, Clone)]
pub enum Event {
    /// Text for the clipboard (`ctx.copy`).
    Copy(String),
    /// The lines to save to a file the user picks.
    Download(Download),
    /// The Timestamps toggle: a page whose stream is asked for timestamps
    /// only while they show opens it again.
    Timestamps(bool),
    /// The lines were cleared.
    Cleared,
}

/// The sources of the lines, in the order they were first seen: each has
/// its colour and its own terminal state.
#[derive(Debug, Default)]
pub(super) struct Sources {
    pub labels: Vec<Arc<str>>,
    by_label: HashMap<Arc<str>, u16>,
    parsers: Vec<Parser>,
    /// The cells of the longest label.
    pub width: usize,
    /// For the lines of no source.
    plain: Parser,
}

impl Sources {
    /// The number of the source `label`, new or known.
    pub fn id(&mut self, label: &Arc<str>) -> u16 {
        if let Some(id) = self.by_label.get(label) {
            return *id;
        }
        // More sources than a number can tell apart share the last one.
        let Ok(id) = u16::try_from(self.labels.len()) else {
            return u16::MAX - 1;
        };
        if id >= u16::MAX - 1 {
            return u16::MAX - 1;
        }
        self.labels.push(label.clone());
        self.parsers.push(Parser::default());
        self.by_label.insert(label.clone(), id);
        self.width = self.width.max(super::line::cells(label));
        id
    }

    pub fn parser(&mut self, source: Option<u16>) -> &mut Parser {
        match source.and_then(|id| self.parsers.get_mut(usize::from(id))) {
            Some(parser) => parser,
            None => &mut self.plain,
        }
    }

    pub fn label(&self, source: u16) -> &str {
        self.labels.get(usize::from(source)).map_or("", |l| &**l)
    }
}

/// A log viewer's state.
#[derive(Debug)]
pub struct State {
    pub(super) buffer: Buffer,
    pub(super) sources: Sources,
    pub(super) search: Search,
    pub(super) filter: bool,
    pub(super) wrap: bool,
    pub(super) timestamps: bool,
    pub(super) follow: bool,
    pub(super) anchor: Anchor,
    /// How many cells the lines are scrolled sideways.
    pub(super) across: usize,
    /// The cells of the widest line shown so far.
    pub(super) widest: usize,
    pub(super) selection: Option<Selection>,
    pub(super) metrics: Metrics,
    /// Changes when what a row shows does: the timestamps, the wrap, the
    /// width of the labels, the lines themselves when cleared.
    pub(super) revision: u64,
    pub(super) search_id: Id,
}

impl Default for State {
    fn default() -> Self {
        Self::new(DEFAULT_CAP)
    }
}

impl State {
    /// A viewer that keeps the last `cap` lines: the setting upstream.
    pub fn new(cap: usize) -> Self {
        Self {
            buffer: Buffer::new(cap),
            sources: Sources::default(),
            search: Search::default(),
            filter: false,
            wrap: true,
            timestamps: false,
            follow: true,
            anchor: Anchor::default(),
            across: 0,
            widest: 0,
            selection: None,
            metrics: Metrics::default(),
            revision: 0,
            search_id: Id::unique(),
        }
    }

    /// Lines longer than the viewer is wide go on the next row (they do
    /// unless told), or scroll sideways.
    #[must_use]
    pub fn wrapping(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Shows the time each line starts with, for lines that have one.
    #[must_use]
    pub fn with_timestamps(mut self, timestamps: bool) -> Self {
        self.timestamps = timestamps;
        self
    }

    /// The sources in the order their colours are given: those known
    /// before any line came (the containers of a Stack, in its order).
    #[must_use]
    pub fn with_sources<S: Into<Arc<str>>>(mut self, labels: impl IntoIterator<Item = S>) -> Self {
        for label in labels {
            self.sources.id(&label.into());
        }
        self
    }

    /// How many lines are kept.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn cap(&self) -> usize {
        self.buffer.cap()
    }

    pub fn wraps(&self) -> bool {
        self.wrap
    }

    pub fn shows_timestamps(&self) -> bool {
        self.timestamps
    }

    /// Whether the view stays on the latest line as lines come.
    pub fn follows(&self) -> bool {
        self.follow
    }

    /// Where the view starts while it does not follow.
    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    pub fn query(&self) -> &Query {
        &self.search.query
    }

    /// Whether only the lines that match show.
    pub fn filters(&self) -> bool {
        self.filter
    }

    /// Why the query can't be read: a regular expression that isn't one,
    /// in the words of the `regex` crate.
    pub fn search_error(&self) -> Option<&str> {
        self.search.error.as_deref()
    }

    /// How many matches were found so far, and which one the search
    /// stands on (from 1).
    pub fn matches(&self) -> (Option<u64>, u64) {
        (
            self.search.current.map(|current| current.ordinal),
            self.search.total,
        )
    }

    /// Whether lines are still being read for the query.
    pub fn is_searching(&self) -> bool {
        self.search.is_active() && self.search.frontier < self.buffer.range().end
    }

    /// Whether some text is selected.
    pub fn has_selection(&self) -> bool {
        self.selection
            .is_some_and(|selection| !selection.is_empty())
    }

    /// The search field, for `/` to give focus to (`form::focus_search`).
    pub fn search_id(&self) -> Id {
        self.search_id.clone()
    }

    /// The labels of the sources, in the order they were first seen.
    pub fn sources(&self) -> &[Arc<str>] {
        &self.sources.labels
    }
}

/// The lines of a viewer as they were when the user asked to save them:
/// what a page hands to the file dialog and writes on another thread.
#[derive(Debug, Clone)]
pub struct Download {
    pub(super) snapshot: Snapshot,
    pub(super) labels: Arc<[Arc<str>]>,
    pub(super) timestamps: bool,
}

impl Download {
    /// How many lines there are to save.
    pub fn len(&self) -> usize {
        self.lines().count()
    }

    pub fn is_empty(&self) -> bool {
        self.lines().next().is_none()
    }

    /// The lines as the React viewer saved them: the text without its
    /// colours, after `source | ` when there are several sources, with
    /// its timestamp while timestamps show.
    pub fn lines(&self) -> impl Iterator<Item = String> + '_ {
        self.snapshot
            .lines()
            .filter(|(_, line)| line.marker.is_none())
            .map(|(_, line)| {
                let text = if self.timestamps {
                    line.text
                } else {
                    line.message()
                };
                match line.source.and_then(|id| self.labels.get(usize::from(id))) {
                    Some(label) => format!("{label} | {text}"),
                    None => text.to_owned(),
                }
            })
    }

    /// Writes the lines, each ended by a line feed.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        for line in self.lines() {
            out.write_all(line.as_bytes())?;
            out.write_all(b"\n")?;
        }
        out.flush()
    }
}
