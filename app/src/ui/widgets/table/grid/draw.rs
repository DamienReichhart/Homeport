//! What the table draws: its frame, the header's line, each line's fill
//! and hairline, the cells (each cut where its column ends), the ring
//! around the line the keyboard is on and the scrollbars. Every colour is
//! the theme's (`TableRow`, `--border`, `--ring`, the table's thumb).

use iced::{
    advanced::{mouse, renderer, widget::Tree, Layout, Renderer as _},
    Border, Color, Rectangle,
};

use super::{
    drags::{thumb_across, thumb_of},
    geometry::Geometry,
    Focus, Grid, Internal, What, BORDER, HANDLE, LINE,
};
use crate::ui::theme::{
    self,
    color::alpha,
    metrics::{control, radius},
    Element, Theme,
};

/// How thick the focus ring is, inside the line.
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

/// A scrollbar's thumb, thinner than its rail.
fn thumb(renderer: &mut iced::Renderer, theme: &Theme, bounds: Rectangle, upright: bool) {
    let inset = (control::SCROLLBAR - control::SCROLLER) / 2.0;
    let bounds = if upright {
        Rectangle {
            x: bounds.x + inset,
            width: control::SCROLLER,
            ..bounds
        }
    } else {
        Rectangle {
            y: bounds.y + inset,
            height: control::SCROLLER,
            ..bounds
        }
    };
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: Border::default().rounded(radius::FULL),
            ..renderer::Quad::default()
        },
        theme::scrollable::table_thumb(theme),
    );
}

/// What is needed to draw a cell.
struct Pen<'p, 'a, Message> {
    theme: &'p Theme,
    style: &'p renderer::Style,
    cursor: mouse::Cursor,
    cells: Vec<(&'p Element<'a, Message>, &'p Tree, Layout<'p>)>,
}

impl<Message> Pen<'_, '_, Message> {
    /// Draws the cell `index` inside `clip`, in a layer of its own: text
    /// is cut by where it may show, an icon or a badge only by a layer.
    fn cell(&self, renderer: &mut iced::Renderer, index: usize, clip: Rectangle) {
        let (child, tree, layout) = self.cells[index];
        child.as_widget().draw(
            tree,
            renderer,
            self.theme,
            self.style,
            layout,
            self.cursor,
            &clip,
        );
    }
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
    let geometry = Geometry::of(grid, bounds);
    let tokens = theme.tokens();
    let columns = grid.columns();
    let shows_focus = internal.focus.is_focus_visible() && !grid.menu_open;
    // A row's text is the page's (`TableRow`), whatever is around the table.
    let style = &renderer::Style {
        text_color: tokens.foreground,
    };
    let pen = Pen {
        theme,
        style,
        cursor,
        cells: grid
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, tree), layout)| (child, tree, layout))
            .collect(),
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

    // The header: its cells, each cut at its column and at the table.
    if let Some(heads) = geometry.heads().intersection(viewport) {
        for column in 0..columns {
            let cell = geometry.cell(column, geometry.header.y, geometry.header.height);
            if let Some(clip) = cell.intersection(&heads) {
                renderer.with_layer(clip, |renderer| pen.cell(renderer, column, clip));
            }
        }
        let held = internal
            .drag
            .and_then(|drag| grid.ids.iter().position(|id| *id == Some(drag.column)));
        if let Some(column) = held.or(internal.over_handle).filter(|at| *at < columns) {
            let edge = Rectangle {
                x: geometry.edge(column, grid.rtl) - LINE,
                y: geometry.header.y + HANDLE,
                width: 2.0 * LINE,
                height: geometry.header.height - 2.0 * HANDLE,
            };
            if let Some(edge) = edge.intersection(&heads) {
                fill(renderer, edge, tokens.ring);
            }
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

    let Some(body) = geometry.body.intersection(viewport) else {
        return;
    };
    if grid.placeholder {
        renderer.with_layer(body, |renderer| {
            pen.cell(renderer, pen.cells.len() - 1, body)
        });
        return;
    }

    let hovered = grid.row_under(&geometry, cursor);
    // Under the cells: the lines' fills and hairlines, and the wide cells.
    renderer.with_layer(body, |renderer| {
        lines(grid, &geometry, renderer, theme, hovered);
        for (at, row) in grid.rows.iter().enumerate() {
            if let Some((from, _)) = row.span {
                let cell = grid.cell_of(&geometry, at, from, grid.row_height);
                if let Some(clip) = cell.intersection(&body) {
                    pen.cell(renderer, columns * (1 + at) + from, clip);
                }
            }
        }
    });
    // The cells, a column at a time: one layer cuts them all.
    for column in 0..columns {
        let whole = Rectangle {
            x: geometry.xs[column],
            width: geometry.widths[column],
            ..body
        };
        let Some(clip) = whole.intersection(&body) else {
            continue;
        };
        renderer.with_layer(clip, |renderer| {
            for (at, row) in grid.rows.iter().enumerate() {
                let spanned = row
                    .span
                    .is_some_and(|(from, to)| (from..to).contains(&column));
                if !spanned {
                    pen.cell(renderer, columns * (1 + at) + column, clip);
                }
            }
        });
    }
    // Over them: the ring of the line the keyboard is on, or of the rows
    // as a whole when it is on none yet. On a line out of view: no ring.
    if shows_focus {
        renderer.with_layer(body, |renderer| match grid.focused {
            Focus::At(index) => ring(
                renderer,
                Rectangle {
                    y: grid.row_top(&geometry, index),
                    height: grid.row_height - LINE,
                    ..geometry.body
                },
                theme,
            ),
            Focus::None => ring(renderer, geometry.body, theme),
            Focus::Away => {}
        });
    }
    if let Some(bounds) = thumb_of(grid, &geometry) {
        thumb(renderer, theme, bounds, true);
    }
    if let Some(bounds) = thumb_across(grid, &geometry) {
        thumb(renderer, theme, bounds, false);
    }
}

/// The fill and the hairline of every line in view.
fn lines<K, Message>(
    grid: &Grid<'_, K, Message>,
    geometry: &Geometry,
    renderer: &mut iced::Renderer,
    theme: &Theme,
    hovered: Option<usize>,
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
        let hovered = hovered == Some(index);
        let look = match row.what {
            What::Row(_) => theme::container::table_row(hovered, row.selected)(theme),
            What::Group { .. } => theme::container::table_group(hovered)(theme),
            What::Skeleton => theme::container::transparent(theme),
        };
        if let Some(background) = look.background {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    ..renderer::Quad::default()
                },
                background,
            );
        }
        // `border-b`, but under the last line of the list.
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
    }
}
