//! The chart's tests: what the rings keep of a stream that misbehaves
//! (`samples`), which points a long line keeps (`thinning`), where the
//! axes and the tooltip put things (`axes`), whether the series can be
//! told apart without colour (`palette`), what a frame works out and
//! what the keyboard does on a window that lasts (`driven`), what is
//! drawn (`drawn`), and how long a frame takes with ten series of an
//! hour (`perf`).

mod axes;
mod drawn;
mod driven;
mod palette;
mod perf;
mod samples;
mod thinning;

use std::time::{Duration, Instant};

use iced::{
    advanced::{
        clipboard, mouse,
        renderer::{self, Headless},
    },
    futures::executor::block_on,
    widget::container,
    window, Color, Event, Length, Point, Size,
};
use iced_runtime::{user_interface::Cache, UserInterface};
use iced_test::Simulator;

use super::{plot, scene};
use crate::{
    test_support::snapshot::settings,
    ui::theme::{self, Element, Theme},
};

/// How many scenes, series and points this thread has worked out.
fn worked() -> (usize, usize, usize) {
    (
        scene::scans::BUILT.with(std::cell::Cell::get),
        plot::decimated::SERIES.with(std::cell::Cell::get),
        plot::decimated::POINTS.with(std::cell::Cell::get),
    )
}

/// What each part of a frame took.
#[derive(Debug, Clone, Copy, Default)]
struct Took {
    view: Duration,
    layout: Duration,
    draw: Duration,
    pixels: Duration,
}

impl Took {
    /// The chart's share: everything but the renderer's pixels.
    fn own(self) -> Duration {
        self.view + self.layout + self.draw
    }
}

/// The window, without one: a renderer in software and the tree the
/// views share, so a canvas keeps what it drew from one frame to the
/// next as it does in the app.
struct Bench {
    renderer: iced::Renderer,
    cache: Option<Cache>,
    size: Size,
    theme: Theme,
}

impl Bench {
    fn new(size: impl Into<Size>) -> Self {
        let size = size.into();
        let settings = settings();
        // A simulator loads the fonts into the process.
        drop(Simulator::<(), Theme, iced::Renderer>::with_size(
            settings.clone(),
            size,
            iced::widget::Space::new(),
        ));
        let renderer = block_on(<iced::Renderer as Headless>::new(
            theme::font::DEFAULT,
            settings.default_text_size,
            Some("tiny-skia"),
        ))
        .expect("a renderer without a window");
        Self {
            renderer,
            cache: Some(Cache::default()),
            size,
            theme: Theme::Light,
        }
    }

    /// One frame of `view` with the pointer at `pointer`, timed part by
    /// part; `pixels` says whether the renderer fills them too.
    fn frame<'a, Message: 'a>(
        &mut self,
        view: impl FnOnce() -> Element<'a, Message>,
        pointer: Option<Point>,
        pixels: bool,
    ) -> Took {
        let mut took = Took::default();
        let start = Instant::now();
        let page = container(view())
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::container::page);
        took.view = start.elapsed();

        let start = Instant::now();
        let cache = self.cache.take().unwrap_or_default();
        let mut ui = UserInterface::build(page, self.size, cache, &mut self.renderer);
        took.layout = start.elapsed();

        let cursor = pointer.map_or(mouse::Cursor::Unavailable, mouse::Cursor::Available);
        let mut messages = vec![];
        let mut events = vec![Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )];
        if let Some(position) = pointer {
            events.insert(0, Event::Mouse(mouse::Event::CursorMoved { position }));
        }
        let _ = ui.update(
            &events,
            cursor,
            &mut self.renderer,
            &mut clipboard::Null,
            &mut messages,
        );

        let start = Instant::now();
        let style = renderer::Style {
            text_color: self.theme.tokens().foreground,
        };
        ui.draw(&mut self.renderer, &self.theme, &style, cursor);
        took.draw = start.elapsed();
        self.cache = Some(ui.into_cache());

        if pixels {
            let start = Instant::now();
            // The window's size is whole pixels.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let size = Size::new(self.size.width as u32, self.size.height as u32);
            let drawn = self.renderer.screenshot(size, 1.0, Color::WHITE);
            took.pixels = start.elapsed();
            assert!(!drawn.is_empty());
        }
        took
    }
}
