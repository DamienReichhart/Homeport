//! The terminal's tests: escape sequences against recorded transcripts,
//! the keys, the pointer, the selection, the search, the size, and what
//! it draws.

mod drawn;
mod driven;
mod emulation;
mod keys;
mod perf;
mod search;
mod select;
mod transcripts;

use std::path::PathBuf;

use alacritty_terminal::{
    index::{Column, Line, Point},
    term::cell::Cell,
};

use super::{Event, Message, State, Viewport};

/// A recorded transcript: the bytes a program wrote to its terminal,
/// under `app/tests/fixtures/terminal`.
pub(super) fn fixture(name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/terminal")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// A terminal of `cols` by `rows`, as after its first layout.
pub(super) fn sized(cols: usize, rows: usize) -> State {
    let mut state = State::new();
    let (_, resized) = state.update(Message::Viewport(Viewport {
        cols,
        rows,
        cell_width: 7.2,
        cell_height: 16.0,
    }));
    assert!(matches!(resized, Some(Event::Resize { .. }) | None));
    state
}

/// What the rows of the screen say, their trailing blanks off.
pub(super) fn screen(state: &State) -> Vec<String> {
    let term = state.term();
    let (cols, rows) = state.size();
    (0..rows)
        .map(|row| {
            let line = Line(i32::try_from(row).unwrap());
            let text: String = (0..cols)
                .map(|col| &term.grid()[line][Column(col)])
                .filter(|cell| {
                    !cell
                        .flags
                        .contains(alacritty_terminal::term::cell::Flags::WIDE_CHAR_SPACER)
                })
                .map(|cell| {
                    let mut text = cell.c.to_string();
                    text.extend(cell.zerowidth().unwrap_or_default());
                    text
                })
                .collect();
            text.trim_end().to_owned()
        })
        .collect()
}

/// The cell at `row`, `col` of the screen.
pub(super) fn cell(state: &State, row: i32, col: usize) -> &Cell {
    &state.term().grid()[Point::new(Line(row), Column(col))]
}

/// What the program was answered, joined.
pub(super) fn answers(events: &[Event]) -> String {
    events
        .iter()
        .filter_map(|event| match event {
            Event::Input(text) => Some(text.as_str()),
            _ => None,
        })
        .collect()
}
