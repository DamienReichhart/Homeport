//! What the grid hears: the keyboard and the input method while it has
//! focus, the pointer on its cells and its scrollbar, and the wheel. What
//! a key sends is `keys.rs`'s; what the pointer tells a program that asked
//! is `mouse.rs`'s. At each frame it tells the [`State`] the room it has,
//! the theme it was drawn in and whether it has focus.
//!
//! [`State`]: crate::ui::widgets::terminal::State

use std::time::Instant;

use alacritty_terminal::{
    index::{Column, Line, Point, Side},
    term::TermMode,
};
use iced::{
    advanced::input_method::{self, InputMethod, Preedit, Purpose},
    advanced::{
        clipboard::{self, Clipboard},
        mouse,
        widget::{operation::Focusable, Tree},
        Layout, Shell,
    },
    keyboard, window, Event, Pixels,
};

use super::{
    draw::{thumb_of, BLINK},
    rows::Link,
    Geometry, Internal, Pane, THUMB_MIN,
};
use crate::ui::widgets::{
    focus::modality,
    terminal::{
        keys::{self, Action as KeyAction},
        mouse::{self as pointer, Button, Pointer},
        state::{Click, Message as Action, Scroll},
    },
};

/// The lines a notch of the wheel scrolls (xterm.js's).
const WHEEL_LINES: f32 = 3.0;

/// A button the program was told is held, and the cell it was last told of.
#[derive(Debug, Clone, Copy)]
pub(super) struct Held {
    button: Button,
    cell: (usize, usize),
}

/// Gives the grid focus anew, so its ring shows or not by what was last used.
fn refocus(internal: &mut Internal) {
    internal.focus.unfocus();
    internal.focus.focus();
}

/// The link under the pointer, if one is.
pub(super) fn link_under<'a>(
    internal: &'a Internal,
    geometry: &Geometry,
    cursor: mouse::Cursor,
) -> Option<&'a Link> {
    let at = cursor.position_over(geometry.text)?;
    let (col, row, _) = geometry.cell_at(at);
    internal
        .laid
        .rows
        .get(row)?
        .links
        .iter()
        .find(|link| (link.from..link.to).contains(&col))
}

/// The grid's point for the cell `col`, `row` of the view.
fn point(internal: &Internal, col: usize, row: usize) -> Point {
    let row = i32::try_from(row).unwrap_or(i32::MAX);
    let offset = i32::try_from(internal.laid.offset).unwrap_or(0);
    Point::new(Line(row - offset), Column(col))
}

fn side(right: bool) -> Side {
    if right {
        Side::Right
    } else {
        Side::Left
    }
}

/// Whether Cmd (macOS) or Ctrl (elsewhere) is held: what opens a link.
fn command<Message>(pane: &Pane<'_, Message>, internal: &Internal) -> bool {
    if pane.mac {
        internal.modifiers.logo()
    } else {
        internal.modifiers.control()
    }
}

fn press<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    geometry: &Geometry,
    button: mouse::Button,
    at: iced::Point,
    shell: &mut Shell<'_, Message>,
) {
    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((pane.on_event)(message));
    let mode = pane.state.mode();
    let (col, row, right) = geometry.cell_at(at);
    if button == mouse::Button::Left && command(pane, internal) {
        if let Some(link) = link_under(internal, geometry, mouse::Cursor::Available(at)) {
            send(shell, Action::Open(link.uri.clone()));
            return;
        }
    }
    if pointer::reported(mode, internal.modifiers) {
        if let Some(button) = Button::of(button) {
            let report =
                pointer::report(Pointer::Press(button), col, row, internal.modifiers, mode);
            if let Some(report) = report {
                send(shell, Action::Input(report));
            }
            internal.reported = Some(Held {
                button,
                cell: (col, row),
            });
        }
        return;
    }
    if button != mouse::Button::Left {
        return;
    }
    let click = mouse::Click::new(at, mouse::Button::Left, internal.click.take());
    let kind = match click.kind() {
        mouse::click::Kind::Single => Click::Single,
        mouse::click::Kind::Double => Click::Double,
        mouse::click::Kind::Triple => Click::Triple,
    };
    internal.click = Some(click);
    internal.selecting = true;
    send(
        shell,
        Action::Press {
            at: point(internal, col, row),
            side: side(right),
            click: kind,
            extend: internal.modifiers.shift() && kind == Click::Single,
        },
    );
}

/// The pointer moved: the thumb it holds, the selection it drags, a link
/// it is over, or a program told where it is.
fn moved<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    geometry: &Geometry,
    position: iced::Point,
    shell: &mut Shell<'_, Message>,
) {
    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((pane.on_event)(message));
    if let Some(grab) = internal.grab {
        let length = thumb_of(internal, geometry).map_or(THUMB_MIN, |thumb| thumb.height);
        let travel = (geometry.track.height - length).max(1.0);
        let share = ((position.y - geometry.track.y - grab) / travel).clamp(0.0, 1.0);
        send(shell, Action::Scroll(Scroll::To(share)));
        return;
    }
    let (col, row, right) = geometry.cell_at(position);
    let over = geometry.text.contains(position).then_some((col, row));
    if over != internal.hovered {
        internal.hovered = over;
        shell.request_redraw();
    }
    if internal.selecting {
        // Past the rows, the view goes a line further with each move, and
        // the selection with it.
        if position.y < geometry.text.y {
            send(shell, Action::Scroll(Scroll::Lines(-1)));
        } else if position.y > geometry.text.y + geometry.text.height {
            send(shell, Action::Scroll(Scroll::Lines(1)));
        }
        send(
            shell,
            Action::Drag {
                at: point(internal, col, row),
                side: side(right),
            },
        );
        return;
    }
    let mode = pane.state.mode();
    let held = internal.reported.map(|held| held.button);
    let last = internal.reported.map(|held| held.cell);
    if pointer::reported(mode, internal.modifiers) && over.is_some() && last != Some((col, row)) {
        if let Some(report) =
            pointer::report(Pointer::Move(held), col, row, internal.modifiers, mode)
        {
            send(shell, Action::Input(report));
        }
        if let Some(held) = internal.reported.as_mut() {
            held.cell = (col, row);
        }
    }
}

/// The wheel: reported to a program that asked, arrows to one on the
/// alternate screen, the scrollback otherwise. At either end of the
/// scrollback the wheel is the page's.
// What the wheel scrolls in a frame is a few lines.
#[allow(clippy::cast_possible_truncation)]
fn wheel<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    geometry: &Geometry,
    delta: mouse::ScrollDelta,
    at: iced::Point,
    shell: &mut Shell<'_, Message>,
) {
    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((pane.on_event)(message));
    let lines = match delta {
        mouse::ScrollDelta::Lines { y, .. } => y * WHEEL_LINES,
        mouse::ScrollDelta::Pixels { y, .. } => y / geometry.cell.height,
    };
    internal.wheel += lines;
    let whole = internal.wheel.trunc();
    internal.wheel -= whole;
    let notches = whole as i32;
    if notches == 0 {
        return;
    }
    let up = notches > 0;
    let mode = pane.state.mode();
    let count = notches.unsigned_abs() as usize;
    if pointer::reported(mode, internal.modifiers) {
        let (col, row, _) = geometry.cell_at(at);
        let report = pointer::report(Pointer::Wheel { up }, col, row, internal.modifiers, mode);
        if let Some(report) = report {
            // A report a notch, as xterm sends.
            send(shell, Action::Input(report.repeat(count.div_ceil(3))));
        }
        shell.capture_event();
        return;
    }
    if mode.contains(TermMode::ALT_SCREEN | TermMode::ALTERNATE_SCROLL) {
        let arrow = match (up, mode.contains(TermMode::APP_CURSOR)) {
            (true, true) => "\x1bOA",
            (true, false) => "\x1b[A",
            (false, true) => "\x1bOB",
            (false, false) => "\x1b[B",
        };
        send(shell, Action::Input(arrow.repeat(count)));
        shell.capture_event();
        return;
    }
    let laid = &internal.laid;
    let can_move = if up {
        laid.offset < laid.history
    } else {
        laid.offset > 0
    };
    if can_move {
        send(shell, Action::Scroll(Scroll::Lines(-notches)));
        shell.capture_event();
    }
}

fn key<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    event: &keyboard::Event,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
) {
    let keyboard::Event::KeyPressed {
        key,
        modifiers,
        text,
        location,
        physical_key,
        ..
    } = event
    else {
        return;
    };
    let press = keys::Press {
        key,
        modifiers: *modifiers,
        text: text.as_deref(),
        location: *location,
        physical: keys::latin(physical_key),
    };
    let state = pane.state;
    let Some(action) = keys::action(&press, state.mode(), pane.mac) else {
        return;
    };
    let message = match action {
        KeyAction::Send(bytes) => Some(Action::Input(bytes)),
        KeyAction::Copy => state.has_selection().then_some(Action::Copy),
        KeyAction::Paste => clipboard.read(clipboard::Kind::Standard).map(Action::Paste),
        KeyAction::Find => Some(Action::OpenSearch),
        KeyAction::SelectAll => Some(Action::SelectAll),
        KeyAction::Scroll(to) => Some(Action::Scroll(to.into())),
        KeyAction::Nothing => None,
    };
    if let Some(message) = message {
        shell.publish((pane.on_event)(message));
    }
    // A key of the terminal's: the cursor shows, and nothing else hears it.
    internal.since = Instant::now();
    shell.request_redraw();
    shell.capture_event();
}

/// At each frame: the room, the theme and the focus the state should know
/// of, the cursor's next beat, and where the input method composes.
fn frame<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    geometry: &Geometry,
    now: Instant,
    shell: &mut Shell<'_, Message>,
) {
    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((pane.on_event)(message));
    let state = pane.state;
    internal.now = now;
    let focused = internal.focus.is_focused();
    if focused != state.is_focused() {
        internal.since = now;
        send(shell, Action::Focus(focused));
    }
    if geometry.viewport() != state.viewport() {
        send(shell, Action::Viewport(geometry.viewport()));
    }
    if state.is_behind() {
        // Output waits: the next part is read before the next frame.
        send(shell, Action::Read);
        shell.request_redraw();
    }
    let drawn = *internal.dark.borrow();
    if let Some(dark) = drawn.filter(|dark| *dark != state.is_dark()) {
        send(shell, Action::Dark(dark));
    }
    if !focused {
        return;
    }
    if state.blinks() && state.term().cursor_style().blinking {
        let beat = BLINK.as_millis();
        let into = now.saturating_duration_since(internal.since).as_millis() % beat;
        let wait = u64::try_from(beat - into).unwrap_or(0);
        shell.request_redraw_at(now + std::time::Duration::from_millis(wait));
    }
    let at = internal
        .laid
        .cursor
        .as_ref()
        .map_or(geometry.text, |cursor| {
            geometry.cells(cursor.row, cursor.col, cursor.col + cursor.cells)
        });
    shell.request_input_method(&InputMethod::Enabled {
        cursor: at,
        purpose: Purpose::Terminal,
        preedit: internal.preedit.as_ref().map(Preedit::as_ref),
    });
}

#[allow(clippy::too_many_lines)]
pub(super) fn update<Message>(
    pane: &Pane<'_, Message>,
    tree: &mut Tree,
    event: &Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
) {
    modality::note(event);
    let internal = tree.state.downcast_mut::<Internal>();
    let geometry = pane.geometry(internal, layout.bounds());
    match event {
        Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
            internal.modifiers = *modifiers;
        }
        Event::Mouse(mouse::Event::ButtonPressed(button)) => {
            if !cursor.is_over(geometry.bounds) {
                internal.focus.unfocus();
                return;
            }
            if shell.is_event_captured() {
                return;
            }
            refocus(internal);
            internal.since = Instant::now();
            shell.request_redraw();
            shell.capture_event();
            if let (mouse::Button::Left, Some(at)) = (button, cursor.position_over(geometry.track))
            {
                match thumb_of(internal, &geometry) {
                    Some(thumb) if thumb.contains(at) => internal.grab = Some(at.y - thumb.y),
                    Some(thumb) => {
                        internal.grab = Some(thumb.height / 2.0);
                        moved(pane, internal, &geometry, at, shell);
                    }
                    None => {}
                }
                return;
            }
            if let Some(at) = cursor.position_over(geometry.text) {
                press(pane, internal, &geometry, *button, at, shell);
            }
        }
        Event::Mouse(mouse::Event::CursorMoved { position }) => {
            moved(pane, internal, &geometry, *position, shell);
        }
        Event::Mouse(mouse::Event::ButtonReleased(button)) => {
            internal.grab = None;
            internal.selecting = false;
            if let Some(held) = internal.reported.take() {
                let mode = pane.state.mode();
                let (col, row) = cursor.position().map_or(held.cell, |at| {
                    let (col, row, _) = geometry.cell_at(at);
                    (col, row)
                });
                let released = Button::of(*button).unwrap_or(held.button);
                let report = pointer::report(
                    Pointer::Release(released),
                    col,
                    row,
                    internal.modifiers,
                    mode,
                );
                if let Some(report) = report {
                    shell.publish((pane.on_event)(Action::Input(report)));
                }
            }
        }
        Event::Mouse(mouse::Event::WheelScrolled { delta }) if !shell.is_event_captured() => {
            if let Some(at) = cursor.position_over(geometry.bounds) {
                wheel(pane, internal, &geometry, *delta, at, shell);
            }
        }
        Event::Keyboard(event @ keyboard::Event::KeyPressed { .. })
            if internal.focus.is_focused() && !shell.is_event_captured() =>
        {
            key(pane, internal, event, clipboard, shell);
        }
        Event::InputMethod(event) if internal.focus.is_focused() => {
            match event {
                input_method::Event::Opened | input_method::Event::Closed => {
                    internal.preedit =
                        matches!(event, input_method::Event::Opened).then(Preedit::new);
                }
                input_method::Event::Preedit(content, selection) => {
                    internal.preedit = Some(Preedit {
                        content: content.clone(),
                        selection: selection.clone(),
                        text_size: Some(Pixels(pane.state.font_size())),
                    });
                }
                input_method::Event::Commit(text) => {
                    internal.preedit = Some(Preedit::new());
                    if !text.is_empty() {
                        shell.publish((pane.on_event)(Action::Input(text.clone())));
                    }
                }
            }
            shell.request_redraw();
            shell.capture_event();
        }
        Event::Window(window::Event::RedrawRequested(now)) => {
            frame(pane, internal, &geometry, *now, shell);
        }
        _ => {}
    }
}
