//! The table as iced sees it: one widget that lays out the header and the
//! lines in view, scrolls them both ways, hears the pointer and the
//! keyboard, and draws what belongs to lines rather than to their cells
//! (the fill of a hovered or selected row, the hairlines, the focus ring,
//! the scrollbars).
//!
//! It keeps nothing the page should know: how far it is scrolled, what is
//! selected and how wide the columns are come from the table's `State` and
//! go back to it as messages. Its own state is what iced keeps for a
//! widget: whether it has focus, a drag in progress, the last press.

mod drags;
mod draw;
mod geometry;
mod input;

use std::{collections::HashMap, time::Instant};

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::Modifiers,
    Event, Length, Point, Rectangle, Size, Vector,
};

use self::geometry::Geometry;
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
/// The shortest a scrollbar's thumb gets.
const THUMB_MIN: f32 = 24.0;

/// What a line of the body is.
pub enum What<K> {
    Row(K),
    /// The header of a group of rows.
    Group {
        name: String,
        collapsed: bool,
    },
    /// A row that stands for one still being read.
    Skeleton,
}

/// A line in view.
pub struct Row<K> {
    pub what: What<K>,
    /// What its widgets are kept under: its key, hashed.
    pub id: u64,
    pub selected: bool,
    /// The columns its one wide cell runs over, from the first to before
    /// the last: the name of a group.
    pub span: Option<(usize, usize)>,
}

/// Where the keyboard is among the lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    /// On no line yet.
    None,
    /// On a line that is not in view.
    Away,
    /// On the line at this place.
    At(usize),
}

type OnEvent<'a, K, Message> = Box<dyn Fn(Action<K>) -> Message + 'a>;

pub struct Grid<'a, K, Message> {
    /// One per column, the checkboxes' and the menus' included.
    pub tracks: Vec<Track>,
    /// The columns the user may resize, by name.
    pub ids: Vec<Option<ColumnId>>,
    /// What each column's widgets are kept under.
    pub keys: Vec<u64>,
    /// The header's cells, then every cell of every line in view, then
    /// what stands in for the rows, if anything does.
    pub children: Vec<Element<'a, Message>>,
    /// What a header sends when it is pressed with Shift, for those that sort.
    pub additive: Vec<Option<Message>>,
    pub rows: Vec<Row<K>>,
    /// The place of the first line in view among those that show.
    pub first: usize,
    /// How many lines show, in view or not.
    pub count: usize,
    pub offset: f32,
    /// How far the columns are scrolled sideways, from where text starts.
    pub across: f32,
    pub row_height: f32,
    /// How much room the view assumed the rows have.
    pub viewport: f32,
    /// Whether the last child stands in for the rows (empty, failed).
    pub placeholder: bool,
    pub focused: Focus,
    /// A row's menu is open: it has the keyboard, and the one ring.
    pub menu_open: bool,
    pub has_menu: bool,
    pub any_selected: bool,
    /// A press opens the row: the pointer says so.
    pub pointer: bool,
    /// Letters typed go to the row that starts with them.
    pub types: bool,
    /// Takes all the height it is given, however few the rows.
    pub fill: bool,
    pub max_height: f32,
    pub rtl: bool,
    pub on_event: OnEvent<'a, K, Message>,
}

/// A header's edge being dragged. The column is named, not counted: the
/// columns may change under a drag.
#[derive(Debug, Clone, Copy)]
struct Drag {
    column: ColumnId,
    from_x: f32,
    from_width: f32,
}

/// What iced keeps for the table between views.
#[derive(Debug, Default)]
pub struct Internal {
    focus: pressable::State,
    /// The lines and the columns whose widgets the tree holds, in order.
    rows: Vec<u64>,
    columns: Vec<u64>,
    placeholder: bool,
    modifiers: Modifiers,
    drag: Option<Drag>,
    /// A thumb is held, this far from its start.
    grab: Option<f32>,
    grab_across: Option<f32>,
    hovered: Option<usize>,
    over_handle: Option<usize>,
    /// The last press on a line, and which line it was.
    click: Option<(u64, mouse::Click)>,
    /// The last press on a header's edge.
    handle_click: Option<(ColumnId, mouse::Click)>,
    /// What was typed, and when the last letter was.
    typed: String,
    typed_at: Option<Instant>,
}

impl<K, Message> Grid<'_, K, Message> {
    fn extent(&self) -> f32 {
        window::extent(self.count, self.row_height)
    }

    fn columns(&self) -> usize {
        self.tracks.len()
    }

    /// Whether a line can be pressed or have the keyboard.
    fn is_interactive(&self) -> bool {
        self.rows
            .iter()
            .any(|row| !matches!(row.what, What::Skeleton))
    }

    /// Where the line at `index` among those that show is drawn.
    fn row_top(&self, geometry: &Geometry, index: usize) -> f32 {
        geometry.body.y + window::top_of(index, self.row_height) - self.offset
    }

    /// The line in view at `index` among those that show.
    fn row(&self, index: usize) -> Option<&Row<K>> {
        self.rows.get(index.checked_sub(self.first)?)
    }

    /// The line under the pointer, among those that show.
    fn row_under(&self, geometry: &Geometry, cursor: mouse::Cursor) -> Option<usize> {
        let at = cursor.position_in(geometry.body)?;
        let index = window::row_at(at.y, self.offset, self.row_height, self.count)?;
        self.row(index).map(|_| index)
    }

    /// The column whose edge is under the pointer.
    fn handle_under(&self, geometry: &Geometry, cursor: mouse::Cursor) -> Option<usize> {
        let at = cursor.position_over(geometry.heads())?;
        (0..self.columns()).find(|&column| {
            self.tracks[column].resizable
                && self.ids[column].is_some()
                && (at.x - geometry.edge(column, self.rtl)).abs() <= HANDLE
        })
    }

    /// Where the cell of `column` on the line `row` goes.
    fn cell_of(&self, geometry: &Geometry, row: usize, column: usize, height: f32) -> Rectangle {
        let top = self.row_top(geometry, self.first + row);
        match self.rows[row].span {
            Some((from, to)) if column == from => geometry.span(from, to, top, height),
            // Under the wide cell: nothing.
            Some((from, to)) if column > from && column < to => Rectangle {
                width: 0.0,
                ..geometry.cell(column, top, height)
            },
            _ => geometry.cell(column, top, height),
        }
    }

    /// The place of a cell's content in `cell`.
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
        // What the first children are kept under is known from the start,
        // so the next view finds them.
        tree::State::new(Internal {
            rows: self.rows.iter().map(|row| row.id).collect(),
            columns: self.keys.clone(),
            placeholder: self.placeholder,
            ..Internal::default()
        })
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    /// A cell's widgets follow its line and its column: as the lines in
    /// view change, as a column is hidden, as the rows give way to what
    /// stands in for them and come back, each cell keeps what it had (a
    /// header its focus, a menu that it is open) and what comes into view
    /// starts anew.
    fn diff(&self, tree: &mut Tree) {
        let rows: Vec<u64> = self.rows.iter().map(|row| row.id).collect();
        let Tree {
            state, children, ..
        } = tree;
        let internal = state.downcast_mut::<Internal>();
        let width = internal.columns.len();
        let held = width * (1 + internal.rows.len()) + usize::from(internal.placeholder);
        if children.len() != held {
            // Not what was noted: nothing to go by.
            *children = self.children();
        } else if internal.columns != self.keys
            || internal.rows != rows
            || internal.placeholder != self.placeholder
        {
            let mut old = std::mem::take(children).into_iter();
            // The cells of the header, then of each line, by column.
            let line = |old: &mut std::vec::IntoIter<Tree>| -> HashMap<u64, Tree> {
                internal.columns.iter().copied().zip(old.by_ref()).collect()
            };
            let mut header = line(&mut old);
            let mut by_row: HashMap<u64, HashMap<u64, Tree>> = HashMap::new();
            for id in &internal.rows {
                by_row.insert(*id, line(&mut old));
            }
            let stand_in = old.next();
            let columns = self.keys.len();
            let mut kept = Vec::with_capacity(self.children.len());
            let mut cells = self.children.iter();
            let mut take = |from: Option<&mut HashMap<u64, Tree>>, kept: &mut Vec<Tree>| {
                let mut from = from;
                for key in &self.keys {
                    let Some(cell) = cells.next() else { return };
                    let old = from.as_mut().and_then(|from| from.remove(key));
                    kept.push(old.unwrap_or_else(|| Tree::new(cell)));
                }
            };
            take(Some(&mut header), &mut kept);
            for id in &rows {
                take(by_row.get_mut(id), &mut kept);
            }
            if self.placeholder {
                let last = &self.children[columns * (1 + rows.len())];
                kept.push(stand_in.unwrap_or_else(|| Tree::new(last)));
            }
            *children = kept;
        }
        internal.rows = rows;
        internal.columns.clone_from(&self.keys);
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
        // Columns that don't fit scroll sideways, over a scrollbar.
        let least: f32 = column::resolve(&self.tracks, inner_width).iter().sum();
        let across = if least > inner_width + 0.5 {
            control::SCROLLBAR
        } else {
            0.0
        };
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
                    (max.height - chrome - across).max(0.0)
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
        let size = Size::new(max.width, chrome + body_height + across);
        let geometry = Geometry::of(self, Rectangle::with_size(size));
        let mut nodes = Vec::with_capacity(self.children.len());
        let cells = self.children.len() - usize::from(self.placeholder);
        for index in 0..cells {
            let column = index % columns;
            let cell = if index < columns {
                geometry.cell(column, geometry.header.y, HEADER_HEIGHT - LINE)
            } else {
                self.cell_of(
                    &geometry,
                    index / columns - 1,
                    column,
                    self.row_height - LINE,
                )
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
        let body = Geometry::of(self, layout.bounds()).body;
        let takes_focus = self.is_interactive();
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
