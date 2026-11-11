//! What the user last used: the keyboard or the pointer. A focus ring and a
//! tooltip are for someone at the keyboard (CSS's `:focus-visible`): focus
//! given by a click shows neither, focus given by Tab, an arrow or a dialog
//! that opens after a key does.
//!
//! The window's events all come on one thread, and so does each test, so
//! this is the thread's.

use std::cell::Cell;

use iced::{keyboard, mouse, touch, Event};

thread_local! {
    static KEYBOARD: Cell<bool> = const { Cell::new(true) };
}

/// Notes what `event` was made with. Every widget of the kit that cares
/// calls it with the events it sees; noting one twice changes nothing.
pub fn note(event: &Event) {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed { .. }) => KEYBOARD.set(true),
        Event::Mouse(mouse::Event::ButtonPressed(_))
        | Event::Touch(touch::Event::FingerPressed { .. }) => KEYBOARD.set(false),
        _ => {}
    }
}

/// Whether the last thing pressed was a key: focus given now is shown.
pub fn keyboard() -> bool {
    KEYBOARD.get()
}

/// Whether `event` is a press of the pointer or of a finger.
pub fn is_pointer_down(event: &Event) -> bool {
    matches!(
        event,
        Event::Mouse(mouse::Event::ButtonPressed(_))
            | Event::Touch(touch::Event::FingerPressed { .. })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_last_press_says_who_is_there() {
        assert!(keyboard(), "nothing pressed yet: focus shows");
        note(&Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        )));
        assert!(!keyboard());
        note(&Event::Mouse(mouse::Event::CursorMoved {
            position: iced::Point::ORIGIN,
        }));
        assert!(!keyboard(), "moving the pointer changes nothing");
        note(&Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(keyboard::key::Named::Tab),
            modified_key: keyboard::Key::Named(keyboard::key::Named::Tab),
            physical_key: keyboard::key::Physical::Code(keyboard::key::Code::Tab),
            location: keyboard::Location::Standard,
            modifiers: keyboard::Modifiers::default(),
            text: None,
            repeat: false,
        }));
        assert!(keyboard());
    }
}
