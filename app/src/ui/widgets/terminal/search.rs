//! Searching what the terminal holds, its scrollback and its screen:
//! `@xterm/addon-search`'s next and previous, with the log viewer's case
//! and regular expression toggles and its count of matches.
//!
//! The query is read as the log viewer reads it ([`Query::compile`] says
//! why a regular expression can't be), then given to
//! `alacritty_terminal`'s own search, which finds matches across the
//! rows a long line wrapped onto. Every match is listed, up to
//! [`MAX_MATCHES`]: the list is made again when the query changes and
//! when output came, since new lines move the old ones up.

use alacritty_terminal::{
    grid::Dimensions,
    index::{Column, Direction, Line, Point},
    term::search::{Match, RegexIter, RegexSearch},
    Term,
};
use homeport_core::error::AppError;
use iced::advanced::widget::Id;

use crate::ui::widgets::log::search::{invalid, Query};

/// The most matches counted: past it the count says so ("10,000+").
pub const MAX_MATCHES: usize = 10_000;

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
    /// Where the current match starts, counted from the oldest line kept:
    /// what finds it again once new lines moved it up.
    anchor: Option<(i64, usize)>,
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
            id: Id::unique(),
        }
    }
}

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

/// How far `point` is from the oldest line `term` keeps.
fn absolute<T>(term: &Term<T>, point: Point) -> (i64, usize) {
    let history = i64::try_from(term.history_size()).unwrap_or(i64::MAX);
    (i64::from(point.line.0) + history, point.column.0)
}

impl Search {
    /// Reads the query anew; the matches are then found by [`Search::refresh`].
    pub fn set(&mut self, query: Query) {
        self.query = query;
        self.compiled = None;
        self.error = None;
        self.anchor = None;
        match self.query.compile() {
            Ok(None) => {}
            Ok(Some(_)) => match RegexSearch::new(&pattern(&self.query)) {
                Ok(compiled) => self.compiled = Some(compiled),
                Err(error) => self.error = Some(invalid(&error.to_string())),
            },
            Err(why) => self.error = Some(invalid(&why)),
        }
    }

    pub fn is_active(&self) -> bool {
        self.compiled.is_some()
    }

    /// Lists the matches of what `term` holds now. The current match
    /// stays the one it was; a new query starts on the latest.
    pub fn refresh<T>(&mut self, term: &Term<T>) {
        self.matches.clear();
        let Some(compiled) = self.compiled.as_mut() else {
            self.current = None;
            return;
        };
        let start = Point::new(term.topmost_line(), Column(0));
        let end = Point::new(term.bottommost_line(), term.last_column());
        self.matches
            .extend(RegexIter::new(start, end, Direction::Right, term, compiled).take(MAX_MATCHES));
        self.current = match self.anchor {
            _ if self.matches.is_empty() => None,
            None => Some(self.matches.len() - 1),
            Some(anchor) => {
                let at = self
                    .matches
                    .partition_point(|found| absolute(term, *found.start()) < anchor);
                Some(at.min(self.matches.len() - 1))
            }
        };
        self.remember(term);
    }

    fn remember<T>(&mut self, term: &Term<T>) {
        self.anchor = self
            .current
            .and_then(|index| self.matches.get(index))
            .map(|found| absolute(term, *found.start()));
    }

    /// Goes to the next match (towards the latest line), or the one before,
    /// round at the ends; its start, to scroll to.
    pub fn step<T>(&mut self, term: &Term<T>, forward: bool) -> Option<Point> {
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
