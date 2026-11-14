//! The emulation itself: `alacritty_terminal`'s grid and parser, fed the
//! bytes of a session. What the emulation asks of the outside (an answer
//! to a query, a title, the bell) is gathered while the bytes are read
//! and handed to the page afterwards as [`Event`]s.
//!
//! The pty and the event loop `alacritty_terminal` comes with are not
//! used: a terminal of the app reads a stream, never a process of this
//! computer.

use std::sync::{Arc, Mutex};

use alacritty_terminal::{
    event::{Event as Asked, EventListener, WindowSize},
    grid::Dimensions,
    term::{Config, Osc52},
    vte::ansi::{CursorShape, CursorStyle, Processor, Rgb},
    Term,
};

use super::state::{Event, Viewport};

/// The least and most a grid is: a terminal shrunk to nothing still has a
/// cell, and none is wider than a page holds.
pub const MIN_CELLS: usize = 2;
pub const MAX_CELLS: usize = 1_000;

/// The size of the grid, as `alacritty_terminal` asks for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub cols: usize,
    pub rows: usize,
}

impl Size {
    pub fn clamped(cols: usize, rows: usize) -> Self {
        Self {
            cols: cols.clamp(MIN_CELLS, MAX_CELLS),
            rows: rows.clamp(MIN_CELLS, MAX_CELLS),
        }
    }
}

impl Dimensions for Size {
    fn total_lines(&self) -> usize {
        self.rows
    }

    fn screen_lines(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.cols
    }
}

/// What the emulation asked for while it read, kept until it is handed on.
#[derive(Clone, Default)]
pub struct Listener(Arc<Mutex<Vec<Asked>>>);

impl EventListener for Listener {
    fn send_event(&self, event: Asked) {
        if let Ok(mut asked) = self.0.lock() {
            asked.push(event);
        }
    }
}

impl Listener {
    fn take(&self) -> Vec<Asked> {
        self.0
            .lock()
            .map(|mut asked| std::mem::take(&mut *asked))
            .unwrap_or_default()
    }
}

/// The grid, its scrollback and the parser that writes into them.
pub struct Emulator {
    pub term: Term<Listener>,
    parser: Processor,
    listener: Listener,
}

impl std::fmt::Debug for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emulator")
            .field("cols", &self.term.columns())
            .field("rows", &self.term.screen_lines())
            .finish_non_exhaustive()
    }
}

/// How the terminal is configured: what a program can't change.
fn config(scrollback: usize) -> Config {
    Config {
        scrolling_history: scrollback,
        default_cursor_style: CursorStyle {
            shape: CursorShape::Block,
            blinking: true,
        },
        // A program in a container never reads or writes this computer's
        // clipboard: the user copies and pastes.
        osc52: Osc52::Disabled,
        ..Config::default()
    }
}

impl Emulator {
    pub fn new(size: Size, scrollback: usize) -> Self {
        let listener = Listener::default();
        Self {
            term: Term::new(config(scrollback), &size, listener.clone()),
            parser: Processor::new(),
            listener,
        }
    }

    /// Reads `bytes` into the grid.
    pub fn advance(&mut self, bytes: &[u8]) {
        self.parser.advance(&mut self.term, bytes);
        self.settle_sync();
    }

    /// A synchronised update (`CSI ? 2026 h`) that ran past its time is
    /// drawn as it is, as a terminal does when its program stalls.
    pub fn settle_sync(&mut self) {
        let expired = self
            .parser
            .sync_timeout()
            .sync_timeout()
            .is_some_and(|until| until <= std::time::Instant::now());
        if expired {
            self.parser.stop_sync(&mut self.term);
        }
    }

    /// Until when a synchronised update holds what was read back, if one
    /// does: the grid asks to be told again then.
    pub fn holding_until(&self) -> Option<std::time::Instant> {
        self.parser.sync_timeout().sync_timeout()
    }

    /// Draws what a synchronised update holds now: the app writes a line
    /// of its own after it.
    pub fn release(&mut self) {
        if self.holding_until().is_some() {
            self.parser.stop_sync(&mut self.term);
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.term.resize(size);
    }

    pub fn size(&self) -> Size {
        Size {
            cols: self.term.columns(),
            rows: self.term.screen_lines(),
        }
    }

    /// What the emulation asked for since the last call, as events for
    /// the page: answers go back to the session as input.
    pub fn events(&self, viewport: Viewport, colours: impl Fn(usize) -> Rgb) -> Vec<Event> {
        let mut events = vec![];
        for asked in self.listener.take() {
            match asked {
                Asked::PtyWrite(text) => events.push(Event::Input(text)),
                Asked::Title(title) => events.push(Event::Title(Some(title))),
                Asked::ResetTitle => events.push(Event::Title(None)),
                Asked::Bell => events.push(Event::Bell),
                Asked::ColorRequest(index, answer) => {
                    events.push(Event::Input(answer(colours(index))));
                }
                Asked::TextAreaSizeRequest(answer) => {
                    events.push(Event::Input(answer(window_size(self.size(), viewport))));
                }
                // The clipboard is the user's (OSC 52 is off); the rest is
                // the window's business, not a program's.
                Asked::ClipboardStore(..)
                | Asked::ClipboardLoad(..)
                | Asked::MouseCursorDirty
                | Asked::CursorBlinkingChange
                | Asked::Wakeup
                | Asked::Exit
                | Asked::ChildExit(_) => {}
            }
        }
        events
    }
}

/// The size of the text area, for `CSI 14 t`.
// A grid and its cells are a few thousand pixels at most.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn window_size(size: Size, viewport: Viewport) -> WindowSize {
    let narrow = |value: usize| u16::try_from(value).unwrap_or(u16::MAX);
    WindowSize {
        num_lines: narrow(size.rows),
        num_cols: narrow(size.cols),
        cell_width: viewport.cell_width.round().max(1.0) as u16,
        cell_height: viewport.cell_height.round().max(1.0) as u16,
    }
}
