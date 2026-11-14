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
//! | anything else                              | the program's, as xterm encodes it             |
//!
//! The Shift + Page and Home keys are the program's on the alternate
//! screen, which has no scrollback.

use alacritty_terminal::term::TermMode;
use iced::keyboard::{key::Named, Key, Location, Modifiers};

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
        if let Key::Character(c) = press.key {
            match c.to_lowercase().as_str() {
                "c" => return Some(Action::Copy),
                "v" => return Some(Action::Paste),
                "f" => return Some(Action::Find),
                "a" => return Some(Action::SelectAll),
                _ => {}
            }
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
pub fn action(press: &Press<'_>, mode: TermMode, mac: bool) -> Option<Action> {
    if let Some(own) = own(press, mode, mac) {
        return Some(own);
    }
    let modifiers = press.modifiers;
    // Cmd is the app's on macOS; Ctrl + Tab leaves the terminal.
    if modifiers.logo() || (modifiers.control() && press.key == &Key::Named(Named::Tab)) {
        return None;
    }
    let bytes = match press.key {
        Key::Named(key) => named(*key, modifiers, mode, press.location)?,
        Key::Character(c) => {
            let c = c.as_str();
            let first = c.chars().next()?;
            if mode.contains(TermMode::APP_KEYPAD) && press.location == Location::Numpad {
                if let Some(code) = keypad(first) {
                    return Some(Action::Send(code));
                }
            }
            if modifiers.control() {
                let code = control(first)?;
                with_alt(&code.to_string(), modifiers)
            } else if modifiers.alt() && !mac {
                // Alt and a letter is Escape and the letter, as typed.
                with_alt(press.text.filter(|t| !t.is_empty()).unwrap_or(c), modifiers)
            } else {
                // What the layout types (Option's letters on macOS
                // included), or the key's own character.
                press
                    .text
                    .filter(|text| !text.is_empty() && !text.chars().any(char::is_control))
                    .unwrap_or(c)
                    .to_owned()
            }
        }
        Key::Unidentified => press
            .text
            .filter(|text| !text.is_empty() && !text.chars().any(char::is_control))?
            .to_owned(),
    };
    Some(Action::Send(bytes))
}
