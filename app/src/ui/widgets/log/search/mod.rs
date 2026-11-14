//! Searching the lines: what is looked for, which lines hold it, and the
//! match the search stands on.
//!
//! Only how many matches each line holds is kept (sixteen bytes a matching
//! line, whatever the query): where they are in a line is worked out for
//! the lines in view, when they are drawn. The lines are read in order,
//! from the oldest: [`Search::frontier`] is how far that went, and it
//! never passes a line that was not read. A buffer of a million lines is
//! read in slices of [`SLICE`] lines on another thread ([`scan`] on a
//! [`Snapshot`]), so `update` never waits for it; the few lines a stream
//! appends in a frame are read at once.
//!
//! A query that is plain text and short is made into its matcher where it
//! is typed: that is quick, and bounded ([`QUICK`]). A regular expression,
//! whose cost is its author's, is built on another thread, within
//! [`SIZE_LIMIT`].

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use homeport_core::error::{AppError, ErrorCode, Message as Text};
use regex::{Regex, RegexBuilder};

pub mod hits;

use self::hits::Hits;
use super::buffer::{LineRef, Snapshot};

/// The lines a task reads in one go.
pub const SLICE: u64 = 65_536;
/// Up to this many lines, of up to this many bytes together, are read
/// where they are, without a task.
pub const AT_ONCE: u64 = 2_048;
pub const AT_ONCE_BYTES: usize = 512 * 1024;
/// A query of plain text up to this many bytes is built where it is typed.
pub const QUICK: usize = 256;
/// The longest query, in bytes, and the most memory its matcher may take.
pub const QUERY_MAX: usize = 4_096;
pub const SIZE_LIMIT: usize = 2 * 1024 * 1024;
/// How many times a slice whose task died is read again before the
/// search says it stopped.
pub const RETRIES: u8 = 1;

/// The catalogue's words for a query that can't be read and for a search
/// that stopped.
const INVALID_KEY: &str = "ui.log.invalidRegex";
const FAILED_KEY: &str = "ui.log.searchFailed";

/// A query that can't be read, with why in its detail (the `regex`
/// crate's words, which are not translated).
pub(in crate::ui::widgets) fn invalid(why: &str) -> AppError {
    AppError::validation(Text {
        key: INVALID_KEY,
        en: "Not a valid regular expression",
    })
    .with_detail(why)
}

/// A search whose task died twice on the same lines.
fn failed() -> AppError {
    AppError::new(
        ErrorCode::Unknown,
        Text {
            key: FAILED_KEY,
            en: "The search stopped before reading every line",
        },
    )
}

/// A number no other search of the process has: what tells the answer
/// of a task from one meant for another query, or another viewer.
fn next_generation() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

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
    /// Whether its matcher is built where it is typed: plain text, short.
    pub fn is_quick(&self) -> bool {
        !self.regex && self.text.len() <= QUICK
    }

    /// What finds this query's matches; `None` when nothing is looked for.
    /// A regular expression that can't be read, or one too large, is the
    /// error, as the `regex` crate words it.
    pub fn compile(&self) -> Result<Option<Regex>, String> {
        if self.text.is_empty() {
            return Ok(None);
        }
        if self.text.len() > QUERY_MAX {
            return Err(format!("longer than {QUERY_MAX} bytes"));
        }
        let pattern = if self.regex {
            self.text.clone()
        } else {
            regex::escape(&self.text)
        };
        RegexBuilder::new(&pattern)
            .case_insensitive(!self.case)
            .size_limit(SIZE_LIMIT)
            .dfa_size_limit(SIZE_LIMIT)
            .build()
            .map(Some)
            .map_err(|error| error.to_string())
    }
}

/// What building a query's matcher gave.
pub type Compiled = Result<Option<Arc<Regex>>, Arc<str>>;

/// [`Query::compile`] on a thread that may block, where the runtime has
/// one.
pub async fn compile_elsewhere(query: Query) -> Compiled {
    let build = move || {
        query
            .compile()
            .map(|matcher| matcher.map(Arc::new))
            .map_err(Arc::from)
    };
    match tokio::runtime::Handle::try_current() {
        Ok(runtime) => runtime
            .spawn_blocking(build)
            .await
            .unwrap_or_else(|error| Err(Arc::from(error.to_string()))),
        Err(_) => build(),
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

/// A line that shows while only what matches does: one that holds
/// matches, and how many, or something the viewer says (no match, and
/// still there: "1,234 lines skipped" is not to be filtered away).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hit {
    pub seq: u64,
    pub count: u32,
}

/// What a line is to the search, if anything.
pub fn hit(matcher: &Regex, seq: u64, line: &LineRef<'_>, timestamps: bool) -> Option<Hit> {
    if line.marker.is_some() {
        return Some(Hit { seq, count: 0 });
    }
    let count = matches(matcher, line, timestamps).count();
    // A line holds fewer matches than it has bytes.
    #[allow(clippy::cast_possible_truncation)]
    (count > 0).then_some(Hit {
        seq,
        count: count as u32,
    })
}

/// The lines of `snapshot` that hold a match, in order.
pub fn scan(snapshot: &Snapshot, matcher: &Regex, timestamps: bool) -> Vec<Hit> {
    snapshot
        .lines()
        .filter_map(|(seq, line)| hit(matcher, seq, &line, timestamps))
        .collect()
}

/// [`scan`] on a thread that may block, where the runtime has one.
/// `None` when that thread died before it answered: the lines were not
/// read.
pub async fn scan_elsewhere(
    snapshot: Snapshot,
    matcher: Arc<Regex>,
    timestamps: bool,
) -> Option<Vec<Hit>> {
    match tokio::runtime::Handle::try_current() {
        Ok(runtime) => {
            let work = runtime.spawn_blocking(move || scan(&snapshot, &matcher, timestamps));
            work.await.ok()
        }
        // No runtime to hand it to (a test driven by hand): read it here.
        Err(_) => Some(scan(&snapshot, &matcher, timestamps)),
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
#[derive(Debug)]
pub struct Search {
    pub query: Query,
    pub matcher: Option<Arc<Regex>>,
    /// Why the query can't be read, or why the search stopped.
    pub error: Option<AppError>,
    /// The lines that show while only what matches does, among those
    /// read so far.
    pub hits: Hits,
    /// How many matches they hold together.
    pub total: u64,
    /// Every line before this one was read.
    pub frontier: u64,
    /// Changes with the query, and is no other search's: what a task of
    /// another query, or of another viewer, found is dropped.
    pub generation: u64,
    /// A task is reading a slice.
    pub busy: bool,
    /// A task is building the query's matcher.
    pub compiling: bool,
    /// How many times in a row the task of a slice died.
    pub failures: u8,
    pub current: Option<Current>,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            query: Query::default(),
            matcher: None,
            error: None,
            hits: Hits::default(),
            total: 0,
            frontier: 0,
            generation: next_generation(),
            busy: false,
            compiling: false,
            failures: 0,
            current: None,
        }
    }
}

impl Search {
    /// Whether something is looked for.
    pub fn is_active(&self) -> bool {
        self.matcher.is_some()
    }

    /// Whether the search gave up reading: its task died on the same
    /// lines more than [`RETRIES`] times.
    pub fn is_stopped(&self) -> bool {
        self.failures > RETRIES
    }

    /// Starts over with `query`, from the line `first`. Returns whether
    /// its matcher is still to be built, on another thread
    /// ([`compile_elsewhere`], then [`Search::compiled`]).
    pub fn restart(&mut self, query: Query, first: u64) -> bool {
        self.matcher = None;
        self.error = None;
        self.rescan(first);
        let later = !query.is_quick();
        if !later {
            let built = query.compile();
            self.compiled(
                built
                    .map(|matcher| matcher.map(Arc::new))
                    .map_err(Arc::from),
            );
        }
        self.compiling = later;
        self.query = query;
        later
    }

    /// Takes the matcher of the query, or why there is none.
    pub fn compiled(&mut self, compiled: Compiled) {
        self.compiling = false;
        match compiled {
            Ok(matcher) => self.matcher = matcher,
            Err(why) => self.error = Some(invalid(&why)),
        }
    }

    /// Reads the lines again with the same query, from the line `first`:
    /// what is searched changed (the timestamps show or hide), or the
    /// lines did (they were cleared).
    pub fn rescan(&mut self, first: u64) {
        self.hits.clear();
        self.total = 0;
        self.frontier = first;
        self.generation = next_generation();
        self.busy = false;
        self.compiling = false;
        if self.is_stopped() {
            self.error = None;
        }
        self.failures = 0;
        self.current = None;
    }

    /// Adds what was found in the lines up to `to`, which follow those
    /// read before.
    pub fn found(&mut self, hits: Vec<Hit>, to: u64) {
        self.hits.push(hits);
        self.total = self.hits.matches();
        self.frontier = self.frontier.max(to);
        self.failures = 0;
    }

    /// The task of a slice died: its lines were not read, and the
    /// frontier stays before them. Returns whether to read them again;
    /// if not, the search says it stopped.
    pub fn lost(&mut self) -> bool {
        self.busy = false;
        self.failures = self.failures.saturating_add(1);
        if self.is_stopped() {
            self.error = Some(failed());
            return false;
        }
        true
    }

    /// Lets go of the lines before `first`, which the buffer dropped.
    pub fn evict(&mut self, first: u64) {
        let gone = self.hits.evict(first);
        self.total = self.hits.matches();
        self.frontier = self.frontier.max(first);
        self.current = match self.current {
            Some(current) if current.seq >= first => Some(Current {
                ordinal: current.ordinal - gone,
                ..current
            }),
            _ => None,
        };
    }

    /// Where the line `seq` is among the lines that show filtered, or
    /// where it would be.
    pub fn position(&self, seq: u64) -> Result<usize, usize> {
        self.hits.position(seq)
    }

    /// How many matches the lines before the one at `index` hold.
    fn before(&self, index: usize) -> u64 {
        self.hits.matches_before(index)
    }

    /// How many matches the line at `index` holds.
    fn count(&self, index: usize) -> u32 {
        self.hits.get(index).map_or(0, |hit| hit.count)
    }

    /// The line that holds matches nearest to the one at `index`, itself
    /// included, going on (or `back`) and round at the ends.
    fn matching(&self, index: usize, back: bool) -> Option<usize> {
        let len = self.hits.len();
        (0..len)
            .map(|step| {
                if back {
                    (index + len - step) % len
                } else {
                    (index + step) % len
                }
            })
            .find(|at| self.count(*at) > 0)
    }

    /// Moves to the match after the current one (or before it, `back`),
    /// going round at the ends. With none yet, to the first match at or
    /// after the line `from` (or the last at or before it).
    pub fn step(&mut self, back: bool, from: u64) -> Option<Current> {
        if self.total == 0 {
            self.current = None;
            return None;
        }
        let len = self.hits.len();
        let at = |index: usize, nth: u32, ordinal: u64| Current {
            seq: self.hits.get(index).map_or(0, |hit| hit.seq),
            nth,
            ordinal,
        };
        let standing = self.current.and_then(|current| {
            let index = self.position(current.seq).ok()?;
            Some((current, index))
        });
        let next = match standing {
            Some((current, _)) if back && current.nth > 0 => Current {
                nth: current.nth - 1,
                ordinal: current.ordinal - 1,
                ..current
            },
            Some((current, index)) if !back && current.nth + 1 < self.count(index) => Current {
                nth: current.nth + 1,
                ordinal: current.ordinal + 1,
                ..current
            },
            Some((current, index)) if back => {
                let to = self.matching((index + len - 1) % len, true)?;
                let ordinal = if to < index {
                    current.ordinal - 1
                } else {
                    self.total
                };
                at(to, self.count(to) - 1, ordinal)
            }
            Some((current, index)) => {
                let to = self.matching((index + 1) % len, false)?;
                let ordinal = if to > index { current.ordinal + 1 } else { 1 };
                at(to, 0, ordinal)
            }
            None if back => {
                // The last matching line at or before `from`.
                let start = match self.position(from) {
                    Ok(index) => index,
                    Err(0) => len - 1,
                    Err(after) => after - 1,
                };
                let to = self.matching(start, true)?;
                let count = self.count(to);
                at(to, count - 1, self.before(to) + u64::from(count))
            }
            None => {
                let start = match self.position(from) {
                    Ok(index) => index,
                    Err(after) => after % len,
                };
                let to = self.matching(start, false)?;
                at(to, 0, self.before(to) + 1)
            }
        };
        self.current = Some(next);
        self.current
    }
}
