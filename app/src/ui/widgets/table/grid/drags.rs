//! What the pointer holds and drags: the edge of a header, which resizes
//! its column, and the thumbs of the two scrollbars.

use iced::{
    advanced::{mouse, Shell},
    window as iced_window, Event, Rectangle,
};

use super::{geometry::Geometry, Drag, Grid, Internal, THUMB_MIN};
use crate::ui::widgets::table::{column, state::Message as Action, window};

/// Where the thumb beside the rows is, if they scroll.
pub(super) fn thumb_of<K, Message>(
    grid: &Grid<'_, K, Message>,
    geometry: &Geometry,
) -> Option<Rectangle> {
    let rail = geometry.rail;
    let (top, length) = geometry
        .scrolls
        .then(|| {
            window::thumb(
                grid.offset,
                rail.height,
                grid.extent(),
                rail.height,
                THUMB_MIN,
            )
        })
        .flatten()?;
    Some(Rectangle {
        y: rail.y + top,
        height: length,
        ..rail
    })
}

/// Where the thumb under the rows is, if the columns scroll. It starts
/// where text starts.
pub(super) fn thumb_across<K, Message>(
    grid: &Grid<'_, K, Message>,
    geometry: &Geometry,
) -> Option<Rectangle> {
    let rail = geometry.rail_across;
    let (start, length) = geometry
        .overflows
        .then(|| {
            window::thumb(
                geometry.across,
                rail.width,
                geometry.content,
                rail.width,
                THUMB_MIN,
            )
        })
        .flatten()?;
    let x = if grid.rtl {
        rail.x + rail.width - start - length
    } else {
        rail.x + start
    };
    Some(Rectangle {
        x,
        width: length,
        ..rail
    })
}

/// Lets go of whatever the pointer holds. Returns whether it held the
/// edge of a header.
fn let_go(internal: &mut Internal) -> bool {
    internal.grab = None;
    internal.grab_across = None;
    internal.drag.take().is_some()
}

/// The edge of a header and the scrollbars: a press there is theirs, and
/// the cells never hear it. Returns whether the event was one of those.
pub(super) fn drags<K: Clone, Message>(
    grid: &Grid<'_, K, Message>,
    internal: &mut Internal,
    event: &Event,
    geometry: &Geometry,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) -> bool {
    let send = |shell: &mut Shell<'_, Message>, message| shell.publish((grid.on_event)(message));
    // A drag whose column is gone, or whose release was never heard (the
    // window lost the pointer), is over.
    let column = internal
        .drag
        .and_then(|drag| grid.ids.iter().position(|id| *id == Some(drag.column)));
    let lost = matches!(
        event,
        Event::Mouse(mouse::Event::CursorLeft | mouse::Event::ButtonPressed(_))
            | Event::Window(iced_window::Event::Unfocused)
    );
    let gone = internal.drag.is_some() && column.is_none();
    if (gone || lost) && let_go(internal) {
        send(shell, Action::Resized);
    }
    let rail = geometry.rail;
    let across = geometry.rail_across;
    let offset_at = |y: f32, grab: f32| {
        window::offset_of_thumb(
            y - rail.y - grab,
            rail.height,
            grid.extent(),
            rail.height,
            THUMB_MIN,
        )
    };
    let across_at = |x: f32, grab: f32| {
        // From where text starts.
        let from_start = if grid.rtl {
            across.x + across.width - x
        } else {
            x - across.x
        };
        window::offset_of_thumb(
            from_start - grab,
            across.width,
            geometry.content,
            across.width,
            THUMB_MIN,
        )
    };
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            let Some(at) = cursor.position() else {
                return false;
            };
            if let Some(column) = grid.handle_under(geometry, cursor) {
                let Some(id) = grid.ids[column] else {
                    return false;
                };
                let previous = internal
                    .handle_click
                    .take()
                    .filter(|(of, _)| *of == id)
                    .map(|(_, click)| click);
                let click = mouse::Click::new(at, mouse::Button::Left, previous);
                if click.kind() == mouse::click::Kind::Double {
                    // Twice on an edge: the column as it was.
                    send(shell, Action::ResetWidth(id));
                } else {
                    internal.handle_click = Some((id, click));
                    internal.drag = Some(Drag {
                        column: id,
                        from_x: at.x,
                        from_width: geometry.widths[column],
                    });
                }
                shell.capture_event();
                return true;
            }
            if let (Some(thumb), true) = (thumb_of(grid, geometry), cursor.is_over(rail)) {
                // On the thumb: it is held where it was taken. Beside it:
                // its middle comes under the pointer.
                let grab = if thumb.contains(at) {
                    at.y - thumb.y
                } else {
                    thumb.height / 2.0
                };
                internal.grab = Some(grab);
                send(shell, Action::Scrolled(offset_at(at.y, grab)));
                shell.capture_event();
                return true;
            }
            if let (Some(thumb), true) = (thumb_across(grid, geometry), cursor.is_over(across)) {
                let grab = if thumb.contains(at) {
                    if grid.rtl {
                        thumb.x + thumb.width - at.x
                    } else {
                        at.x - thumb.x
                    }
                } else {
                    thumb.width / 2.0
                };
                internal.grab_across = Some(grab);
                send(shell, Action::ScrolledAcross(across_at(at.x, grab)));
                shell.capture_event();
                return true;
            }
            false
        }
        Event::Mouse(mouse::Event::CursorMoved { position }) => {
            if let (Some(drag), Some(column)) = (internal.drag, column) {
                let wanted = column::resized(
                    &grid.tracks[column],
                    drag.from_width,
                    position.x - drag.from_x,
                    grid.rtl,
                );
                // No further than what the other columns leave.
                let width = wanted.min(column::room(&grid.tracks, column, geometry.room()));
                send(
                    shell,
                    Action::Resize {
                        column: drag.column,
                        width,
                    },
                );
            } else if let Some(grab) = internal.grab {
                send(shell, Action::Scrolled(offset_at(position.y, grab)));
            } else if let Some(grab) = internal.grab_across {
                send(shell, Action::ScrolledAcross(across_at(position.x, grab)));
            } else {
                return false;
            }
            shell.capture_event();
            true
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            let held = internal.grab.is_some() || internal.grab_across.is_some();
            let resized = let_go(internal);
            if resized {
                send(shell, Action::Resized);
            }
            resized || held
        }
        _ => false,
    }
}
