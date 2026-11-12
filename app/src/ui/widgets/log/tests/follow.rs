//! Where the view is as lines come and go: it follows the latest, stays
//! put once the user scrolled up, and comes back when asked.

use super::{feed, numbered, shown, viewer};
use crate::ui::widgets::log::{Line, Message, Metrics, Scroll};

fn first_shown(state: &crate::ui::widgets::log::State) -> String {
    shown(state).first().cloned().unwrap_or_default()
}

fn scroll(state: &mut crate::ui::widgets::log::State, scroll: Scroll) {
    let _ = state.update(Message::Scroll(scroll));
}

#[test]
fn the_view_follows_the_latest_line_as_lines_come() {
    let mut state = viewer(1000);
    assert!(state.follows());
    feed(&mut state, numbered(0, 4));
    assert_eq!(shown(&state), ["line 0", "line 1", "line 2", "line 3"]);
    feed(&mut state, numbered(4, 30));
    assert_eq!(shown(&state).len(), 10);
    assert_eq!(first_shown(&state), "line 24");
    assert_eq!(shown(&state).last().map(String::as_str), Some("line 33"));
    feed(&mut state, numbered(34, 500));
    assert_eq!(shown(&state).last().map(String::as_str), Some("line 533"));
}

#[test]
fn scrolled_up_the_view_stays_where_it_is_while_lines_come() {
    let mut state = viewer(1000);
    feed(&mut state, numbered(0, 100));
    scroll(&mut state, Scroll::Rows(-3));
    assert!(!state.follows(), "scrolling up pauses");
    let before = shown(&state);
    assert_eq!(before[0], "line 87");
    for batch in 0..20 {
        feed(&mut state, numbered(100 + batch * 40, 40));
        assert_eq!(shown(&state), before, "after batch {batch}");
    }
    assert!(!state.follows());
}

#[test]
fn back_at_the_latest_line_the_view_follows_again() {
    let mut state = viewer(1000);
    feed(&mut state, numbered(0, 100));
    scroll(&mut state, Scroll::Pages(-2));
    assert_eq!(first_shown(&state), "line 72");
    scroll(&mut state, Scroll::Rows(5));
    assert!(!state.follows(), "not at the end yet");
    scroll(&mut state, Scroll::Rows(50));
    assert!(state.follows(), "scrolled down to the end");
    feed(&mut state, numbered(100, 10));
    assert_eq!(shown(&state).last().map(String::as_str), Some("line 109"));

    // End, the Follow toggle and "Jump to latest" all come back too.
    for back in [
        Message::Scroll(Scroll::End),
        Message::Follow(true),
        Message::Scroll(Scroll::To(1.0)),
    ] {
        scroll(&mut state, Scroll::Start);
        assert_eq!(first_shown(&state), "line 0");
        assert!(!state.follows());
        let _ = state.update(back);
        assert!(state.follows());
        assert_eq!(shown(&state).last().map(String::as_str), Some("line 109"));
    }
}

#[test]
fn the_follow_toggle_pauses_where_the_view_is() {
    let mut state = viewer(1000);
    feed(&mut state, numbered(0, 50));
    let _ = state.update(Message::Follow(false));
    let before = shown(&state);
    assert_eq!(before[0], "line 40");
    feed(&mut state, numbered(50, 50));
    assert_eq!(shown(&state), before);
}

#[test]
fn paused_the_view_does_not_move_while_the_oldest_lines_are_dropped() {
    let mut state = viewer(100);
    feed(&mut state, numbered(0, 100));
    scroll(&mut state, Scroll::Rows(-30));
    let before = shown(&state);
    assert_eq!(before[0], "line 60");
    // Fifty more: lines 0 to 49 are dropped, above what shows.
    for batch in 0..5 {
        feed(&mut state, numbered(100 + batch * 10, 10));
        assert_eq!(shown(&state), before, "after batch {batch}");
        assert_eq!(state.len(), 100);
    }
    assert_eq!(state.buffer.range(), 50..150);
    assert_eq!(state.anchor().seq, 60, "still on its line");
    // Once the line the view starts on is dropped itself, the view is
    // on the oldest line kept, and goes on with it.
    feed(&mut state, numbered(150, 30));
    assert_eq!(state.buffer.range(), 80..180);
    assert_eq!(first_shown(&state), "line 80");
    assert!(!state.follows());
    feed(&mut state, numbered(180, 5));
    assert_eq!(first_shown(&state), "line 85");
}

#[test]
fn a_short_log_shows_whole_and_scrolling_it_changes_nothing() {
    let mut state = viewer(100);
    feed(&mut state, numbered(0, 3));
    scroll(&mut state, Scroll::Rows(-5));
    assert!(state.follows(), "nothing to scroll: still following");
    scroll(&mut state, Scroll::Start);
    assert!(state.follows());
    assert_eq!(shown(&state), ["line 0", "line 1", "line 2"]);
}

#[test]
fn wrapped_lines_take_the_rows_their_cells_need() {
    let mut state = viewer(1000);
    let _ = state.update(Message::Viewport(Metrics { cols: 20, rows: 6 }));
    // 50 cells in 20: three rows; 20 cells: one; nothing: one.
    feed(
        &mut state,
        [
            Line::new("a".repeat(50)),
            Line::new("b".repeat(20)),
            Line::new(""),
            Line::new("c".repeat(45)),
        ],
    );
    let rows: Vec<usize> = (0..4).map(|index| state.rows_of(index, 20)).collect();
    assert_eq!(rows, [3, 1, 1, 3]);
    // Eight rows in all, six in view: the view starts on the third row
    // of the first line.
    let top = state.window(state.metrics).top;
    assert_eq!((top.index, top.row), (0, 2));
    // A row up, and it is paused on the second row of that line; adding
    // lines leaves it there.
    scroll(&mut state, Scroll::Rows(-1));
    assert_eq!(state.anchor().row, 1);
    feed(&mut state, numbered(0, 10));
    let top = state.window(state.metrics).top;
    assert_eq!((top.index, top.row), (0, 1));
    // Not wrapped, every line is a row, and the line at the top stays.
    let _ = state.update(Message::Wrap(false));
    assert!(!state.wraps());
    assert_eq!(state.rows_of(0, 20), 1);
    assert_eq!(state.window(state.metrics).top.index, 0);
    // A narrower viewer wraps the same lines onto more rows.
    let _ = state.update(Message::Wrap(true));
    assert_eq!(state.rows_of(0, 10), 5);
}

#[test]
fn a_million_lines_are_no_further_than_ten() {
    let mut state = viewer(1_000_000);
    feed(&mut state, numbered(0, 1_000_000));
    assert_eq!(
        shown(&state).last().map(String::as_str),
        Some("line 999999")
    );
    scroll(&mut state, Scroll::To(0.5));
    assert_eq!(first_shown(&state), "line 499995");
    scroll(&mut state, Scroll::Pages(1));
    assert_eq!(first_shown(&state), "line 500004");
    scroll(&mut state, Scroll::Start);
    assert_eq!(first_shown(&state), "line 0");
    // Wrapped as well: from the end, without a walk through the lines.
    let _ = state.update(Message::Viewport(Metrics { cols: 5, rows: 10 }));
    let _ = state.update(Message::Scroll(Scroll::End));
    let top = state.window(state.metrics).top;
    // Eleven cells in five: three rows a line, so three lines and a row.
    assert_eq!((top.index, top.row), (999_996, 2));
}
