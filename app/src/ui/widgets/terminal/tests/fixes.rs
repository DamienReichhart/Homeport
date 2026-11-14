//! What the review of the first version found, each held to its fix: a
//! paste that could close its own brackets, notices a program could hide
//! or colour, a synchronised update that never ended, a search that went
//! on after its bar closed or lost its match, output that piled up
//! unread, and what reading it later lost.

use std::time::Duration;

use alacritty_terminal::{
    term::cell::Flags,
    vte::ansi::{Color, NamedColor},
};

use super::{cell, screen, sized, told};
use crate::ui::widgets::terminal::{Event, Message, State};

#[test]
fn a_paste_cannot_close_its_brackets_by_nesting_them() {
    let mut state = sized(20, 5);
    let _ = state.feed("\x1b[?2004h");
    for nested in [
        "\x1b[20\x1b[201~1~\nrm -rf ~\n",
        "\x1b[2\x1b[20\x1b[201~1~1~\nrm -rf ~\n",
        "\x1b\x1b[201~[201~\nrm -rf ~\n",
    ] {
        let sent = told(&mut state, Message::Paste(nested.to_owned()));
        let [Event::Input(sent)] = sent.as_slice() else {
            panic!("one input: {sent:?}");
        };
        let inside = sent
            .strip_prefix("\x1b[200~")
            .and_then(|rest| rest.strip_suffix("\x1b[201~"))
            .expect("between the brackets");
        assert!(!inside.contains('\x1b'), "no Escape inside: {inside:?}");
    }
}

#[test]
fn a_notice_after_a_program_died_mid_sequence_shows_in_its_own_colours() {
    let mut state = sized(30, 5);
    // Cut in the middle of an OSC string, in red.
    let _ = state.feed("\x1b[31mred \x1b]0;a title that never ends");
    state.notice("The process exited.");
    let rows = screen(&state);
    assert!(rows.contains(&"The process exited.".to_owned()), "{rows:?}");
    let row = rows
        .iter()
        .position(|row| row == "The process exited.")
        .unwrap();
    let first = cell(&state, i32::try_from(row).unwrap(), 0);
    assert_eq!(
        first.fg,
        Color::Named(NamedColor::Foreground),
        "not the program's red"
    );
    assert!(first.flags.contains(Flags::DIM));
    // Inside a DCS string too.
    let _ = state.feed("\x1bPq#0;2;0;0;0");
    state.notice("Again.");
    assert!(screen(&state).contains(&"Again.".to_owned()));
}

#[test]
fn a_notice_shows_even_while_a_synchronised_update_holds_the_screen() {
    let mut state = sized(30, 5);
    let _ = state.feed("\x1b[?2026hhalf");
    state.notice("The process exited.");
    let rows = screen(&state);
    assert_eq!(rows[0], "half");
    assert!(rows.contains(&"The process exited.".to_owned()), "{rows:?}");
}

#[test]
fn a_notice_after_output_that_waits_waits_behind_it_and_reads_none_of_it_at_once() {
    let mut state = sized(30, 5);
    // A program printed 3 MB and exited: most of it waits.
    let lines: String = (0..400_000).map(|i| format!("{i:07}\r\n")).collect();
    let _ = state.feed(&lines);
    let waiting = state.unread();
    assert!(waiting > 0);
    state.notice("The process exited.");
    assert_eq!(state.unread(), waiting, "nothing more is read at once");
    assert!(state.is_behind(), "the notice waits too");
    let mut frames = 0;
    while state.is_behind() {
        let _ = told(&mut state, Message::Read);
        frames += 1;
        assert!(frames <= 4, "a megabyte a frame");
    }
    let rows: Vec<String> = screen(&state)
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();
    assert_eq!(rows[rows.len() - 2..], ["0399999", "The process exited."]);
}

#[test]
fn a_synchronised_update_that_never_ends_is_shown_once_its_time_is_up() {
    let mut state = sized(30, 5);
    let _ = state.feed("\x1b[?2026hhalf a frame");
    assert_eq!(screen(&state)[0], "", "held");
    // alacritty's timeout is 150 ms; the grid asks for a read at it.
    std::thread::sleep(Duration::from_millis(200));
    let _ = told(&mut state, Message::Read);
    assert_eq!(screen(&state)[0], "half a frame");
}

#[test]
fn closing_the_search_lets_go_of_its_matches_and_keeps_its_query() {
    let mut state = sized(30, 5);
    let _ = state.feed("an error\r\nanother error\r\n");
    let _ = state.update(Message::OpenSearch);
    let _ = state.update(Message::Query("error".to_owned()));
    assert_eq!(state.search().matches.len(), 2);
    let _ = state.update(Message::CloseSearch);
    assert!(state.search().matches.is_empty(), "nothing is marked");
    assert_eq!(state.search().query.text, "error");
    // Output while it is closed is not searched.
    let _ = state.feed("a third error\r\n");
    assert!(state.search().matches.is_empty());
    let _ = state.update(Message::OpenSearch);
    assert_eq!(
        state.search().matches.len(),
        3,
        "searched again when it opens"
    );
}

#[test]
fn the_current_match_stays_itself_once_the_scrollback_is_full() {
    let mut state = State::new().with_scrollback(20);
    let _ = state.update(Message::Viewport(crate::ui::widgets::terminal::Viewport {
        cols: 30,
        rows: 5,
        cell_width: 7.2,
        cell_height: 16.0,
    }));
    let lines: String = (0..40).map(|i| format!("line {i:02} match\r\n")).collect();
    let _ = state.feed(&lines);
    let _ = state.update(Message::OpenSearch);
    let _ = state.update(Message::Query("match".to_owned()));
    // Up three, to "line 37".
    for _ in 0..3 {
        let _ = state.update(Message::Previous);
    }
    let on = |state: &State| {
        let found = state.search().matches[state.search().current.unwrap()]
            .start()
            .line;
        let row = &state.term().grid()[found];
        (0..30)
            .map(|col| row[alacritty_terminal::index::Column(col)].c)
            .collect::<String>()
    };
    assert!(on(&state).starts_with("line 36"), "{}", on(&state));
    // The scrollback is full: every new line drops the oldest.
    let _ = state.feed("line 40 match\r\nline 41 match\r\n");
    std::thread::sleep(Duration::from_millis(300));
    let _ = told(&mut state, Message::Read);
    assert!(on(&state).starts_with("line 36"), "{}", on(&state));
}

#[test]
fn a_flood_faster_than_it_is_read_is_bounded_and_caught_up() {
    use crate::ui::widgets::terminal::state::{READ_AT_ONCE, UNREAD_MAX};
    let mut state = sized(20, 5);
    let batch: String = (0..600_000).map(|i| format!("{i:07}\r\n")).collect();
    assert!(batch.len() > READ_AT_ONCE);
    // Ten frames of a producer five times as fast as reading: what is unread
    // never passes the cap, the oldest dropped first.
    for _ in 0..10 {
        let _ = state.feed(&batch);
        let _ = told(&mut state, Message::Read);
        assert!(state.unread() <= UNREAD_MAX, "{} unread", state.unread());
    }
    assert!(state.dropped() > 0);
    // Then the program stops (Ctrl + C): the last lines show within a
    // bounded number of frames.
    let _ = state.feed("^C\r\n$ ");
    let mut frames = 0;
    while state.is_behind() {
        let _ = told(&mut state, Message::Read);
        frames += 1;
    }
    assert!(frames <= UNREAD_MAX / READ_AT_ONCE + 1, "{frames} frames");
    assert_eq!(screen(&state)[3..5], ["^C", "$"]);
}

#[test]
fn what_is_read_later_keeps_its_title_and_its_bell() {
    use crate::ui::widgets::terminal::state::READ_AT_ONCE;
    let mut state = sized(20, 5);
    let mut output = "x\r\n".repeat(READ_AT_ONCE / 3 + 10);
    output.push_str("\x1b]0;later\x07\x07\x1b[6n");
    let first = state.feed(&output);
    assert!(first.is_empty(), "{first:?}");
    let mut later = vec![];
    while state.is_behind() {
        later.extend(told(&mut state, Message::Read));
    }
    assert!(
        later.contains(&Event::Title(Some("later".to_owned()))),
        "{later:?}"
    );
    assert!(later.contains(&Event::Bell), "{later:?}");
    assert!(later
        .iter()
        .any(|e| matches!(e, Event::Input(text) if text.ends_with('R'))));
}

#[test]
fn a_title_is_kept_short() {
    let mut state = sized(20, 5);
    let long = "t".repeat(100_000);
    let events = state.feed(&format!("\x1b]0;{long}\x07"));
    let Some(Event::Title(Some(title))) = events.first() else {
        panic!("{events:?}");
    };
    assert!(title.chars().count() <= 256);
    assert!(state
        .title()
        .is_some_and(|kept| kept.chars().count() <= 256));
}

#[test]
fn a_dark_terminal_answers_its_colours_before_it_is_drawn() {
    let mut state = State::new().dark(true);
    let events = state.feed("\x1b]11;?\x07");
    assert_eq!(
        events,
        [Event::Input("\x1b]11;rgb:0f0f/0f0f/0f0f\x07".to_owned())]
    );
}
