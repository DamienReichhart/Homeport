//! The search's tasks: the slices of a large buffer read on another
//! thread, a query built there, and what happens to their answers when
//! the lines, the query or the viewer changed while they were out.

use std::sync::Arc;

use super::{drive, feed, numbered, viewer};
use crate::{
    test_support::tasks::{is_none, messages},
    ui::widgets::log::{search::AT_ONCE, Line, Message, State},
};

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

fn type_in(state: &mut State, text: &str) {
    drive(state, Message::Query(text.to_owned()));
}

fn tell(state: &mut State, message: Message) {
    drive(state, message);
}

#[tokio::test]
async fn a_large_buffer_is_read_in_slices_on_another_thread() {
    let mut state = viewer(1_000_000);
    let lines = 300_000;
    feed(&mut state, numbered(0, lines));
    // Typing returns at once with a task: nothing was read yet.
    let (mut task, _) = state.update(Message::Query("line 1234".to_owned()));
    assert_eq!(state.matches().1, 0);
    assert!(state.is_searching());
    let mut slices = 0;
    loop {
        let found = messages(task).await;
        if found.is_empty() {
            break;
        }
        assert_eq!(found.len(), 1);
        slices += 1;
        // Lines keep coming while the search reads.
        feed_quietly(&mut state, numbered(lines + slices * 10, 10));
        task = state.update(found.into_iter().next().expect("one")).0;
    }
    assert!(slices >= 4, "{slices} slices for {lines} lines");
    assert!(!state.is_searching());
    // 1234, 12340..12349, 123400..123499: 111 lines.
    assert_eq!(state.matches().1, 111);
}

/// Appends lines whatever task that returns: while a search reads, the
/// lines wait for it.
fn feed_quietly(state: &mut State, lines: impl IntoIterator<Item = Line>) {
    drop(state.extend(lines));
}

#[tokio::test]
async fn what_a_task_found_for_another_query_is_dropped() {
    let mut state = viewer(100_000);
    feed(&mut state, numbered(0, 3 * AT_ONCE as usize));
    let (stale, _) = state.update(Message::Query("line 1".to_owned()));
    // Before it answers, the query changes.
    let (fresh, _) = state.update(Message::Query("line 22".to_owned()));
    for message in messages(stale).await {
        let (task, _) = state.update(message);
        assert!(is_none(task), "an old answer starts nothing");
    }
    assert_eq!(state.matches().1, 0, "nothing of the old query is kept");
    let mut task = fresh;
    loop {
        let found = messages(task).await;
        let Some(message) = found.into_iter().next() else {
            break;
        };
        task = state.update(message).0;
    }
    // 22, 220..229, 2200..2299: 111 lines of the 6144.
    assert_eq!(state.matches().1, 111);
}

#[tokio::test]
async fn hits_of_lines_dropped_while_a_task_read_them_are_not_kept() {
    let mut state = viewer(10_000);
    feed(&mut state, numbered(0, 10_000));
    let (task, _) = state.update(Message::Query("line".to_owned()));
    // The buffer moves on by half while the task reads the old lines.
    feed_quietly(&mut state, numbered(10_000, 5_000));
    let mut task = task;
    loop {
        let found = messages(task).await;
        let Some(message) = found.into_iter().next() else {
            break;
        };
        task = state.update(message).0;
    }
    assert_eq!(state.matches().1, 10_000, "one a line kept, and no more");
    assert_eq!(state.search.hits.front().map(|hit| hit.seq), Some(5_000));
    let hits = Arc::new(vec![]);
    let (task, _) = state.update(Message::Scanned {
        generation: 0,
        to: 99_999,
        hits: Some(hits),
    });
    assert!(is_none(task));
    assert!(!state.is_searching());
}

#[test]
fn what_a_task_of_another_viewer_found_is_not_this_ones() {
    let mut first = viewer(100);
    let mut second = viewer(100);
    feed(&mut first, numbered(0, 10));
    feed(&mut second, numbered(0, 10));
    assert_ne!(first.search.generation, second.search.generation);
    type_in(&mut first, "line");
    assert_ne!(first.search.generation, second.search.generation);
    // An answer meant for the first, handed to the second (a page
    // opened again on a new state while a task was out).
    let stray = Message::Scanned {
        generation: first.search.generation,
        to: 10,
        hits: Some(Arc::new(vec![crate::ui::widgets::log::search::Hit {
            seq: 3,
            count: 2,
        }])),
    };
    let (task, _) = second.update(stray);
    assert!(is_none(task));
    assert_eq!(second.matches(), (None, 0));
    assert_eq!(second.search.frontier, 0);
}

#[test]
fn a_slice_whose_task_died_is_read_again_then_the_search_says_it_stopped() {
    let mut state = viewer(100_000);
    feed(&mut state, numbered(0, 3 * AT_ONCE as usize));
    let (task, _) = state.update(Message::Query("line 1".to_owned()));
    drop(task);
    let generation = state.search.generation;
    let died = |to| Message::Scanned {
        generation,
        to,
        hits: None,
    };
    // Once: the same lines are asked for again, and none is taken as read.
    let (again, _) = state.update(died(3 * u64::from(AT_ONCE as u32)));
    assert!(!is_none(again), "the slice is read again");
    assert_eq!(state.search.frontier, 0);
    assert!(state.search_error().is_none());
    assert!(state.is_searching());
    // Twice: the search stops and says so, in the app's error type.
    let (task, _) = state.update(died(3 * u64::from(AT_ONCE as u32)));
    assert!(is_none(task));
    assert_eq!(
        state.search.frontier, 0,
        "never past lines that were not read"
    );
    let error = state.search_error().expect("said");
    assert_eq!(error.key, "ui.log.searchFailed");
    assert!(!state.is_searching());
    assert_eq!(state.matches().1, 0);
    // Lines that come start nothing; a new query starts over.
    drop(state.extend(numbered(10_000, 10)));
    assert!(!state.search.busy && state.search_error().is_some());
    drive(&mut state, Message::Query("line 12".to_owned()));
    assert!(state.search_error().is_none());
    assert!(state.matches().1 > 0);
}

#[test]
fn a_regular_expression_is_built_by_a_task_and_plain_text_where_it_is_typed() {
    let mut state = errors();
    let (task, _) = state.update(Message::Query("error".to_owned()));
    assert!(is_none(task));
    assert_eq!(state.matches().1, 4);
    let _ = state.update(Message::Regex(true));
    let (task, _) = state.update(Message::Query("err.r".to_owned()));
    assert!(!is_none(task), "update does not build it");
    assert_eq!(state.matches().1, 0);
    assert!(state.is_searching());
    // Its answer starts the search; an answer for a query since replaced
    // starts nothing.
    let (stale, _) = state.update(Message::Query("err.".to_owned()));
    drop(stale);
    let generation = state.search.generation;
    let _ = state.update(Message::Query("e".to_owned()));
    let matcher = crate::ui::widgets::log::search::Query {
        text: "err.".to_owned(),
        regex: true,
        case: false,
    }
    .compile()
    .expect("a readable one")
    .map(Arc::new);
    let _ = state.update(Message::Compiled {
        generation,
        matcher: Ok(matcher),
    });
    assert_eq!(state.matches().1, 0);
    drive(&mut state, Message::Query("err.r".to_owned()));
    assert_eq!(state.matches().1, 4);
    // Plain text too long to be quick goes the same way.
    let _ = state.update(Message::Regex(false));
    let long = "x".repeat(crate::ui::widgets::log::search::QUICK + 1);
    let (task, _) = state.update(Message::Query(long));
    assert!(!is_none(task));
}

#[test]
fn a_query_too_large_to_build_is_refused_in_the_apps_words() {
    use crate::{i18n, test_support::snapshot::in_language};
    let mut state = errors();
    tell(&mut state, Message::Regex(true));
    // A hundred thousand letters' worth of states, and one too long.
    for huge in [r"(\pL{100}){1000}".to_owned(), "a".repeat(5000)] {
        type_in(&mut state, &huge);
        let error = state.search_error().expect("refused");
        assert_eq!(error.key, "ui.log.invalidRegex");
        assert_eq!(error.code, homeport_core::error::ErrorCode::Validation);
        assert!(error.detail.as_deref().is_some_and(|why| !why.is_empty()));
        assert!(!state.is_searching());
    }
    type_in(&mut state, "(");
    let error = state.search_error().expect("refused").clone();
    // What the user reads is the catalogue's, in the app's language; the
    // crate's English is the detail.
    let _french = in_language("fr");
    assert_eq!(
        i18n::errors::message(&error),
        "Expression régulière non valide"
    );
    assert!(error
        .detail
        .as_deref()
        .is_some_and(|why| why.contains("unclosed")));
}

#[test]
fn long_lines_are_not_read_in_update_however_few_they_are() {
    let mut state = viewer(1000);
    type_in(&mut state, "needle");
    // Fifty lines of 64 KiB: few, and more than a frame reads.
    let line = format!("{} needle", "x".repeat(64 * 1024));
    let task = state.extend((0..50).map(|_| Line::new(line.clone())));
    assert!(!is_none(task), "read by a task");
    assert_eq!(state.matches().1, 0);
    // Short ones still are.
    let mut state = viewer(1000);
    type_in(&mut state, "needle");
    feed(&mut state, (0..50).map(|_| Line::new("a needle")));
    assert_eq!(state.matches().1, 50);
}

/// Runs a task to its end, answers fed back.
async fn finish(state: &mut State, task: iced::Task<Message>) {
    let mut pending = vec![task];
    while let Some(task) = pending.pop() {
        for message in messages(task).await {
            pending.push(state.update(message).0);
        }
    }
}

#[tokio::test]
async fn the_lines_cleared_while_a_task_read_them_leave_nothing() {
    let mut state = viewer(100_000);
    feed(&mut state, numbered(0, 10_000));
    let (task, _) = state.update(Message::Query("line".to_owned()));
    let _ = state.update(Message::Clear);
    finish(&mut state, task).await;
    assert_eq!(state.matches(), (None, 0));
    assert!(state.search.hits.is_empty());
    assert!(!state.is_searching() && !state.search.busy);
    // The search goes on with what comes next.
    feed(&mut state, numbered(0, 5));
    assert_eq!(state.matches().1, 5);
}

#[tokio::test]
async fn timestamps_shown_while_a_task_read_start_the_search_over() {
    let mut state = viewer(100_000);
    let stamped =
        (0..10_000).map(|n| Line::new(format!("2026-05-01T10:00:00Z up {n}")).timestamped(true));
    feed(&mut state, stamped);
    let (stale, _) = state.update(Message::Query("T10:".to_owned()));
    // With the timestamps hidden nothing matches; shown, every line does.
    let (fresh, _) = state.update(Message::Timestamps(true));
    finish(&mut state, stale).await;
    assert_eq!(state.matches().1, 0, "the old answer is not this search's");
    finish(&mut state, fresh).await;
    assert_eq!(state.matches().1, 10_000);
    assert!(!state.is_searching());
}

#[tokio::test]
async fn a_smaller_cap_set_while_a_task_read_keeps_only_the_lines_kept() {
    let mut state = viewer(100_000);
    feed(&mut state, numbered(0, 10_000));
    let (task, _) = state.update(Message::Query("line".to_owned()));
    state.set_cap(1_000);
    finish(&mut state, task).await;
    assert_eq!(state.len(), 1_000);
    assert_eq!(state.matches().1, 1_000);
    assert_eq!(state.search.hits.front().map(|hit| hit.seq), Some(9_000));
    assert!(!state.is_searching());
}
