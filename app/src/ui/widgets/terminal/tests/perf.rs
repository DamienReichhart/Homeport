//! What a frame of the terminal costs under a flood: 50 MB of `yes`, and
//! a rain of letters that writes every cell again each frame (`cmatrix`),
//! measured, with a key typed during each frame.
//!
//! `cargo test --release -p homeport-app --lib terminal::tests::perf --
//! --ignored --nocapture --test-threads=1`
//!
//! A frame is what the window does when a batch comes: the batch read into
//! the grid (`feed`), a key handled (`update`), `view`, layout, draw, and
//! the software renderer's pixels, reported apart. The target is thirty
//! frames a second with the pixels, so 33 ms; the terminal's own share
//! (all but the pixels) is held to it, eight times over in a build that
//! is not optimised.

use std::time::{Duration, Instant};

use iced::{
    advanced::{
        clipboard, mouse,
        renderer::{self, Headless},
    },
    futures::executor::block_on,
    widget::container,
    window as iced_window, Color, Event, Length, Size,
};
use iced_runtime::{user_interface::Cache, UserInterface};
use iced_test::Simulator;

use crate::{
    test_support::snapshot::settings,
    ui::{
        theme::{self, Element, Theme},
        widgets::{
            gallery::terminal::{rain_frame, yes_output, YES_BATCH},
            terminal::{Message, State, Terminal},
        },
    },
};

const WINDOW: Size = Size::new(1100.0, 720.0);
/// What `yes | head -c 50M` writes.
const YES_TOTAL: usize = 50 * 1024 * 1024;
/// Thirty frames a second.
const FRAME_BUDGET: Duration = Duration::from_millis(33);

#[derive(Debug, Clone, Copy, Default)]
struct Frame {
    feed: Duration,
    key: Duration,
    view: Duration,
    layout: Duration,
    draw: Duration,
    pixels: Duration,
}

impl Frame {
    fn own(self) -> Duration {
        self.feed + self.key + self.view + self.layout + self.draw
    }

    fn total(self) -> Duration {
        self.own() + self.pixels
    }
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// The window without one: a renderer in software and the tree it keeps.
struct Bench {
    renderer: iced::Renderer,
    cache: Option<Cache>,
}

impl Bench {
    fn new() -> Self {
        let settings = settings();
        drop(Simulator::<(), Theme, iced::Renderer>::with_size(
            settings.clone(),
            WINDOW,
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
        }
    }

    /// A frame of `state`, timed; what its redraw said back.
    fn frame(&mut self, state: &State) -> (Frame, Vec<Message>) {
        let mut took = Frame::default();
        let start = Instant::now();
        let grid: Element<'_, Message> = Terminal::new(state, |message| message).view();
        let page = container(grid)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::container::page);
        took.view = start.elapsed();
        let start = Instant::now();
        let cache = self.cache.take().unwrap_or_default();
        let mut ui = UserInterface::build(page, WINDOW, cache, &mut self.renderer);
        took.layout = start.elapsed();
        let mut messages = vec![];
        let _ = ui.update(
            &[Event::Window(iced_window::Event::RedrawRequested(
                Instant::now(),
            ))],
            mouse::Cursor::Unavailable,
            &mut self.renderer,
            &mut clipboard::Null,
            &mut messages,
        );
        let start = Instant::now();
        ui.draw(
            &mut self.renderer,
            &Theme::Dark,
            &renderer::Style {
                text_color: Theme::Dark.tokens().foreground,
            },
            mouse::Cursor::Unavailable,
        );
        took.draw = start.elapsed();
        self.cache = Some(ui.into_cache());
        let start = Instant::now();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let size = Size::new(WINDOW.width as u32, WINDOW.height as u32);
        assert!(!self.renderer.screenshot(size, 1.0, Color::BLACK).is_empty());
        took.pixels = start.elapsed();
        (took, messages)
    }

    /// The first frames tell the terminal its room.
    fn settle(&mut self, state: &mut State) {
        for _ in 0..3 {
            let (_, messages) = self.frame(state);
            for message in messages {
                let _ = state.update(message);
            }
        }
    }

    /// A frame after `output` came and a key was typed.
    fn frame_after(&mut self, state: &mut State, output: &str) -> Frame {
        let start = Instant::now();
        drop(state.feed(output));
        let feed = start.elapsed();
        let start = Instant::now();
        let (_, typed) = state.update(Message::Input("q".to_owned()));
        let key = start.elapsed();
        assert!(typed.is_some(), "the key is handled");
        let (mut frame, _) = self.frame(state);
        frame.feed = feed;
        frame.key = key;
        frame
    }
}

fn report(what: &str, frames: &[Frame]) -> Frame {
    let mut sorted = frames.to_vec();
    sorted.sort_by_key(|frame| frame.own());
    let middle = sorted[sorted.len() / 2];
    let worst = sorted[sorted.len() - 1];
    let total: Duration = frames.iter().map(|frame| frame.total()).sum();
    #[allow(clippy::cast_precision_loss)]
    let fps = frames.len() as f64 / total.as_secs_f64();
    println!(
        "{what:<44} median: feed {:>6.2} key {:>5.3} view {:>5.3} layout {:>6.2} draw {:>6.2} \
         = own {:>6.2} ms, pixels {:>6.2} ms | worst own {:>6.2} ms | {fps:>5.1} frames/s with pixels",
        ms(middle.feed),
        ms(middle.key),
        ms(middle.view),
        ms(middle.layout),
        ms(middle.draw),
        ms(middle.own()),
        ms(middle.pixels),
        ms(worst.own()),
    );
    middle
}

fn within(what: &str, took: Duration) {
    let allowed = if cfg!(debug_assertions) {
        FRAME_BUDGET * 8
    } else {
        FRAME_BUDGET
    };
    assert!(
        took <= allowed,
        "{what}: {:.2} ms of the terminal's own work, over {:.0} ms",
        ms(took),
        ms(allowed)
    );
}

#[test]
#[ignore = "a measure: run it in the release profile"]
fn fifty_megabytes_of_yes_and_a_rain_of_letters() {
    let build = if cfg!(debug_assertions) {
        "NOT OPTIMISED (budgets eight times over)"
    } else {
        "the release profile"
    };
    println!(
        "Terminal, {build}, a {}×{} window",
        WINDOW.width, WINDOW.height
    );
    let mut bench = Bench::new();
    let mut state = State::new();
    bench.settle(&mut state);
    let (cols, rows) = state.size();
    println!(
        "The grid: {cols} × {rows} cells, {} lines of scrollback",
        super::super::DEFAULT_SCROLLBACK
    );

    // 50 MB of `yes`, as the stream gives it: a sixtieth of a second's
    // worth a frame while the window keeps up.
    let batch = yes_output(YES_BATCH);
    let started = Instant::now();
    let mut frames = vec![];
    let mut fed = 0;
    while fed < YES_TOTAL {
        frames.push(bench.frame_after(&mut state, &batch));
        fed += batch.len();
    }
    let elapsed = started.elapsed();
    #[allow(clippy::cast_precision_loss)]
    let rate = fed as f64 / 1024.0 / 1024.0 / elapsed.as_secs_f64();
    println!(
        "yes: {} frames of {} KiB, 50 MB drawn in {:.2} s ({rate:.1} MB/s with every frame drawn)",
        frames.len(),
        YES_BATCH / 1024,
        elapsed.as_secs_f64()
    );
    let yes = report("yes, a frame of 160 KiB", &frames);
    within("yes", yes.own());

    // The same bytes read without drawing: what the parser and the grid
    // take, in batches of a megabyte (a window that fell behind).
    let mut reading = State::new();
    let _ = reading.update(Message::Viewport(state.viewport()));
    let big = yes_output(1024 * 1024);
    let start = Instant::now();
    for _ in 0..50 {
        drop(reading.feed(&big));
    }
    let read = start.elapsed();
    println!(
        "yes, read only: 50 MB in {:.2} s ({:.1} MB/s), {:.2} ms a megabyte",
        read.as_secs_f64(),
        50.0 / read.as_secs_f64(),
        ms(read) / 50.0
    );

    // A rain of letters: every cell written again, each frame.
    let frames: Vec<Frame> = (0..120)
        .map(|seed| bench.frame_after(&mut state, &rain_frame(seed, cols, rows)))
        .collect();
    let rain = report(&format!("rain, {cols}×{rows} cells a frame"), &frames);
    within("rain", rain.own());

    // And on a terminal of 80 × 24, React's usual.
    let mut small = State::new();
    let _ = small.update(Message::Viewport(crate::ui::widgets::terminal::Viewport {
        cols: 80,
        rows: 24,
        ..state.viewport()
    }));
    let frames: Vec<Frame> = (0..120)
        .map(|seed| bench.frame_after(&mut small, &rain_frame(seed, 80, 24)))
        .collect();
    report("rain, 80×24 cells a frame (drawn in the window)", &frames);
}
