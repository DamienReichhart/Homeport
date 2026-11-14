//! What the pointer tells a program that asked for it (`CSI ? 1000 h` and
//! its kin): presses, releases, moves and the wheel, in SGR's encoding
//! (`CSI ? 1006 h`), UTF-8's (`1005`) or xterm's first one.
//!
//! Shift with the pointer is not reported: it selects, as in xterm. A
//! release is, whatever is held, so a press the program was told of ends.

use alacritty_terminal::term::TermMode;
use iced::keyboard::Modifiers;

/// What the pointer did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pointer {
    Press(Button),
    Release(Button),
    /// It moved to another cell, with this button held, or none.
    Move(Option<Button>),
    /// A notch of the wheel, up (towards the oldest) or down.
    Wheel {
        up: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left,
    Middle,
    Right,
}

impl Button {
    fn code(self) -> u32 {
        match self {
            Self::Left => 0,
            Self::Middle => 1,
            Self::Right => 2,
        }
    }

    pub fn of(button: iced::mouse::Button) -> Option<Self> {
        match button {
            iced::mouse::Button::Left => Some(Self::Left),
            iced::mouse::Button::Middle => Some(Self::Middle),
            iced::mouse::Button::Right => Some(Self::Right),
            _ => None,
        }
    }
}

/// Whether the program asked for the pointer at all.
pub fn reported(mode: TermMode, modifiers: Modifiers) -> bool {
    mode.intersects(TermMode::MOUSE_MODE) && !modifiers.shift()
}

/// What the program is sent for `pointer` at the cell `col`, `row` of the
/// screen (from 0), if it asked for that kind of event and it can be said
/// in the encoding it asked for.
pub fn report(
    pointer: Pointer,
    col: usize,
    row: usize,
    modifiers: Modifiers,
    mode: TermMode,
) -> Option<String> {
    // A release is told whatever is held: the press was, and a drag the
    // program follows must end.
    let release = matches!(pointer, Pointer::Release(_));
    if !mode.intersects(TermMode::MOUSE_MODE) || (modifiers.shift() && !release) {
        return None;
    }
    let (mut code, release) = match pointer {
        Pointer::Press(button) => (button.code(), false),
        Pointer::Release(button) => (button.code(), true),
        Pointer::Move(held) => {
            let any = mode.contains(TermMode::MOUSE_MOTION);
            let dragged = held.is_some() && mode.contains(TermMode::MOUSE_DRAG);
            if !any && !dragged {
                return None;
            }
            // A move with no button is "release" plus the motion bit.
            (held.map_or(3, Button::code) + 32, false)
        }
        Pointer::Wheel { up } => (if up { 64 } else { 65 }, false),
    };
    if modifiers.shift() {
        code += 4;
    }
    if modifiers.alt() {
        code += 8;
    }
    if modifiers.control() {
        code += 16;
    }
    let (x, y) = (col + 1, row + 1);
    if mode.contains(TermMode::SGR_MOUSE) {
        let end = if release { 'm' } else { 'M' };
        return Some(format!("\x1b[<{code};{x};{y}{end}"));
    }
    // The first encodings have no release of a given button.
    let code = if release { 3 + (code & !3) } else { code };
    let utf8 = mode.contains(TermMode::UTF8_MOUSE);
    // Each value is sent plus 32, as one character: a byte up to 127, or
    // up to 2047 with UTF-8. Not up to 255 as xterm's raw byte: input is a
    // `String` through the session's API (`Event::Input`,
    // `TerminalSession::input`, the five `*_input` use cases), where a
    // byte over 127 can't stand alone.
    let most = if utf8 { 2015 } else { 95 };
    let cell = |value: usize| -> Option<char> {
        if value > most {
            return None;
        }
        char::from_u32(u32::try_from(value + 32).ok()?)
    };
    let code = char::from_u32(code + 32)?;
    Some(format!("\x1b[M{code}{}{}", cell(x)?, cell(y)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLICKS: TermMode = TermMode::MOUSE_REPORT_CLICK;

    #[test]
    fn nothing_is_reported_unless_asked_or_with_shift() {
        let press = Pointer::Press(Button::Left);
        assert_eq!(
            report(press, 0, 0, Modifiers::empty(), TermMode::empty()),
            None
        );
        assert_eq!(report(press, 0, 0, Modifiers::SHIFT, CLICKS), None);
        // Moves need 1002 (with a button) or 1003 (any).
        let moved = Pointer::Move(None);
        assert_eq!(report(moved, 1, 1, Modifiers::empty(), CLICKS), None);
        let drag = Pointer::Move(Some(Button::Left));
        let drags = CLICKS | TermMode::MOUSE_DRAG;
        assert_eq!(report(moved, 1, 1, Modifiers::empty(), drags), None);
        assert_eq!(
            report(drag, 1, 1, Modifiers::empty(), drags).as_deref(),
            Some("\x1b[M@\"\"")
        );
    }

    #[test]
    fn sgr_says_each_press_and_release_with_its_modifiers() {
        let sgr = CLICKS | TermMode::SGR_MOUSE;
        let at = |pointer, modifiers| report(pointer, 9, 4, modifiers, sgr).unwrap();
        assert_eq!(
            at(Pointer::Press(Button::Left), Modifiers::empty()),
            "\x1b[<0;10;5M"
        );
        assert_eq!(
            at(Pointer::Release(Button::Left), Modifiers::empty()),
            "\x1b[<0;10;5m"
        );
        assert_eq!(
            at(Pointer::Press(Button::Right), Modifiers::CTRL),
            "\x1b[<18;10;5M"
        );
        assert_eq!(
            at(Pointer::Wheel { up: true }, Modifiers::ALT),
            "\x1b[<72;10;5M"
        );
        assert_eq!(
            at(Pointer::Wheel { up: false }, Modifiers::empty()),
            "\x1b[<65;10;5M"
        );
        // Far cells are no matter in SGR.
        assert_eq!(
            report(
                Pointer::Press(Button::Middle),
                499,
                299,
                Modifiers::empty(),
                sgr
            )
            .as_deref(),
            Some("\x1b[<1;500;300M")
        );
    }

    #[test]
    fn the_first_encoding_stops_where_a_byte_does_and_utf8_goes_further() {
        let press = Pointer::Press(Button::Left);
        assert_eq!(
            report(press, 0, 0, Modifiers::empty(), CLICKS).as_deref(),
            Some("\x1b[M !!")
        );
        assert_eq!(
            report(
                Pointer::Release(Button::Right),
                0,
                0,
                Modifiers::empty(),
                CLICKS
            )
            .as_deref(),
            Some("\x1b[M#!!")
        );
        assert_eq!(report(press, 200, 0, Modifiers::empty(), CLICKS), None);
        let utf8 = CLICKS | TermMode::UTF8_MOUSE;
        assert_eq!(
            report(press, 200, 0, Modifiers::empty(), utf8).as_deref(),
            Some("\x1b[M \u{e9}!")
        );
    }
}
