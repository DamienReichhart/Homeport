//! What the table hears: the pointer on its lines, its header's edges and
//! its scrollbars, the wheel, and the keyboard while its rows have focus.
//!
//! | Key                         | What it does                                  |
//! | --------------------------- | --------------------------------------------- |
//! | Up, Down                    | the line before, the line after               |
//! | Page Up, Page Down          | a screen of lines up or down                  |
//! | Home, End                   | the first line, the last                      |
//! | Shift with any of those     | selects from where the selection started      |
//! | Space                       | selects or deselects the row                  |
//! | Enter                       | opens the row                                 |
//! | Space, Enter on a group     | closes or opens the group                     |
//! | Left, Right on a group      | closes it, opens it (mirrored right to left)  |
//! | Left, Right elsewhere       | scroll the columns that don't fit             |
//! | Ctrl or Cmd + A             | selects every row that shows                  |
//! | Escape                      | deselects everything                          |
//! | Menu, Shift + F10           | opens the row's menu                          |
//! | Letters                     | go to the next row that starts with them      |
//! | Tab                         | leaves the rows: they are one stop            |

use std::time::{Duration, Instant};

use iced::{
    advanced::{
        mouse,
        widget::{operation::Focusable, Tree},
        Clipboard, Layout, Shell,
    },
    keyboard::{self, key::Named, Key, Modifiers},
    window as iced_window, Event, Rectangle,
};

use super::{drags::drags, geometry::Geometry, Focus, Grid, Internal, What};
use crate::ui::widgets::{
    focus::modality,
    table::{
        state::{Message as Action, Motion},
        window,
    },
};

/// What a notch of the wheel scrolls, as iced's scrollable.
const LINE_STEP: f32 = 60.0;
/// What an arrow scrolls sideways.
const ARROW_STEP: f32 = 40.0;
/// Typing pauses this long before it starts a new word (Radix's second).
const TYPEAHEAD: Duration = Duration::from_secs(1);

/// Gives the rows focus anew, so it shows or not by what was last used.
fn refocus(internal: &mut Internal) {
    internal.focus.unfocus();
    internal.focus.focus();
}

pub(super) fn key_message<K>(key: &Key, modifiers: Modifiers, has_menu: bool) -> Option<Action<K>> {
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

/// What an arrow across does: on the header of a group it closes or
/// opens it, elsewhere it scrolls the columns that don't fit.
fn across_message<K, Message>(
    grid: &Grid<'_, K, Message>,
    geometry: &Geometry,
    key: &Key,
    modifiers: Modifiers,
) -> Option<Action<K>> {
    let right = match key.as_ref() {
        Key::Named(Named::ArrowRight) => true,
        Key::Named(Named::ArrowLeft) => false,
        _ => return None,
    };
    if modifiers.alt() || modifiers.command() {
        return None;
    }
    // Towards where text ends: it opens a group, and shows what is further.
    let onward = right != grid.rtl;
    let on_group = match grid.focused {
        Focus::At(index) => grid.row(index).and_then(|row| match &row.what {
            What::Group { name, .. } => Some(name),
            _ => None,
        }),
        _ => None,
    };
    if let Some(name) = on_group {
        return Some(Action::Collapse {
            name: name.clone(),
            collapsed: !onward,
        });
    }
    let step = if onward { ARROW_STEP } else { -ARROW_STEP };
    let most = (geometry.content - geometry.body.width).max(0.0);
    let across = (geometry.across + step).clamp(0.0, most);
    ((across - geometry.across).abs() > f32::EPSILON).then_some(Action::ScrolledAcross(across))
}

/// A letter typed, added to what was typed within the last second.
fn typed(internal: &mut Internal, key: &Key, modifiers: Modifiers) -> Option<String> {
    let Key::Character(letter) = key else {
        return None;
    };
    if modifiers.command() || modifiers.alt() || letter.chars().any(char::is_control) {
        return None;
    }
    let now = Instant::now();
    if internal
        .typed_at
        .is_none_or(|at| now.duration_since(at) > TYPEAHEAD)
    {
        internal.typed.clear();
    }
    internal.typed_at = Some(now);
    internal.typed.push_str(letter);
    Some(internal.typed.clone())
}

/// A press on the rows: on a line, it is told to the table; anywhere, it
/// says whether the rows have focus.
fn press<K: Clone, Message>(
    grid: &Grid<'_, K, Message>,
    internal: &mut Internal,
    button: mouse::Button,
    geometry: &Geometry,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) {
    let over = cursor.is_over(geometry.body) && grid.is_interactive();
    if !over {
        internal.focus.unfocus();
        return;
    }
    refocus(internal);
    shell.request_redraw();
    let (Some(index), false) = (grid.row_under(geometry, cursor), shell.is_event_captured()) else {
        return;
    };
    let Some(row) = grid.row(index) else {
        return;
    };
    let message = match (&row.what, button) {
        (What::Row(key), mouse::Button::Left) => {
            let at = cursor.position().unwrap_or_default();
            // Twice on the same row, not twice at the same place: the
            // rows may have moved under the pointer in between.
            let previous = internal
                .click
                .take()
                .filter(|(id, _)| *id == row.id)
                .map(|(_, click)| click);
            let click = mouse::Click::new(at, mouse::Button::Left, previous);
            let twice = click.kind() == mouse::click::Kind::Double;
            internal.click = Some((row.id, click));
            if twice {
                Action::Opened(key.clone())
            } else {
                Action::Pressed {
                    key: key.clone(),
                    index,
                    modifiers: internal.modifiers,
                }
            }
        }
        (What::Row(key), mouse::Button::Right) if grid.has_menu => {
            Action::MenuFor(Some((key.clone(), index)))
        }
        (What::Group { name, .. }, mouse::Button::Left) => Action::GroupPressed {
            name: name.clone(),
            index,
        },
        _ => return,
    };
    shell.publish((grid.on_event)(message));
    shell.capture_event();
}

/// The wheel: the rows up and down, the columns across (with Shift, or
/// a wheel that tilts).
fn wheel<K: Clone, Message>(
    grid: &Grid<'_, K, Message>,
    internal: &Internal,
    delta: mouse::ScrollDelta,
    geometry: &Geometry,
    shell: &mut Shell<'_, Message>,
) {
    let (dx, dy) = match delta {
        mouse::ScrollDelta::Lines { x, y } => (x * LINE_STEP, y * LINE_STEP),
        mouse::ScrollDelta::Pixels { x, y } => (x, y),
    };
    let (dx, dy) = if internal.modifiers.shift() {
        (dy, 0.0)
    } else {
        (dx, dy)
    };
    let offset = window::clamp(
        grid.offset - dy,
        grid.count,
        grid.row_height,
        geometry.body.height,
    );
    // At either end the wheel is the page's.
    if (offset - grid.offset).abs() > f32::EPSILON {
        shell.publish((grid.on_event)(Action::Scrolled(offset)));
        shell.capture_event();
    }
    let most = (geometry.content - geometry.body.width).max(0.0);
    let onward = if grid.rtl { dx } else { -dx };
    let across = (geometry.across + onward).clamp(0.0, most);
    if (across - geometry.across).abs() > f32::EPSILON {
        shell.publish((grid.on_event)(Action::ScrolledAcross(across)));
        shell.capture_event();
    }
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
    let geometry = Geometry::of(grid, layout.bounds());
    let columns = grid.columns();
    let Tree {
        state, children, ..
    } = tree;
    let internal = state.downcast_mut::<Internal>();
    if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
        internal.modifiers = *modifiers;
    }
    if !grid.is_interactive() {
        // Nothing to be on: an empty list takes no focus.
        internal.focus.unfocus();
    }
    if drags(grid, internal, event, &geometry, cursor, shell) {
        return;
    }

    // The cells first: a button in a row keeps its press. What is scrolled
    // under the header or out of the table is out of the pointer's reach.
    let within = |area: Rectangle| {
        if cursor.is_over(area) {
            cursor
        } else {
            mouse::Cursor::Unavailable
        }
    };
    let (over_heads, over_rows) = (within(geometry.heads()), within(geometry.body));
    let shift = internal.modifiers.shift();
    let cells = grid
        .children
        .iter_mut()
        .zip(children.iter_mut())
        .zip(layout.children());
    for (index, ((child, tree), layout)) in cells.enumerate() {
        let widget = child.as_widget_mut();
        let cursor = if index < columns {
            over_heads
        } else {
            over_rows
        };
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
            _ => widget.update(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            ),
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
            wheel(grid, internal, *delta, &geometry, shell);
        }
        Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
            if internal.focus.is_focused() && !shell.is_event_captured() =>
        {
            let message = key_message(key, *modifiers, grid.has_menu)
                .filter(|message| !matches!(message, Action::Clear) || grid.any_selected)
                .or_else(|| across_message(grid, &geometry, key, *modifiers))
                .or_else(|| {
                    grid.types
                        .then(|| typed(internal, key, *modifiers))
                        .flatten()
                        .map(Action::Typed)
                });
            if let Some(message) = message {
                // A key of the rows: someone is at the keyboard, and the
                // ring shows. A modifier held for a press is not one.
                refocus(internal);
                shell.request_redraw();
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

    // The line and the edge under the pointer are drawn differently.
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
    let geometry = Geometry::of(grid, layout.bounds());
    if internal.drag.is_some() || grid.handle_under(&geometry, cursor).is_some() {
        return mouse::Interaction::ResizingHorizontally;
    }
    if internal.grab.is_some() || internal.grab_across.is_some() {
        return mouse::Interaction::Idle;
    }
    let columns = grid.columns();
    let (over_heads, over_rows) = (
        cursor.is_over(geometry.heads()),
        cursor.is_over(geometry.body),
    );
    let of_cells = grid
        .children
        .iter()
        .zip(&tree.children)
        .zip(layout.children())
        .enumerate()
        .filter(|(index, _)| {
            if *index < columns {
                over_heads
            } else {
                over_rows
            }
        })
        .map(|(_, ((child, tree), layout))| {
            child
                .as_widget()
                .mouse_interaction(tree, layout, cursor, viewport, renderer)
        })
        .max()
        .unwrap_or_default();
    if of_cells != mouse::Interaction::None {
        return of_cells;
    }
    let pressable = grid
        .row_under(&geometry, cursor)
        .and_then(|index| grid.row(index))
        .is_some_and(|row| match row.what {
            What::Row(_) => grid.pointer,
            What::Group { .. } => true,
            What::Skeleton => false,
        });
    if pressable {
        mouse::Interaction::Pointer
    } else {
        of_cells
    }
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
        assert_eq!(told(a(), none), None, "a letter alone is typing's");
        assert_eq!(told(Named::ContextMenu, none), Some(Action::MenuFor(None)));
        assert_eq!(
            told(Named::F10, Modifiers::SHIFT),
            Some(Action::MenuFor(None))
        );
        assert_eq!(told(Named::F10, none), None);
        // Tab is the window's; a modifier alone is nobody's.
        for key in [Named::Tab, Named::Control, Named::Shift] {
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

    #[test]
    fn letters_typed_within_a_second_make_a_word() {
        let mut internal = Internal::default();
        let none = Modifiers::empty();
        let letter = |letter: &str| Key::Character(letter.into());
        assert_eq!(typed(&mut internal, &letter("w"), none), Some("w".into()));
        assert_eq!(typed(&mut internal, &letter("e"), none), Some("we".into()));
        assert_eq!(typed(&mut internal, &letter("n"), Modifiers::COMMAND), None);
        assert_eq!(typed(&mut internal, &Named::Space.into(), none), None);
        // After the pause, a new word.
        internal.typed_at = Instant::now().checked_sub(TYPEAHEAD * 2);
        assert_eq!(typed(&mut internal, &letter("d"), none), Some("d".into()));
    }
}
