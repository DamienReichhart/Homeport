//! The grid as iced sees it: one widget that lays out the rows in view,
//! draws them with the cursor, the selection, the search's matches and
//! the links, hears the pointer, the keyboard and the input method, and
//! tells the terminal's [`State`] how much room it has (fit to its
//! container, as xterm.js's `addon-fit`).
//!
//! It keeps nothing the page should know: the grid, the selection and the
//! search are the [`State`]'s and change through messages. Its own state
//! is what iced keeps for a widget: focus, a drag or a press the program
//! was told of, the input method's text being composed, when the cursor
//! last moved (for its blink), and the rows of the last layout with the
//! text they shaped.

mod boxes;
mod draw;
mod input;
pub mod rows;

use std::{cell::RefCell, collections::HashMap, time::Instant};

use iced::{
    advanced::input_method::Preedit,
    advanced::{
        layout, mouse, renderer,
        text::{self, Paragraph as _},
        widget::{operation::Focusable, tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::Modifiers,
    Color, Event, Length, Rectangle, Size,
};

use self::rows::{Glyphs, Laid};
use super::state::{Message as Action, State, Viewport};
use crate::ui::{
    theme::{metrics::control, Element, Theme},
    widgets::{focus, focus::pressable, log::ansi::Style},
};

type Raw = <iced::Renderer as text::Renderer>::Paragraph;

/// The frame around the grid, and `p-2` inside it (React's `rounded-md
/// border p-2`).
const BORDER: f32 = 1.0;
const PADDING: f32 = 8.0;
/// The shortest the scrollbar's thumb gets.
const THUMB_MIN: f32 = 24.0;

/// How tall a row is for text of `size`: 16 pixels for React's 12.
pub fn row_height(size: f32) -> f32 {
    (size * 4.0 / 3.0).round()
}

/// The width of a cell: what the monospaced face advances by at `size`.
fn measure_cell(size: f32) -> f32 {
    const SAMPLE: &str = "0000000000000000";
    #[allow(clippy::cast_precision_loss)]
    let cell = rows::shape(SAMPLE, false, size, row_height(size)).min_width() / SAMPLE.len() as f32;
    if cell.is_finite() && cell > 0.0 {
        cell
    } else {
        size * 0.6
    }
}

/// Where everything is in the terminal's bounds.
#[derive(Debug, Clone, Copy)]
pub(super) struct Geometry {
    pub bounds: Rectangle,
    /// Where the cells are.
    pub text: Rectangle,
    /// The scrollbar's track, where lines end in the app's language.
    pub track: Rectangle,
    pub cell: Size,
    pub cols: usize,
    pub rows: usize,
}

impl Geometry {
    // A terminal is a few thousand pixels at most.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn of(bounds: Rectangle, cell: Size, rtl: bool) -> Self {
        let inner = bounds.shrink(BORDER);
        let rail = control::SCROLLBAR;
        let room = Size::new(
            (inner.width - rail - 2.0 * PADDING).max(0.0),
            (inner.height - 2.0 * PADDING).max(0.0),
        );
        let cols = ((room.width / cell.width).floor() as usize).max(1);
        let rows = ((room.height / cell.height).floor() as usize).max(1);
        let text = Rectangle {
            x: inner.x + PADDING + if rtl { rail } else { 0.0 },
            y: inner.y + PADDING,
            width: cols as f32 * cell.width,
            height: rows as f32 * cell.height,
        };
        let track = Rectangle {
            x: if rtl {
                inner.x
            } else {
                inner.x + inner.width - rail
            },
            y: inner.y,
            width: rail,
            height: inner.height,
        };
        Self {
            bounds,
            text,
            track,
            cell,
            cols,
            rows,
        }
    }

    /// Where the row `index` of those in view is.
    #[allow(clippy::cast_precision_loss)]
    pub fn row(&self, index: usize) -> Rectangle {
        Rectangle {
            y: self.text.y + index as f32 * self.cell.height,
            height: self.cell.height,
            ..self.text
        }
    }

    /// The cells `from..to` of the row `index`, their edges on whole
    /// pixels: two runs side by side meet without a seam between them.
    #[allow(clippy::cast_precision_loss)]
    pub fn cells(&self, index: usize, from: usize, to: usize) -> Rectangle {
        let row = self.row(index);
        let left = (row.x + from as f32 * self.cell.width).round();
        let right = (row.x + to.max(from) as f32 * self.cell.width).round();
        Rectangle {
            x: left,
            width: right - left,
            ..row
        }
    }

    /// The cell under `point`, clamped to the grid, and whether the point
    /// is on the cell's right half.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn cell_at(&self, point: iced::Point) -> (usize, usize, bool) {
        let across = ((point.x - self.text.x) / self.cell.width).max(0.0);
        let down = ((point.y - self.text.y) / self.cell.height).max(0.0);
        let col = (across.floor() as usize).min(self.cols.saturating_sub(1));
        let row = (down.floor() as usize).min(self.rows.saturating_sub(1));
        (col, row, across.fract() >= 0.5)
    }

    pub fn viewport(&self) -> Viewport {
        Viewport {
            cols: self.cols,
            rows: self.rows,
            cell_width: self.cell.width,
            cell_height: self.cell.height,
        }
    }
}

type OnEvent<'a, Message> = Box<dyn Fn(Action) -> Message + 'a>;

/// The grid of a terminal.
pub struct Pane<'a, Message> {
    pub state: &'a State,
    pub on_event: OnEvent<'a, Message>,
    pub rtl: bool,
    /// Cmd, not Ctrl and Shift, is the terminal's own modifier.
    pub mac: bool,
}

/// What iced keeps for the grid between views.
pub struct Internal {
    focus: pressable::State,
    /// The size of the text, and the width of its cell.
    cell: Option<(f32, f32)>,
    laid: Laid,
    glyphs: Glyphs,
    modifiers: Modifiers,
    /// The left button is held on the text, selecting.
    selecting: bool,
    /// The button a program was told was pressed, and the cell it was
    /// last told of.
    reported: Option<input::Held>,
    /// The thumb is held, this far from its start.
    grab: Option<f32>,
    click: Option<mouse::Click>,
    /// What the wheel scrolled that is less than a line.
    wheel: f32,
    /// The cell under the pointer: a link there is underlined.
    hovered: Option<(usize, usize)>,
    /// What the input method is composing.
    preedit: Option<Preedit>,
    /// When the cursor last had a reason to show, and the frame's time.
    since: Instant,
    now: Instant,
    /// Whether the last draw was in a dark theme, and the colours of the
    /// looks it drew.
    dark: RefCell<Option<bool>>,
    colours: RefCell<HashMap<Style, (Color, Option<Color>)>>,
}

impl Default for Internal {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            focus: pressable::State::default(),
            cell: None,
            laid: Laid::default(),
            glyphs: Glyphs::default(),
            modifiers: Modifiers::default(),
            selecting: false,
            reported: None,
            grab: None,
            click: None,
            wheel: 0.0,
            hovered: None,
            preedit: None,
            since: now,
            now,
            dark: RefCell::new(None),
            colours: RefCell::new(HashMap::new()),
        }
    }
}

impl Internal {
    /// How many pieces of text this grid has shaped.
    #[cfg(test)]
    pub fn shaped(&self) -> usize {
        self.glyphs.shaped
    }
}

impl<Message> Pane<'_, Message> {
    fn geometry(&self, internal: &Internal, bounds: Rectangle) -> Geometry {
        let size = self.state.font_size();
        let width = internal
            .cell
            .filter(|(at, _)| (*at - size).abs() < f32::EPSILON)
            .map_or(size * 0.6, |(_, width)| width);
        Geometry::of(bounds, Size::new(width, row_height(size)), self.rtl)
    }
}

/// What a widget that needs every key reports of itself, for the shell to
/// keep its shortcuts out of a terminal that has focus.
fn every_key(operation: &mut dyn Operation, bounds: Rectangle, focused: bool) {
    let mut marker = focus::EveryKey(focused);
    operation.custom(None, bounds, &mut marker);
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Pane<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Internal>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Internal::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let internal = tree.state.downcast_mut::<Internal>();
        let size = self.state.font_size();
        if internal
            .cell
            .is_none_or(|(at, _)| (at - size).abs() > f32::EPSILON)
        {
            internal.cell = Some((size, measure_cell(size)));
        }
        let mut bounds = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        if !bounds.height.is_finite() {
            // A page that scrolls gives all the height asked for: 24 rows.
            bounds.height = 24.0 * row_height(size) + 2.0 * (BORDER + PADDING);
        }
        let geometry = self.geometry(internal, Rectangle::with_size(bounds));
        let previous = std::mem::take(&mut internal.laid);
        internal.laid = rows::lay(self.state, previous, &mut internal.glyphs, &geometry);
        layout::Node::new(bounds)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let internal = tree.state.downcast_mut::<Internal>();
        let bounds = layout.bounds();
        let geometry = self.geometry(internal, bounds);
        let id = self.state.id();
        operation.focusable(Some(&id), bounds, &mut internal.focus);
        operation.custom(None, bounds, &mut internal.focus);
        every_key(operation, bounds, internal.focus.is_focused());
        for (index, row) in internal.laid.rows.iter().enumerate() {
            operation.text(None, geometry.row(index), &row.text);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        input::update(self, tree, event, layout, cursor, clipboard, shell);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        draw::draw(self, tree, renderer, theme, layout, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let internal = tree.state.downcast_ref::<Internal>();
        let geometry = self.geometry(internal, layout.bounds());
        if internal.grab.is_some() {
            return mouse::Interaction::Idle;
        }
        if cursor.is_over(geometry.text) && input::link_under(internal, &geometry, cursor).is_some()
        {
            let command = if self.mac {
                internal.modifiers.logo()
            } else {
                internal.modifiers.control()
            };
            if command {
                return mouse::Interaction::Pointer;
            }
        }
        if internal.selecting || cursor.is_over(geometry.text) {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::None
        }
    }
}

impl<'a, Message: 'a> From<Pane<'a, Message>> for Element<'a, Message> {
    fn from(pane: Pane<'a, Message>) -> Self {
        Element::new(pane)
    }
}
