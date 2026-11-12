//! The lines as iced sees them: one widget that lays out the rows in view
//! and nothing else, scrolls them, hears the pointer and the keyboard, and
//! draws the text with what lies behind it (a program's colours, the
//! matches of the search, the selection) and its scrollbars.
//!
//! It keeps nothing the page should know: where the view is, what is
//! selected and what is searched are the viewer's [`State`] and go back to
//! it as messages. Its own state is what iced keeps for a widget: whether
//! it has focus, a drag in progress, and the rows it laid out, so a row
//! that stays in view is not shaped again.

mod draw;
mod input;
mod rows;

use iced::{
    advanced::{
        layout, mouse, renderer,
        text::{self, Paragraph as _},
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    keyboard::Modifiers,
    Event, Length, Pixels, Rectangle, Size,
};

use self::rows::Laid;
use super::state::{Message as Action, Metrics, State};
use crate::ui::{
    theme::{
        font,
        metrics::{control, space, text as size},
        Element, Theme,
    },
    widgets::focus::pressable,
};

type Raw = <iced::Renderer as text::Renderer>::Paragraph;

/// The height of a row: `text-xs` on Tailwind's line.
pub const ROW: f32 = 16.0;
/// The frame around the lines.
const BORDER: f32 = 1.0;
/// `p-2` between the frame and the text.
const PADDING: f32 = space::S2;
/// The shortest a scrollbar's thumb gets.
const THUMB_MIN: f32 = 24.0;
/// How tall the viewer is where nothing bounds its height (a page that
/// scrolls gives all the height asked for).
const UNBOUNDED_HEIGHT: f32 = 480.0;

#[cfg(test)]
thread_local! {
    /// How many pieces of text this thread shaped: what a test counts to
    /// hold a layout to the rows that came into view.
    pub(super) static SHAPED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// A line of machine text that never wraps by itself.
fn shaped(content: &str, italic: bool) -> Raw {
    #[cfg(test)]
    SHAPED.with(|count| count.set(count.get() + 1));
    let ascii = content.is_ascii();
    // A piece is a row of a viewer at most: a few hundred characters.
    #[allow(clippy::cast_precision_loss)]
    let room = if ascii {
        f32::INFINITY
    } else {
        (content.chars().count() + 1) as f32 * 2.0 * ROW
    };
    let mut paragraph = Raw::with_text(text::Text {
        content,
        // Room for any glyph of its cells, and finite: text that reads
        // right to left is placed against the end of what it is given.
        bounds: Size::new(room, ROW),
        size: Pixels(size::XS),
        line_height: text::LineHeight::Absolute(Pixels(ROW)),
        font: if italic {
            font::MONO_ITALIC
        } else {
            font::MONO
        },
        // Log text starts on the left whatever the language of the app.
        align_x: text::Alignment::Left,
        align_y: iced::alignment::Vertical::Top,
        // Text the monospaced face has needs no shaping; any other script
        // does, and fonts of the system.
        shaping: if ascii {
            text::Shaping::Basic
        } else {
            text::Shaping::Advanced
        },
        wrapping: text::Wrapping::None,
    });
    if !ascii {
        // As wide as its text and no wider: a piece that reads right to
        // left then starts at its cell like any other.
        let width = paragraph.min_width().ceil();
        paragraph.resize(Size::new(width, ROW));
    }
    paragraph
}

/// The width of a cell: what the monospaced face advances by.
fn measure_cell() -> f32 {
    const SAMPLE: &str = "0000000000000000";
    // The sample is sixteen characters.
    #[allow(clippy::cast_precision_loss)]
    let cell = shaped(SAMPLE, false).min_width() / SAMPLE.len() as f32;
    if cell.is_finite() && cell > 0.0 {
        cell
    } else {
        size::XS * 0.6
    }
}

/// Where everything is in the viewer's bounds.
#[derive(Debug, Clone, Copy)]
pub(super) struct Geometry {
    pub bounds: Rectangle,
    /// Where the rows are drawn.
    pub text: Rectangle,
    /// The track of the scrollbar beside the rows.
    pub track: Rectangle,
    /// The track of the scrollbar under them, when lines don't wrap.
    pub bar: Option<Rectangle>,
    pub cell: f32,
    pub metrics: Metrics,
}

impl Geometry {
    // A viewer is a few thousand pixels at most: its cells fit a `usize`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn of(bounds: Rectangle, cell: f32, wrap: bool, rtl: bool, footer: f32) -> Self {
        let inner = bounds.shrink(BORDER);
        let rail = control::SCROLLBAR;
        let under = if wrap { 0.0 } else { rail };
        // The scrollbar is where lines end in the app's language; the
        // text starts on the left either way.
        let track = Rectangle {
            x: if rtl {
                inner.x
            } else {
                inner.x + inner.width - rail
            },
            width: rail,
            height: (inner.height - under - footer).max(0.0),
            ..inner
        };
        let text = Rectangle {
            x: inner.x + PADDING + if rtl { rail } else { 0.0 },
            y: inner.y + PADDING,
            width: (inner.width - rail - 2.0 * PADDING).max(0.0),
            height: (inner.height - under - footer - 2.0 * PADDING).max(0.0),
        };
        let bar = (!wrap).then_some(Rectangle {
            x: text.x - PADDING,
            y: inner.y + inner.height - footer - rail,
            width: text.width + 2.0 * PADDING,
            height: rail,
        });
        Self {
            bounds,
            text,
            track,
            bar,
            cell,
            metrics: Metrics {
                cols: ((text.width / cell).floor() as usize).max(1),
                rows: ((text.height / ROW).floor() as usize).max(1),
            },
        }
    }

    /// Where the row at `index` among those in view is drawn.
    // A viewer shows a few hundred rows at most.
    #[allow(clippy::cast_precision_loss)]
    pub fn row(&self, index: usize) -> Rectangle {
        Rectangle {
            y: self.text.y + index as f32 * ROW,
            height: ROW,
            ..self.text
        }
    }

    /// The distance from the start of a row to its cell `cell`.
    #[allow(clippy::cast_precision_loss)]
    pub fn across(&self, cell: usize) -> f32 {
        cell as f32 * self.cell
    }
}

type OnEvent<'a, Message> = Box<dyn Fn(Action) -> Message + 'a>;

/// The rows of a viewer.
pub struct Pane<'a, Message> {
    pub state: &'a State,
    pub on_event: OnEvent<'a, Message>,
    pub height: Length,
    pub rtl: bool,
    /// Room kept free under the rows, inside the frame: where "Jump to
    /// latest" goes while the view is paused, so it covers no line.
    pub footer: f32,
}

/// What iced keeps for the rows between views.
#[derive(Default)]
pub struct Internal {
    focus: pressable::State,
    /// The width of a cell, once measured.
    cell: Option<f32>,
    /// The rows of the last layout.
    laid: Laid,
    modifiers: Modifiers,
    /// The button is held on the text.
    selecting: bool,
    /// A thumb is held, this far from its start.
    grab: Option<f32>,
    grab_across: Option<f32>,
    click: Option<mouse::Click>,
    /// What the wheel scrolled that is less than a row, or a cell.
    wheel: f32,
    wheel_across: f32,
}

impl<Message> Pane<'_, Message> {
    fn geometry(&self, internal: &Internal, bounds: Rectangle) -> Geometry {
        let cell = internal.cell.unwrap_or(size::XS * 0.6);
        Geometry::of(bounds, cell, self.state.wraps(), self.rtl, self.footer)
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Pane<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Internal>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Internal::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let mut size = limits.resolve(Length::Fill, self.height, Size::ZERO);
        if !size.height.is_finite() {
            size.height = UNBOUNDED_HEIGHT;
        }
        let internal = tree.state.downcast_mut::<Internal>();
        internal.cell.get_or_insert_with(measure_cell);
        let geometry = self.geometry(internal, Rectangle::with_size(size));
        internal.laid = rows::lay(self.state, std::mem::take(&mut internal.laid), &geometry);
        layout::Node::new(size)
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
        // The rows are one stop for Tab.
        operation.focusable(None, bounds, &mut internal.focus);
        operation.custom(None, bounds, &mut internal.focus);
        // What each row says, for whoever looks for a text.
        let mut said = String::new();
        for (index, row) in internal.laid.rows.iter().enumerate() {
            said.clear();
            rows::say(self.state, row, &mut said);
            operation.text(None, geometry.row(index), &said);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        input::update(self, tree, event, layout, cursor, shell);
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
        if internal.grab.is_some() || internal.grab_across.is_some() {
            mouse::Interaction::Idle
        } else if internal.selecting || cursor.is_over(geometry.text) {
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
