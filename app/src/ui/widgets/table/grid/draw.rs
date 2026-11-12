//! What the table draws: its frame, the header's line, each row's fill and
//! line, the cells, the ring around the row the keyboard is on and the
//! scrollbar. Every colour is the theme's (`TableRow`, `--border`, `--ring`).

use iced::{
    advanced::{mouse, renderer, widget::Tree, Layout, Renderer as _},
    Border, Color, Rectangle,
};

use super::{input::thumb_of, Geometry, Grid, Internal, BORDER, HANDLE, LINE};
use crate::ui::theme::{
    self,
    color::alpha,
    metrics::{control, radius},
    Theme,
};

/// How thick the focus ring is, inside the row.
const RING: f32 = 2.0;

fn fill(renderer: &mut iced::Renderer, bounds: Rectangle, colour: Color) {
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            ..renderer::Quad::default()
        },
        colour,
    );
}

fn ring(renderer: &mut iced::Renderer, bounds: Rectangle, theme: &Theme) {
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: Border {
                color: alpha(theme.tokens().ring, 0.5),
                width: RING,
                radius: radius::SM.into(),
            },
            ..renderer::Quad::default()
        },
        Color::TRANSPARENT,
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw<K: Clone, Message: Clone>(
    grid: &Grid<'_, K, Message>,
    tree: &Tree,
    renderer: &mut iced::Renderer,
    theme: &Theme,
    _style: &renderer::Style,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    viewport: &Rectangle,
) {
    let internal = tree.state.downcast_ref::<Internal>();
    let bounds = layout.bounds();
    let geometry = grid.geometry(bounds);
    let tokens = theme.tokens();
    let columns = grid.columns();
    let shows_focus = internal.focus.is_focus_visible() && !grid.menu_open;
    // A row's text is the page's (`TableRow`), whatever is around the table.
    let style = &renderer::Style {
        text_color: tokens.foreground,
    };

    // `rounded-md border`.
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: Border {
                color: tokens.border,
                width: BORDER,
                radius: radius::MD.into(),
            },
            ..renderer::Quad::default()
        },
        Color::TRANSPARENT,
    );

    let mut cells = grid
        .children
        .iter()
        .zip(&tree.children)
        .zip(layout.children())
        .enumerate();

    // The header: its cells, each cut at its column, and the line under.
    for (column, ((child, tree), layout)) in cells.by_ref().take(columns) {
        let cell = geometry.cell(column, geometry.header.y, geometry.header.height);
        if let Some(clip) = cell.intersection(viewport) {
            child
                .as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, &clip);
        }
    }
    fill(
        renderer,
        Rectangle {
            y: geometry.header.y + geometry.header.height - LINE,
            height: LINE,
            ..geometry.header
        },
        tokens.border,
    );
    if let Some(column) = internal
        .drag
        .map(|drag| drag.column)
        .or(internal.over_handle)
    {
        let edge = if grid.rtl {
            geometry.xs[column]
        } else {
            geometry.xs[column] + geometry.widths[column]
        };
        fill(
            renderer,
            Rectangle {
                x: edge - LINE,
                y: geometry.header.y + HANDLE,
                width: 2.0 * LINE,
                height: geometry.header.height - 2.0 * HANDLE,
            },
            tokens.ring,
        );
    }

    let Some(body) = geometry.body.intersection(viewport) else {
        return;
    };
    if grid.placeholder {
        for (_, ((child, tree), layout)) in cells {
            child
                .as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, &body);
        }
        return;
    }

    let hovered = grid.row_under(&geometry, cursor);
    renderer.with_layer(body, |renderer| {
        rows(grid, &geometry, renderer, theme, hovered, shows_focus);
        for (index, ((child, tree), layout)) in cells {
            let row = grid.first + index / columns - 1;
            let cell = geometry.cell(
                index % columns,
                grid.row_top(&geometry, row),
                grid.row_height,
            );
            if let Some(clip) = cell.intersection(&body) {
                child
                    .as_widget()
                    .draw(tree, renderer, theme, style, layout, cursor, &clip);
            }
        }
    });

    // Focus the keyboard gave, on no row yet: the rows as a whole show it.
    if shows_focus && grid.focused.is_none() {
        ring(renderer, geometry.body, theme);
    }
    if let Some(thumb) = thumb_of(grid, &geometry) {
        let inset = (control::SCROLLBAR - control::SCROLLER) / 2.0;
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: thumb.x + inset,
                    width: control::SCROLLER,
                    ..thumb
                },
                border: Border::default().rounded(radius::FULL),
                ..renderer::Quad::default()
            },
            tokens.border,
        );
    }
}

/// The fill and the line of every row in view, and the ring of the one
/// the keyboard is on.
fn rows<K, Message>(
    grid: &Grid<'_, K, Message>,
    geometry: &Geometry,
    renderer: &mut iced::Renderer,
    theme: &Theme,
    hovered: Option<usize>,
    shows_focus: bool,
) {
    let tokens = theme.tokens();
    for (at, row) in grid.rows.iter().enumerate() {
        let index = grid.first + at;
        let bounds = Rectangle {
            y: grid.row_top(geometry, index),
            height: grid.row_height,
            ..geometry.body
        };
        if bounds.intersection(&geometry.body).is_none() {
            continue;
        }
        let look = theme::container::table_row(hovered == Some(index), row.selected)(theme);
        if let Some(background) = look.background {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    ..renderer::Quad::default()
                },
                background,
            );
        }
        // `border-b`, but under the last row of the list.
        if index + 1 < grid.count {
            fill(
                renderer,
                Rectangle {
                    y: bounds.y + bounds.height - LINE,
                    height: LINE,
                    ..bounds
                },
                tokens.border,
            );
        }
        if shows_focus && grid.focused == Some(index) {
            // Beside the scrollbar, not under it.
            let rail = if geometry.scrolls {
                control::SCROLLBAR
            } else {
                0.0
            };
            ring(
                renderer,
                Rectangle {
                    width: bounds.width - rail,
                    height: bounds.height - LINE,
                    ..bounds
                },
                theme,
            );
        }
    }
}
