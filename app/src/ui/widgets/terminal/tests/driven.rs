//! The grid on a window: what a press, the keys, the wheel and a frame
//! send, and that a key the terminal used reaches no shortcut of the app
//! (the shell's key area hears it as the terminal's).

use iced::{
    keyboard::{self, key::Named, Key, Location, Modifiers},
    mouse, window, Event, Point,
};

use super::sized;
use crate::{
    test_support::window::{drive, Window},
    ui::{
        theme::Element,
        widgets::{
            focus::key_area,
            terminal::{Message, Scroll, State, Terminal},
        },
    },
};

/// What the window hears: the terminal's messages, or a key the shell's
/// key area was told of, and where.
#[derive(Debug, Clone, PartialEq)]
enum Heard {
    Terminal(String),
    Page,
    Text,
    InTerminal,
}

fn heard(message: &Message) -> String {
    format!("{message:?}")
}

/// The terminal under a key area wired as the shell's is.
fn page(state: &State) -> Element<'_, Heard> {
    let grid = Terminal::new(state, |message| Heard::Terminal(heard(&message))).view();
    key_area(grid, |_, _| Some(Heard::Page))
        .always()
        .on_captured(|_, _| Some(Heard::Text))
        .on_terminal(|_, _| Some(Heard::InTerminal))
        .into()
}

const SIZE: (f32, f32) = (640.0, 400.0);

fn key(key: Key, modifiers: Modifiers, text: Option<&str>) -> Event {
    Event::Keyboard(keyboard::Event::KeyPressed {
        key: key.clone(),
        modified_key: key,
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: Location::Standard,
        modifiers,
        text: text.map(Into::into),
        repeat: false,
    })
}

/// Gives the grid focus with a press on it.
fn focus(window: &mut Window<'_, Heard>) {
    window.point_at(Point::new(100.0, 100.0));
    let _ = window.simulate(iced_test::simulator::click());
}

fn frame() -> Event {
    Event::Window(window::Event::RedrawRequested(std::time::Instant::now()))
}

#[test]
fn a_frame_tells_the_room_and_the_focus_and_typing_goes_to_the_program() {
    let state = sized(80, 24);
    let messages = drive(
        SIZE,
        || page(&state),
        |window| {
            let _ = window.simulate([frame()]);
            focus(window);
            let _ = window.simulate([frame()]);
            let _ = window.simulate([key(
                Key::Character("l".into()),
                Modifiers::empty(),
                Some("l"),
            )]);
            let _ = window.simulate([key(
                Key::Named(Named::Enter),
                Modifiers::empty(),
                Some("\r"),
            )]);
        },
    );
    let said: Vec<String> = messages
        .iter()
        .filter_map(|message| match message {
            Heard::Terminal(said) => Some(said.clone()),
            _ => None,
        })
        .collect();
    assert!(
        said[0].starts_with("Viewport(Viewport { cols: "),
        "{said:?}"
    );
    assert!(
        said.iter().any(|said| said.starts_with("Press {")),
        "{said:?}"
    );
    assert!(said.contains(&"Focus(true)".to_owned()), "{said:?}");
    assert!(said.contains(&"Input(\"l\")".to_owned()), "{said:?}");
    assert!(said.contains(&"Input(\"\\r\")".to_owned()), "{said:?}");
    // What the terminal used, the shell's key area was told of as the
    // terminal's: no shortcut of a field fires from it.
    assert!(!messages.contains(&Heard::Text) && !messages.contains(&Heard::Page));
    assert_eq!(
        messages.iter().filter(|m| **m == Heard::InTerminal).count(),
        2
    );
}

#[test]
fn ctrl_with_a_letter_is_the_program_s_and_ctrl_tab_is_left_to_the_window() {
    let state = sized(80, 24);
    let messages = drive(
        SIZE,
        || page(&state),
        |window| {
            focus(window);
            let ctrl = Modifiers::CTRL;
            let _ = window.simulate([Event::Keyboard(keyboard::Event::ModifiersChanged(ctrl))]);
            let _ = window.simulate([key(Key::Character("k".into()), ctrl, None)]);
            let _ = window.simulate([key(Key::Named(Named::Tab), ctrl, None)]);
        },
    );
    assert!(messages.contains(&Heard::Terminal("Input(\"\\u{b}\")".to_owned())));
    assert!(messages.contains(&Heard::InTerminal));
    // The terminal left Ctrl + Tab alone: it reached the page's keys (and
    // the window, which moves focus on).
    assert!(messages.contains(&Heard::Page));
    assert!(!messages
        .iter()
        .any(|m| matches!(m, Heard::Terminal(said) if said == "Input(\"\\t\")")));
}

#[test]
fn a_key_without_focus_is_not_the_terminal_s() {
    let state = sized(80, 24);
    let messages = drive(
        SIZE,
        || page(&state),
        |window| {
            let _ = window.simulate([key(
                Key::Character("n".into()),
                Modifiers::empty(),
                Some("n"),
            )]);
        },
    );
    assert_eq!(messages, [Heard::Page]);
}

#[test]
fn the_wheel_scrolls_the_scrollback_or_is_the_program_s() {
    let wheel = Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: 1.0 },
    });
    let scrolled = |state: &State| {
        drive(
            SIZE,
            || page(state),
            |window| {
                window.point_at(Point::new(100.0, 100.0));
                let _ = window.simulate([frame()]);
                let _ = window.simulate([wheel.clone()]);
            },
        )
    };
    // Nothing above the screen: the wheel is the page's.
    let state = sized(80, 24);
    assert!(!scrolled(&state)
        .iter()
        .any(|m| matches!(m, Heard::Terminal(said) if said.starts_with("Scroll"))));
    let mut state = sized(80, 24);
    let lines: String = (0..100).map(|i| format!("{i}\r\n")).collect();
    let _ = state.feed(&lines);
    let said = heard(&Message::Scroll(Scroll::Lines(-3)));
    assert!(scrolled(&state).contains(&Heard::Terminal(said)));
    // A program that asked for the pointer is told of the wheel.
    let _ = state.feed("\x1b[?1000h\x1b[?1006h");
    assert!(scrolled(&state)
        .iter()
        .any(|m| matches!(m, Heard::Terminal(said) if said.starts_with("Input(\"\\u{1b}[<64;"))));
    // On the alternate screen, without that, the wheel is the arrows.
    let _ = state.feed("\x1b[?1000l\x1b[?1049h");
    let arrows = heard(&Message::Input("\x1b[A".repeat(3)));
    assert!(scrolled(&state).contains(&Heard::Terminal(arrows)));
}
