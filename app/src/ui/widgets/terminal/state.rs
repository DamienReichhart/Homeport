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
use iced::{advanced::widget::Id, widget::operation, Task};

use super::{
    emulator::{Emulator, Listener, Size},
    search::Search,
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

/// How much room the terminal had at its last layout: its cells and their
/// size in pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub cols: usize,
    pub rows: usize,
    pub cell_width: f32,
    pub cell_height: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            cell_width: 7.2,
            cell_height: 16.0,
        }
    }
}

/// Where the view goes in the scrollback.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scroll {
    /// This many lines towards the latest (towards the oldest when negative).
    Lines(i32),
    /// This many screens towards the latest (the oldest when negative).
    Pages(i32),
    Top,
    /// Back to the screen, where the program writes.
    Bottom,
    /// The scrollbar's thumb was dragged this far, from 0 (the oldest
    /// line) to 1 (the screen).
    To(f32),
}

/// How many presses a press of the pointer was: a selection of cells, of
/// a word, or of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Click {
    Single,
    Double,
    Triple,
}

/// What the user did to a terminal. The page hands each to [`State::update`].
#[derive(Debug, Clone)]
pub enum Message {
    /// The terminal was laid out with this much room.
    Viewport(Viewport),
    /// It was drawn in a dark theme, or a light one: what a program that
    /// asks for the colours is told.
    Dark(bool),
    /// It took or lost keyboard focus.
    Focus(bool),
    /// What the keyboard, the pointer or an input method sends the program,
    /// already encoded.
    Input(String),
    /// Text from the clipboard, to be pasted as the program wants it.
    Paste(String),
    Copy,
    SelectAll,
    Deselect,
    /// A press on the grid (`extend`: with Shift, from where the selection
    /// started).
    Press {
        at: Point,
        side: Side,
        click: Click,
        extend: bool,
    },
    /// The pointer moved with the button held.
    Drag {
        at: Point,
        side: Side,
    },
    Scroll(Scroll),
    /// The next part of output that came faster than it is read: the
    /// grid asks for it at each frame while there is some.
    Read,
    /// A link was opened (Ctrl or Cmd and a press).
    Open(String),
    /// The search bar, and what it searches.
    OpenSearch,
    CloseSearch,
    Query(String),
    Case(bool),
    Regex(bool),
    Next,
    Previous,
}

/// What the page does for its terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Bytes for the session: what was typed or pasted, a report of the
    /// pointer, an answer to a program's query.
    Input(String),
    /// The grid has this many cells now: tell the session.
    Resize {
        cols: u16,
        rows: u16,
    },
    /// The selection, for the clipboard (`ctx.copy`).
    Copy(String),
    /// A link to open (`ctx.open_url`, which opens web, mail and phone
    /// links only).
    Open(String),
    /// What the program calls its window, or none any more.
    Title(Option<String>),
    Bell,
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
    unread: std::collections::VecDeque<u8>,
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
            unread: std::collections::VecDeque::new(),
        }
    }

    /// Keeps `lines` lines of scrollback. Anything written before is lost.
    #[must_use]
    pub fn with_scrollback(mut self, lines: usize) -> Self {
        self.emulator = Emulator::new(self.emulator.size(), lines);
        self.scrollback = lines;
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
    /// for the next frames ([`State::is_behind`]).
    pub fn feed_bytes(&mut self, output: &[u8]) -> Vec<Event> {
        if self.unread.is_empty() && output.len() <= READ_AT_ONCE {
            self.emulator.advance(output);
        } else {
            self.unread.extend(output);
            self.read_unread();
        }
        if self.search.is_active() {
            self.search.refresh(&self.emulator.term);
        }
        self.events()
    }

    fn read_unread(&mut self) {
        let take = self.unread.len().min(READ_AT_ONCE);
        let part: Vec<u8> = self.unread.drain(..take).collect();
        self.emulator.advance(&part);
    }

    /// Whether output came faster than it was read, and waits.
    pub fn is_behind(&self) -> bool {
        !self.unread.is_empty()
    }

    /// A line of the app's, dimmed, after what the program printed: how a
    /// session ended (React's `\x1b[2m…\x1b[22m`). The text is the page's,
    /// in the user's language.
    pub fn notice(&mut self, text: &str) {
        let line = format!("\r\n\x1b[2m{}\x1b[22m\r\n", text.replace('\x1b', ""));
        drop(self.feed(&line));
    }

    fn events(&mut self) -> Vec<Event> {
        let dark = self.dark;
        let events = self
            .emulator
            .events(self.viewport, |index| theme_colour(dark, index));
        for event in &events {
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
    /// (xterm.js's `prepareTextForTerminal`), between the brackets of a
    /// bracketed paste when the program asked for them, and never
    /// anything that would end the brackets early.
    pub fn pasted(&self, text: &str) -> String {
        let text = text
            .replace("\x1b[201~", "")
            .replace("\r\n", "\r")
            .replace('\n', "\r");
        if !text.is_empty() && self.mode().contains(TermMode::BRACKETED_PASTE) {
            format!("\x1b[200~{text}\x1b[201~")
        } else {
            text
        }
    }

    fn query_with(&mut self, change: impl FnOnce(&mut Query)) {
        let mut query = self.search.query.clone();
        change(&mut query);
        self.search.set(query);
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

    pub fn update(&mut self, message: Message) -> (Task<Message>, Option<Event>) {
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
            Message::Deselect => {
                self.emulator.term.selection = None;
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
                self.read_unread();
                if self.search.is_active() {
                    self.search.refresh(&self.emulator.term);
                }
                // What the program asked meanwhile: its answers as one
                // input; a title is kept (`title()`), a bell is let go.
                let answers: String = self
                    .events()
                    .into_iter()
                    .filter_map(|event| match event {
                        Event::Input(text) => Some(text),
                        _ => None,
                    })
                    .collect();
                (!answers.is_empty()).then_some(Event::Input(answers))
            }
            Message::Open(url) => Some(Event::Open(url)),
            Message::OpenSearch => {
                self.search.open = true;
                // What is selected is what is looked for, as in a browser.
                if let Some(text) = self.selection_text().filter(|t| !t.contains('\n')) {
                    self.query_with(|query| query.text = text);
                }
                return (operation::focus(self.search.id.clone()), None);
            }
            Message::CloseSearch => {
                self.search.open = false;
                return (operation::focus(self.id.clone()), None);
            }
            Message::Query(text) => {
                self.query_with(|query| query.text = text);
                None
            }
            Message::Case(case) => {
                self.query_with(|query| query.case = case);
                None
            }
            Message::Regex(regex) => {
                self.query_with(|query| query.regex = regex);
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
        (Task::none(), event)
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
