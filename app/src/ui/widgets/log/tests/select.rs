//! What is selected and what copying and saving give.

use super::{feed, numbered, viewer};
use crate::{
    test_support::tasks::{is_none, messages},
    ui::widgets::log::{Event, Line, Mark, Message, State, Stream},
};

/// What a message gave for the page to act on, when it started no task.
fn tell(state: &mut State, message: Message) -> Option<Event> {
    let (task, event) = state.update(message);
    assert!(is_none(task));
    event
}

fn copied(state: &mut State) -> String {
    match tell(state, Message::Copy) {
        Some(Event::Copy(text)) => text,
        other => panic!("nothing was copied: {other:?}"),
    }
}

fn drag(state: &mut State, from: Mark, to: Mark) {
    tell(
        state,
        Message::Press {
            at: from,
            extend: false,
        },
    );
    tell(state, Message::Drag(to));
}

#[test]
fn a_drag_selects_from_where_it_started_to_where_it_is() {
    let mut state = viewer(100);
    feed(&mut state, numbered(0, 5));
    assert!(
        tell(&mut state, Message::Copy).is_none(),
        "nothing selected"
    );
    drag(&mut state, Mark::new(1, 5), Mark::new(1, 6));
    assert_eq!(copied(&mut state), "1");
    // Over lines: the end of the first, the lines between, the start of
    // the last; one under the other.
    drag(&mut state, Mark::new(1, 2), Mark::new(3, 4));
    assert_eq!(copied(&mut state), "ne 1\nline 2\nline");
    // Backwards is the same selection.
    drag(&mut state, Mark::new(3, 4), Mark::new(1, 2));
    assert_eq!(copied(&mut state), "ne 1\nline 2\nline");
    // A press alone selects nothing.
    tell(
        &mut state,
        Message::Press {
            at: Mark::new(2, 3),
            extend: false,
        },
    );
    assert!(tell(&mut state, Message::Copy).is_none());
    assert!(!state.has_selection());
    drag(&mut state, Mark::new(1, 2), Mark::new(3, 4));
    assert!(state.has_selection());
    tell(&mut state, Message::Deselect);
    assert!(!state.has_selection());
}

#[test]
fn a_press_with_shift_takes_the_selection_from_where_it_started() {
    let mut state = viewer(100);
    feed(&mut state, numbered(0, 5));
    let press = |at, extend| Message::Press { at, extend };
    tell(&mut state, press(Mark::new(1, 0), false));
    tell(&mut state, press(Mark::new(3, 6), true));
    assert_eq!(copied(&mut state), "line 1\nline 2\nline 3");
    // Again, the other way from the same start.
    tell(&mut state, press(Mark::new(0, 5), true));
    assert_eq!(copied(&mut state), "0\n");
    // With nothing selected, Shift starts a selection as a press does.
    tell(&mut state, Message::Deselect);
    tell(&mut state, press(Mark::new(2, 0), true));
    tell(&mut state, press(Mark::new(2, 4), true));
    assert_eq!(copied(&mut state), "line");
}

#[test]
fn everything_a_word_and_a_line() {
    let mut state = viewer(100);
    feed(
        &mut state,
        [
            Line::new("GET /api/v1/users 200"),
            Line::new("second"),
            Line::new(""),
            Line::new("last"),
        ],
    );
    tell(&mut state, Message::SelectAll);
    assert_eq!(
        copied(&mut state),
        "GET /api/v1/users 200\nsecond\n\nlast\n"
    );
    tell(&mut state, Message::SelectWord(Mark::new(0, 8)));
    assert_eq!(copied(&mut state), "/api/v1/users");
    tell(&mut state, Message::SelectLine(1));
    assert_eq!(copied(&mut state), "second\n");
    // Lines that come after "select all" are not in it.
    tell(&mut state, Message::SelectAll);
    feed(&mut state, [Line::new("later")]);
    assert_eq!(
        copied(&mut state),
        "GET /api/v1/users 200\nsecond\n\nlast\n"
    );
    // Nothing to select in an empty viewer.
    let mut empty = viewer(10);
    tell(&mut empty, Message::SelectAll);
    assert!(!empty.has_selection());
}

#[test]
fn what_is_copied_is_what_shows_without_its_colours() {
    let mut state = viewer(100);
    feed(
        &mut state,
        [
            Line::new("2026-05-01T10:00:00Z \x1b[31mfailed\x1b[0m to bind")
                .source("web.1")
                .timestamped(true),
            Line::new("2026-05-01T10:00:01Z ready")
                .source("worker.12")
                .timestamped(true),
            Line::new("日本語 ok"),
        ],
    );
    state.end();
    tell(&mut state, Message::SelectAll);
    // The labels are padded as they show; what the viewer says itself
    // ("end of logs") is not the log's.
    assert_eq!(
        copied(&mut state),
        "web.1     | failed to bind\nworker.12 | ready\n日本語 ok\n"
    );
    tell(&mut state, Message::Timestamps(true));
    tell(&mut state, Message::SelectAll);
    assert_eq!(
        copied(&mut state),
        "2026-05-01T10:00:00Z web.1     | failed to bind\n\
         2026-05-01T10:00:01Z worker.12 | ready\n日本語 ok\n"
    );
    // Cells, not bytes: the third cell of a wide character's line.
    drag(&mut state, Mark::new(2, 2), Mark::new(2, 9));
    assert_eq!(copied(&mut state), "本語 ok");
}

#[test]
fn filtered_only_the_lines_that_show_are_copied() {
    let mut state = viewer(100);
    feed(&mut state, numbered(0, 30));
    tell(&mut state, Message::Query("line 1".to_owned()));
    tell(&mut state, Message::Filter(true));
    tell(&mut state, Message::SelectAll);
    let text = copied(&mut state);
    assert_eq!(text.lines().count(), 11);
    assert!(text.starts_with("line 1\nline 10\nline 11\n"));
    drag(&mut state, Mark::new(12, 0), Mark::new(15, 7));
    assert_eq!(copied(&mut state), "line 12\nline 13\nline 14\nline 15");
}

#[test]
fn a_selection_lets_go_of_the_lines_that_are_dropped() {
    let mut state = viewer(10);
    feed(&mut state, numbered(0, 10));
    drag(&mut state, Mark::new(2, 3), Mark::new(6, 4));
    // Lines 0 to 4 are dropped: what is left of the selection starts
    // with the oldest line kept.
    feed(&mut state, numbered(10, 5));
    assert_eq!(copied(&mut state), "line 5\nline");
    // Dropped whole, it is gone.
    feed(&mut state, numbered(15, 5));
    assert!(!state.has_selection());
}

#[tokio::test]
async fn a_large_selection_is_copied_on_another_thread() {
    let mut state = viewer(100_000);
    feed(&mut state, numbered(0, 50_000));
    tell(&mut state, Message::SelectAll);
    let (task, event) = state.update(Message::Copy);
    assert!(event.is_none(), "not yet: update does not wait for it");
    let mut answers = messages(task).await;
    assert_eq!(answers.len(), 1);
    let Some(Event::Copy(text)) = tell(&mut state, answers.remove(0)) else {
        panic!("the text never came");
    };
    assert_eq!(text.lines().count(), 50_000);
    assert!(text.ends_with("line 49999\n"));
}

#[test]
fn the_lines_to_save_are_the_text_after_its_source() {
    let mut state = viewer(100);
    feed(
        &mut state,
        [
            Line::new("2026-05-01T10:00:00Z \x1b[1mlistening\x1b[0m on :80")
                .source("web.1")
                .timestamped(true),
            Line::new("2026-05-01T10:00:01Z warning: slow")
                .source("db.1")
                .stream(Stream::Stderr)
                .timestamped(true),
            Line::new("no source"),
        ],
    );
    state.skipped(3);
    state.end();
    let Some(Event::Download(lines)) = tell(&mut state, Message::Download) else {
        panic!("nothing to save");
    };
    // As the React viewer kept them: `source | text`, no padding, and
    // nothing of what the viewer says itself.
    let saved: Vec<String> = lines.lines().collect();
    assert_eq!(
        saved,
        [
            "web.1 | listening on :80",
            "db.1 | warning: slow",
            "no source"
        ]
    );
    assert_eq!(lines.len(), 3);
    // What was handed over is as it was, whatever the viewer does next.
    tell(&mut state, Message::Clear);
    let mut file = vec![];
    lines.write_to(&mut file).expect("written");
    assert_eq!(
        String::from_utf8(file).expect("text"),
        "web.1 | listening on :80\ndb.1 | warning: slow\nno source\n"
    );
    // With timestamps showing, they are saved too.
    let mut stamped = State::new(10).with_timestamps(true);
    feed(
        &mut stamped,
        [Line::new("2026-05-01T10:00:00Z up").timestamped(true)],
    );
    let Some(Event::Download(lines)) = tell(&mut stamped, Message::Download) else {
        panic!("nothing to save");
    };
    assert_eq!(
        lines.lines().collect::<Vec<_>>(),
        ["2026-05-01T10:00:00Z up"]
    );
}
