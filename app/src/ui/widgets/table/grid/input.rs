//! What the table hears: the pointer on its rows, its header's edges and
//! its scrollbar, the wheel, and the keyboard while its rows have focus.
//!
//! | Key                         | What it does                                  |
//! | --------------------------- | --------------------------------------------- |
//! | Up, Down                    | the row before, the row after                 |
//! | Page Up, Page Down          | a screen of rows up or down                   |
//! | Home, End                   | the first row, the last                       |
//! | Shift with any of those     | selects from where the selection started      |
//! | Space                       | selects or deselects the row                  |
//! | Enter                       | opens the row                                 |
//! | Ctrl or Cmd + A             | selects every row                             |
//! | Escape                      | deselects everything                          |
//! | Menu, Shift + F10           | opens the row's menu                          |
//! | Tab                         | leaves the rows: they are one stop            |

use iced::{
    advanced::{
        mouse,
        widget::{operation::Focusable, Tree},
        Clipboard, Layout, Shell,
    },
    keyboard::{self, key::Named, Key, Modifiers},
    window as iced_window, Event, Rectangle,
};

use super::{Drag, Geometry, Grid, Internal, THUMB_MIN};
use crate::ui::widgets::{
    focus::modality,
    table::{
        column,
        state::{Message as Action, Motion},
        window,
    },
};

/// What a notch of the wheel scrolls, as iced's scrollable.
const LINE_STEP: f32 = 60.0;

/// Gives the rows focus anew, so it shows or not by what was last used.
fn refocus(internal: &mut Internal) {
    internal.focus.unfocus();
    internal.focus.focus();
}

fn key_message<K>(key: &Key, modifiers: Modifiers, has_menu: bool) -> Option<Action<K>> {
    if modifiers.alt() {
        return None;
    }
    let motion = |to| Action::Move {
        to,
        extend: modifiers.shift(),
    };
    Some(match key.as_ref() {
        Key::Named(Named::ArrowUp) => motion(Motion::Up),
        Key::Named(Named::ArrowDown) => motion(Motion::Down),
        Key::Named(Named::PageUp) => motion(Motion::PageUp),
        Key::Named(Named::PageDown) => motion(Motion::PageDown),
        Key::Named(Named::Home) => motion(Motion::Home),
        Key::Named(Named::End) => motion(Motion::End),
        Key::Named(Named::Space) if !modifiers.command() => Action::ToggleFocused,
        Key::Named(Named::Enter) if !modifiers.command() => Action::OpenFocused,
        Key::Named(Named::Escape) => Action::Clear,
        Key::Character("a" | "A") if modifiers.command() => Action::SelectAll,
        Key::Named(Named::ContextMenu) if has_menu => Action::MenuFor(None),
        Key::Named(Named::F10) if has_menu && modifiers.shift() => Action::MenuFor(None),
        _ => return None,
    })
}

/// Where the thumb is in the window, if the rows scroll.
fn thumb<K, Message>(grid: &Grid<'_, K, Message>, geometry: &Geometry) -> Option<Rectangle> {
    let rail = geometry.rail();
    let (top, length) = window::thumb(
        grid.offset,
        rail.height,
        grid.extent(),
        rail.height,
        THUMB_MIN,
    )?;
    Some(Rectangle {
        y: rail.y + top,
        height: length,
        ..rail
    })
}

pub(super) fn thumb_of<K, Message>(
    grid: &Grid<'_, K, Message>,
    geometry: &Geometry,
) -> Option<Rectangle> {
    geometry.scrolls.then(|| thumb(grid, geometry)).flatten()
}

/// The edge of a header and the scrollbar: a press there is theirs, and
/// the cells never hear it. Returns whether the event was one of those.
fn drags<K: Clone, Message>(
    grid: &Grid<'_, K, Message>,
    internal: &mut Internal,
    event: &Event,
    geometry: &Geometry,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) -> bool {
    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((grid.on_event)(message));
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            let at = cursor.position();
            if let (Some(column), Some(at)) = (grid.handle_under(geometry, cursor), at) {
                internal.drag = Some(Drag {
                    column,
                    from_x: at.x,
                    from_width: geometry.widths[column],
                });
                shell.capture_event();
                return true;
            }
            let rail = geometry.rail();
            let (Some(thumb), Some(at)) = (thumb_of(grid, geometry), cursor.position_over(rail))
            else {
                return false;
            };
            // On the thumb: it is held where it was taken. Beside it: its
            // middle comes under the pointer.
            let grab = if thumb.contains(at) {
                at.y - thumb.y
            } else {
                thumb.height / 2.0
            };
            internal.grab = Some(grab);
            let offset = window::offset_of_thumb(
                at.y - rail.y - grab,
                rail.height,
                grid.extent(),
                rail.height,
                THUMB_MIN,
            );
            send(shell, Action::Scrolled(offset));
            shell.capture_event();
            true
        }
        Event::Mouse(mouse::Event::CursorMoved { position }) => {
            if let Some(drag) = internal.drag {
                if let Some(id) = grid.ids[drag.column] {
                    let width = column::resized(
                        &grid.tracks[drag.column],
                        drag.from_width,
                        position.x - drag.from_x,
                        grid.rtl,
                    );
                    send(shell, Action::Resize { column: id, width });
                }
                shell.capture_event();
                return true;
            }
            let Some(grab) = internal.grab else {
                return false;
            };
            let rail = geometry.rail();
            let offset = window::offset_of_thumb(
                position.y - rail.y - grab,
                rail.height,
                grid.extent(),
                rail.height,
                THUMB_MIN,
            );
            send(shell, Action::Scrolled(offset));
            shell.capture_event();
            true
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            let resized = internal.drag.take().is_some();
            if resized {
                send(shell, Action::Resized);
            }
            resized || internal.grab.take().is_some()
        }
        _ => false,
    }
}

/// A press on the rows: on a row, it is told to the table; anywhere, it
/// says whether the rows have focus.
fn press<K: Clone, Message>(
    grid: &Grid<'_, K, Message>,
    internal: &mut Internal,
    button: mouse::Button,
    geometry: &Geometry,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) {
    let over = cursor.is_over(geometry.body) && !grid.rows.is_empty();
    if !over {
        internal.focus.unfocus();
        return;
    }
    refocus(internal);
    shell.request_redraw();
    let (Some(index), false) = (grid.row_under(geometry, cursor), shell.is_event_captured()) else {
        return;
    };
    let row = &grid.rows[index - grid.first];
    let message = match button {
        mouse::Button::Left => {
            let at = cursor.position().unwrap_or_default();
            let click = mouse::Click::new(at, mouse::Button::Left, internal.click);
            let twice =
                click.kind() == mouse::click::Kind::Double && internal.hovered == Some(index);
            internal.click = Some(click);
            if twice {
                Action::Opened(row.key.clone())
            } else {
                Action::Pressed {
                    key: row.key.clone(),
                    index,
                    modifiers: internal.modifiers,
                }
            }
        }
        mouse::Button::Right if grid.has_menu => Action::MenuFor(Some((row.key.clone(), index))),
        _ => return,
    };
    shell.publish((grid.on_event)(message));
    shell.capture_event();
}

#[allow(clippy::too_many_arguments)]
pub(super) fn update<K: Clone, Message: Clone>(
    grid: &mut Grid<'_, K, Message>,
    tree: &mut Tree,
    event: &Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    renderer: &iced::Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
    viewport: &Rectangle,
) {
    modality::note(event);
    let geometry = grid.geometry(layout.bounds());
    let columns = grid.columns();
    let Tree {
        state, children, ..
    } = tree;
    let internal = state.downcast_mut::<Internal>();
    if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
        internal.modifiers = *modifiers;
    }
    if grid.rows.is_empty() {
        // Nothing to be on: an empty list takes no focus.
        internal.focus.unfocus();
    }
    if drags(grid, internal, event, &geometry, cursor, shell) {
        return;
    }

    // The cells first: a button in a row keeps its press. What is scrolled
    // under the header is out of the pointer's reach.
    let over_rows = if cursor.is_over(geometry.body) {
        cursor
    } else {
        mouse::Cursor::Unavailable
    };
    let shift = internal.modifiers.shift();
    let cells = grid
        .children
        .iter_mut()
        .zip(children.iter_mut())
        .zip(layout.children());
    for (index, ((child, tree), layout)) in cells.enumerate() {
        let widget = child.as_widget_mut();
        match grid.additive.get(index).filter(|_| shift) {
            // A header pressed with Shift adds its column to the sort.
            Some(Some(additive)) => {
                let mut own = vec![];
                let mut inner = Shell::new(&mut own);
                widget.update(
                    tree, event, layout, cursor, renderer, clipboard, &mut inner, viewport,
                );
                shell.merge(inner, |_| additive.clone());
            }
            _ => {
                let cursor = if index < columns { cursor } else { over_rows };
                widget.update(
                    tree, event, layout, cursor, renderer, clipboard, shell, viewport,
                );
            }
        }
    }

    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((grid.on_event)(message));
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(button)) => {
            press(grid, internal, *button, &geometry, cursor, shell);
        }
        Event::Mouse(mouse::Event::WheelScrolled { delta })
            if cursor.is_over(layout.bounds()) && !shell.is_event_captured() =>
        {
            let dy = match delta {
                mouse::ScrollDelta::Lines { y, .. } => y * LINE_STEP,
                mouse::ScrollDelta::Pixels { y, .. } => *y,
            };
            let offset = window::clamp(
                grid.offset - dy,
                grid.count,
                grid.row_height,
                geometry.body.height,
            );
            // At either end the wheel is the page's.
            if (offset - grid.offset).abs() > f32::EPSILON {
                send(shell, Action::Scrolled(offset));
                shell.capture_event();
            }
        }
        Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
            if internal.focus.is_focused() && !shell.is_event_captured() =>
        {
            // A key while the rows have focus: someone is at the keyboard.
            refocus(internal);
            shell.request_redraw();
            let message = key_message(key, *modifiers, grid.has_menu)
                .filter(|message| !matches!(message, Action::Clear) || grid.any_selected);
            if let Some(message) = message {
                send(shell, message);
                shell.capture_event();
            }
        }
        Event::Window(iced_window::Event::RedrawRequested(_))
            if !grid.placeholder && (geometry.body.height - grid.viewport).abs() > 0.5 =>
        {
            send(shell, Action::Viewport(geometry.body.height));
        }
        _ => {}
    }

    // The row and the edge under the pointer are drawn differently.
    let hovered = grid.row_under(&geometry, cursor);
    let over_handle = grid.handle_under(&geometry, cursor);
    if hovered != internal.hovered || over_handle != internal.over_handle {
        internal.hovered = hovered;
        internal.over_handle = over_handle;
        shell.request_redraw();
    }
}

pub(super) fn mouse_interaction<K: Clone, Message: Clone>(
    grid: &Grid<'_, K, Message>,
    tree: &Tree,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    viewport: &Rectangle,
    renderer: &iced::Renderer,
) -> mouse::Interaction {
    let internal = tree.state.downcast_ref::<Internal>();
    let geometry = grid.geometry(layout.bounds());
    if internal.drag.is_some() || grid.handle_under(&geometry, cursor).is_some() {
        return mouse::Interaction::ResizingHorizontally;
    }
    if internal.grab.is_some() {
        return mouse::Interaction::Idle;
    }
    let columns = grid.columns();
    let over_rows = cursor.is_over(geometry.body);
    let of_cells = grid
        .children
        .iter()
        .zip(&tree.children)
        .zip(layout.children())
        .enumerate()
        .filter(|(index, _)| *index < columns || over_rows)
        .map(|(_, ((child, tree), layout))| {
            child
                .as_widget()
                .mouse_interaction(tree, layout, cursor, viewport, renderer)
        })
        .max()
        .unwrap_or_default();
    if of_cells == mouse::Interaction::None
        && grid.pointer
        && grid.row_under(&geometry, cursor).is_some()
    {
        return mouse::Interaction::Pointer;
    }
    of_cells
}

#[cfg(test)]
mod tests {
    use super::*;

    fn told(key: impl Into<Key>, modifiers: Modifiers) -> Option<Action<u8>> {
        key_message(&key.into(), modifiers, true)
    }

    #[test]
    fn every_key_of_the_rows_says_what_it_does() {
        let none = Modifiers::empty();
        let to = |to, extend| Some(Action::Move { to, extend });
        for (key, motion) in [
            (Named::ArrowUp, Motion::Up),
            (Named::ArrowDown, Motion::Down),
            (Named::PageUp, Motion::PageUp),
            (Named::PageDown, Motion::PageDown),
            (Named::Home, Motion::Home),
            (Named::End, Motion::End),
        ] {
            assert_eq!(told(key, none), to(motion, false));
            assert_eq!(told(key, Modifiers::SHIFT), to(motion, true));
        }
        assert_eq!(told(Named::Space, none), Some(Action::ToggleFocused));
        assert_eq!(told(Named::Enter, none), Some(Action::OpenFocused));
        assert_eq!(told(Named::Escape, none), Some(Action::Clear));
        let a = || Key::Character("a".into());
        assert_eq!(told(a(), Modifiers::COMMAND), Some(Action::SelectAll));
        assert_eq!(told(a(), none), None, "a letter alone is not the table's");
        assert_eq!(told(Named::ContextMenu, none), Some(Action::MenuFor(None)));
        assert_eq!(
            told(Named::F10, Modifiers::SHIFT),
            Some(Action::MenuFor(None))
        );
        assert_eq!(told(Named::F10, none), None);
        // Tab is the window's, and so are the arrows across.
        for key in [Named::Tab, Named::ArrowLeft, Named::ArrowRight] {
            assert_eq!(told(key, none), None);
        }
        assert_eq!(told(Named::ArrowDown, Modifiers::ALT), None);
    }

    #[test]
    fn a_table_without_menus_leaves_the_menu_keys_alone() {
        let none = Modifiers::empty();
        let told = |key: Named, modifiers| key_message::<u8>(&key.into(), modifiers, false);
        assert_eq!(told(Named::ContextMenu, none), None);
        assert_eq!(told(Named::F10, Modifiers::SHIFT), None);
    }
}
