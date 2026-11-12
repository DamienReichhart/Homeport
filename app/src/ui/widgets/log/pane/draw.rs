//! What the viewer draws: its frame on the terminal's background, then,
//! row by row, what lies behind the text (a line of stderr, a program's
//! background, the selection, the matches of the search), the text, and
//! over it all the focus ring and the scrollbars. Every colour is the
//! theme's terminal palette.

use iced::{
    advanced::{renderer, text::Renderer as _, widget::Tree, Layout, Renderer as _},
    Border, Color, Point, Rectangle,
};

use super::{
    rows::{Paint, Row},
    Geometry, Internal, Pane, BORDER, PADDING, ROW, THUMB_MIN,
};
use crate::ui::{
    theme::{
        self,
        color::alpha,
        metrics::{control, radius},
        Theme,
    },
    widgets::{
        log::{
            ansi::Attrs,
            line::Stream,
            palette::{self, DIM},
        },
        table::window,
    },
};

/// How thick the focus ring is, inside the frame.
const RING: f32 = 2.0;
/// How much of the Signal red a line of stderr is tinted with, and its
/// mark before the line.
const STDERR_TINT: f32 = 0.07;
const STDERR_MARK: f32 = 0.7;
/// The match the search stands on, in the dark theme: amber seen through,
/// so the light text on it still reads.
const ACTIVE_DARK: f32 = 0.45;

fn fill(renderer: &mut iced::Renderer, bounds: Rectangle, colour: Color) {
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            ..renderer::Quad::default()
        },
        colour,
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

/// The thumb beside the rows: where the view is among the lines.
// Lines are counted well within what an `f32` holds whole.
#[allow(clippy::cast_precision_loss)]
pub(super) fn thumb_of(internal: &Internal, geometry: &Geometry) -> Option<Rectangle> {
    let laid = &internal.laid;
    let (top, length) = window::thumb(
        laid.top.index as f32,
        laid.lines.max(1) as f32,
        laid.count as f32,
        geometry.track.height,
        THUMB_MIN,
    )?;
    Some(Rectangle {
        y: geometry.track.y + top,
        height: length,
        ..geometry.track
    })
}

/// The thumb under the rows: how far long lines are scrolled.
#[allow(clippy::cast_precision_loss)]
pub(super) fn thumb_across<Message>(
    pane: &Pane<'_, Message>,
    geometry: &Geometry,
) -> Option<Rectangle> {
    let bar = geometry.bar?;
    let cols = geometry.metrics.cols;
    let (left, length) = window::thumb(
        pane.state.across as f32,
        cols as f32,
        (pane.state.across_max() + cols) as f32,
        bar.width,
        THUMB_MIN,
    )?;
    Some(Rectangle {
        x: bar.x + left,
        width: length,
        ..bar
    })
}

/// The cells `from..to` of a row as a rectangle.
fn span(geometry: &Geometry, row: Rectangle, from: usize, to: usize) -> Rectangle {
    Rectangle {
        x: row.x + geometry.across(from),
        width: geometry.across(to - from),
        ..row
    }
}

fn row<Message>(
    pane: &Pane<'_, Message>,
    renderer: &mut iced::Renderer,
    theme: &Theme,
    geometry: &Geometry,
    bounds: Rectangle,
    row: &Row,
) {
    let terminal = &theme.tokens().terminal;
    let clip = geometry.text;
    if row.stream == Stream::Stderr && !row.marker {
        let signal = theme.tokens().destructive;
        fill(renderer, bounds, alpha(signal, STDERR_TINT));
        let mark = Rectangle {
            x: bounds.x - PADDING + 2.0,
            width: 2.0,
            ..bounds
        };
        fill(renderer, mark, alpha(signal, STDERR_MARK));
    }
    // What a program put behind its text.
    for piece in &row.pieces {
        if let Paint::Text(style) = piece.paint {
            if let (_, Some(behind)) = palette::colours(terminal, style) {
                let area = span(geometry, bounds, piece.col, piece.col + piece.cells);
                fill(renderer, area, behind);
            }
        }
    }
    let width = row.to - row.from;
    if let Some((from, to)) = pane.state.selection.and_then(|s| s.cells_of(row.seq)) {
        // A line selected to its end shows it with one cell more.
        let end = if to == usize::MAX && row.last {
            row.to + 1
        } else {
            to.min(row.to)
        };
        let from = from.max(row.from);
        if from < end {
            let area = span(geometry, bounds, from - row.from, end - row.from);
            fill(renderer, area, terminal.selection);
        }
    }
    for (from, to, current) in &row.hits {
        let area = span(geometry, bounds, *from, (*to).min(width.max(1)));
        if *current {
            let amber = terminal.search_active;
            let seen = if theme.is_dark() { ACTIVE_DARK } else { 1.0 };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: area,
                    border: Border {
                        color: amber,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..renderer::Quad::default()
                },
                alpha(amber, seen),
            );
        } else {
            fill(renderer, area, terminal.search_match);
        }
    }
    for piece in &row.pieces {
        let (colour, lined) = match piece.paint {
            Paint::Text(style) => (palette::colours(terminal, style).0, style.attrs),
            Paint::Stamp | Paint::Marker => (alpha(terminal.foreground, DIM), Attrs::default()),
            Paint::Label(source) => (palette::source(terminal, source), Attrs::default()),
        };
        let at = Point::new(bounds.x + geometry.across(piece.col), bounds.y);
        renderer.fill_paragraph(&piece.paragraph, at, colour, clip);
        let area = span(geometry, bounds, piece.col, piece.col + piece.cells);
        if lined.has(Attrs::UNDERLINE) {
            let line = Rectangle {
                y: bounds.y + ROW - 2.0,
                height: 1.0,
                ..area
            };
            fill(renderer, line, colour);
        }
        if lined.has(Attrs::STRIKE) {
            let line = Rectangle {
                y: bounds.y + (ROW / 2.0).floor(),
                height: 1.0,
                ..area
            };
            fill(renderer, line, colour);
        }
    }
}

pub(super) fn draw<Message>(
    pane: &Pane<'_, Message>,
    tree: &Tree,
    renderer: &mut iced::Renderer,
    theme: &Theme,
    layout: Layout<'_>,
    viewport: &Rectangle,
) {
    let internal = tree.state.downcast_ref::<Internal>();
    let tokens = theme.tokens();
    let bounds = layout.bounds();
    let geometry = pane.geometry(internal, bounds);
    // `rounded-md border`, on the terminal's own background.
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
        tokens.terminal.background,
    );
    // The rows, cut at the frame: what is behind a line of stderr goes
    // from one edge to the other.
    let inside = Rectangle {
        x: geometry.text.x - PADDING,
        width: geometry.text.width + 2.0 * PADDING,
        ..geometry.text
    };
    if let Some(clip) = inside.intersection(viewport) {
        renderer.with_layer(clip, |renderer| {
            for (index, laid) in internal.laid.rows.iter().enumerate() {
                let bounds = Rectangle {
                    x: inside.x,
                    width: inside.width,
                    ..geometry.row(index)
                };
                let text = Rectangle {
                    x: geometry.text.x,
                    ..bounds
                };
                // The tint of a line of stderr fills the row; the rest is
                // placed from where text starts.
                row(pane, renderer, theme, &geometry, text, laid);
            }
        });
    }
    if internal.focus.is_focus_visible() {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: alpha(tokens.ring, 0.5),
                    width: RING,
                    radius: radius::MD.into(),
                },
                ..renderer::Quad::default()
            },
            Color::TRANSPARENT,
        );
    }
    if let Some(bounds) = thumb_of(internal, &geometry) {
        thumb(renderer, theme, bounds, true);
    }
    if let Some(bounds) = thumb_across(pane, &geometry) {
        thumb(renderer, theme, bounds, false);
    }
}
