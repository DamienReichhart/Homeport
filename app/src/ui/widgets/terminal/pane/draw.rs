//! What the grid draws: its frame on the terminal's background, then, row
//! by row, what a program put behind its text, the selection, the
//! search's matches, the text and its lines, the link under the pointer;
//! then the cursor, the focus ring and the scrollbar. Every colour is the
//! theme's terminal palette.

use std::time::Duration;

use alacritty_terminal::{
    index::{Line, Point},
    vte::ansi::CursorShape,
};
use iced::{
    advanced::{
        renderer,
        text::Renderer as _,
        widget::{operation::Focusable, Tree},
        Layout, Renderer as _,
    },
    Border, Color, Rectangle,
};

use super::{
    boxes,
    rows::{Kind, Row},
    Geometry, Internal, Pane, BORDER, THUMB_MIN,
};
use crate::ui::{
    theme::{
        self,
        color::alpha,
        metrics::{control, radius},
        tokens::Terminal,
        Theme,
    },
    widgets::{
        log::{ansi::Attrs, ansi::Style, palette},
        table::window,
    },
};

/// How thick the focus ring is, inside the frame.
const RING: f32 = 2.0;
/// The match the search stands on, in the dark theme: amber seen through,
/// so the light text on it still reads (the log viewer's).
const ACTIVE_DARK: f32 = 0.45;
/// How long the cursor shows, then hides, while it blinks (xterm.js's).
pub(super) const BLINK: Duration = Duration::from_millis(600);

fn fill(renderer: &mut iced::Renderer, bounds: Rectangle, colour: Color) {
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            ..renderer::Quad::default()
        },
        colour,
    );
}

/// The cells `from..to` that a range of the grid covers on `line`, in a
/// grid of `cols`: a selection's or a match's.
pub(super) fn span_on(
    line: Line,
    start: Point,
    end: Point,
    block: bool,
    cols: usize,
) -> Option<(usize, usize)> {
    if line < start.line || line > end.line {
        return None;
    }
    if block {
        return Some((start.column.0, end.column.0 + 1));
    }
    let from = if line == start.line {
        start.column.0
    } else {
        0
    };
    let to = if line == end.line {
        end.column.0 + 1
    } else {
        cols
    };
    (from < to).then_some((from, to))
}

/// The colours of `style`, kept for the next draw.
fn colours_of(internal: &Internal, terminal: &Terminal, style: Style) -> (Color, Option<Color>) {
    let mut colours = internal.colours.borrow_mut();
    if let Some(known) = colours.get(&style) {
        return *known;
    }
    // A program of 24-bit colours has as many looks as it likes: past a
    // screen's worth of them, they are worked out again.
    if colours.len() >= COLOURS_KEPT {
        colours.clear();
    }
    #[cfg(test)]
    COLOURED.with(|coloured| coloured.set(coloured.get() + 1));
    let worked_out = palette::colours(terminal, style);
    colours.insert(style, worked_out);
    #[cfg(test)]
    HELD.with(|held| held.set(held.get().max(colours.len())));
    worked_out
}

/// How many looks' colours are kept between draws.
pub(in crate::ui::widgets::terminal) const COLOURS_KEPT: usize = 4_096;

#[cfg(test)]
thread_local! {
    /// How many looks' colours this thread worked out.
    pub(crate) static COLOURED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// The most looks it held at once.
    pub(crate) static HELD: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn row<Message>(
    pane: &Pane<'_, Message>,
    internal: &Internal,
    renderer: &mut iced::Renderer,
    theme: &Theme,
    geometry: &Geometry,
    index: usize,
    laid: &Row,
) {
    let terminal = &theme.tokens().terminal;
    let state = pane.state;
    let clip = geometry.text;
    // What is behind the text, runs of one colour drawn as one.
    let mut behind: Option<(usize, usize, Color)> = None;
    for piece in &laid.pieces {
        let colour = colours_of(internal, terminal, piece.style).1;
        let (from, to) = (piece.col, piece.col + piece.cells);
        match (behind, colour) {
            (Some((start, end, open)), Some(colour)) if end == from && open == colour => {
                behind = Some((start, to, open));
            }
            (open, colour) => {
                if let Some((start, end, open)) = open {
                    fill(renderer, geometry.cells(index, start, end), open);
                }
                behind = colour.map(|colour| (from, to, colour));
            }
        }
    }
    if let Some((start, end, colour)) = behind {
        fill(renderer, geometry.cells(index, start, end), colour);
    }
    let term = state.term();
    if let Some(range) = term.selection.as_ref().and_then(|s| s.to_range(term)) {
        if let Some((from, to)) = span_on(
            laid.line,
            range.start,
            range.end,
            range.is_block,
            geometry.cols,
        ) {
            fill(
                renderer,
                geometry.cells(index, from, to),
                terminal.selection,
            );
        }
    }
    for (found, current) in state.search().within(laid.line, laid.line) {
        let Some((from, to)) = span_on(
            laid.line,
            *found.start(),
            *found.end(),
            false,
            geometry.cols,
        ) else {
            continue;
        };
        let area = geometry.cells(index, from, to);
        if current {
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
    let height = geometry.cell.height;
    for piece in &laid.pieces {
        let (colour, _) = colours_of(internal, terminal, piece.style);
        let area = geometry.cells(index, piece.col, piece.col + piece.cells);
        match &piece.kind {
            Kind::Blank => {}
            Kind::Text { paragraph, inset } => {
                let at = iced::Point::new(area.x + inset, area.y);
                renderer.fill_paragraph(paragraph, at, colour, clip);
            }
            Kind::Drawn(c) => boxes::draw(renderer, *c, area, colour),
        }
        let attrs = piece.style.attrs;
        if attrs.has(Attrs::UNDERLINE) {
            let line = Rectangle {
                y: area.y + height - 2.0,
                height: 1.0,
                ..area
            };
            fill(renderer, line, colour);
        }
        if attrs.has(Attrs::STRIKE) {
            let line = Rectangle {
                y: area.y + (height / 2.0).floor(),
                height: 1.0,
                ..area
            };
            fill(renderer, line, colour);
        }
    }
    // The link under the pointer is underlined, as a browser does.
    if let Some((col, hovered)) = internal.hovered {
        if hovered == index {
            if let Some(link) = laid
                .links
                .iter()
                .find(|link| (link.from..link.to).contains(&col))
            {
                let area = geometry.cells(index, link.from, link.to);
                let line = Rectangle {
                    y: area.y + height - 1.0,
                    height: 1.0,
                    ..area
                };
                fill(renderer, line, terminal.foreground);
            }
        }
    }
}

/// Whether the cursor shows at this frame: always, unless it blinks and
/// is in the half of its beat where it hides.
fn cursor_shows<Message>(pane: &Pane<'_, Message>, internal: &Internal) -> bool {
    let blinks =
        pane.state.blinks() && pane.state.term().cursor_style().blinking && pane.state.is_focused();
    if !blinks {
        return true;
    }
    let beats = internal
        .now
        .saturating_duration_since(internal.since)
        .as_millis()
        / BLINK.as_millis();
    beats.is_multiple_of(2)
}

fn cursor<Message>(
    pane: &Pane<'_, Message>,
    internal: &Internal,
    renderer: &mut iced::Renderer,
    terminal: &Terminal,
    geometry: &Geometry,
) {
    let Some(cursor) = &internal.laid.cursor else {
        return;
    };
    if !cursor_shows(pane, internal) {
        return;
    }
    let area = geometry.cells(cursor.row, cursor.col, cursor.col + cursor.cells);
    if !internal.focus.is_focused() {
        // Without focus, a hollow block, as every terminal draws it.
        renderer.fill_quad(
            renderer::Quad {
                bounds: area,
                border: Border {
                    color: terminal.cursor,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..renderer::Quad::default()
            },
            Color::TRANSPARENT,
        );
        return;
    }
    match cursor.shape {
        CursorShape::Underline => {
            let bar = Rectangle {
                y: area.y + area.height - 2.0,
                height: 2.0,
                ..area
            };
            fill(renderer, bar, terminal.cursor);
        }
        CursorShape::Beam => {
            let bar = Rectangle { width: 2.0, ..area };
            fill(renderer, bar, terminal.cursor);
        }
        CursorShape::HollowBlock => {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: area,
                    border: Border {
                        color: terminal.cursor,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
        CursorShape::Block | CursorShape::Hidden => {
            fill(renderer, area, terminal.cursor);
            if let Some(glyph) = &cursor.glyph {
                let at = iced::Point::new(area.x, area.y);
                renderer.fill_paragraph(glyph, at, terminal.cursor_accent, geometry.text);
            }
            if let Some(c) = cursor.drawn {
                boxes::draw(renderer, c, area, terminal.cursor_accent);
            }
        }
    }
}

/// The scrollbar's thumb: where the view is in the scrollback.
// Lines are counted well within what an `f32` holds whole.
#[allow(clippy::cast_precision_loss)]
pub(super) fn thumb_of(internal: &Internal, geometry: &Geometry) -> Option<Rectangle> {
    let laid = &internal.laid;
    let rows = laid.rows.len();
    let (top, length) = window::thumb(
        (laid.history - laid.offset.min(laid.history)) as f32,
        rows as f32,
        (laid.history + rows) as f32,
        geometry.track.height,
        THUMB_MIN,
    )?;
    Some(Rectangle {
        y: geometry.track.y + top,
        height: length,
        ..geometry.track
    })
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
    let dark = theme.is_dark();
    if internal.dark.replace(Some(dark)) != Some(dark) {
        internal.colours.borrow_mut().clear();
    }
    let bounds = layout.bounds();
    let geometry = pane.geometry(internal, bounds);
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
    if let Some(clip) = geometry.text.intersection(viewport) {
        renderer.with_layer(clip, |renderer| {
            for (index, laid) in internal.laid.rows.iter().enumerate() {
                row(pane, internal, renderer, theme, &geometry, index, laid);
            }
            cursor(pane, internal, renderer, &tokens.terminal, &geometry);
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
    if let Some(thumb) = thumb_of(internal, &geometry) {
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
            theme::scrollable::table_thumb(theme),
        );
    }
}
