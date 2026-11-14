//! Selecting with the pointer, as cells, words and lines, across the
//! scrollback; what a copy gives.

use alacritty_terminal::index::{Column, Line, Point, Side};

use super::sized;
use crate::ui::widgets::terminal::{Click, Event, Message, State};

fn at(line: i32, col: usize) -> Point {
    Point::new(Line(line), Column(col))
}

fn press(state: &mut State, point: Point, click: Click, extend: bool) {
    let _ = state.update(Message::Press {
        at: point,
        side: Side::Left,
        click,
        extend,
    });
}

fn copied(state: &mut State) -> Option<String> {
    match state.update(Message::Copy).1 {
        Some(Event::Copy(text)) => Some(text),
        None => None,
        other => panic!("a copy gave {other:?}"),
    }
}

fn filled() -> State {
    let mut state = sized(30, 6);
    let _ = state.feed("first line here\r\nsecond: some-word/path.txt end\r\nthird");
    state
}

#[test]
fn a_drag_selects_the_cells_it_crossed() {
    let mut state = filled();
    press(&mut state, at(0, 6), Click::Single, false);
    assert!(!state.has_selection(), "a press alone selects nothing");
    let _ = state.update(Message::Drag {
        at: at(1, 5),
        side: Side::Right,
    });
    assert!(state.has_selection());
    assert_eq!(copied(&mut state).as_deref(), Some("line here\nsecond"));
}

#[test]
fn shift_with_a_press_takes_the_selection_further() {
    let mut state = filled();
    press(&mut state, at(0, 0), Click::Single, false);
    let _ = state.update(Message::Drag {
        at: at(0, 4),
        side: Side::Right,
    });
    let _ = state.update(Message::Press {
        at: at(2, 2),
        side: Side::Right,
        click: Click::Single,
        extend: true,
    });
    assert_eq!(
        copied(&mut state).as_deref(),
        Some("first line here\nsecond: some-word/path.txt end\nthi")
    );
}

#[test]
fn two_presses_take_a_word_and_three_a_line() {
    let mut state = filled();
    press(&mut state, at(1, 10), Click::Double, false);
    // A path is one word: what ends a word is a space or a bracket.
    assert_eq!(copied(&mut state).as_deref(), Some("some-word/path.txt"));
    press(&mut state, at(1, 2), Click::Triple, false);
    assert_eq!(
        copied(&mut state).as_deref(),
        Some("second: some-word/path.txt end\n")
    );
}

#[test]
fn a_line_that_wrapped_is_copied_as_one() {
    let mut state = sized(10, 4);
    let _ = state.feed("0123456789abcdefghij\r\nnext");
    press(&mut state, at(0, 0), Click::Triple, false);
    assert_eq!(
        copied(&mut state).as_deref(),
        Some("0123456789abcdefghij\n")
    );
}

#[test]
fn select_all_takes_the_scrollback_and_escape_deselects() {
    let mut state = sized(20, 3);
    let lines: String = (0..10).map(|i| format!("line {i}\r\n")).collect();
    let _ = state.feed(&lines);
    let _ = state.update(Message::SelectAll);
    let all = copied(&mut state).unwrap();
    assert!(all.starts_with("line 0\nline 1\n"));
    assert!(all.trim_end().ends_with("line 9"));
    let _ = state.update(Message::Deselect);
    assert!(!state.has_selection());
    assert_eq!(copied(&mut state), None);
}

#[test]
fn the_selection_moves_with_its_text_when_new_lines_come() {
    let mut state = sized(20, 3);
    let _ = state.feed("keep me\r\n");
    press(&mut state, at(0, 0), Click::Triple, false);
    let _ = state.feed("a\r\nb\r\nc\r\nd\r\n");
    assert_eq!(copied(&mut state).as_deref(), Some("keep me\n"));
}

#[test]
fn copying_a_wide_character_copies_it_once() {
    let mut state = sized(20, 3);
    let _ = state.feed("ab日本cd");
    press(&mut state, at(0, 0), Click::Single, false);
    let _ = state.update(Message::Drag {
        at: at(0, 7),
        side: Side::Right,
    });
    assert_eq!(copied(&mut state).as_deref(), Some("ab日本cd"));
}
