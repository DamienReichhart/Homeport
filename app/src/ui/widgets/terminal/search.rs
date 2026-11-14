//! Searching what the terminal holds, its scrollback and its screen:
//! `@xterm/addon-search`'s next and previous, with the log viewer's case
//! and regular expression toggles and its count of matches.
//!
//! The query is read as the log viewer reads it ([`Query::compile`] says
//! why a regular expression can't be), then given to
//! `alacritty_terminal`'s own search, which finds matches across the
//! rows a long line wrapped onto. As in the log viewer, plain text of up
//! to 256 bytes is made into its matcher where it is typed; anything else
//! is built on a thread that may block ([`compile`]).
//!
//! Every match is listed, up to [`MAX_MATCHES`], while the bar is open:
//! the list is made again when the query changes, and when output came,
//! since new lines move the old ones up. A list is made at most every
//! [`RESCAN_EVERY`] while output pours in. Until the scrollback is full a
//! row only moves up by the lines that came under it, so the match the
//! search stood on is followed exactly. Once it is full,
//! `alacritty_terminal` keeps no count of the lines it let go of: the
//! match is found again by its place in its row and what its row says,
//! the nearest at or above where it was (among rows that all say the
//! same, the one now where it stood). As only the new lines can't be
//! read either, the list is made whole each time.
//! Closing the bar lets go of the matcher and the matches; the query stays.

use std::time::{Duration, Instant};

use alacritty_terminal::{
    grid::Dimensions,
    index::{Column, Direction, Line, Point},
    term::{
        cell::Flags,
        search::{Match, RegexIter, RegexSearch},
    },
    Term,
};
use homeport_core::error::AppError;
use iced::advanced::widget::Id;

use crate::ui::widgets::log::search::{invalid, Query};

/// The most matches counted: past it the count says so ("10,000+").
pub const MAX_MATCHES: usize = 10_000;
/// The least time between two lists of the matches while output comes.
pub const RESCAN_EVERY: Duration = Duration::from_millis(250);

/// A query's matcher, or why it can't be made (the `regex` crate's words).
pub type Compiled = Result<RegexSearch, String>;

/// The pattern `alacritty_terminal` is given for `query`: text is
/// escaped, and capitals matter only when asked (its own rule, capitals
/// in the query, is not the toggle's).
fn pattern(query: &Query) -> String {
    let body = if query.regex {
        query.text.clone()
    } else {
        regex::escape(&query.text)
    };
    let flag = if query.case { "(?-i)" } else { "(?i)" };
    format!("{flag}{body}")
}

/// Builds `query`'s matcher: checked by the `regex` crate (within the log
/// viewer's limits), then built by `alacritty_terminal`.
fn build(query: &Query) -> Compiled {
    query.compile()?;
    RegexSearch::new(&pattern(query)).map_err(|error| error.to_string())
}

/// [`build`] on a thread that may block, where the runtime has one.
pub async fn compile(query: Query) -> Compiled {
    let built = move || build(&query);
    match tokio::runtime::Handle::try_current() {
        Ok(runtime) => runtime
            .spawn_blocking(built)
            .await
            .unwrap_or_else(|error| Err(error.to_string())),
        Err(_) => built(),
    }
}

/// Where the current match was: its place, what its row said, and how
/// many lines the scrollback held then.
#[derive(Debug, Clone)]
struct Anchor {
    start: Point,
    row: String,
    history: usize,
}

/// What `term`'s row `line` says.
fn row_text<T>(term: &Term<T>, line: Line) -> String {
    let row = &term.grid()[line];
    (0..term.columns())
        .map(|col| &row[Column(col)])
        .filter(|cell| !cell.flags.contains(Flags::WIDE_CHAR_SPACER))
        .map(|cell| cell.c)
        .collect()
}

/// What is searched for and what was found.
#[derive(Debug)]
pub struct Search {
    /// The search bar shows.
    pub open: bool,
    pub query: Query,
    compiled: Option<RegexSearch>,
    pub error: Option<AppError>,
    /// Every match, oldest first, in the grid's lines (the screen's top
    /// is line 0, the scrollback above it negative).
    pub matches: Vec<Match>,
    /// The match the search stands on.
    pub current: Option<usize>,
    anchor: Option<Anchor>,
    /// Which query a matcher being built is for.
    pub(super) generation: u64,
    /// When the matches were last listed, and whether output came since.
    scanned: Option<Instant>,
    stale: bool,
    /// How many lines the scrollback keeps.
    pub(super) scrollback: usize,
    pub id: Id,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            open: false,
            query: Query::default(),
            compiled: None,
            error: None,
            matches: vec![],
            current: None,
            anchor: None,
            generation: 0,
            scanned: None,
            stale: false,
            scrollback: super::DEFAULT_SCROLLBACK,
            id: Id::unique(),
        }
    }
}

impl Search {
    /// Takes `query`: its matcher is made at once when it is quick, and
    /// `Some(generation)` otherwise, for [`compile`] to make it elsewhere
    /// and [`Search::compiled`] to take it.
    pub fn set(&mut self, query: Query) -> Option<u64> {
        self.query = query;
        self.compiled = None;
        self.error = None;
        self.anchor = None;
        self.matches.clear();
        self.current = None;
        self.generation += 1;
        if self.query.text.is_empty() {
            return None;
        }
        if self.query.is_quick() {
            self.take(build(&self.query));
            return None;
        }
        Some(self.generation)
    }

    fn take(&mut self, compiled: Compiled) {
        match compiled {
            Ok(compiled) => self.compiled = Some(compiled),
            Err(why) => self.error = Some(invalid(&why)),
        }
    }

    /// A matcher built elsewhere came: taken if it is for the query that
    /// is searched now. Whether it was.
    pub fn compiled(&mut self, generation: u64, compiled: Compiled) -> bool {
        if generation != self.generation || !self.open {
            return false;
        }
        self.take(compiled);
        true
    }

    /// The bar closed: the matcher and the matches are let go of; what was
    /// typed stays for the next time.
    pub fn close(&mut self) {
        self.open = false;
        self.compiled = None;
        self.matches.clear();
        self.current = None;
        self.anchor = None;
        self.stale = false;
        self.generation += 1;
    }

    pub fn is_active(&self) -> bool {
        self.open && self.compiled.is_some()
    }

    /// Output came: the matches are listed again, or soon if they just were.
    pub fn output_came<T>(&mut self, term: &Term<T>) {
        if !self.is_active() {
            return;
        }
        let due = self
            .scanned
            .is_none_or(|scanned| scanned.elapsed() >= RESCAN_EVERY);
        if due {
            self.refresh(term);
        } else {
            self.stale = true;
        }
    }

    /// When the matches should be listed again, if output came since.
    pub fn due(&self) -> Option<Instant> {
        if !self.stale || !self.is_active() {
            return None;
        }
        Some(
            self.scanned
                .map_or_else(Instant::now, |at| at + RESCAN_EVERY),
        )
    }

    /// Lists them again if output came since and their time came.
    pub fn catch_up<T>(&mut self, term: &Term<T>) {
        if self.due().is_some_and(|due| due <= Instant::now()) {
            self.refresh(term);
        }
    }

    /// Lists the matches of what `term` holds now. The current match
    /// stays the one it was; a new query starts on the latest.
    pub fn refresh<T>(&mut self, term: &Term<T>) {
        self.matches.clear();
        self.stale = false;
        self.scanned = Some(Instant::now());
        let compiled = match self.compiled.as_mut() {
            Some(compiled) if self.open => compiled,
            _ => {
                self.current = None;
                return;
            }
        };
        let start = Point::new(term.topmost_line(), Column(0));
        let end = Point::new(term.bottommost_line(), term.last_column());
        self.matches
            .extend(RegexIter::new(start, end, Direction::Right, term, compiled).take(MAX_MATCHES));
        self.current = match &self.anchor {
            _ if self.matches.is_empty() => None,
            None => Some(self.matches.len() - 1),
            Some(anchor) => Some(self.found_again(term, anchor)),
        };
        self.remember(term);
    }

    /// The match `anchor` was: where its row moved up to while the
    /// scrollback is not full, if it still says the same; else the same
    /// place in a row that says the same, as near below where it was as
    /// there is (new lines only move rows up); the oldest when its row is
    /// gone.
    fn found_again<T>(&self, term: &Term<T>, anchor: &Anchor) -> usize {
        let history = term.history_size();
        if history < self.scrollback && history >= anchor.history {
            let moved = i32::try_from(history - anchor.history).unwrap_or(i32::MAX);
            let start = Point::new(anchor.start.line - moved, anchor.start.column);
            if let Ok(index) = self
                .matches
                .binary_search_by(|found| found.start().cmp(&start))
            {
                if row_text(term, start.line) == anchor.row {
                    return index;
                }
            }
        }
        let mut rows: Vec<(Line, String)> = vec![];
        let mut best = None;
        for (index, found) in self.matches.iter().enumerate().rev() {
            let start = *found.start();
            if start.line > anchor.start.line || start.column != anchor.start.column {
                continue;
            }
            let said = match rows.iter().find(|(line, _)| *line == start.line) {
                Some((_, said)) => said.clone(),
                None => {
                    let said = row_text(term, start.line);
                    rows.push((start.line, said.clone()));
                    said
                }
            };
            if said == anchor.row {
                best = Some(index);
                break;
            }
        }
        best.unwrap_or(0)
    }

    fn remember<T>(&mut self, term: &Term<T>) {
        self.anchor = self
            .current
            .and_then(|index| self.matches.get(index))
            .map(|found| Anchor {
                start: *found.start(),
                row: row_text(term, found.start().line),
                history: term.history_size(),
            });
    }

    /// Goes to the next match (towards the latest line), or the one before,
    /// round at the ends; its start, to scroll to.
    pub fn step<T>(&mut self, term: &Term<T>, forward: bool) -> Option<Point> {
        if self.stale {
            self.refresh(term);
        }
        let count = self.matches.len();
        if count == 0 {
            return None;
        }
        let next = match (self.current, forward) {
            (None, true) => 0,
            (None, false) => count - 1,
            (Some(at), true) => (at + 1) % count,
            (Some(at), false) => (at + count - 1) % count,
        };
        self.current = Some(next);
        self.remember(term);
        Some(*self.matches[next].start())
    }

    /// The matches that touch the lines `top..=bottom`, each with whether
    /// it is the current one.
    pub fn within(&self, top: Line, bottom: Line) -> impl Iterator<Item = (&Match, bool)> {
        let first = self.matches.partition_point(|found| found.end().line < top);
        self.matches[first..]
            .iter()
            .enumerate()
            .take_while(move |(_, found)| found.start().line <= bottom)
            .map(move |(index, found)| (found, Some(first + index) == self.current))
    }

    /// Which match is current, from one, and how many there are; whether
    /// the count stopped at [`MAX_MATCHES`].
    pub fn counted(&self) -> (usize, usize, bool) {
        let current = self.current.map_or(0, |at| at + 1);
        (
            current,
            self.matches.len(),
            self.matches.len() >= MAX_MATCHES,
        )
    }
}
