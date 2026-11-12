//! What a search finds: as it is typed, with or without capitals, as a
//! regular expression, among lines that come and go, and without `update`
//! waiting for a large buffer.

use super::{drive, feed, numbered, shown, viewer};
use crate::{
    test_support::tasks::is_none,
    ui::widgets::log::{Line, Message, Scroll, State},
};

/// Types a query. Plain text over a few lines is read at once; a regular
/// expression is built by a task, run here to its end.
fn type_in(state: &mut State, text: &str) {
    if state.query().regex {
        drive(state, Message::Query(text.to_owned()));
        return;
    }
    let (task, _) = state.update(Message::Query(text.to_owned()));
    assert!(is_none(task), "few lines are read at once");
}

fn tell(state: &mut State, message: Message) {
    drive(state, message);
}

/// Six lines, three of which say "error" one way or another.
fn errors() -> State {
    let mut state = viewer(1000);
    feed(
        &mut state,
        [
            Line::new("listening on :80"),
            Line::new("ERROR failed to bind: error 98"),
            Line::new("retrying"),
            Line::new("Error: address in use"),
            Line::new("ok"),
            Line::new("no errors at all"),
        ],
    );
    state
}

#[test]
fn matches_are_found_as_the_query_is_typed() {
    let mut state = errors();
    assert_eq!(state.matches(), (None, 0));
    for (typed, count) in [("e", 9), ("er", 4), ("err", 4), ("erro", 4), ("error", 4)] {
        type_in(&mut state, typed);
        assert_eq!(state.matches(), (None, count), "{typed:?}");
    }
    type_in(&mut state, "error 9");
    assert_eq!(state.matches().1, 1);
    type_in(&mut state, "nothing like it");
    assert_eq!(state.matches().1, 0);
    assert!(!state.is_searching());
    // An empty field looks for nothing.
    type_in(&mut state, "");
    assert_eq!(state.matches(), (None, 0));
}

#[test]
fn capitals_matter_only_when_asked() {
    let mut state = errors();
    type_in(&mut state, "ERROR");
    assert_eq!(
        state.matches().1,
        4,
        "as the React search: whatever the case"
    );
    tell(&mut state, Message::Case(true));
    assert_eq!(state.matches().1, 1);
    type_in(&mut state, "Error");
    assert_eq!(state.matches().1, 1);
    tell(&mut state, Message::Case(false));
    assert_eq!(state.matches().1, 4);
}

#[test]
fn a_query_is_text_until_it_is_asked_to_be_a_regular_expression() {
    let mut state = errors();
    type_in(&mut state, "err.r");
    assert_eq!(state.matches().1, 0, "a dot is a dot");
    type_in(&mut state, ":80");
    assert_eq!(state.matches().1, 1);
    type_in(&mut state, "(");
    assert_eq!(state.matches().1, 0);
    assert_eq!(
        state.search_error(),
        None,
        "text is never a wrong expression"
    );

    tell(&mut state, Message::Regex(true));
    type_in(&mut state, "err.r");
    assert_eq!(state.matches().1, 4);
    type_in(&mut state, r"^error|\d+$");
    assert_eq!(state.matches().1, 4, "two at line starts, 98 and 80");
    tell(&mut state, Message::Case(true));
    assert_eq!(state.matches().1, 2);
    // What matches nothing at all is not a match on every line.
    type_in(&mut state, "x*");
    assert_eq!(state.matches().1, 0);
}

#[test]
fn a_regular_expression_that_cannot_be_read_is_said_and_nothing_breaks() {
    let mut state = errors();
    tell(&mut state, Message::Regex(true));
    for wrong in ["(", "[a-", "a{2,1}", "\\", "(?P<n>x)(?P<n>y)", "*"] {
        type_in(&mut state, wrong);
        assert!(state.search_error().is_some(), "{wrong:?}");
        assert_eq!(state.matches(), (None, 0));
        // Stepping, filtering and new lines go on as if nothing was typed.
        tell(&mut state, Message::Next);
        tell(&mut state, Message::Previous);
        tell(&mut state, Message::Filter(true));
        assert_eq!(
            state.count(),
            state.len(),
            "an unreadable query hides no line"
        );
        feed(&mut state, [Line::new("one more")]);
        tell(&mut state, Message::Filter(false));
    }
    // Typing on mends it.
    type_in(&mut state, "(error)");
    assert_eq!(state.search_error(), None);
    assert!(state.matches().1 > 0);
    // As text, the same characters were never wrong.
    tell(&mut state, Message::Regex(false));
    type_in(&mut state, "[a-");
    assert_eq!(state.search_error(), None);
}

#[test]
fn next_and_previous_go_through_the_matches_and_round() {
    let mut state = errors();
    type_in(&mut state, "error");
    // The first step is the first match from where the view starts.
    tell(&mut state, Message::Next);
    assert_eq!(state.matches(), (Some(1), 4));
    tell(&mut state, Message::Next);
    assert_eq!(
        state.matches().0,
        Some(2),
        "the second match of the same line"
    );
    tell(&mut state, Message::Next);
    tell(&mut state, Message::Next);
    assert_eq!(state.matches().0, Some(4));
    tell(&mut state, Message::Next);
    assert_eq!(state.matches().0, Some(1), "round to the first");
    tell(&mut state, Message::Previous);
    assert_eq!(state.matches().0, Some(4), "and back round to the last");
    tell(&mut state, Message::Previous);
    assert_eq!(state.matches().0, Some(3));
    // A new query starts over.
    type_in(&mut state, "retry");
    assert_eq!(state.matches(), (None, 1));
    // Going back first: the last match that shows.
    tell(&mut state, Message::Previous);
    assert_eq!(state.matches(), (Some(1), 1));
}

#[test]
fn stepping_to_a_match_out_of_view_brings_it_there_and_stops_following() {
    let mut state = viewer(10_000);
    feed(&mut state, numbered(0, 500));
    feed(&mut state, [Line::new("needle one")]);
    feed(&mut state, numbered(501, 1000));
    feed(&mut state, [Line::new("needle two")]);
    feed(&mut state, numbered(1502, 300));
    type_in(&mut state, "needle");
    assert_eq!(state.matches(), (None, 2));
    assert!(state.follows());
    // Back from the latest line: the match nearest above it.
    tell(&mut state, Message::Previous);
    assert_eq!(state.matches().0, Some(2));
    assert!(
        !state.follows(),
        "the view is on the match, not on the latest"
    );
    assert!(shown(&state).contains(&"needle two".to_owned()));
    tell(&mut state, Message::Previous);
    assert_eq!(state.matches().0, Some(1));
    assert!(shown(&state).contains(&"needle one".to_owned()));
    // A match already in view moves nothing.
    let before = shown(&state);
    tell(&mut state, Message::Next);
    tell(&mut state, Message::Previous);
    assert_eq!(shown(&state), before);
    // Lines that come leave the view on the match.
    feed(&mut state, numbered(4000, 100));
    assert_eq!(shown(&state), before);
}

#[test]
fn a_match_far_along_a_line_that_is_not_wrapped_is_scrolled_to() {
    let mut state = viewer(100);
    tell(&mut state, Message::Wrap(false));
    feed(
        &mut state,
        [Line::new(format!("{}needle", " ".repeat(300)))],
    );
    type_in(&mut state, "needle");
    assert_eq!(state.across, 0);
    tell(&mut state, Message::Next);
    assert!(
        (state.across..state.across + 80).contains(&300),
        "{}",
        state.across
    );
}

#[test]
fn lines_that_come_are_searched_and_lines_dropped_are_forgotten() {
    // Every other line says "hit".
    let lines = |from: usize, count: usize| {
        (from..from + count)
            .map(|n| Line::new(format!("{} {n}", if n % 2 == 0 { "hit" } else { "miss" })))
    };
    let mut state = viewer(10);
    feed(&mut state, lines(0, 10));
    type_in(&mut state, "hit");
    assert_eq!(state.matches().1, 5);
    feed(&mut state, lines(10, 5));
    // Lines 5 to 14 are kept: 6, 8, 10, 12 and 14 match.
    assert_eq!(state.matches().1, 5);
    tell(&mut state, Message::Scroll(Scroll::Start));
    tell(&mut state, Message::Next);
    tell(&mut state, Message::Next);
    assert_eq!(state.matches(), (Some(2), 5), "on line 8");
    // Two more lines drop line 6: the search is still on line 8, which
    // is now the first match.
    feed(&mut state, lines(15, 2));
    assert_eq!(state.matches(), (Some(1), 5));
    // Three more drop line 8 itself: the search is on no match.
    feed(&mut state, lines(17, 3));
    assert_eq!(state.matches(), (None, 5));
    assert_eq!(state.search.hits.front().map(|hit| hit.seq), Some(10));
}

#[test]
fn filtered_only_the_lines_that_match_show_and_the_view_keeps_its_place() {
    let mut state = viewer(10_000);
    feed(&mut state, numbered(0, 2000));
    tell(&mut state, Message::Regex(true));
    type_in(&mut state, "line \\d*00$");
    assert_eq!(state.matches().1, 19);
    tell(&mut state, Message::Filter(true));
    assert!(state.filters());
    assert_eq!(shown(&state).len(), 10);
    assert_eq!(shown(&state).last().map(String::as_str), Some("line 1900"));
    // Paused in the middle, then unfiltered: the line at the top stays.
    tell(&mut state, Message::Scroll(Scroll::Rows(-5)));
    assert_eq!(shown(&state)[0], "line 500");
    tell(&mut state, Message::Filter(false));
    assert_eq!(shown(&state)[0], "line 500");
    assert_eq!(shown(&state)[1], "line 501");
    // Filtered again from a line that does not match: the next that does.
    tell(&mut state, Message::Scroll(Scroll::Rows(5)));
    tell(&mut state, Message::Filter(true));
    assert_eq!(shown(&state)[0], "line 600");
    // New matches join at the end without moving the view.
    feed(&mut state, numbered(2000, 301));
    assert_eq!(shown(&state)[0], "line 600");
    assert_eq!(state.matches().1, 23);
    // With nothing typed, the filter hides nothing.
    type_in(&mut state, "");
    assert_eq!(state.count(), 2301);
}

#[test]
fn timestamps_are_searched_only_while_they_show() {
    let mut state = State::new(100);
    feed(
        &mut state,
        [Line::new("2026-05-01T10:00:00Z started in 2026").timestamped(true)],
    );
    type_in(&mut state, "2026");
    assert_eq!(state.matches().1, 1, "the one in the message");
    let (task, _) = state.update(Message::Timestamps(true));
    assert!(is_none(task));
    assert_eq!(state.matches().1, 2);
}

#[test]
fn what_the_viewer_says_between_the_lines_still_shows_when_filtered() {
    let mut state = viewer(1000);
    feed(&mut state, numbered(0, 30));
    state.skipped(1234);
    feed(&mut state, numbered(30, 30));
    state.end();
    type_in(&mut state, "line 5");
    tell(&mut state, Message::Filter(true));
    // "line 5" and 50 to 59, with the two things the viewer says.
    assert_eq!(state.matches().1, 11);
    assert_eq!(state.count(), 13);
    let markers: Vec<u64> = (0..state.count())
        .filter_map(|index| state.seq_at(index))
        .filter(|seq| {
            state
                .buffer
                .line(*seq)
                .is_some_and(|line| line.marker.is_some())
        })
        .collect();
    assert_eq!(markers, [30, 61]);
    // They are no match: stepping goes past them, round and back.
    tell(&mut state, Message::Scroll(Scroll::Start));
    for expected in [1, 2, 3] {
        tell(&mut state, Message::Next);
        assert_eq!(state.matches().0, Some(expected));
    }
    for _ in 0..9 {
        tell(&mut state, Message::Next);
    }
    assert_eq!(state.matches().0, Some(1), "round past the end of logs");
    tell(&mut state, Message::Previous);
    assert_eq!(state.matches().0, Some(11));
    // One said after the search read everything shows too.
    state.skipped(7);
    assert_eq!(state.count(), 14);
    // With only what the viewer says and no match, there is nothing to step to.
    type_in(&mut state, "no such text");
    tell(&mut state, Message::Next);
    assert_eq!(state.matches(), (None, 0));
    assert_eq!(state.count(), 3);
}
