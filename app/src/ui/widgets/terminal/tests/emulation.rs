//! The grid as programs drive it: the scrollback, the alternate screen,
//! a new size and what is told of it, titles, synchronised updates, focus
//! reports, pastes and the app's own notices.

use alacritty_terminal::term::TermMode;

use super::{answers, screen, sized};
use crate::ui::widgets::terminal::{Event, Message, Scroll, State, Viewport};

fn lines(count: usize) -> String {
    (0..count).map(|i| format!("line {i}\r\n")).collect()
}

fn viewport(cols: usize, rows: usize) -> Message {
    Message::Viewport(Viewport {
        cols,
        rows,
        cell_width: 7.2,
        cell_height: 16.0,
    })
}

#[test]
fn what_scrolls_off_the_screen_is_kept_and_typing_brings_the_view_back() {
    let mut state = sized(20, 5);
    let _ = state.feed(&lines(100));
    assert_eq!(state.total_lines(), 101, "96 lines up, 5 on the screen");
    assert_eq!(screen(&state)[3], "line 99");
    assert!(state.at_bottom());
    let _ = state.update(Message::Scroll(Scroll::Lines(-10)));
    assert!(!state.at_bottom());
    assert_eq!(screen_in_view(&state)[0], "line 86");
    let _ = state.update(Message::Scroll(Scroll::Top));
    assert_eq!(screen_in_view(&state)[0], "line 0");
    let _ = state.update(Message::Scroll(Scroll::To(0.5)));
    assert_eq!(screen_in_view(&state)[0], "line 48");
    let (_, event) = state.update(Message::Input("l".to_owned()));
    assert_eq!(event, Some(Event::Input("l".to_owned())));
    assert!(state.at_bottom());
}

/// What the rows in view say, the scrollback included.
fn screen_in_view(state: &State) -> Vec<String> {
    let term = state.term();
    let offset = i32::try_from(term.grid().display_offset()).unwrap();
    let (cols, rows) = state.size();
    (0..i32::try_from(rows).unwrap())
        .map(|row| {
            let line = alacritty_terminal::index::Line(row - offset);
            (0..cols)
                .map(|col| term.grid()[line][alacritty_terminal::index::Column(col)].c)
                .collect::<String>()
                .trim_end()
                .to_owned()
        })
        .collect()
}

#[test]
fn the_scrollback_keeps_what_it_was_told_to() {
    let mut state = State::new().with_scrollback(10);
    let _ = state.update(viewport(20, 5));
    let _ = state.feed(&lines(100));
    assert_eq!(state.total_lines(), 15);
}

#[test]
fn the_alternate_screen_has_no_scrollback_and_gives_the_screen_back() {
    let mut state = sized(20, 5);
    let _ = state.feed("prompt$ vim\r\n");
    let _ = state.feed("\x1b[?1049h\x1b[H\x1b[2Jfull screen");
    assert!(state.mode().contains(TermMode::ALT_SCREEN));
    assert_eq!(screen(&state)[0], "full screen");
    let _ = state.feed(&lines(50));
    assert_eq!(
        state.total_lines(),
        5,
        "nothing is kept off the alternate screen"
    );
    let _ = state.feed("\x1b[?1049l");
    assert!(!state.mode().contains(TermMode::ALT_SCREEN));
    assert_eq!(screen(&state)[0], "prompt$ vim");
}

#[test]
fn a_new_size_is_told_once_and_the_text_rewraps() {
    let mut state = sized(20, 5);
    let _ = state.feed("0123456789abcdefghij0123456789");
    assert_eq!(screen(&state)[..2], ["0123456789abcdefghij", "0123456789"]);
    let (_, told) = state.update(viewport(40, 10));
    assert_eq!(told, Some(Event::Resize { cols: 40, rows: 10 }));
    assert_eq!(state.size(), (40, 10));
    assert_eq!(screen(&state)[0], "0123456789abcdefghij0123456789");
    // The same room again tells nothing.
    let (_, again) = state.update(viewport(40, 10));
    assert_eq!(again, None);
    // A terminal squeezed to nothing keeps two cells each way.
    let (_, squeezed) = state.update(viewport(0, 0));
    assert_eq!(squeezed, Some(Event::Resize { cols: 2, rows: 2 }));
    // What a program asks of the size is answered in cells and pixels.
    let _ = state.update(viewport(80, 24));
    let events = state.feed("\x1b[18t\x1b[14t");
    assert_eq!(answers(&events), "\x1b[8;24;80t\x1b[4;384;560t");
}

#[test]
fn a_title_and_the_bell_are_told() {
    let mut state = sized(20, 5);
    let events = state.feed("\x1b]0;root@web: /srv\x07\x07");
    assert_eq!(
        events,
        [Event::Title(Some("root@web: /srv".to_owned())), Event::Bell]
    );
    assert_eq!(state.title(), Some("root@web: /srv"));
}

#[test]
fn a_synchronised_update_shows_once_it_is_whole() {
    let mut state = sized(20, 5);
    let _ = state.feed("\x1b[?2026hhalf");
    assert_eq!(screen(&state)[0], "", "held while the program draws");
    let _ = state.feed(" done\x1b[?2026l");
    assert_eq!(screen(&state)[0], "half done");
}

#[test]
fn focus_is_reported_only_to_a_program_that_asked() {
    let mut state = sized(20, 5);
    assert_eq!(state.update(Message::Focus(true)).1, None);
    let _ = state.feed("\x1b[?1004h");
    assert_eq!(
        state.update(Message::Focus(false)).1,
        Some(Event::Input("\x1b[O".to_owned()))
    );
    assert_eq!(
        state.update(Message::Focus(true)).1,
        Some(Event::Input("\x1b[I".to_owned()))
    );
    assert!(state.is_focused());
}

#[test]
fn a_paste_ends_its_lines_with_enter_is_bracketed_when_asked_and_cannot_end_the_brackets() {
    let mut state = sized(20, 5);
    let pasted = |state: &mut State, text: &str| state.update(Message::Paste(text.to_owned())).1;
    assert_eq!(
        pasted(&mut state, "ls\nps\r\n"),
        Some(Event::Input("ls\rps\r".to_owned()))
    );
    let _ = state.feed("\x1b[?2004h");
    assert_eq!(
        pasted(&mut state, "echo\x1b[201~; rm -rf /\n"),
        Some(Event::Input(
            "\x1b[200~echo; rm -rf /\r\x1b[201~".to_owned()
        ))
    );
    // An empty clipboard sends nothing, brackets or not.
    assert_eq!(pasted(&mut state, ""), None);
    let _ = state.feed("\x1b[?2004l");
    assert_eq!(pasted(&mut state, ""), None);
}

#[test]
fn a_notice_is_a_dimmed_line_of_its_own_that_no_sequence_gets_into() {
    let mut state = sized(30, 5);
    let _ = state.feed("$ exit");
    state.notice("The process exited.\x1b[2J");
    let rows = screen(&state);
    assert_eq!(rows[..2], ["$ exit", "The process exited.[2J"]);
    let dim = alacritty_terminal::term::cell::Flags::DIM;
    assert!(super::cell(&state, 1, 0).flags.contains(dim));
    let _ = state.feed("next");
    assert!(!super::cell(&state, 2, 0).flags.contains(dim));
}

#[test]
fn a_reset_forgets_the_last_session_but_keeps_the_size() {
    let mut state = sized(30, 5);
    let _ = state.feed("\x1b]0;old\x07\x1b[?1049hold");
    state.reset();
    assert_eq!(state.size(), (30, 5));
    assert_eq!(state.title(), None);
    assert!(!state.mode().contains(TermMode::ALT_SCREEN));
    assert!(screen(&state).iter().all(String::is_empty));
}
