//! What a frame of the viewer costs with a million lines in it,
//! measured: a scroll step, a batch of lines appended, and a frame of a
//! view that follows a stream of fifty thousand lines a second.
//!
//! `cargo test -p homeport-app --lib log::tests::perf -- --ignored
//! --nocapture --test-threads=1` prints the numbers and holds the
//! viewer's own share of a frame (`update`, `view`, layout and draw; not
//! the renderer's pixels) to the budgets below. A test build is not
//! optimised and is given eight times each budget. For what a release
//! build does, set `LOG_PERF_OPTIMISED=1`, which holds it to the budgets
//! themselves, and add `--config
//! profile.dev.package.homeport-app.opt-level=2` and the same with
//! `opt-level=3` for what shapes and draws text and what searches it:
//! `cosmic-text`, `harfrust`, `skrifa`, `read-fonts`, `font-types`,
//! `swash`, `zeno`, `fontdb`, `ttf-parser`, `unicode-bidi`,
//! `unicode-linebreak`, `unicode-script`, `unicode-segmentation`,
//! `rangemap`, `rustc-hash`, `hashbrown`, `smol_str`, `bytemuck`,
//! `iced_core`, `iced_graphics`, `iced_runtime`, `iced_widget`,
//! `iced_tiny_skia`, `tiny-skia`, `tiny-skia-path`, `regex`,
//! `regex-automata`, `regex-syntax`, `aho-corasick` and `memchr`. Without
//! the shaper among them (the table's shorter list), a row shaped anew
//! costs five times as much and the frames that shape a whole screen go
//! over their budget.
//!
//! The budgets, of a frame of 16.7 ms at sixty a second:
//!
//! - a scroll step: 2 ms, the table's;
//! - a sixtieth of a second of a stream at 50,000 lines a second (833
//!   lines) appended to a full buffer: 1 ms;
//! - a frame of a view that follows that stream (the lines appended, and
//!   every row in view shaped anew): 4 ms, a quarter of the frame;
//! - any `update` of a search over the million lines: 2 ms, however long
//!   the lines take to read on their thread.

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

type Page = gallery::Message;

const WINDOW: Size = Size::new(1100.0, 720.0);
const MILLION: usize = 1_000_000;
/// A sixtieth of a second at 50,000 lines a second.
const BATCH: usize = 50_000 / 60;

const SCROLL: Duration = Duration::from_millis(2);
const APPEND: Duration = Duration::from_millis(1);
const FOLLOW: Duration = Duration::from_millis(4);
const SEARCH: Duration = Duration::from_millis(2);

/// What each part of a frame took.
#[derive(Debug, Clone, Copy, Default)]
struct Frame {
    /// The viewer's `update` or `extend` before the frame.
    update: Duration,
    view: Duration,
    layout: Duration,
    draw: Duration,
    pixels: Duration,
}

impl Frame {
    /// The viewer's share: everything but the renderer's pixels.
    fn own(self) -> Duration {
        self.update + self.view + self.layout + self.draw
    }

    fn total(self) -> Duration {
        self.own() + self.pixels
    }
}

/// The window, without one: a renderer in software and the tree the
/// views share.
struct Bench {
    renderer: iced::Renderer,
    cache: Option<Cache>,
}

impl Bench {
    fn new() -> Self {
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
    fn frame(&mut self, state: &State) -> (Frame, Vec<Page>) {
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
    fn settle(&mut self, state: &mut State) {
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
fn median(mut frames: Vec<Frame>) -> Frame {
    frames.sort_by_key(|frame| frame.own());
    frames[frames.len() / 2]
}

fn middle(mut times: Vec<Duration>) -> Duration {
    times.sort();
    times[times.len() / 2]
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn report(what: &str, frame: Frame) {
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

fn optimised() -> bool {
    std::env::var_os("LOG_PERF_OPTIMISED").is_some()
}

/// Holds `took` to `budget`, eight times over in a build that is not
/// optimised.
fn within(what: &str, took: Duration, budget: Duration) {
    let allowed = if optimised() { budget } else { budget * 8 };
    assert!(
        took <= allowed,
        "{what}: {:.3} ms of the viewer's own work, over {:.0} ms",
        ms(took),
        ms(allowed)
    );
}

/// A viewer that holds `lines` lines of a busy service, with its
/// timestamps showing, and keeps no more.
fn full(lines: usize) -> State {
    let mut state = State::new(lines).with_timestamps(true);
    drop(state.extend(many(0, lines)));
    state
}

/// Frames that each scroll by a notch of the wheel, from the middle of
/// the lines: the middle one.
fn scrolling(bench: &mut Bench, state: &mut State) -> Frame {
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

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn a_scroll_step_among_a_million_lines() {
    let _english = in_language("en");
    let mut bench = Bench::new();
    println!(
        "\nA window of {} by {}, software renderer, {}:",
        WINDOW.width,
        WINDOW.height,
        if optimised() {
            "optimised"
        } else {
            "not optimised"
        }
    );
    let mut few = full(1000);
    let scrolling_few = scrolling(&mut bench, &mut few);
    report("1,000 lines, wrapped, a scroll step", scrolling_few);

    let mut state = full(MILLION);
    bench.settle(&mut state);
    let still = median((0..40).map(|_| bench.frame(&state).0).collect());
    report("1,000,000 lines, wrapped, still", still);
    let wrapped = scrolling(&mut bench, &mut state);
    report("1,000,000 lines, wrapped, a scroll step", wrapped);
    let _ = state.update(Message::Wrap(false));
    let unwrapped = scrolling(&mut bench, &mut state);
    report("1,000,000 lines, not wrapped, a scroll step", unwrapped);
    // A page at a time: every row in view is new.
    let _ = state.update(Message::Wrap(true));
    bench.settle(&mut state);
    let paging = median(
        (0..40)
            .map(|_| {
                let start = Instant::now();
                let _ = state.update(Message::Scroll(Scroll::Pages(1)));
                let update = start.elapsed();
                let (mut frame, _) = bench.frame(&state);
                frame.update = update;
                frame
            })
            .collect(),
    );
    report("1,000,000 lines, wrapped, a page down", paging);
    // The thumb dragged from one end to the other.
    let dragging = median(
        (0..40_u8)
            .map(|step| {
                let start = Instant::now();
                let to = Scroll::To(f32::from(step) / 40.0);
                let _ = state.update(Message::Scroll(to));
                let update = start.elapsed();
                let (mut frame, _) = bench.frame(&state);
                frame.update = update;
                frame
            })
            .collect(),
    );
    report("1,000,000 lines, wrapped, the thumb dragged", dragging);

    within("a scroll step, wrapped", wrapped.own(), SCROLL);
    within("a scroll step, not wrapped", unwrapped.own(), SCROLL);
    within("a page down", paging.own(), FOLLOW);
    within("the thumb dragged", dragging.own(), FOLLOW);
    // What makes it a virtual list: a thousand times the lines is not a
    // thousand times the work.
    assert!(
        wrapped.own() < scrolling_few.own() * 4,
        "1,000,000 lines cost {:?} a scroll step, 1,000 lines {:?}",
        wrapped.own(),
        scrolling_few.own()
    );
}

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn a_batch_appended_to_a_million_lines() {
    let mut state = full(MILLION);
    assert_eq!(state.len(), MILLION);
    println!();
    // The lines are made before the clock starts: they are the stream's.
    let mut next = MILLION;
    let mut timed = |state: &mut State, size: usize, turns: usize| {
        let times = (0..turns)
            .map(|_| {
                let lines: Vec<Line> = many(next, size).collect();
                next += size;
                let start = Instant::now();
                drop(state.extend(lines));
                start.elapsed()
            })
            .collect();
        middle(times)
    };
    let frame = timed(&mut state, BATCH, 120);
    println!(
        "{BATCH} lines appended to 1,000,000 (the oldest dropped):      {:>7.3} ms",
        ms(frame)
    );
    let second = timed(&mut state, 50_000, 10);
    println!(
        "50,000 lines appended at once (a second's worth):          {:>7.3} ms",
        ms(second)
    );
    // With a search on, the new lines are read for it as they come.
    let _ = state.update(Message::Query("ERROR".to_owned()));
    state.search.frontier = state.buffer.range().end;
    let searched = timed(&mut state, BATCH, 120);
    println!(
        "{BATCH} lines appended and searched for \"ERROR\":               {:>7.3} ms",
        ms(searched)
    );
    assert_eq!(state.len(), MILLION, "the cap holds");
    within("a frame's worth of lines appended", frame, APPEND);
    within("the same with a search on", searched, APPEND * 2);
    // Appending does not look at what is already there: a batch costs
    // the same in a buffer a thousand times smaller.
    let mut small = full(1000);
    let small_frame = timed(&mut small, BATCH, 120);
    println!(
        "{BATCH} lines appended to 1,000:                               {:>7.3} ms",
        ms(small_frame)
    );
    assert!(
        frame < small_frame * 4 + Duration::from_micros(200),
        "{frame:?} into a million lines, {small_frame:?} into a thousand"
    );
}

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn following_a_stream_of_fifty_thousand_lines_a_second() {
    let _english = in_language("en");
    let mut bench = Bench::new();
    let mut state = full(MILLION);
    bench.settle(&mut state);
    println!();
    let mut next = MILLION;
    // Two seconds of the stream, a frame at a time.
    let mut run = |bench: &mut Bench, state: &mut State| -> Vec<Frame> {
        (0..120)
            .map(|_| {
                let lines: Vec<Line> = many(next, BATCH).collect();
                next += BATCH;
                let start = Instant::now();
                drop(state.extend(lines));
                let update = start.elapsed();
                let (mut frame, _) = bench.frame(state);
                frame.update = update;
                frame
            })
            .collect()
    };
    let frames = run(&mut bench, &mut state);
    assert!(state.follows());
    assert_eq!(state.len(), MILLION);
    let worst = frames
        .iter()
        .map(|frame| frame.own())
        .max()
        .expect("frames");
    let whole: Duration = frames.iter().map(|frame| frame.own()).sum();
    let following = median(frames);
    report("following 50,000 lines a second, a frame", following);
    println!(
        "  the worst frame {:.3} ms; two seconds of the stream took {:.1} ms of the viewer's work \
         ({:.1} % of the time)",
        ms(worst),
        ms(whole),
        ms(whole) / 2000.0 * 100.0
    );
    // Paused, the lines still come and the view stays: nothing is shaped.
    let _ = state.update(Message::Scroll(Scroll::Rows(-200)));
    bench.settle(&mut state);
    let paused = median(run(&mut bench, &mut state));
    report("the same, paused 200 rows up", paused);
    assert!(!state.follows());
    // Following again, with a search on: the new lines are read for it.
    let _ = state.update(Message::Scroll(Scroll::End));
    let _ = state.update(Message::Query("ERROR".to_owned()));
    state.search.frontier = state.buffer.range().end;
    bench.settle(&mut state);
    let searching = median(run(&mut bench, &mut state));
    report("following, with a search on", searching);

    within("a frame of a view that follows", following.own(), FOLLOW);
    within("the worst frame of two seconds", worst, FOLLOW * 2);
    within("a frame while paused", paused.own(), FOLLOW);
    within(
        "a frame that follows with a search on",
        searching.own(),
        FOLLOW,
    );
    println!(
        "  a frame is {:.1} % of a 60 fps frame with the software renderer's pixels, {:.1} % \
         without",
        ms(following.total()) / (1000.0 / 60.0) * 100.0,
        ms(following.own()) / (1000.0 / 60.0) * 100.0,
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
async fn a_search_over_a_million_lines_never_holds_update() {
    let mut state = full(MILLION);
    println!();
    for (query, regex) in [("ERROR", false), ("id=\\d+7 path", true), ("e", false)] {
        let _ = state.update(Message::Regex(regex));
        let start = Instant::now();
        let (mut task, _) = state.update(Message::Query(query.to_owned()));
        let mut worst = start.elapsed();
        let mut slices = 0;
        loop {
            let found = messages(task).await;
            let Some(message) = found.into_iter().next() else {
                break;
            };
            slices += 1;
            let update = Instant::now();
            task = state.update(message).0;
            worst = worst.max(update.elapsed());
        }
        let whole = start.elapsed();
        let (_, total) = state.matches();
        println!(
            "{query:?}{}: {total} matches in {slices} slices, {:.0} ms in all on another thread; \
             the longest update {:.3} ms",
            if regex {
                " as a regular expression"
            } else {
                ""
            },
            ms(whole),
            ms(worst)
        );
        assert!(!state.is_searching());
        assert!(total > 0);
        within(
            &format!("an update of the search for {query:?}"),
            worst,
            SEARCH,
        );
    }
    // Stepping through matches of a million lines: the first step counts
    // the matches before it, the others don't.
    let start = Instant::now();
    let _ = state.update(Message::Previous);
    let first = start.elapsed();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = state.update(Message::Previous);
    }
    let step = start.elapsed() / 100;
    println!(
        "the first step to a match {:.3} ms, each one after {:.4} ms",
        ms(first),
        ms(step)
    );
    within("the first step to a match", first, SEARCH * 2);
    within("a step to a match", step, SEARCH);
}
