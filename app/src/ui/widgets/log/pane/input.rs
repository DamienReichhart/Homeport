//! What the viewer hears: the pointer on its text and its scrollbars, the
//! wheel, and the keyboard while it has focus.
//!
//! | Key                   | What it does                                    |
//! | --------------------- | ----------------------------------------------- |
//! | Up, Down              | a row up, a row down                            |
//! | Page Up, Page Down    | a screen of rows                                |
//! | Home                  | the oldest line                                 |
//! | End                   | the latest line, which is followed again        |
//! | Left, Right           | scroll long lines sideways, when they don't wrap |
//! | Ctrl or Cmd + A       | selects every line                              |
//! | Ctrl or Cmd + C       | copies the selection                            |
//! | Ctrl or Cmd + F       | goes to the search field                        |
//! | F3, Shift + F3        | the next match, the one before                  |
//! | Escape                | deselects                                       |
//! | Tab                   | leaves the rows: they are one stop              |

use iced::{
    advanced::{
        mouse,
        widget::{operation::Focusable, Tree},
        Layout, Shell,
    },
    keyboard::{self, key::Named, Key, Modifiers},
    window as iced_window, Event, Point,
};

use super::{
    draw::{thumb_across, thumb_of},
    Geometry, Internal, Pane, ROW, THUMB_MIN,
};
use crate::ui::widgets::{
    focus::modality,
    log::{
        selection::Mark,
        state::{Message as Action, Scroll},
    },
};

/// The rows a notch of the wheel scrolls, and the cells with Shift.
const WHEEL_ROWS: f32 = 3.0;
const WHEEL_CELLS: f32 = 6.0;
/// The cells an arrow scrolls sideways.
const ARROW_CELLS: i64 = 4;

/// Gives the rows focus anew, so it shows or not by what was last used.
fn refocus(internal: &mut Internal) {
    internal.focus.unfocus();
    internal.focus.focus();
}

/// What a key does while the rows have focus.
pub(super) fn key_message(
    key: &Key,
    modifiers: Modifiers,
    wraps: bool,
    selected: bool,
) -> Option<Action> {
    if modifiers.alt() {
        return None;
    }
    let command = modifiers.command();
    Some(match key.as_ref() {
        Key::Character("a" | "A") if command => Action::SelectAll,
        Key::Character("c" | "C") if command && selected => Action::Copy,
        Key::Character("f" | "F") if command => Action::FocusSearch,
        _ if command => return None,
        Key::Named(Named::ArrowUp) => Action::Scroll(Scroll::Rows(-1)),
        Key::Named(Named::ArrowDown) => Action::Scroll(Scroll::Rows(1)),
        Key::Named(Named::PageUp) => Action::Scroll(Scroll::Pages(-1)),
        Key::Named(Named::PageDown) => Action::Scroll(Scroll::Pages(1)),
        Key::Named(Named::Home) => Action::Scroll(Scroll::Start),
        Key::Named(Named::End) => Action::Scroll(Scroll::End),
        // The text reads left to right whatever the app's language.
        Key::Named(Named::ArrowLeft) if !wraps => Action::Scroll(Scroll::Across(-ARROW_CELLS)),
        Key::Named(Named::ArrowRight) if !wraps => Action::Scroll(Scroll::Across(ARROW_CELLS)),
        Key::Named(Named::F3) if modifiers.shift() => Action::Previous,
        Key::Named(Named::F3) => Action::Next,
        Key::Named(Named::Escape) if selected => Action::Deselect,
        _ => return None,
    })
}

/// The place in the text nearest to `point`.
// A viewer shows a few hundred rows of a few hundred cells.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn mark_at(internal: &Internal, geometry: &Geometry, point: Point) -> Option<Mark> {
    let rows = &internal.laid.rows;
    let last = rows.len().checked_sub(1)?;
    let down = ((point.y - geometry.text.y) / ROW).floor();
    if down > last as f32 {
        // Under the last row: its end.
        return Some(Mark::new(rows[last].seq, rows[last].to));
    }
    let row = &rows[down.max(0.0) as usize];
    if row.marker {
        return Some(Mark::new(row.seq, 0));
    }
    let across = ((point.x - geometry.text.x) / geometry.cell)
        .round()
        .max(0.0) as usize;
    Some(Mark::new(row.seq, (row.from + across).min(row.to)))
}

/// A press on the text: it starts a selection, takes it further with
/// Shift, takes a word when it is the second and a line when the third.
fn press<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    geometry: &Geometry,
    at: Point,
    shell: &mut Shell<'_, Message>,
) {
    let click = mouse::Click::new(at, mouse::Button::Left, internal.click.take());
    let kind = click.kind();
    internal.click = Some(click);
    internal.selecting = true;
    let Some(mark) = mark_at(internal, geometry, at) else {
        return;
    };
    let message = match kind {
        mouse::click::Kind::Single => Action::Press {
            at: mark,
            extend: internal.modifiers.shift(),
        },
        mouse::click::Kind::Double => Action::SelectWord(mark),
        mouse::click::Kind::Triple => Action::SelectLine(mark.seq),
    };
    shell.publish((pane.on_event)(message));
}

/// The wheel: the rows up and down, long lines sideways with Shift (or a
/// wheel that tilts). At either end the wheel is the page's.
// What the wheel scrolls in a frame is a few rows.
#[allow(clippy::cast_possible_truncation)]
fn wheel<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    delta: mouse::ScrollDelta,
    shell: &mut Shell<'_, Message>,
) {
    let (dx, dy) = match delta {
        mouse::ScrollDelta::Lines { x, y } => (x * WHEEL_CELLS, y * WHEEL_ROWS),
        mouse::ScrollDelta::Pixels { x, y } => (x / 8.0, y / ROW),
    };
    let (dx, dy) = if internal.modifiers.shift() {
        (dy * WHEEL_CELLS / WHEEL_ROWS, 0.0)
    } else {
        (dx, dy)
    };
    internal.wheel -= dy;
    let rows = internal.wheel.trunc();
    internal.wheel -= rows;
    let laid = &internal.laid;
    // Down from the latest line there is nowhere to go, unless the view
    // is paused on it: one notch then follows again.
    let can_move = if rows < 0.0 {
        laid.top != crate::ui::widgets::log::display::Pos::default()
    } else {
        !laid.at_end || !pane.state.follows()
    };
    if rows != 0.0 && can_move {
        shell.publish((pane.on_event)(Action::Scroll(Scroll::Rows(rows as i64))));
        shell.capture_event();
    }
    internal.wheel_across -= dx;
    let cells = internal.wheel_across.trunc();
    internal.wheel_across -= cells;
    let most = pane.state.across_max();
    let can_move = if cells < 0.0 {
        pane.state.across > 0
    } else {
        pane.state.across < most
    };
    if cells != 0.0 && can_move {
        shell.publish((pane.on_event)(Action::Scroll(Scroll::Across(
            cells as i64,
        ))));
        shell.capture_event();
    }
}

/// The scrollbars: a press on a thumb holds it, a press beside it brings
/// it there, and a held thumb follows the pointer. Returns whether the
/// event was theirs.
fn bars<Message>(
    pane: &Pane<'_, Message>,
    internal: &mut Internal,
    event: &Event,
    geometry: &Geometry,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) -> bool {
    let send = |shell: &mut Shell<'_, Message>, scroll| {
        shell.publish((pane.on_event)(Action::Scroll(scroll)));
        shell.capture_event();
    };
    // How far a thumb whose start is `start` in a track of `track` is.
    let share = |start: f32, track: f32, length: f32| {
        let travel = (track - length).max(1.0);
        (start / travel).clamp(0.0, 1.0)
    };
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            if let Some(at) = cursor.position_over(geometry.track) {
                let thumb = thumb_of(internal, geometry);
                let Some(thumb) = thumb else { return true };
                if thumb.contains(at) {
                    internal.grab = Some(at.y - thumb.y);
                } else {
                    internal.grab = Some(thumb.height / 2.0);
                    let start = at.y - geometry.track.y - thumb.height / 2.0;
                    let to = share(start, geometry.track.height, thumb.height);
                    send(shell, Scroll::To(to));
                }
                shell.capture_event();
                return true;
            }
            if let Some((bar, at)) = geometry
                .bar
                .and_then(|bar| Some((bar, cursor.position_over(bar)?)))
            {
                let Some(thumb) = thumb_across(pane, geometry) else {
                    return true;
                };
                if thumb.contains(at) {
                    internal.grab_across = Some(at.x - thumb.x);
                } else {
                    internal.grab_across = Some(thumb.width / 2.0);
                    let start = at.x - bar.x - thumb.width / 2.0;
                    send(
                        shell,
                        Scroll::AcrossTo(share(start, bar.width, thumb.width)),
                    );
                }
                shell.capture_event();
                return true;
            }
            false
        }
        Event::Mouse(mouse::Event::CursorMoved { position }) => {
            if let Some(grab) = internal.grab {
                let length = thumb_of(internal, geometry).map_or(THUMB_MIN, |thumb| thumb.height);
                let start = position.y - geometry.track.y - grab;
                send(
                    shell,
                    Scroll::To(share(start, geometry.track.height, length)),
                );
                return true;
            }
            if let (Some(grab), Some(bar)) = (internal.grab_across, geometry.bar) {
                let length = thumb_across(pane, geometry).map_or(THUMB_MIN, |thumb| thumb.width);
                let start = position.x - bar.x - grab;
                send(shell, Scroll::AcrossTo(share(start, bar.width, length)));
                return true;
            }
            false
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            internal.grab.take().is_some() | internal.grab_across.take().is_some()
        }
        _ => false,
    }
}

pub(super) fn update<Message>(
    pane: &Pane<'_, Message>,
    tree: &mut Tree,
    event: &Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) {
    modality::note(event);
    let internal = tree.state.downcast_mut::<Internal>();
    let geometry = pane.geometry(internal, layout.bounds());
    if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
        internal.modifiers = *modifiers;
    }
    if bars(pane, internal, event, &geometry, cursor, shell) {
        shell.request_redraw();
        return;
    }
    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((pane.on_event)(message));
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(button)) => {
            if !cursor.is_over(geometry.bounds) {
                internal.focus.unfocus();
                return;
            }
            if shell.is_event_captured() {
                return;
            }
            refocus(internal);
            shell.request_redraw();
            if let (mouse::Button::Left, Some(at)) = (button, cursor.position_over(geometry.text)) {
                press(pane, internal, &geometry, at, shell);
                shell.capture_event();
            }
        }
        Event::Mouse(mouse::Event::CursorMoved { position }) if internal.selecting => {
            // Past the rows, the view goes with the pointer and the
            // selection with the view, in one step.
            let past = if position.y < geometry.text.y {
                Some(false)
            } else if position.y > geometry.text.y + geometry.text.height {
                Some(true)
            } else {
                None
            };
            // A viewer is a few hundred cells wide.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let cell = ((position.x - geometry.text.x) / geometry.cell)
                .round()
                .max(0.0) as usize;
            if let Some(down) = past {
                send(shell, Action::DragPast { down, cell });
            } else if let Some(mark) = mark_at(internal, &geometry, *position) {
                send(shell, Action::Drag(mark));
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            internal.selecting = false;
        }
        Event::Mouse(mouse::Event::WheelScrolled { delta })
            if cursor.is_over(geometry.bounds) && !shell.is_event_captured() =>
        {
            wheel(pane, internal, *delta, shell);
        }
        Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
            if internal.focus.is_focused() && !shell.is_event_captured() =>
        {
            let state = pane.state;
            if let Some(message) =
                key_message(key, *modifiers, state.wraps(), state.has_selection())
            {
                // A key of the rows: someone is at the keyboard, and the
                // ring shows.
                refocus(internal);
                shell.request_redraw();
                send(shell, message);
                shell.capture_event();
            }
        }
        Event::Window(iced_window::Event::RedrawRequested(_))
            if geometry.metrics != pane.state.metrics =>
        {
            send(shell, Action::Viewport(geometry.metrics));
        }
        _ => {}
    }
}
