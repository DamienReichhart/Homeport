//! What each key sends, as xterm sends it, in each mode a program can set;
//! which keys are the terminal's own; and which it leaves to the app.

use alacritty_terminal::term::TermMode;
use iced::keyboard::{key::Named, Key, Location, Modifiers};

use crate::ui::widgets::terminal::keys::{action, Action, Press, ScrollTo};

const NONE: Modifiers = Modifiers::empty();
const CTRL: Modifiers = Modifiers::CTRL;
const ALT: Modifiers = Modifiers::ALT;
const SHIFT: Modifiers = Modifiers::SHIFT;
const LOGO: Modifiers = Modifiers::LOGO;

fn with(modifiers: &[Modifiers]) -> Modifiers {
    modifiers.iter().fold(NONE, |all, one| all | *one)
}

/// What `key` does with `modifiers` and the text it types, on Linux.
fn sent_in(key: Key, modifiers: Modifiers, text: Option<&str>, mode: TermMode) -> Option<Action> {
    let press = Press {
        key: &key,
        modifiers,
        text,
        location: Location::Standard,
        physical: None,
    };
    action(&press, mode, false)
}

fn bytes(key: Key, modifiers: Modifiers, mode: TermMode) -> String {
    let text = match &key {
        Key::Character(c) if modifiers.is_empty() || modifiers == SHIFT => Some(c.to_string()),
        _ => None,
    };
    match sent_in(key.clone(), modifiers, text.as_deref(), mode) {
        Some(Action::Send(bytes)) => bytes,
        other => panic!("{key:?} with {modifiers:?} did {other:?}"),
    }
}

fn named(key: Named) -> Key {
    Key::Named(key)
}

fn character(c: &str) -> Key {
    Key::Character(c.into())
}

#[test]
fn the_cursor_keys_follow_the_cursor_mode_and_say_their_modifiers() {
    let normal = TermMode::empty();
    let application = TermMode::APP_CURSOR;
    let table = [
        (Named::ArrowUp, NONE, normal, "\x1b[A"),
        (Named::ArrowDown, NONE, normal, "\x1b[B"),
        (Named::ArrowRight, NONE, normal, "\x1b[C"),
        (Named::ArrowLeft, NONE, normal, "\x1b[D"),
        (Named::Home, NONE, normal, "\x1b[H"),
        (Named::End, NONE, normal, "\x1b[F"),
        (Named::ArrowUp, NONE, application, "\x1bOA"),
        (Named::ArrowLeft, NONE, application, "\x1bOD"),
        (Named::Home, NONE, application, "\x1bOH"),
        (Named::End, NONE, application, "\x1bOF"),
        // Modifiers win over the mode: CSI 1 ; m.
        (Named::ArrowUp, SHIFT, application, "\x1b[1;2A"),
        (Named::ArrowRight, ALT, normal, "\x1b[1;3C"),
        (Named::ArrowLeft, CTRL, normal, "\x1b[1;5D"),
        (Named::ArrowDown, with(&[CTRL, SHIFT]), normal, "\x1b[1;6B"),
        (
            Named::ArrowUp,
            with(&[CTRL, ALT, SHIFT]),
            normal,
            "\x1b[1;8A",
        ),
        (Named::End, CTRL, normal, "\x1b[1;5F"),
    ];
    for (key, modifiers, mode, sends) in table {
        assert_eq!(
            bytes(named(key), modifiers, mode),
            sends,
            "{key:?} {modifiers:?} {mode:?}"
        );
    }
}

#[test]
fn the_editing_and_function_keys_are_xterm_s() {
    let mode = TermMode::empty();
    let table = [
        (Named::Insert, NONE, "\x1b[2~"),
        (Named::Delete, NONE, "\x1b[3~"),
        (Named::PageUp, NONE, "\x1b[5~"),
        (Named::PageDown, NONE, "\x1b[6~"),
        (Named::Delete, CTRL, "\x1b[3;5~"),
        (Named::PageDown, ALT, "\x1b[6;3~"),
        (Named::F1, NONE, "\x1bOP"),
        (Named::F2, NONE, "\x1bOQ"),
        (Named::F3, NONE, "\x1bOR"),
        (Named::F4, NONE, "\x1bOS"),
        (Named::F1, SHIFT, "\x1b[1;2P"),
        (Named::F4, CTRL, "\x1b[1;5S"),
        (Named::F5, NONE, "\x1b[15~"),
        (Named::F6, NONE, "\x1b[17~"),
        (Named::F7, NONE, "\x1b[18~"),
        (Named::F8, NONE, "\x1b[19~"),
        (Named::F9, NONE, "\x1b[20~"),
        (Named::F10, NONE, "\x1b[21~"),
        (Named::F11, NONE, "\x1b[23~"),
        (Named::F12, NONE, "\x1b[24~"),
        (Named::F12, SHIFT, "\x1b[24;2~"),
        (Named::F5, with(&[CTRL, ALT]), "\x1b[15;7~"),
        (Named::F13, NONE, "\x1b[25~"),
        (Named::F20, NONE, "\x1b[34~"),
    ];
    for (key, modifiers, sends) in table {
        assert_eq!(
            bytes(named(key), modifiers, mode),
            sends,
            "{key:?} {modifiers:?}"
        );
    }
}

#[test]
fn enter_tab_backspace_escape_and_space() {
    let mode = TermMode::empty();
    let table = [
        (Named::Enter, NONE, "\r"),
        (Named::Enter, ALT, "\x1b\r"),
        (Named::Tab, NONE, "\t"),
        (Named::Tab, SHIFT, "\x1b[Z"),
        (Named::Backspace, NONE, "\x7f"),
        (Named::Backspace, CTRL, "\x08"),
        (Named::Backspace, ALT, "\x1b\x7f"),
        (Named::Escape, NONE, "\x1b"),
        (Named::Space, NONE, " "),
        (Named::Space, CTRL, "\0"),
        (Named::Space, ALT, "\x1b "),
    ];
    for (key, modifiers, sends) in table {
        assert_eq!(
            bytes(named(key), modifiers, mode),
            sends,
            "{key:?} {modifiers:?}"
        );
    }
    // A program that asked for new lines gets CR LF.
    assert_eq!(
        bytes(named(Named::Enter), NONE, TermMode::LINE_FEED_NEW_LINE),
        "\r\n"
    );
}

#[test]
fn ctrl_makes_control_characters_and_alt_puts_escape_before() {
    let mode = TermMode::empty();
    let table = [
        ("a", CTRL, "\x01"),
        ("c", CTRL, "\x03"),
        ("d", CTRL, "\x04"),
        ("k", CTRL, "\x0b"),
        ("z", CTRL, "\x1a"),
        ("[", CTRL, "\x1b"),
        ("\\", CTRL, "\x1c"),
        ("]", CTRL, "\x1d"),
        ("6", CTRL, "\x1e"),
        ("/", CTRL, "\x1f"),
        ("2", CTRL, "\0"),
        ("?", CTRL, "\x7f"),
        ("b", ALT, "\x1bb"),
        ("x", with(&[CTRL, ALT]), "\x1b\x18"),
        ("a", NONE, "a"),
        ("A", SHIFT, "A"),
        ("é", NONE, "é"),
        ("日", NONE, "日"),
    ];
    for (c, modifiers, sends) in table {
        assert_eq!(
            bytes(character(c), modifiers, mode),
            sends,
            "{c:?} {modifiers:?}"
        );
    }
    // Ctrl with Shift and a letter is the same control character, but for
    // the terminal's own letters.
    assert_eq!(bytes(character("K"), with(&[CTRL, SHIFT]), mode), "\x0b");
    // What the layout types is what is sent: a dead key's letter, Option's
    // on macOS.
    assert_eq!(
        sent_in(character("e"), NONE, Some("ê"), mode),
        Some(Action::Send("ê".to_owned()))
    );
}

#[test]
fn the_keypad_sends_its_own_codes_when_the_program_asks() {
    let keypad = |c: &str, mode| {
        let key = character(c);
        let press = Press {
            key: &key,
            modifiers: NONE,
            text: Some(c),
            location: Location::Numpad,
            physical: None,
        };
        action(&press, mode, false)
    };
    let application = TermMode::APP_KEYPAD;
    assert_eq!(
        keypad("5", application),
        Some(Action::Send("\x1bOu".to_owned()))
    );
    assert_eq!(
        keypad("+", application),
        Some(Action::Send("\x1bOk".to_owned()))
    );
    assert_eq!(
        keypad("5", TermMode::empty()),
        Some(Action::Send("5".to_owned()))
    );
    let enter = named(Named::Enter);
    let press = Press {
        key: &enter,
        modifiers: NONE,
        text: None,
        location: Location::Numpad,
        physical: None,
    };
    assert_eq!(
        action(&press, application, false),
        Some(Action::Send("\x1bOM".to_owned()))
    );
}

#[test]
fn the_terminal_s_own_keys_with_ctrl_and_shift_or_cmd_on_macos() {
    let mode = TermMode::empty();
    let own = |c: &str, modifiers: Modifiers, mac: bool| {
        let key = character(c);
        let press = Press {
            key: &key,
            modifiers,
            text: None,
            location: Location::Standard,
            physical: None,
        };
        action(&press, mode, mac)
    };
    let linux = with(&[CTRL, SHIFT]);
    assert_eq!(own("C", linux, false), Some(Action::Copy));
    assert_eq!(own("V", linux, false), Some(Action::Paste));
    assert_eq!(own("F", linux, false), Some(Action::Find));
    assert_eq!(own("A", linux, false), Some(Action::SelectAll));
    assert_eq!(own("c", LOGO, true), Some(Action::Copy));
    assert_eq!(own("v", LOGO, true), Some(Action::Paste));
    assert_eq!(own("f", LOGO, true), Some(Action::Find));
    // Ctrl + C is the program's interrupt everywhere, macOS included.
    assert_eq!(own("c", CTRL, false), Some(Action::Send("\x03".to_owned())));
    assert_eq!(own("c", CTRL, true), Some(Action::Send("\x03".to_owned())));
    assert_eq!(
        sent_in(named(Named::Insert), SHIFT, None, mode),
        Some(Action::Paste)
    );
}

#[test]
fn shift_with_the_page_keys_scrolls_the_scrollback_but_not_on_the_alternate_screen() {
    let main = TermMode::empty();
    let scrolls = [
        (Named::PageUp, ScrollTo::PageUp),
        (Named::PageDown, ScrollTo::PageDown),
        (Named::Home, ScrollTo::Top),
        (Named::End, ScrollTo::Bottom),
    ];
    for (key, to) in scrolls {
        assert_eq!(
            sent_in(named(key), SHIFT, None, main),
            Some(Action::Scroll(to))
        );
    }
    let alternate = TermMode::ALT_SCREEN;
    assert_eq!(
        sent_in(named(Named::PageUp), SHIFT, None, alternate),
        Some(Action::Send("\x1b[5;2~".to_owned()))
    );
}

#[test]
fn what_is_left_to_the_app_never_reaches_the_program() {
    let mode = TermMode::empty();
    // Ctrl + Tab and Ctrl + Shift + Tab leave the terminal.
    assert_eq!(sent_in(named(Named::Tab), CTRL, None, mode), None);
    assert_eq!(
        sent_in(named(Named::Tab), with(&[CTRL, SHIFT]), None, mode),
        None
    );
    // Cmd is the app's on macOS: its shortcuts fire from a terminal.
    let key = character("k");
    let press = Press {
        key: &key,
        modifiers: LOGO,
        text: Some("k"),
        location: Location::Standard,
        physical: None,
    };
    assert_eq!(action(&press, mode, true), None);
    // A key that types nothing and that xterm has no code for.
    assert_eq!(sent_in(named(Named::CapsLock), NONE, None, mode), None);
}

/// A key pressed where `physical` is on a US layout.
fn at(key: Key, physical: char, modifiers: Modifiers, text: Option<&str>) -> Option<Action> {
    let press = Press {
        key: &key,
        modifiers,
        text,
        location: Location::Standard,
        physical: Some(physical),
    };
    action(&press, TermMode::empty(), false)
}

#[test]
fn on_a_layout_without_latin_letters_ctrl_goes_by_where_the_key_is() {
    let send = |bytes: &str| Some(Action::Send(bytes.to_owned()));
    // A Russian layout: С is where C is, В where D, Я where Z.
    assert_eq!(at(character("с"), 'c', CTRL, None), send("\x03"));
    assert_eq!(at(character("в"), 'd', CTRL, None), send("\x04"));
    assert_eq!(at(character("я"), 'z', CTRL, None), send("\x1a"));
    assert_eq!(at(character("х"), '[', CTRL, None), send("\x1b"));
    // The terminal's own keys too.
    let own = with(&[CTRL, SHIFT]);
    assert_eq!(at(character("С"), 'c', own, None), Some(Action::Copy));
    assert_eq!(at(character("М"), 'v', own, None), Some(Action::Paste));
    assert_eq!(at(character("А"), 'f', own, None), Some(Action::Find));
    assert_eq!(at(character("Ф"), 'a', own, None), Some(Action::SelectAll));
    // A Latin letter of the layout wins over its place (AZERTY's A is
    // where Q is).
    assert_eq!(at(character("a"), 'q', CTRL, None), send("\x01"));
    // Typing, without Ctrl, types the layout's letter.
    assert_eq!(at(character("с"), 'c', NONE, Some("с")), send("с"));
}

#[test]
fn altgr_on_windows_types_its_text_and_ctrl_alt_elsewhere_is_escape_and_control() {
    let send = |bytes: &str| Some(Action::Send(bytes.to_owned()));
    let altgr = with(&[CTRL, ALT]);
    // German AltGr + Q is @, reported as Ctrl and Alt with the text.
    assert_eq!(at(character("@"), 'q', altgr, Some("@")), send("@"));
    assert_eq!(at(character("€"), 'e', altgr, Some("€")), send("€"));
    // Ctrl + Alt + X, whose text (if any) is a control character.
    assert_eq!(
        at(character("x"), 'x', altgr, Some("\x18")),
        send("\x1b\x18")
    );
    assert_eq!(at(character("x"), 'x', altgr, None), send("\x1b\x18"));
}

#[test]
fn every_ctrl_or_alt_combination_is_the_terminal_s_away_from_macos() {
    let mode = TermMode::empty();
    let send = |bytes: &str| Some(Action::Send(bytes.to_owned()));
    // The app's Mod + 1..9, Mod + , and the like are Ctrl's here: xterm
    // sends the sign itself, and no shortcut fires.
    assert_eq!(sent_in(character("1"), CTRL, None, mode), send("1"));
    assert_eq!(sent_in(character("9"), CTRL, None, mode), send("9"));
    assert_eq!(sent_in(character(","), CTRL, None, mode), send(","));
    assert_eq!(sent_in(character("."), CTRL, None, mode), send("."));
    assert_eq!(sent_in(character(";"), CTRL, None, mode), send(";"));
    assert_eq!(sent_in(character("="), CTRL, None, mode), send("="));
    assert_eq!(sent_in(character("é"), CTRL, None, mode), send("é"));
    // A key xterm has no code for, with Ctrl or Alt, sends nothing but
    // is kept from the app.
    assert_eq!(
        sent_in(named(Named::CapsLock), CTRL, None, mode),
        Some(Action::Nothing)
    );
    assert_eq!(
        sent_in(named(Named::PrintScreen), ALT, None, mode),
        Some(Action::Nothing)
    );
    // Ctrl + Tab still leaves.
    assert_eq!(sent_in(named(Named::Tab), CTRL, None, mode), None);
}

#[test]
fn the_us_place_of_a_key_is_read_from_its_code() {
    use crate::ui::widgets::terminal::keys::latin;
    use iced::keyboard::key::{Code, NativeCode, Physical};
    assert_eq!(latin(&Physical::Code(Code::KeyC)), Some('c'));
    assert_eq!(latin(&Physical::Code(Code::Digit7)), Some('7'));
    assert_eq!(latin(&Physical::Code(Code::BracketRight)), Some(']'));
    assert_eq!(latin(&Physical::Code(Code::F1)), None);
    assert_eq!(
        latin(&Physical::Unidentified(NativeCode::Unidentified)),
        None
    );
}
