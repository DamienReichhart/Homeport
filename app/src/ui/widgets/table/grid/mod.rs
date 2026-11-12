//! The table as iced sees it: one widget that lays out the header and the
//! rows in view, scrolls them, hears the pointer and the keyboard, and
//! draws what belongs to rows rather than to their cells (the fill of a
//! hovered or selected row, the lines, the focus ring, the scrollbar).
//!
//! It keeps nothing the page should know: how far it is scrolled, what is
//! selected and how wide the columns are come from the table's `State` and
//! go back to it as messages. Its own state is what iced keeps for a
//! widget: whether it has focus, a drag in progress, the last press.

mod draw;
mod input;

use std::collections::HashMap;

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::Modifiers,
    Event, Length, Point, Rectangle, Size, Vector,
};

use super::{
    column::{self, Align, Track},
    state::{Message as Action, HEADER_HEIGHT},
    window, ColumnId,
};
use crate::ui::{
    theme::{metrics::control, Element, Theme},
    widgets::focus::pressable,
};

/// The frame around the table.
const BORDER: f32 = 1.0;
/// The hairline under the header and under every row.
const LINE: f32 = 1.0;
/// How far from the edge of a header a press still takes the edge.
const HANDLE: f32 = 4.0;
/// The shortest the scrollbar's thumb gets.
const THUMB_MIN: f32 = 24.0;

/// A row in view: who it is, and whether it is selected.
pub struct Row<K> {
    pub key: K,
    /// The key, hashed: what the row's widgets are kept under.
    pub id: u64,
    pub selected: bool,
}

type OnEvent<'a, K, Message> = Box<dyn Fn(Action<K>) -> Message + 'a>;

pub struct Grid<'a, K, Message> {
    /// One per column, the checkboxes' and the menus' included.
    pub tracks: Vec<Track>,
    /// The columns the user may resize, by name.
    pub ids: Vec<Option<ColumnId>>,
    /// The header's cells, then every cell of every row in view, then
    /// what stands in for the rows, if anything does.
    pub children: Vec<Element<'a, Message>>,
    /// What a header sends when it is pressed with Shift, for those that sort.
    pub additive: Vec<Option<Message>>,
    pub rows: Vec<Row<K>>,
    /// The place of the first row in view among those that show.
    pub first: usize,
    /// How many rows show, in view or not.
    pub count: usize,
    pub offset: f32,
    pub row_height: f32,
    /// How much room the view assumed the rows have.
    pub viewport: f32,
    /// Whether the last child stands in for the rows (empty, loading, failed).
    pub placeholder: bool,
    /// The row the keyboard is on, if it is in view.
    pub focused: Option<usize>,
    /// A row's menu is open: it has the keyboard, and the one ring.
    pub menu_open: bool,
    pub has_menu: bool,
    pub any_selected: bool,
    /// A press opens the row: the pointer says so.
    pub pointer: bool,
    /// Takes all the height it is given, however few the rows.
    pub fill: bool,
    pub max_height: f32,
    pub rtl: bool,
    pub on_event: OnEvent<'a, K, Message>,
}

/// A header's edge being dragged.
#[derive(Debug, Clone, Copy)]
struct Drag {
    column: usize,
    from_x: f32,
    from_width: f32,
}

/// What iced keeps for the table between views.
#[derive(Debug, Default)]
pub struct Internal {
    focus: pressable::State,
    /// The rows whose widgets the tree holds, in order, and how many
    /// columns each has.
    ids: Vec<u64>,
    columns: usize,
    placeholder: bool,
    modifiers: Modifiers,
    drag: Option<Drag>,
    /// The thumb is held, this far from its top.
    grab: Option<f32>,
    hovered: Option<usize>,
    over_handle: Option<usize>,
    click: Option<mouse::Click>,
}

/// Where everything is, for bounds the table was given.
struct Geometry {
    inner: Rectangle,
    header: Rectangle,
    body: Rectangle,
    widths: Vec<f32>,
    /// Where each column starts.
    xs: Vec<f32>,
    /// The rows are taller than the body: there is a scrollbar.
    scrolls: bool,
}

impl Geometry {
    fn cell(&self, column: usize, y: f32, height: f32) -> Rectangle {
        Rectangle {
            x: self.xs[column],
            y,
            width: self.widths[column],
            height,
        }
    }

    /// The scrollbar's track.
    fn rail(&self) -> Rectangle {
        Rectangle {
            x: self.body.x + self.body.width - control::SCROLLBAR,
            width: control::SCROLLBAR,
            ..self.body
        }
    }
}

impl<K, Message> Grid<'_, K, Message> {
    fn extent(&self) -> f32 {
        window::extent(self.count, self.row_height)
    }

    fn geometry(&self, bounds: Rectangle) -> Geometry {
        let inner = bounds.shrink(BORDER);
        let header = Rectangle {
            height: HEADER_HEIGHT.min(inner.height),
            ..inner
        };
        let body = Rectangle {
            y: inner.y + header.height,
            height: (inner.height - header.height).max(0.0),
            ..inner
        };
        let scrolls = !self.placeholder && self.extent() > body.height + 0.5;
        let available = inner.width - if scrolls { control::SCROLLBAR } else { 0.0 };
        let widths = column::resolve(&self.tracks, available);
        let xs = column::offsets(&widths, available, self.rtl)
            .into_iter()
            .map(|x| inner.x + x)
            .collect();
        Geometry {
            inner,
            header,
            body,
            widths,
            xs,
            scrolls,
        }
    }

    /// Where the row at `index` among those that show is drawn.
    fn row_top(&self, geometry: &Geometry, index: usize) -> f32 {
        geometry.body.y + window::top_of(index, self.row_height) - self.offset
    }

    /// The row under the pointer, among those that show.
    fn row_under(&self, geometry: &Geometry, cursor: mouse::Cursor) -> Option<usize> {
        let at = cursor.position_in(geometry.body)?;
        if geometry.scrolls && cursor.is_over(geometry.rail()) {
            return None;
        }
        let index = window::row_at(at.y, self.offset, self.row_height, self.count)?;
        (index >= self.first && index < self.first + self.rows.len()).then_some(index)
    }

    /// The column whose edge is under the pointer.
    fn handle_under(&self, geometry: &Geometry, cursor: mouse::Cursor) -> Option<usize> {
        let at = cursor.position_over(geometry.header)?;
        (0..self.tracks.len()).find(|&column| {
            let edge = if self.rtl {
                geometry.xs[column]
            } else {
                geometry.xs[column] + geometry.widths[column]
            };
            self.tracks[column].resizable
                && self.ids[column].is_some()
                && (at.x - edge).abs() <= HANDLE
        })
    }

    fn columns(&self) -> usize {
        self.tracks.len()
    }

    /// The limits and the place of a cell's content in `cell`.
    fn place(&self, column: usize, cell: Rectangle, node: layout::Node) -> layout::Node {
        let track = &self.tracks[column];
        let room = (cell.width - 2.0 * track.padding).max(0.0);
        let size = node.size();
        let start = cell.x + track.padding;
        let end = start + room - size.width;
        let x = match (track.align, self.rtl) {
            (Align::Center, _) => start + (room - size.width) / 2.0,
            (Align::Start, false) | (Align::End, true) => start,
            (Align::End, false) | (Align::Start, true) => end,
        };
        let y = cell.y + (cell.height - size.height) / 2.0;
        // Whole pixels: text between two is drawn blurred.
        node.move_to(Point::new(x.max(start).round(), y.round()))
    }
}

impl<K, Message> Widget<Message, Theme, iced::Renderer> for Grid<'_, K, Message>
where
    K: Clone,
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Internal>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Internal::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    /// A row's widgets follow the row: as the rows in view change, each
    /// keeps what it had (a menu that is open, a field with focus) and
    /// the rows that come into view start anew.
    fn diff(&self, tree: &mut Tree) {
        let columns = self.columns();
        let ids: Vec<u64> = self.rows.iter().map(|row| row.id).collect();
        let Tree {
            state, children, ..
        } = tree;
        let internal = state.downcast_mut::<Internal>();
        let same_shape = internal.columns == columns
            && internal.placeholder == self.placeholder
            && children.len()
                == columns * (1 + internal.ids.len()) + usize::from(internal.placeholder);
        if !same_shape {
            *children = self.children();
        } else if internal.ids != ids {
            let mut old = std::mem::take(children).into_iter();
            let mut kept: Vec<Tree> = old.by_ref().take(columns).collect();
            let mut by_row: HashMap<u64, Vec<Tree>> = HashMap::new();
            for id in &internal.ids {
                by_row.insert(*id, old.by_ref().take(columns).collect());
            }
            let cells = self.children[columns..].chunks(columns.max(1));
            for (row, cells) in ids.iter().zip(cells) {
                match by_row.remove(row) {
                    Some(trees) => kept.extend(trees),
                    None => kept.extend(cells.iter().map(Tree::new)),
                }
            }
            kept.extend(old);
            *children = kept;
        }
        internal.ids = ids;
        internal.columns = columns;
        internal.placeholder = self.placeholder;
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fill,
            if self.fill {
                Length::Fill
            } else {
                Length::Shrink
            },
        )
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max = limits.max();
        let columns = self.columns();
        let chrome = 2.0 * BORDER + HEADER_HEIGHT;
        let inner_width = (max.width - 2.0 * BORDER).max(0.0);
        // What stands in for the rows is as tall as it wants to be.
        let stand_in = self.placeholder.then(|| {
            let last = self.children.len() - 1;
            let limits = layout::Limits::new(Size::ZERO, Size::new(inner_width, f32::INFINITY));
            self.children[last]
                .as_widget_mut()
                .layout(&mut tree.children[last], renderer, &limits)
        });
        let body_height = match &stand_in {
            Some(node) => node.size().height,
            None => {
                let room = if max.height.is_finite() {
                    (max.height - chrome).max(0.0)
                } else {
                    f32::INFINITY
                };
                let most = room.min(self.max_height);
                if self.fill && most.is_finite() {
                    most
                } else {
                    self.extent().min(most)
                }
            }
        };
        let size = Size::new(max.width, chrome + body_height);
        let geometry = self.geometry(Rectangle::with_size(size));
        let mut nodes = Vec::with_capacity(self.children.len());
        let cells = self.children.len() - usize::from(self.placeholder);
        for index in 0..cells {
            let column = index % columns;
            let cell = if index < columns {
                geometry.cell(column, geometry.header.y, HEADER_HEIGHT - LINE)
            } else {
                let row = self.first + index / columns - 1;
                geometry.cell(column, self.row_top(&geometry, row), self.row_height - LINE)
            };
            let room = Size::new(
                (cell.width - 2.0 * self.tracks[column].padding).max(0.0),
                cell.height,
            );
            let node = self.children[index].as_widget_mut().layout(
                &mut tree.children[index],
                renderer,
                &layout::Limits::new(Size::ZERO, room),
            );
            nodes.push(self.place(column, cell, node));
        }
        nodes.extend(stand_in.map(|node| node.move_to(geometry.body.position())));
        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let columns = self.columns();
        let body = self.geometry(layout.bounds()).body;
        let takes_focus = !self.rows.is_empty();
        let Tree {
            state, children, ..
        } = tree;
        let internal = state.downcast_mut::<Internal>();
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            let cells = self
                .children
                .iter_mut()
                .zip(children.iter_mut())
                .zip(layout.children());
            for (index, ((child, tree), layout)) in cells.enumerate() {
                // The rows are one stop, after the header's buttons.
                if index == columns && takes_focus {
                    operation.focusable(None, body, &mut internal.focus);
                    operation.custom(None, body, &mut internal.focus);
                }
                child
                    .as_widget_mut()
                    .operate(tree, layout, renderer, operation);
            }
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        input::update(
            self, tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        draw::draw(self, tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        input::mouse_interaction(self, tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, K, Message> From<Grid<'a, K, Message>> for Element<'a, Message>
where
    K: Clone + 'a,
    Message: Clone + 'a,
{
    fn from(grid: Grid<'a, K, Message>) -> Self {
        Element::new(grid)
    }
}
