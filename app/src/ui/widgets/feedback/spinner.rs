//! `Spinner`: the turning icon of something in progress, with what is
//! loading beside it. iced animates nothing by itself, so the icon is a
//! widget that turns with the clock and asks for the next frame.

use std::{
    f32::consts::TAU,
    time::{Duration, Instant},
};

use iced::{
    advanced::{
        layout, mouse, renderer, svg,
        svg::Renderer as _,
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    widget::text,
    window, Event, Length, Radians, Rectangle, Size,
};

use crate::{
    t,
    ui::{
        theme::{
            self,
            metrics::{icon as icon_size, space, text as size},
            Element, Icon, Theme, Tone,
        },
        widgets::line,
    },
};

/// `animate-spin`: one turn a second.
const TURN: Duration = Duration::from_secs(1);

/// The turning icon alone, in the colour of `tone`.
pub struct Turning {
    size: f32,
    tone: Tone,
}

/// The turning icon, for a button or a status that is in progress.
pub fn turning() -> Turning {
    Turning {
        size: icon_size::MD,
        tone: Tone::Muted,
    }
}

impl Turning {
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }
}

/// When the icon started turning, and how far it is.
#[derive(Debug, Default)]
struct State {
    started: Option<Instant>,
    turns: f32,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Turning {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.size, self.size)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<State>();
            // The first frame is the icon upright, whenever it is drawn.
            let started = *state.started.get_or_insert(*now);
            let turned = now.duration_since(started).as_secs_f32() / TURN.as_secs_f32();
            state.turns = turned.fract();
            shell.request_redraw();
        }
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
        let state = tree.state.downcast_ref::<State>();
        let handle = svg::Handle::from_memory(Icon::LoaderCircle.bytes());
        let drawing = svg::Svg::new(handle)
            .color(self.tone.colour(theme.tokens()))
            .rotation(Radians(state.turns * TAU));
        renderer.draw_svg(drawing, layout.bounds(), *viewport);
    }
}

impl<'a, Message: 'a> From<Turning> for Element<'a, Message> {
    fn from(turning: Turning) -> Self {
        Element::new(turning)
    }
}

/// `Spinner`: "Loading…" beside the turning icon.
pub fn spinner<'a, Message: 'a>() -> Element<'a, Message> {
    spinner_saying(t!("common.loading"))
}

/// The same, saying what is in progress.
pub fn spinner_saying<'a, Message: 'a>(label: impl Into<String>) -> Element<'a, Message> {
    line(vec![
        turning().into(),
        text(label.into())
            .size(size::SM)
            .style(theme::text::muted)
            .into(),
    ])
    .spacing(space::S2)
    .into()
}
