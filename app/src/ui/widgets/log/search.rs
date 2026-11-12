//! Searching the lines: what is looked for, which lines hold it, and the
//! match the search stands on.
//!
//! Only how many matches each line holds is kept (sixteen bytes a matching
//! line, whatever the query): where they are in a line is worked out for
//! the lines in view, when they are drawn. The lines are read in order,
//! from the oldest: [`Search::frontier`] is how far that went. A buffer of
//! a million lines is read in slices of [`SLICE`] lines on another thread
//! ([`scan`] on a [`Snapshot`]), so `update` never waits for it; the few
//! lines a stream appends in a frame are read at once.

use std::{collections::VecDeque, sync::Arc};

use regex::{Regex, RegexBuilder};

use super::buffer::{LineRef, Snapshot};

/// The lines a task reads in one go.
pub const SLICE: u64 = 65_536;
/// Up to this many lines are read where they are, without a task.
pub const AT_ONCE: u64 = 2_048;

/// What is looked for, and how.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Query {
    pub text: String,
    /// The text is a regular expression.
    pub regex: bool,
    /// Capitals matter.
    pub case: bool,
}

impl Query {
    /// What finds this query's matches; `None` when nothing is looked for.
    /// A regular expression that can't be read is the error, as the
    /// `regex` crate words it.
    pub fn compile(&self) -> Result<Option<Regex>, String> {
        if self.text.is_empty() {
            return Ok(None);
        }
        let pattern = if self.regex {
            self.text.clone()
        } else {
            regex::escape(&self.text)
        };
        RegexBuilder::new(&pattern)
            .case_insensitive(!self.case)
            .build()
            .map(Some)
            .map_err(|error| error.to_string())
    }
}

/// The part of a line that is searched: its message, and its timestamp
/// too while timestamps show.
pub fn haystack<'a>(line: &LineRef<'a>, timestamps: bool) -> (&'a str, usize) {
    if line.marker.is_some() {
        return ("", 0);
    }
    let from = if timestamps { 0 } else { line.message_start() };
    (&line.text[from..], from)
}

/// The matches of `matcher` in a line, as bytes of its whole text. A
/// match of nothing (`a*` where there is no `a`) is not one.
pub fn matches<'a>(
    matcher: &'a Regex,
    line: &LineRef<'a>,
    timestamps: bool,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let (text, from) = haystack(line, timestamps);
    matcher
        .find_iter(text)
        .filter(|found| !found.is_empty())
        .map(move |found| (from + found.start(), from + found.end()))
}

/// A line that holds matches, and how many.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hit {
    pub seq: u64,
    pub count: u32,
}

/// The lines of `snapshot` that hold a match, in order.
pub fn scan(snapshot: &Snapshot, matcher: &Regex, timestamps: bool) -> Vec<Hit> {
    snapshot
        .lines()
        .filter_map(|(seq, line)| {
            let count = matches(matcher, &line, timestamps).count();
            // A line holds fewer matches than it has bytes.
            #[allow(clippy::cast_possible_truncation)]
            (count > 0).then_some(Hit {
                seq,
                count: count as u32,
            })
        })
        .collect()
}

/// [`scan`] on a thread that may block, where the runtime has one.
pub async fn scan_elsewhere(snapshot: Snapshot, matcher: Arc<Regex>, timestamps: bool) -> Vec<Hit> {
    match tokio::runtime::Handle::try_current() {
        Ok(runtime) => {
            let work = runtime.spawn_blocking(move || scan(&snapshot, &matcher, timestamps));
            work.await.unwrap_or_default()
        }
        // No runtime to hand it to (a test driven by hand): read it here.
        Err(_) => scan(&snapshot, &matcher, timestamps),
    }
}

/// The match the search stands on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Current {
    pub seq: u64,
    /// Which of the line's matches, from 0.
    pub nth: u32,
    /// Which of all the matches found so far, from 1.
    pub ordinal: u64,
}

/// A search over a buffer.
#[derive(Debug, Default)]
pub struct Search {
    pub query: Query,
    pub matcher: Option<Arc<Regex>>,
    /// Why the query can't be read.
    pub error: Option<String>,
    /// The lines that hold a match, among those read so far.
    pub hits: VecDeque<Hit>,
    /// How many matches they hold together.
    pub total: u64,
    /// Every line before this one was read.
    pub frontier: u64,
    /// Changes with the query: what a task of another query found is dropped.
    pub generation: u64,
    /// A task is reading a slice.
    pub busy: bool,
    pub current: Option<Current>,
}

impl Search {
    /// Whether something is looked for.
    pub fn is_active(&self) -> bool {
        self.matcher.is_some()
    }

    /// Starts over with `query`, from the line `first`.
    pub fn restart(&mut self, query: Query, first: u64) {
        match query.compile() {
            Ok(matcher) => {
                self.matcher = matcher.map(Arc::new);
                self.error = None;
            }
            Err(error) => {
                self.matcher = None;
                self.error = Some(error);
            }
        }
        self.query = query;
        self.rescan(first);
    }

    /// Reads the lines again with the same query, from the line `first`:
    /// what is searched changed (the timestamps show or hide), or the
    /// lines did (they were cleared).
    pub fn rescan(&mut self, first: u64) {
        self.hits.clear();
        self.total = 0;
        self.frontier = first;
        self.generation += 1;
        self.busy = false;
        self.current = None;
    }

    /// Adds what was found in the lines up to `to`, which follow those
    /// read before.
    pub fn found(&mut self, hits: impl IntoIterator<Item = Hit>, to: u64) {
        for hit in hits {
            debug_assert!(self.hits.back().is_none_or(|last| last.seq < hit.seq));
            self.total += u64::from(hit.count);
            self.hits.push_back(hit);
        }
        self.frontier = self.frontier.max(to);
    }

    /// Lets go of the lines before `first`, which the buffer dropped.
    pub fn evict(&mut self, first: u64) {
        let mut gone = 0;
        while let Some(hit) = self.hits.front().filter(|hit| hit.seq < first) {
            gone += u64::from(hit.count);
            self.hits.pop_front();
        }
        self.total -= gone;
        self.frontier = self.frontier.max(first);
        self.current = match self.current {
            Some(current) if current.seq >= first => Some(Current {
                ordinal: current.ordinal - gone,
                ..current
            }),
            _ => None,
        };
    }

    /// Where the line `seq` is among the matching lines, or where it
    /// would be.
    pub fn position(&self, seq: u64) -> Result<usize, usize> {
        self.hits.binary_search_by_key(&seq, |hit| hit.seq)
    }

    /// How many matches the lines before the matching line `index` hold.
    fn before(&self, index: usize) -> u64 {
        self.hits
            .iter()
            .take(index)
            .map(|hit| u64::from(hit.count))
            .sum()
    }

    /// Moves to the match after the current one (or before it, `back`),
    /// going round at the ends. With none yet, to the first match at or
    /// after the line `from` (or the last at or before it).
    pub fn step(&mut self, back: bool, from: u64) -> Option<Current> {
        if self.hits.is_empty() {
            self.current = None;
            return None;
        }
        let last = self.hits.len() - 1;
        let at = |index: usize, nth: u32, ordinal: u64| Current {
            seq: self.hits[index].seq,
            nth,
            ordinal,
        };
        let next = match self.current.and_then(|current| {
            let index = self.position(current.seq).ok()?;
            Some((current, index))
        }) {
            Some((current, index)) if back => {
                if current.nth > 0 {
                    at(index, current.nth - 1, current.ordinal - 1)
                } else if index > 0 {
                    at(
                        index - 1,
                        self.hits[index - 1].count - 1,
                        current.ordinal - 1,
                    )
                } else {
                    at(last, self.hits[last].count - 1, self.total)
                }
            }
            Some((current, index)) => {
                if current.nth + 1 < self.hits[index].count {
                    at(index, current.nth + 1, current.ordinal + 1)
                } else if index < last {
                    at(index + 1, 0, current.ordinal + 1)
                } else {
                    at(0, 0, 1)
                }
            }
            None if back => {
                // The last matching line at or before `from`.
                let index = match self.position(from) {
                    Ok(index) => index,
                    Err(0) => last,
                    Err(after) => after - 1,
                };
                let count = self.hits[index].count;
                at(index, count - 1, self.before(index) + u64::from(count))
            }
            None => {
                let index = match self.position(from) {
                    Ok(index) => index,
                    Err(after) if after > last => 0,
                    Err(after) => after,
                };
                at(index, 0, self.before(index) + 1)
            }
        };
        self.current = Some(next);
        self.current
    }
}
