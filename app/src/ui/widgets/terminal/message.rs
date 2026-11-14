//! What a terminal hears and says: the widget's [`Message`]s, which the
//! page hands to `State::update`, and the [`Event`]s that come back for
//! the session and the page.

use alacritty_terminal::index::{Point, Side};

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
