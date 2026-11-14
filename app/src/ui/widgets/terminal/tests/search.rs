//! Searching the scrollback and the screen: as text or a regular
//! expression, whatever the case or not, counted, stepped through round
//! the ends, brought into view, and found again as new lines come.

use super::{drive, sized};
use crate::ui::widgets::terminal::{Message, State};

fn searched(state: &mut State, text: &str) {
    let _ = drive(state, Message::OpenSearch);
    let _ = drive(state, Message::Query(text.to_owned()));
}

/// (current, count) as the bar says them.
fn count(state: &State) -> (usize, usize) {
    let (current, total, _) = state.search().counted();
    (current, total)
}

/// The match the search stands on is the `index`th, and shows.
fn assert_in_view(state: &State, index: usize) {
    assert_eq!(state.search().current, Some(index));
    let found = state.search().matches[index].start().line.0;
    let offset = i32::try_from(state.term().grid().display_offset()).unwrap();
    assert!(
        (-offset..-offset + 5).contains(&found),
        "line {found} in view"
    );
}

fn log() -> State {
    let mut state = sized(40, 5);
    let lines: String = (0..60)
        .map(|i| {
            let level = if i % 10 == 3 { "ERROR" } else { "info" };
            format!("{i:02} {level} request served\r\n")
        })
        .collect();
    let _ = state.feed(&lines);
    state
}

#[test]
fn a_query_counts_its_matches_in_the_scrollback_and_stands_on_the_latest() {
    let mut state = log();
    searched(&mut state, "error");
    assert!(state.search().open);
    assert_eq!(count(&state), (6, 6), "whatever the case unless asked");
    let _ = drive(&mut state, Message::Case(true));
    assert_eq!(count(&state), (0, 0));
    let _ = drive(&mut state, Message::Query("ERROR".to_owned()));
    assert_eq!(count(&state), (6, 6));
    // The latest match is brought into view.
    assert_in_view(&state, 5);
}

#[test]
fn next_and_previous_go_round_and_bring_the_match_into_view() {
    let mut state = log();
    searched(&mut state, "ERROR");
    let _ = drive(&mut state, Message::Next);
    assert_eq!(count(&state), (1, 6), "round from the last to the first");
    assert!(
        !state.at_bottom(),
        "the first match is up in the scrollback"
    );
    assert_in_view(&state, 0);
    let _ = drive(&mut state, Message::Previous);
    assert_eq!(count(&state), (6, 6));
    let _ = drive(&mut state, Message::Previous);
    assert_eq!(count(&state), (5, 6));
}

#[test]
fn a_regular_expression_is_read_and_one_that_cannot_be_says_so() {
    let mut state = log();
    searched(&mut state, r"\d3 ERROR");
    assert_eq!(count(&state), (0, 0), "as text, it is not found");
    let _ = drive(&mut state, Message::Regex(true));
    assert!(state.search().error.is_none());
    assert_eq!(count(&state), (6, 6));
    let _ = drive(&mut state, Message::Query("(unclosed".to_owned()));
    let error = state
        .search()
        .error
        .clone()
        .expect("a query that can't be read");
    assert_eq!(error.key, "ui.log.invalidRegex");
    assert_eq!(count(&state), (0, 0));
    // As text it is found where it is written.
    let _ = drive(&mut state, Message::Regex(false));
    assert!(state.search().error.is_none());
}

#[test]
fn the_current_match_stays_itself_as_new_lines_come() {
    let mut state = log();
    searched(&mut state, "ERROR");
    let _ = drive(&mut state, Message::Next);
    let _ = drive(&mut state, Message::Next);
    assert_eq!(count(&state), (2, 6));
    let _ = state.feed("60 ERROR late\r\n61 info\r\n");
    // Listed again once its time comes (output may pour in meanwhile).
    std::thread::sleep(crate::ui::widgets::terminal::search::RESCAN_EVERY);
    let _ = drive(&mut state, Message::Read);
    assert_eq!(count(&state), (2, 7), "the same match, one more found");
}

/// A terminal of 20 × 5 keeping `scrollback` lines, `rows` rows of
/// "same" written, searched for "same" and stepped five back from the
/// latest; where the match it stands on starts.
fn among_the_same(scrollback: usize, rows: usize) -> (State, i32) {
    let mut state = State::new().with_scrollback(scrollback);
    let _ = state.update(Message::Viewport(crate::ui::widgets::terminal::Viewport {
        cols: 20,
        rows: 5,
        cell_width: 7.2,
        cell_height: 16.0,
    }));
    let _ = state.feed(&"same\r\n".repeat(rows));
    searched(&mut state, "same");
    for _ in 0..5 {
        let _ = drive(&mut state, Message::Previous);
    }
    let line = current_line(&state);
    (state, line)
}

fn current_line(state: &State) -> i32 {
    let at = state.search().current.unwrap();
    state.search().matches[at].start().line.0
}

fn three_more(state: &mut State) {
    let _ = state.feed(&"same\r\n".repeat(3));
    std::thread::sleep(crate::ui::widgets::terminal::search::RESCAN_EVERY);
    let _ = drive(state, Message::Read);
}

#[test]
fn among_rows_that_say_the_same_the_current_match_moves_up_with_its_own() {
    let (mut state, line) = among_the_same(1000, 20);
    assert_eq!(count(&state), (15, 20));
    three_more(&mut state);
    assert_eq!(count(&state), (15, 23), "the same match, three more found");
    assert_eq!(current_line(&state), line - 3, "moved up with its row");
}

#[test]
fn once_the_scrollback_is_full_the_same_row_is_found_again_where_it_was() {
    // 10 lines kept and 5 shown: the oldest are let go of, uncounted, so
    // a row is known again only by what it says; among rows that all say
    // the same, the one now where the current one was.
    let (mut state, line) = among_the_same(10, 40);
    assert_eq!(count(&state), (9, 14));
    three_more(&mut state);
    assert_eq!(count(&state), (9, 14));
    assert_eq!(current_line(&state), line);
}

#[test]
fn what_is_selected_is_what_the_search_opens_on() {
    let mut state = log();
    let _ = drive(
        &mut state,
        Message::Press {
            at: alacritty_terminal::index::Point::new(
                alacritty_terminal::index::Line(0),
                alacritty_terminal::index::Column(4),
            ),
            side: alacritty_terminal::index::Side::Left,
            click: crate::ui::widgets::terminal::Click::Double,
            extend: false,
        },
    );
    let _ = drive(&mut state, Message::OpenSearch);
    assert!(!state.search().query.text.is_empty());
    let _ = drive(&mut state, Message::CloseSearch);
    assert!(!state.search().open);
}
