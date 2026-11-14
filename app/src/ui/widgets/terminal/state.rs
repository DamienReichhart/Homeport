//! What a terminal remembers from one view to the next: its grid and
//! scrollback, how big it is, what is selected and what is searched. The
//! page keeps one per terminal, feeds it what the session printed and
//! hands it the widget's [`Message`]s; what goes back to the session (keys,
//! answers, a new size) comes out as [`Event`]s.

use alacritty_terminal::{
    grid::{Dimensions, Scroll as GridScroll},
    index::{Column, Point, Side},
    selection::{Selection, SelectionType},
    term::TermMode,
    vte::ansi::{NamedColor, Rgb},
    Term,
};
use std::{collections::VecDeque, time::Instant};

use iced::{advanced::widget::Id, widget::operation, Task};

pub use super::message::{Click, Event, Message, Scroll, Viewport};
use super::{
    emulator::{Emulator, Listener, Size},
    search::{self, Search},
};
use crate::ui::{
    theme::{metrics::text as size, Theme},
    widgets::log::{palette, search::Query},
};

/// How many lines scroll out of view and are kept: `alacritty_terminal`'s
/// default. A cell takes 24 bytes, so 10,000 lines of 200 columns are
/// 48 MB; the page can ask for more or less.
pub const DEFAULT_SCROLLBACK: usize = 10_000;

/// The text's size unless the page says (xterm.js's `fontSize` in React).
pub const DEFAULT_FONT_SIZE: f32 = size::XS;
/// The smallest and largest text a terminal is drawn in.
pub const MIN_FONT_SIZE: f32 = 8.0;
pub const MAX_FONT_SIZE: f32 = 32.0;

/// The most output read in one `update`: a window that fell behind gets
/// a backlog of megabytes in one batch (16 MB at most, `data::streams`),
/// which is read a megabyte a frame, drawn in between.
pub const READ_AT_ONCE: usize = 1024 * 1024;
/// The most output that waits to be read: eight frames' worth. A program
/// that writes faster than it is read (60 MB a second) has its oldest
/// unread output dropped, from a line's start, as the stream's backlog
/// drops the oldest when the window is behind; so what shows after
/// Ctrl + C is never more than eight frames late.
pub const UNREAD_MAX: usize = 8 * READ_AT_ONCE;
/// The longest title kept and told, in characters.
pub const TITLE_MAX: usize = 256;

/// When the grid is to ask for what waits ([`State::due`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Due {
    /// At the next frame: output waits.
    Now,
    /// Once this time has come.
    At(Instant),
}

/// A terminal's state.
#[derive(Debug)]
pub struct State {
    pub(super) emulator: Emulator,
    viewport: Viewport,
    font_size: f32,
    pub(super) search: Search,
    scrollback: usize,
    /// What names the grid, for focus to be given back to it.
    pub(super) id: Id,
    dark: bool,
    focused: bool,
    blinking: bool,
    title: Option<String>,
    /// Output not read yet: past [`READ_AT_ONCE`] of a batch.
    unread: VecDeque<u8>,
    /// How many bytes of it were dropped past [`UNREAD_MAX`].
    dropped: u64,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// The colour the theme gives `index` (one of the 256, or the text, the
/// background or the cursor), for a program that asks.
fn theme_colour(dark: bool, index: usize) -> Rgb {
    let theme = if dark { Theme::Dark } else { Theme::Light };
    let terminal = theme.tokens().terminal;
    let colour = match index {
        0..=15 => terminal.ansi[index],
        16..=255 => palette::fixed(u8::try_from(index).unwrap_or(u8::MAX)),
        _ if index == NamedColor::Background as usize => terminal.background,
        _ if index == NamedColor::Cursor as usize => terminal.cursor,
        _ => terminal.foreground,
    };
    let [r, g, b, _] = colour.into_rgba8();
    Rgb { r, g, b }
}

impl State {
    /// A terminal of 80 by 24 until it is laid out, keeping
    /// [`DEFAULT_SCROLLBACK`] lines.
    pub fn new() -> Self {
        let viewport = Viewport::default();
        Self {
            emulator: Emulator::new(
                Size::clamped(viewport.cols, viewport.rows),
                DEFAULT_SCROLLBACK,
            ),
            viewport,
            font_size: DEFAULT_FONT_SIZE,
            search: Search::default(),
            scrollback: DEFAULT_SCROLLBACK,
            id: Id::unique(),
            dark: false,
            focused: false,
            blinking: true,
            title: None,
            unread: VecDeque::new(),
            dropped: 0,
        }
    }

    /// Keeps `lines` lines of scrollback. Anything written before is lost.
    #[must_use]
    pub fn with_scrollback(mut self, lines: usize) -> Self {
        self.emulator = Emulator::new(self.emulator.size(), lines);
        self.scrollback = lines;
        self.search.scrollback = lines;
        self
    }

    /// Draws its text at `size` pixels (xterm.js's `fontSize`).
    #[must_use]
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.set_font_size(size);
        self
    }

    /// Whether the cursor blinks while the terminal has focus: not when the
    /// user asked the OS for less motion (React's `cursorBlink`).
    #[must_use]
    pub fn blinking(mut self, blinking: bool) -> Self {
        self.blinking = blinking;
        self
    }

    /// Drawn in a dark theme: what a program that asks for the colours is
    /// told before the terminal was first drawn (the drawing tells it after).
    #[must_use]
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// The text's size changes: the grid is laid out again and its new
    /// size is told as any other.
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size.clamp(MIN_FONT_SIZE, MAX_FONT_SIZE);
    }

    /// A terminal for a new session: nothing of the last one is kept but
    /// its size, its scrollback's length and what it searches.
    pub fn reset(&mut self) {
        self.emulator = Emulator::new(self.emulator.size(), self.scrollback);
        self.title = None;
        self.unread.clear();
        self.search.refresh(&self.emulator.term);
    }

    /// What the session printed, in order. Returns what the program asked
    /// of the terminal (answers to its queries go back as input).
    pub fn feed(&mut self, output: &str) -> Vec<Event> {
        self.feed_bytes(output.as_bytes())
    }

    /// Up to [`READ_AT_ONCE`] is read at once; the rest waits, in order,
    /// for the next frames ([`State::is_behind`]), and no more than
    /// [`UNREAD_MAX`] of it.
    pub fn feed_bytes(&mut self, output: &[u8]) -> Vec<Event> {
        if self.unread.is_empty() && output.len() <= READ_AT_ONCE {
            self.emulator.advance(output);
        } else {
            self.unread.extend(output);
            self.bound_unread();
            self.read_unread();
        }
        self.search.output_came(&self.emulator.term);
        self.events()
    }

    /// Drops the oldest unread output past [`UNREAD_MAX`], up to the start
    /// of a line, so what is read next starts where a line does.
    fn bound_unread(&mut self) {
        let Some(over) = self.unread.len().checked_sub(UNREAD_MAX) else {
            return;
        };
        let line = self
            .unread
            .range(over..)
            .position(|byte| *byte == b'\n')
            .map_or(over, |at| over + at + 1);
        self.unread.drain(..line);
        self.dropped += line as u64;
    }

    fn read_unread(&mut self) {
        let take = self.unread.len().min(READ_AT_ONCE);
        let (front, back) = self.unread.as_slices();
        if front.len() >= take {
            self.emulator.advance(&front[..take]);
        } else {
            let mut part = Vec::with_capacity(take);
            part.extend_from_slice(front);
            part.extend_from_slice(&back[..take - front.len()]);
            self.emulator.advance(&part);
        }
        self.unread.drain(..take);
    }

    /// Whether output came faster than it was read, and waits.
    pub fn is_behind(&self) -> bool {
        !self.unread.is_empty()
    }

    /// How many bytes wait to be read, and how many were dropped.
    pub fn unread(&self) -> usize {
        self.unread.len()
    }

    pub fn dropped(&self) -> u64 {
        self.dropped
    }

    /// When the grid should ask for [`Message::Read`]: at the next frame
    /// while output waits, or at the time a synchronised update that never
    /// ended runs out, or the search's matches are due to be listed again.
    pub fn due(&self) -> Option<Due> {
        if self.is_behind() {
            return Some(Due::Now);
        }
        [self.emulator.holding_until(), self.search.due()]
            .into_iter()
            .flatten()
            .min()
            .map(Due::At)
    }

    /// A line of the app's, dimmed, after what the program printed: how a
    /// session ended (React's `\x1b[2m…\x1b[22m`). The text is the page's,
    /// in the user's language. What the program left unfinished (a string
    /// it was writing, a synchronised update, its colours) doesn't hide or
    /// colour it.
    pub fn notice(&mut self, text: &str) {
        while self.is_behind() {
            self.read_unread();
        }
        self.emulator.release();
        let text: String = text.chars().filter(|c| !c.is_control()).collect();
        let line = format!("\x18\x1b[0m\r\n\x1b[2m{text}\x1b[22m\r\n");
        drop(self.feed(&line));
    }

    fn events(&mut self) -> Vec<Event> {
        let dark = self.dark;
        let mut events = self
            .emulator
            .events(self.viewport, |index| theme_colour(dark, index));
        for event in &mut events {
            if let Event::Title(Some(title)) = event {
                if title.chars().count() > TITLE_MAX {
                    *title = title.chars().take(TITLE_MAX).collect();
                }
            }
            if let Event::Title(title) = event {
                self.title.clone_from(title);
            }
        }
        events
    }

    /// A press on the grid starts a selection of `click`'s kind, or takes
    /// the one there further.
    fn press(&mut self, at: Point, side: Side, click: Click, extend: bool) {
        let term = &mut self.emulator.term;
        if let (true, Some(selection)) = (extend, term.selection.as_mut()) {
            selection.update(at, side);
            return;
        }
        let kind = match click {
            Click::Single => SelectionType::Simple,
            Click::Double => SelectionType::Semantic,
            Click::Triple => SelectionType::Lines,
        };
        term.selection = Some(Selection::new(kind, at, side));
    }

    fn scroll(&mut self, scroll: Scroll) {
        let term = &mut self.emulator.term;
        let to = match scroll {
            Scroll::Lines(lines) => GridScroll::Delta(-lines),
            Scroll::Pages(pages) => {
                let rows = i32::try_from(term.screen_lines()).unwrap_or(i32::MAX);
                GridScroll::Delta(-pages.saturating_mul(rows))
            }
            Scroll::Top => GridScroll::Top,
            Scroll::Bottom => GridScroll::Bottom,
            Scroll::To(share) => {
                let history = term.history_size();
                // A share of the scrollback: lines, far below an `f32`'s whole range.
                #[allow(
                    clippy::cast_precision_loss,
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss
                )]
                let offset = history - (share.clamp(0.0, 1.0) * history as f32).round() as usize;
                let now = term.grid().display_offset();
                let delta = i64::try_from(offset).unwrap_or(0) - i64::try_from(now).unwrap_or(0);
                GridScroll::Delta(i32::try_from(delta).unwrap_or(0))
            }
        };
        term.scroll_display(to);
    }

    /// Shows `point` if it is out of view, a few lines from the top.
    fn reveal(&mut self, point: Point) {
        let term = &mut self.emulator.term;
        let offset = i32::try_from(term.grid().display_offset()).unwrap_or(0);
        let rows = i32::try_from(term.screen_lines()).unwrap_or(1);
        let top = -offset;
        if point.line.0 < top || point.line.0 >= top + rows {
            let wanted = (-point.line.0 + rows / 3).clamp(0, i32::MAX);
            term.scroll_display(GridScroll::Delta(wanted - offset));
        }
    }

    fn resized(&mut self, viewport: Viewport) -> Option<Event> {
        self.viewport = viewport;
        let size = Size::clamped(viewport.cols, viewport.rows);
        if size == self.emulator.size() {
            return None;
        }
        self.emulator.resize(size);
        if self.search.is_active() {
            self.search.refresh(&self.emulator.term);
        }
        let narrow = |value: usize| u16::try_from(value).unwrap_or(u16::MAX);
        Some(Event::Resize {
            cols: narrow(size.cols),
            rows: narrow(size.rows),
        })
    }

    /// What the program is sent for `text` pasted: its line ends as Enter
    /// (xterm.js's `prepareTextForTerminal`), and between the brackets of
    /// a bracketed paste when the program asked for them, with every
    /// Escape taken out (as alacritty does), so nothing in it can end the
    /// brackets early, however it is nested.
    pub fn pasted(&self, text: &str) -> String {
        let text = text.replace("\r\n", "\r").replace('\n', "\r");
        if text.is_empty() || !self.mode().contains(TermMode::BRACKETED_PASTE) {
            return text;
        }
        let text: String = text
            .chars()
            .filter(|c| !matches!(c, '\x1b' | '\u{9b}'))
            .collect();
        format!("\x1b[200~{text}\x1b[201~")
    }

    /// The search looks for something else: its matches are listed, and
    /// the latest brought into view; a query that is not quick is built on
    /// another thread first (the task).
    fn query_with(&mut self, change: impl FnOnce(&mut Query)) -> Task<Message> {
        let mut query = self.search.query.clone();
        change(&mut query);
        match self.search.set(query.clone()) {
            Some(generation) => Task::perform(search::compile(query), move |compiled| {
                Message::Compiled(generation, Box::new(compiled))
            }),
            None => {
                self.found();
                Task::none()
            }
        }
    }

    /// The matcher is there: the matches are listed and the latest shown.
    fn found(&mut self) {
        self.search.refresh(&self.emulator.term);
        if let Some(found) = self
            .search
            .current
            .and_then(|at| self.search.matches.get(at))
        {
            let start = *found.start();
            self.reveal(start);
        }
    }

    fn step(&mut self, forward: bool) {
        if let Some(start) = self.search.step(&self.emulator.term, forward) {
            self.reveal(start);
        }
    }

    /// Takes a message of the widget's: what goes back to the session and
    /// the page, in order, and a task to map (a search field given focus,
    /// a regular expression built elsewhere).
    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Event>) {
        let event = match message {
            Message::Viewport(viewport) => self.resized(viewport),
            Message::Dark(dark) => {
                self.dark = dark;
                None
            }
            Message::Focus(focused) => {
                self.focused = focused;
                self.emulator.term.is_focused = focused;
                let reports = self.mode().contains(TermMode::FOCUS_IN_OUT);
                reports.then(|| Event::Input(if focused { "\x1b[I" } else { "\x1b[O" }.into()))
            }
            Message::Input(text) => {
                // What is typed goes where the program writes.
                self.scroll(Scroll::Bottom);
                Some(Event::Input(text))
            }
            Message::Paste(text) => {
                self.scroll(Scroll::Bottom);
                let text = self.pasted(&text);
                (!text.is_empty()).then_some(Event::Input(text))
            }
            Message::Copy => self.selection_text().map(Event::Copy),
            Message::SelectAll => {
                let term = &mut self.emulator.term;
                let start = Point::new(term.topmost_line(), Column(0));
                let end = Point::new(term.bottommost_line(), term.last_column());
                let mut all = Selection::new(SelectionType::Simple, start, Side::Left);
                all.update(end, Side::Right);
                term.selection = Some(all);
                None
            }
            Message::Press {
                at,
                side,
                click,
                extend,
            } => {
                self.press(at, side, click, extend);
                None
            }
            Message::Drag { at, side } => {
                if let Some(selection) = self.emulator.term.selection.as_mut() {
                    selection.update(at, side);
                }
                None
            }
            Message::Scroll(scroll) => {
                self.scroll(scroll);
                None
            }
            Message::Read => {
                self.emulator.settle_sync();
                if self.is_behind() {
                    self.read_unread();
                    self.search.output_came(&self.emulator.term);
                }
                self.search.catch_up(&self.emulator.term);
                return (Task::none(), self.events());
            }
            Message::Open(url) => Some(Event::Open(url)),
            Message::OpenSearch => {
                self.search.open = true;
                // What is selected is what is looked for, as in a browser;
                // else what was looked for last.
                let selected = self.selection_text().filter(|t| !t.contains('\n'));
                let compiling = self.query_with(|query| {
                    if let Some(text) = selected {
                        query.text = text;
                    }
                });
                let focus = operation::focus(self.search.id.clone());
                return (Task::batch([compiling, focus]), vec![]);
            }
            Message::CloseSearch => {
                self.search.close();
                return (operation::focus(self.id.clone()), vec![]);
            }
            Message::Query(text) => return (self.query_with(|query| query.text = text), vec![]),
            Message::Case(case) => return (self.query_with(|query| query.case = case), vec![]),
            Message::Regex(regex) => return (self.query_with(|query| query.regex = regex), vec![]),
            Message::Compiled(generation, compiled) => {
                if self.search.compiled(generation, *compiled) {
                    self.found();
                }
                None
            }
            Message::Next => {
                self.step(true);
                None
            }
            Message::Previous => {
                self.step(false);
                None
            }
        };
        (Task::none(), event.into_iter().collect())
    }

    /// What is selected, as text: wrapped lines joined, trailing blanks off.
    pub fn selection_text(&self) -> Option<String> {
        self.emulator
            .term
            .selection_to_string()
            .filter(|text| !text.is_empty())
    }

    pub fn has_selection(&self) -> bool {
        self.emulator
            .term
            .selection
            .as_ref()
            .is_some_and(|selection| !selection.is_empty())
    }

    /// The grid, its scrollback and its modes, to read.
    pub fn term(&self) -> &Term<Listener> {
        &self.emulator.term
    }

    pub fn mode(&self) -> TermMode {
        *self.emulator.term.mode()
    }

    /// The size the grid has now.
    pub fn size(&self) -> (usize, usize) {
        let size = self.emulator.size();
        (size.cols, size.rows)
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn is_dark(&self) -> bool {
        self.dark
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn blinks(&self) -> bool {
        self.blinking
    }

    /// What the program last called its window.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Whether the search bar shows, and what it holds.
    pub fn search(&self) -> &Search {
        &self.search
    }

    pub fn search_id(&self) -> Id {
        self.search.id.clone()
    }

    /// What names the grid: `iced::widget::operation::focus(state.id())`
    /// gives it the keyboard.
    pub fn id(&self) -> Id {
        self.id.clone()
    }

    /// Whether the view is on the screen, not up in the scrollback.
    pub fn at_bottom(&self) -> bool {
        self.emulator.term.grid().display_offset() == 0
    }

    /// The lines the grid keeps, the screen's included.
    pub fn total_lines(&self) -> usize {
        self.emulator.term.total_lines()
    }
}
