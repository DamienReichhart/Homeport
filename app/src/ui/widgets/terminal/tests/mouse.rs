//! What the pointer tells a program, past the first version: a release
//! with Shift held, and where xterm's first encoding stops.

use alacritty_terminal::term::TermMode;
use iced::keyboard::Modifiers;

use crate::ui::widgets::terminal::mouse::{report, Button, Pointer};

const CLICKS: TermMode = TermMode::MOUSE_REPORT_CLICK;

#[test]
fn a_release_is_reported_even_with_shift_held() {
    let sgr = CLICKS | TermMode::SGR_MOUSE | TermMode::MOUSE_DRAG;
    // Pressed without Shift (reported), released with it: the program
    // must hear the release, or its drag never ends.
    let release = report(Pointer::Release(Button::Left), 4, 2, Modifiers::SHIFT, sgr);
    assert_eq!(release.as_deref(), Some("\x1b[<4;5;3m"));
    // A press with Shift still selects instead.
    let press = report(Pointer::Press(Button::Left), 4, 2, Modifiers::SHIFT, sgr);
    assert_eq!(press, None);
}

/// xterm's first encoding stops at cell 95 here, not 223: a value over
/// 127 is one byte xterm sends raw, and input is a `String` through the
/// session's API (`Event::Input`, `TerminalSession::input`, the five
/// `*_input` use cases; the runtimes' writers under them take bytes),
/// whose bytes over 127 come in pairs. Such a report would name another cell; none is sent, and a
/// program that wants far cells asks for SGR (`?1006h`), as vim, tmux
/// and htop do.
#[test]
fn the_first_encoding_stops_where_text_can_still_say_a_byte() {
    let at = |col| {
        report(
            Pointer::Press(Button::Left),
            col,
            0,
            Modifiers::empty(),
            CLICKS,
        )
    };
    assert_eq!(at(94).as_deref(), Some("\x1b[M \x7f!"));
    assert_eq!(at(95), None);
    let sgr = CLICKS | TermMode::SGR_MOUSE;
    let far = report(
        Pointer::Press(Button::Left),
        222,
        0,
        Modifiers::empty(),
        sgr,
    );
    assert_eq!(far.as_deref(), Some("\x1b[<0;223;1M"));
}
