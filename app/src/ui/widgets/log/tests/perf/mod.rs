//! What a frame of the viewer costs with a million lines in it,
//! measured: a scroll step, a batch of lines appended, and a frame of a
//! view that follows a stream of fifty thousand lines a second.
//!
//! The numbers that count are those of the build that ships, the
//! workspace's release profile (`opt-level = "s"`, LTO, one codegen
//! unit):
//!
//! `cargo test --release -p homeport-app --lib log::tests::perf --
//! --ignored --nocapture --test-threads=1`
//!
//! It prints them and holds the viewer's own share of a frame (`update`,
//! `view`, layout and draw; not the renderer's pixels) to the budgets
//! below. Without `--release` the build has its debug assertions and is
//! not optimised: it says so, and is given eight times each budget, so a
//! change that makes a frame much slower still fails there. No other
//! mix of optimisation levels is called optimised: an earlier version
//! of these tests took an environment variable's word for it.
//!
//! The budgets, of a frame of 16.7 ms at sixty a second:
//!
//! - a scroll step: 2 ms, the table's;
//! - a sixtieth of a second of a stream at 50,000 lines a second (833
//!   lines) appended to a full buffer: 1 ms, and 2 ms with a search on,
//!   which reads them;
//! - a frame of a view that follows that stream (the lines appended, and
//!   every row in view shaped anew): 4 ms, a quarter of the frame;
//! - the worst such frame of two seconds: 8 ms, half a frame, so none is
//!   dropped (a build that is not optimised is held to two frames,
//!   33 ms: it may drop one, not stall);
//! - any `update` of a search over the million lines, the one that reads
//!   a query of plain text included: 2 ms, however long the lines take
//!   to read on their thread.

mod scroll;
mod stream;

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
    test_support::{
        snapshot::{in_language, settings},
        tasks::messages,
    },
    ui::{
        theme::{self, Element, Theme},
        widgets::{
            gallery::{self, log::many},
            log::{Line, Message, Scroll, State, Viewer},
        },
    },
};

pub(super) type Page = gallery::Message;

pub(super) const WINDOW: Size = Size::new(1100.0, 720.0);
pub(super) const MILLION: usize = 1_000_000;
/// A sixtieth of a second at 50,000 lines a second.
pub(super) const BATCH: usize = 50_000 / 60;

pub(super) const SCROLL: Duration = Duration::from_millis(2);
pub(super) const APPEND: Duration = Duration::from_millis(1);
pub(super) const FOLLOW: Duration = Duration::from_millis(4);
pub(super) const SEARCH: Duration = Duration::from_millis(2);
/// The worst frame of a view that follows: half a frame in the build
/// that ships, two frames in one that is not optimised.
pub(super) const WORST: Duration = Duration::from_millis(8);
pub(super) const WORST_UNOPTIMISED: Duration = Duration::from_millis(33);

/// What each part of a frame took.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Frame {
    /// The viewer's `update` or `extend` before the frame.
    pub(super) update: Duration,
    pub(super) view: Duration,
    pub(super) layout: Duration,
    pub(super) draw: Duration,
    pub(super) pixels: Duration,
}

impl Frame {
    /// The viewer's share: everything but the renderer's pixels.
    pub(super) fn own(self) -> Duration {
        self.update + self.view + self.layout + self.draw
    }

    pub(super) fn total(self) -> Duration {
        self.own() + self.pixels
    }
}

/// The window, without one: a renderer in software and the tree the
/// views share.
pub(super) struct Bench {
    renderer: iced::Renderer,
    cache: Option<Cache>,
}

impl Bench {
    pub(super) fn new() -> Self {
        let settings = settings();
        // A simulator loads the fonts into the process.
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

    /// One frame of the viewer, timed part by part, and the messages its
    /// first redraw produced.
    pub(super) fn frame(&mut self, state: &State) -> (Frame, Vec<Page>) {
        let mut took = Frame::default();
        let start = Instant::now();
        let viewer: Element<'_, Page> = Viewer::new(state, Page::Log).view();
        let page = container(viewer)
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
            &Theme::Light,
            &renderer::Style {
                text_color: Theme::Light.tokens().foreground,
            },
            mouse::Cursor::Unavailable,
        );
        took.draw = start.elapsed();
        self.cache = Some(ui.into_cache());

        let start = Instant::now();
        // The window's size is whole pixels.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let size = Size::new(WINDOW.width as u32, WINDOW.height as u32);
        let pixels = self.renderer.screenshot(size, 1.0, Color::WHITE);
        took.pixels = start.elapsed();
        assert!(!pixels.is_empty());
        (took, messages)
    }

    /// The first frames tell the viewer its room and fill the caches.
    pub(super) fn settle(&mut self, state: &mut State) {
        for _ in 0..3 {
            let (_, messages) = self.frame(state);
            for message in messages {
                if let Page::Log(message) = message {
                    let _ = state.update(message);
                }
            }
        }
    }
}

/// The frame in the middle of `frames`, by the viewer's share.
pub(super) fn median(mut frames: Vec<Frame>) -> Frame {
    frames.sort_by_key(|frame| frame.own());
    frames[frames.len() / 2]
}

pub(super) fn middle(mut times: Vec<Duration>) -> Duration {
    times.sort();
    times[times.len() / 2]
}

pub(super) fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

pub(super) fn report(what: &str, frame: Frame) {
    println!(
        "{what:<52} update {:>6.3}  view {:>6.3}  layout {:>6.3}  draw {:>6.3}  = own {:>6.3} ms   \
         pixels {:>7.3} ms",
        ms(frame.update),
        ms(frame.view),
        ms(frame.layout),
        ms(frame.draw),
        ms(frame.own()),
        ms(frame.pixels),
    );
}

/// Whether this is the build that ships: the release profile, which has
/// no debug assertions. Nothing else is taken for it.
pub(super) fn optimised() -> bool {
    !cfg!(debug_assertions)
}

pub(super) fn build() -> &'static str {
    if optimised() {
        "the release profile"
    } else {
        "NOT OPTIMISED (a debug build: budgets are eight times over)"
    }
}

/// Holds `took` to `budget`, eight times over in a build that is not
/// optimised.
pub(super) fn within(what: &str, took: Duration, budget: Duration) {
    let allowed = if optimised() { budget } else { budget * 8 };
    assert!(
        took <= allowed,
        "{what}: {:.3} ms of the viewer's own work, over {:.0} ms",
        ms(took),
        ms(allowed)
    );
}

/// Lines of a busy service that are longer than the viewer is wide:
/// about 330 cells, three rows of a window of 140.
pub(super) fn long(from: usize, count: usize) -> impl Iterator<Item = Line> {
    many(from, count).map(|line| {
        let tail = " attrs={\"stack\":\"shop\",\"services\":[\"web\",\"db\",\"cache\",\"worker\"],\
                    \"host\":\"prod-eu-1\",\"attempt\":3,\"next\":\"2026-05-01T10:05:00Z\",\
                    \"trace\":\"4bf92f3577b34da6a3ce929d0e0e4736\",\"span\":\"00f067aa0ba902b7\"}";
        Line {
            text: line.text + tail,
            ..line
        }
    })
}

/// Runs what the viewer asked for to its end, answers fed back.
pub(super) fn settle(state: &mut State, task: iced::Task<Message>) {
    let mut pending = vec![task];
    while let Some(task) = pending.pop() {
        for message in block_on(messages(task)) {
            pending.push(state.update(message).0);
        }
    }
}

/// Starts a search for "ERROR" and reads every line for it, as the
/// window would before the next lines come.
pub(super) fn searching(state: &mut State) {
    let (task, _) = state.update(Message::Query("ERROR".to_owned()));
    settle(state, task);
    assert!(!state.search.busy && !state.is_searching());
    assert!(state.matches().1 > 0);
}

/// A viewer that holds `lines` lines of a busy service, with its
/// timestamps showing, and keeps no more. `wrapping`: lines of three
/// rows each; else lines that fit a row.
pub(super) fn full(lines: usize, wrapping: bool) -> State {
    let mut state = State::new(lines).with_timestamps(true);
    if wrapping {
        drop(state.extend(long(0, lines)));
    } else {
        drop(state.extend(many(0, lines)));
    }
    state
}

/// Frames that each scroll by a notch of the wheel, from the middle of
/// the lines: the middle one.
pub(super) fn scrolling(bench: &mut Bench, state: &mut State) -> Frame {
    let _ = state.update(Message::Scroll(Scroll::To(0.5)));
    bench.settle(state);
    let frames = (0..40)
        .map(|_| {
            let start = Instant::now();
            let _ = state.update(Message::Scroll(Scroll::Rows(3)));
            let update = start.elapsed();
            let (mut frame, _) = bench.frame(state);
            frame.update = update;
            frame
        })
        .collect();
    median(frames)
}
