//! What a key does in a terminal: the bytes xterm sends for it, or one of
//! the terminal's own actions, or nothing, so the app's shortcut has it.
//!
//! | Key                                        | What it does                                   |
//! | ------------------------------------------ | ---------------------------------------------- |
//! | Ctrl + Shift + C (Cmd + C on macOS)        | copies the selection                           |
//! | Ctrl + Shift + V, Shift + Insert (Cmd + V) | pastes, bracketed when the program asked       |
//! | Ctrl + Shift + F (Cmd + F)                 | opens the search                               |
//! | Ctrl + Shift + A (Cmd + A)                 | selects everything the terminal holds          |
//! | Shift + Page Up, Page Down                 | a screen of the scrollback up, down            |
//! | Shift + Home, End                          | the oldest line kept, the screen               |
//! | Ctrl + Tab, Ctrl + Shift + Tab             | not the terminal's: they leave it              |
//! | Cmd with any other key (macOS)             | not the terminal's: the app's shortcuts        |
//! | any other Ctrl or Alt combination          | the program's, or nothing: never the app's     |
//! | anything else                              | the program's, as xterm encodes it             |
//!
//! Ctrl and the terminal's own letters go by where the key is on a US
//! layout when the layout's letter there is not Latin (Ctrl + С on a
//! Russian layout is ^C). Ctrl and Alt together that type text (AltGr on
//! Windows) send the text.
//!
//! The Shift + Page and Home keys are the program's on the alternate
//! screen, which has no scrollback.

use alacritty_terminal::term::TermMode;
use iced::keyboard::{
    key::{Code, Named, Physical},
    Key, Location, Modifiers,
};

use super::state::Scroll;

/// What a key press is for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// These bytes, to the program.
    Send(String),
    Copy,
    Paste,
    Find,
    SelectAll,
    Scroll(ScrollTo),
    /// The terminal's, though it sends nothing: a Ctrl or Alt
    /// combination xterm has no code for still reaches no shortcut.
    Nothing,
}

/// Where Shift with Page Up, Page Down, Home and End take the view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollTo {
    PageUp,
    PageDown,
    Top,
    Bottom,
}

impl From<ScrollTo> for Scroll {
    fn from(to: ScrollTo) -> Self {
        match to {
            ScrollTo::PageUp => Scroll::Pages(-1),
            ScrollTo::PageDown => Scroll::Pages(1),
            ScrollTo::Top => Scroll::Top,
            ScrollTo::Bottom => Scroll::Bottom,
        }
    }
}

/// A key as the window gave it.
#[derive(Debug, Clone, Copy)]
pub struct Press<'a> {
    pub key: &'a Key,
    pub modifiers: Modifiers,
    /// What the key types in the user's layout, if anything.
    pub text: Option<&'a str>,
    pub location: Location,
    /// The ASCII character of the key where it is on a US layout
    /// ([`latin`]): what Ctrl and the terminal's own keys go by on a layout
    /// whose letters are not Latin (Ctrl + С on a Russian one is ^C).
    pub physical: Option<char>,
}

/// The character the key at `physical` has on a US layout.
pub fn latin(physical: &Physical) -> Option<char> {
    let Physical::Code(code) = physical else {
        return None;
    };
    let letters = [
        (Code::KeyA, 'a'),
        (Code::KeyB, 'b'),
        (Code::KeyC, 'c'),
        (Code::KeyD, 'd'),
        (Code::KeyE, 'e'),
        (Code::KeyF, 'f'),
        (Code::KeyG, 'g'),
        (Code::KeyH, 'h'),
        (Code::KeyI, 'i'),
        (Code::KeyJ, 'j'),
        (Code::KeyK, 'k'),
        (Code::KeyL, 'l'),
        (Code::KeyM, 'm'),
        (Code::KeyN, 'n'),
        (Code::KeyO, 'o'),
        (Code::KeyP, 'p'),
        (Code::KeyQ, 'q'),
        (Code::KeyR, 'r'),
        (Code::KeyS, 's'),
        (Code::KeyT, 't'),
        (Code::KeyU, 'u'),
        (Code::KeyV, 'v'),
        (Code::KeyW, 'w'),
        (Code::KeyX, 'x'),
        (Code::KeyY, 'y'),
        (Code::KeyZ, 'z'),
        (Code::Digit0, '0'),
        (Code::Digit1, '1'),
        (Code::Digit2, '2'),
        (Code::Digit3, '3'),
        (Code::Digit4, '4'),
        (Code::Digit5, '5'),
        (Code::Digit6, '6'),
        (Code::Digit7, '7'),
        (Code::Digit8, '8'),
        (Code::Digit9, '9'),
        (Code::BracketLeft, '['),
        (Code::BracketRight, ']'),
        (Code::Backslash, '\\'),
        (Code::Minus, '-'),
        (Code::Equal, '='),
        (Code::Slash, '/'),
        (Code::Backquote, '`'),
        (Code::Comma, ','),
        (Code::Period, '.'),
        (Code::Semicolon, ';'),
        (Code::Quote, '\''),
        (Code::Space, ' '),
    ];
    letters.iter().find(|(at, _)| at == code).map(|(_, c)| *c)
}

/// xterm's number for the modifiers of a key: 1, plus 1 for Shift, 2 for
/// Alt and 4 for Ctrl. Cmd is never sent.
fn parameter(modifiers: Modifiers) -> u8 {
    1 + u8::from(modifiers.shift())
        + 2 * u8::from(modifiers.alt())
        + 4 * u8::from(modifiers.control())
}

/// `CSI 1 ; m X`, `SS3 X` or `CSI X`: the cursor keys and F1 to F4.
fn lettered(letter: char, modifiers: Modifiers, application: bool) -> String {
    match parameter(modifiers) {
        1 if application => format!("\x1bO{letter}"),
        1 => format!("\x1b[{letter}"),
        m => format!("\x1b[1;{m}{letter}"),
    }
}

/// `CSI n ~` or `CSI n ; m ~`: the editing keys and F5 onwards.
fn tilde(number: u8, modifiers: Modifiers) -> String {
    match parameter(modifiers) {
        1 => format!("\x1b[{number}~"),
        m => format!("\x1b[{number};{m}~"),
    }
}

/// F1 to F4 by letter, F5 to F20 by number.
fn function(named: Named) -> Option<Result<char, u8>> {
    Some(match named {
        Named::F1 => Ok('P'),
        Named::F2 => Ok('Q'),
        Named::F3 => Ok('R'),
        Named::F4 => Ok('S'),
        Named::F5 => Err(15),
        Named::F6 => Err(17),
        Named::F7 => Err(18),
        Named::F8 => Err(19),
        Named::F9 => Err(20),
        Named::F10 => Err(21),
        Named::F11 => Err(23),
        Named::F12 => Err(24),
        Named::F13 => Err(25),
        Named::F14 => Err(26),
        Named::F15 => Err(28),
        Named::F16 => Err(29),
        Named::F17 => Err(31),
        Named::F18 => Err(32),
        Named::F19 => Err(33),
        Named::F20 => Err(34),
        _ => return None,
    })
}

/// Alt before a key is Escape before its bytes.
fn with_alt(bytes: &str, modifiers: Modifiers) -> String {
    if modifiers.alt() {
        format!("\x1b{bytes}")
    } else {
        bytes.to_owned()
    }
}

/// What a named key sends.
fn named(named: Named, modifiers: Modifiers, mode: TermMode, location: Location) -> Option<String> {
    let application = mode.contains(TermMode::APP_CURSOR);
    let keypad = mode.contains(TermMode::APP_KEYPAD) && location == Location::Numpad;
    Some(match named {
        Named::ArrowUp => lettered('A', modifiers, application),
        Named::ArrowDown => lettered('B', modifiers, application),
        Named::ArrowRight => lettered('C', modifiers, application),
        Named::ArrowLeft => lettered('D', modifiers, application),
        Named::Home => lettered('H', modifiers, application),
        Named::End => lettered('F', modifiers, application),
        Named::Insert => tilde(2, modifiers),
        Named::Delete => tilde(3, modifiers),
        Named::PageUp => tilde(5, modifiers),
        Named::PageDown => tilde(6, modifiers),
        Named::Enter if keypad => "\x1bOM".to_owned(),
        Named::Enter => {
            let enter = if mode.contains(TermMode::LINE_FEED_NEW_LINE) {
                "\r\n"
            } else {
                "\r"
            };
            with_alt(enter, modifiers)
        }
        Named::Tab if modifiers.shift() => "\x1b[Z".to_owned(),
        Named::Tab => with_alt("\t", modifiers),
        Named::Backspace if modifiers.control() => with_alt("\x08", modifiers),
        Named::Backspace => with_alt("\x7f", modifiers),
        Named::Escape => with_alt("\x1b", modifiers),
        Named::Space if modifiers.control() => with_alt("\0", modifiers),
        Named::Space => with_alt(" ", modifiers),
        other => match function(other)? {
            Ok(letter) => lettered(letter, modifiers, true),
            Err(number) => tilde(number, modifiers),
        },
    })
}

/// What Ctrl with `c` sends: the control character of a letter, and
/// xterm's for the signs that have one.
fn control(c: char) -> Option<char> {
    let byte = match c.to_ascii_lowercase() {
        letter @ 'a'..='z' => letter as u8 - b'a' + 1,
        '@' | ' ' | '2' | '`' => 0,
        '[' | '3' => 0x1b,
        '\\' | '4' => 0x1c,
        ']' | '5' => 0x1d,
        '^' | '6' | '~' => 0x1e,
        '_' | '-' | '/' | '7' => 0x1f,
        '?' | '8' => 0x7f,
        _ => return None,
    };
    Some(char::from(byte))
}

/// What a key of the numeric keypad sends while the program asked for
/// the keypad's own codes (`DECKPAM`).
fn keypad(c: char) -> Option<String> {
    let letter = match c {
        '0'..='9' => char::from(b'p' + (c as u8 - b'0')),
        '.' => 'n',
        '+' => 'k',
        '-' => 'm',
        '*' => 'j',
        '/' => 'o',
        '=' => 'X',
        _ => return None,
    };
    Some(format!("\x1bO{letter}"))
}

/// The key's ASCII character, lowered: its own when the layout has one
/// there, else the one of its place on a US layout.
fn letter(press: &Press<'_>) -> Option<char> {
    let own = match press.key {
        Key::Character(c) => c.chars().next().filter(char::is_ascii),
        _ => None,
    };
    own.or(press.physical).map(|c| c.to_ascii_lowercase())
}

/// What the layout typed, if it is text and not a control character.
fn typed<'a>(press: &Press<'a>) -> Option<&'a str> {
    press
        .text
        .filter(|text| !text.is_empty() && !text.chars().any(char::is_control))
}

/// The terminal's own keys: copy, paste, search, select all and the
/// scrollback's. With Shift and Ctrl elsewhere, with Cmd on macOS.
fn own(press: &Press<'_>, mode: TermMode, mac: bool) -> Option<Action> {
    let modifiers = press.modifiers;
    let command = if mac {
        modifiers.logo() && !modifiers.control()
    } else {
        modifiers.control() && modifiers.shift() && !modifiers.logo()
    };
    if command && !modifiers.alt() {
        match letter(press) {
            Some('c') => return Some(Action::Copy),
            Some('v') => return Some(Action::Paste),
            Some('f') => return Some(Action::Find),
            Some('a') => return Some(Action::SelectAll),
            _ => {}
        }
    }
    let shift_only =
        modifiers.shift() && !modifiers.control() && !modifiers.alt() && !modifiers.logo();
    let scrollback = !mode.contains(TermMode::ALT_SCREEN);
    match press.key {
        Key::Named(Named::Insert) if shift_only => Some(Action::Paste),
        Key::Named(Named::PageUp) if shift_only && scrollback => {
            Some(Action::Scroll(ScrollTo::PageUp))
        }
        Key::Named(Named::PageDown) if shift_only && scrollback => {
            Some(Action::Scroll(ScrollTo::PageDown))
        }
        Key::Named(Named::Home) if shift_only && scrollback => Some(Action::Scroll(ScrollTo::Top)),
        Key::Named(Named::End) if shift_only && scrollback => {
            Some(Action::Scroll(ScrollTo::Bottom))
        }
        _ => None,
    }
}

/// What `press` does in a terminal in `mode`; `None` leaves it to the app.
/// Away from macOS every combination with Ctrl or Alt is the terminal's
/// (what xterm sends, or [`Action::Nothing`]), but Ctrl + Tab: the app's
/// shortcuts with Mod are Ctrl's there, and programs need them.
pub fn action(press: &Press<'_>, mode: TermMode, mac: bool) -> Option<Action> {
    if let Some(own) = own(press, mode, mac) {
        return Some(own);
    }
    let modifiers = press.modifiers;
    // Cmd is the app's on macOS; Ctrl + Tab leaves the terminal.
    if modifiers.logo() || (modifiers.control() && press.key == &Key::Named(Named::Tab)) {
        return None;
    }
    let chord = modifiers.control() || modifiers.alt();
    let bytes = match press.key {
        Key::Named(key) => match named(*key, modifiers, mode, press.location) {
            Some(bytes) => bytes,
            None if chord => return Some(Action::Nothing),
            None => return None,
        },
        Key::Character(c) => {
            let c = c.as_str();
            let first = c.chars().next()?;
            if mode.contains(TermMode::APP_KEYPAD) && press.location == Location::Numpad {
                if let Some(code) = keypad(first) {
                    return Some(Action::Send(code));
                }
            }
            if modifiers.control() && modifiers.alt() && typed(press).is_some() && !mac {
                // AltGr, which Windows reports as Ctrl and Alt: what it
                // types (`@` with Q on a German layout).
                typed(press).unwrap_or(c).to_owned()
            } else if modifiers.control() {
                match letter(press).and_then(control) {
                    Some(code) => with_alt(&code.to_string(), modifiers),
                    // A sign with no control character: xterm sends it as
                    // it is (Ctrl + 1 is "1").
                    None => with_alt(typed(press).unwrap_or(c), modifiers),
                }
            } else if modifiers.alt() && !mac {
                // Alt and a letter is Escape and the letter, as typed.
                with_alt(typed(press).unwrap_or(c), modifiers)
            } else {
                // What the layout types (Option's letters on macOS
                // included), or the key's own character.
                typed(press).unwrap_or(c).to_owned()
            }
        }
        Key::Unidentified => match typed(press) {
            Some(text) => text.to_owned(),
            None if chord => return Some(Action::Nothing),
            None => return None,
        },
    };
    Some(Action::Send(bytes))
}
