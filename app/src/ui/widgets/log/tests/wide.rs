//! Characters that take two cells, or none: where the rows of a line
//! start, and where a match and a selection are among them.

use super::{feed, viewer};
use crate::ui::widgets::log::{
    index::WALKED, line::cells, Event, Line, Mark, Message, Metrics, State,
};

fn narrow(cols: usize) -> State {
    let mut state = viewer(100);
    let _ = state.update(Message::Viewport(Metrics { cols, rows: 10 }));
    state
}

/// The cells each row of the line `seq` shows.
fn rows(state: &State, seq: u64) -> Vec<(usize, usize)> {
    let line = state.buffer.line(seq).expect("kept");
    let starts = state.starts(seq, &line, state.shape(), state.metrics.cols);
    (0..starts.len()).map(|part| starts.range(part)).collect()
}

#[test]
fn a_wide_character_that_would_straddle_the_last_cell_goes_to_the_next_row() {
    let mut state = narrow(10);
    feed(
        &mut state,
        [
            // Nine cells, then a character of two.
            Line::new("aaaaaaaaa日bb"),
            // Wide characters all along, from an odd cell.
            Line::new(format!("a{}", "日".repeat(40))),
            // No wide character: a division, accents or not.
            Line::new("é".repeat(25)),
        ],
    );
    assert_eq!(rows(&state, 0), [(0, 9), (9, 13)]);
    assert_eq!(state.rows_of(0, 10), 2);
    let all = rows(&state, 1);
    assert_eq!(all[0], (0, 9));
    assert_eq!(all[1], (9, 19));
    for (from, to) in &all {
        assert!(to - from <= 10, "a row of {} cells", to - from);
    }
    assert_eq!(all.last().map(|row| row.1), Some(81));
    assert_eq!(state.rows_of(1, 10), all.len());
    assert_eq!(rows(&state, 2), [(0, 10), (10, 20), (20, 25)]);
}

#[test]
fn a_match_and_a_selection_are_where_the_characters_are() {
    let mut state = narrow(80);
    let text = "ok 🚀 done ✅ fin";
    feed(&mut state, [Line::new(text)]);
    let _ = state.update(Message::Query("done".to_owned()));
    {
        let line = state.buffer.line(0).expect("kept");
        assert_eq!(line.cells, 17, "two emoji of two cells each");
        let matches = state.matches_of(0, &line);
        assert_eq!(matches.len(), 1);
        let (start, end) = (matches[0].0 as usize, matches[0].1 as usize);
        let shape = state.shape();
        // After "ok ", the rocket's two cells and a space.
        assert_eq!(state.shown_cell(0, &line, shape, start), 6);
        assert_eq!(state.shown_cell(0, &line, shape, end), 10);
        assert_eq!(cells(&text[..start]), 6);
    }
    // The same cells, selected, are the same text.
    let _ = state.update(Message::Press {
        at: Mark::new(0, 6),
        extend: false,
    });
    let _ = state.update(Message::Drag(Mark::new(0, 10)));
    let (_, event) = state.update(Message::Copy);
    assert!(matches!(event, Some(Event::Copy(text)) if text == "done"));
    // Either cell of a wide character is that character.
    let _ = state.update(Message::SelectWord(Mark::new(0, 12)));
    let (_, event) = state.update(Message::Copy);
    assert!(matches!(event, Some(Event::Copy(text)) if text == "✅"));
}

#[test]
fn a_long_line_is_walked_once_then_only_where_it_is_looked_at() {
    let mut state = narrow(100);
    // A megabyte, not ASCII, with a wide character in it.
    let text = format!("é日{}", "abcdefghi ".repeat(100_000));
    feed(&mut state, [Line::new(text.clone())]);
    let line = state.buffer.line(0).expect("kept");
    let walked = || WALKED.with(std::cell::Cell::get);
    let before = walked();
    let far = line.cells - 50;
    let (byte, cell) = state.byte_of_cell(0, &line, far);
    assert_eq!((byte, cell), (text.len() - 50, far));
    let first = walked() - before;
    assert!(first <= 2 * text.len(), "{first} characters walked");
    // Again, anywhere: from the nearest mark.
    let before = walked();
    for cell in [10, 5_000, 123_456, 999_000] {
        let (byte, at) = state.byte_of_cell(0, &line, cell);
        assert_eq!(at, cell);
        assert_eq!(state.cell_of_byte(0, &line, byte), cell);
    }
    assert!(walked() - before <= 8 * 256, "{} walked", walked() - before);
    // Its rows are listed once for a width.
    let starts = state.starts(0, &line, state.shape(), 100);
    let before = walked();
    let again = state.starts(0, &line, state.shape(), 100);
    assert_eq!(starts.len(), again.len());
    assert_eq!(walked(), before);
}
