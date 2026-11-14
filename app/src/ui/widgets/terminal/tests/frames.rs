//! The grid's own frames, with nothing sent by hand: output that came
//! faster than it is read is asked for frame after frame until none
//! waits, and then no frame asks to be drawn again.

use iced::window::RedrawRequest;

use super::{perf::Bench, screen};
use crate::ui::{
    theme::Theme,
    widgets::terminal::{state::READ_AT_ONCE, Message, State},
};

/// A frame drawn as the window draws it: what it said back is handed to
/// the state, as the page does. Whether it asked for a read.
fn frame(bench: &mut Bench, state: &mut State) -> bool {
    let (_, messages) = bench.frame_in(state, &Theme::Light);
    let mut read = false;
    for message in messages {
        read |= matches!(message, Message::Read);
        let _ = state.update(message);
    }
    read
}

#[test]
fn output_that_waits_is_read_frame_by_frame_and_then_nothing_redraws() {
    let mut bench = Bench::new();
    let mut state = State::new();
    for _ in 0..3 {
        let _ = frame(&mut bench, &mut state);
    }
    // A program printed 3 MB and went quiet.
    let lines: String = (0..400_000).map(|i| format!("{i:07}\r\n")).collect();
    assert!(lines.len() > 3 * READ_AT_ONCE);
    let _ = state.feed(&lines);
    assert!(state.is_behind());
    let mut frames = 0;
    while state.is_behind() {
        assert!(frame(&mut bench, &mut state), "a frame asks for what waits");
        assert_eq!(bench.redraw, Some(RedrawRequest::NextFrame));
        frames += 1;
        assert!(frames <= lines.len() / READ_AT_ONCE + 1, "{frames} frames");
    }
    let rows = screen(&state);
    let last = rows.iter().rev().find(|row| !row.is_empty()).cloned();
    assert_eq!(last.as_deref(), Some("0399999"));
    // Nothing waits: the frame after asks for nothing, and no redraw.
    assert!(!frame(&mut bench, &mut state));
    assert_eq!(bench.redraw, Some(RedrawRequest::Wait));
}
