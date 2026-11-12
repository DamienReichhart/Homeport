//! Where everything is in the bounds the table was given: the header, the
//! room of the rows, each column, and the two scrollbars' rails.

use iced::Rectangle;

use super::{Grid, BORDER};
use crate::ui::{
    theme::metrics::control,
    widgets::table::{column, state::HEADER_HEIGHT},
};

pub struct Geometry {
    pub header: Rectangle,
    /// The room of the rows: beside the rail when they scroll, over the
    /// rail across when the columns do.
    pub body: Rectangle,
    pub widths: Vec<f32>,
    /// Where each column starts, scrolled.
    pub xs: Vec<f32>,
    /// The rows are taller than their room.
    pub scrolls: bool,
    /// The columns are wider than their room.
    pub overflows: bool,
    /// How wide the columns are together.
    pub content: f32,
    /// How far the columns are scrolled, within what they allow.
    pub across: f32,
    /// The rail of the scrollbar beside the rows: where text ends.
    pub rail: Rectangle,
    /// The rail of the scrollbar under the rows.
    pub rail_across: Rectangle,
}

impl Geometry {
    pub fn of<K, Message>(grid: &Grid<'_, K, Message>, bounds: Rectangle) -> Self {
        let inner = bounds.shrink(BORDER);
        let header = Rectangle {
            height: HEADER_HEIGHT.min(inner.height),
            ..inner
        };
        let bar = control::SCROLLBAR;
        let fits = |widths: &[f32], room: f32| widths.iter().sum::<f32>() <= room + 0.5;
        // Columns too wide whether the rows scroll or not take a line of
        // the rows' room for their scrollbar; then the rows may scroll,
        // which takes a rail's width, which may be what makes the columns
        // too wide.
        let unscrolled = column::resolve(&grid.tracks, inner.width);
        let wide = !fits(&unscrolled, inner.width);
        let mut height = (inner.height - header.height - if wide { bar } else { 0.0 }).max(0.0);
        let scrolls = !grid.placeholder && grid.extent() > height + 0.5;
        let room = inner.width - if scrolls { bar } else { 0.0 };
        let widths = if scrolls {
            column::resolve(&grid.tracks, room)
        } else {
            unscrolled
        };
        let overflows = !fits(&widths, room);
        if overflows && !wide {
            height = (height - bar).max(0.0);
        }
        let content: f32 = widths.iter().sum();
        let across = grid.across.clamp(0.0, (content - room).max(0.0));
        // The rail is where text ends: on the left, right to left.
        let (start, rail_x) = if grid.rtl {
            (inner.x + if scrolls { bar } else { 0.0 }, inner.x)
        } else {
            (inner.x, inner.x + inner.width - bar)
        };
        let shift = if grid.rtl { across } else { -across };
        let xs = column::offsets(&widths, room, grid.rtl)
            .into_iter()
            .map(|x| start + x + shift)
            .collect();
        let body = Rectangle {
            x: start,
            y: inner.y + header.height,
            width: room,
            height,
        };
        Self {
            header,
            body,
            widths,
            xs,
            scrolls,
            overflows,
            content,
            across,
            rail: Rectangle {
                x: rail_x,
                width: bar,
                ..body
            },
            rail_across: Rectangle {
                y: body.y + body.height,
                height: bar,
                ..body
            },
        }
    }

    /// The part of the header over the columns: its cells never show or
    /// hear the pointer outside it.
    pub fn heads(&self) -> Rectangle {
        Rectangle {
            x: self.body.x,
            width: self.body.width,
            ..self.header
        }
    }

    pub fn cell(&self, column: usize, y: f32, height: f32) -> Rectangle {
        Rectangle {
            x: self.xs[column],
            y,
            width: self.widths[column],
            height,
        }
    }

    /// One cell over the columns `from..to`.
    pub fn span(&self, from: usize, to: usize, y: f32, height: f32) -> Rectangle {
        let cells = (from..to.min(self.xs.len())).map(|column| self.cell(column, y, height));
        cells
            .reduce(|all, cell| all.union(&cell))
            .unwrap_or_else(|| self.cell(from, y, height))
    }

    /// The edge a column ends at, which a drag moves.
    pub fn edge(&self, column: usize, rtl: bool) -> f32 {
        if rtl {
            self.xs[column]
        } else {
            self.xs[column] + self.widths[column]
        }
    }

    /// The most a column may be dragged to: what the others leave.
    pub fn room(&self) -> f32 {
        self.body.width
    }
}
