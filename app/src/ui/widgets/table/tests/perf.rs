//! What a frame of the table costs, measured: `view`, layout, draw and
//! the software renderer's pixels, for a list of a hundred thousand rows
//! and for the ticket's budget (20,000 rows of 8 columns at 60 frames a
//! second, and a changed row in under 2 ms of `view`).
//!
//! `cargo test -p homeport-app --lib table::tests::perf -- --ignored
//! --nocapture --test-threads=1` prints the numbers and holds the table's
//! own share of a frame (not the renderer's pixels) to the budget. A test
//! build is not optimised and is given five times the budget; for what a
//! release build does, add `--config
//! profile.dev.package.homeport-app.opt-level=2`, the same with
//! `opt-level=3` for `tiny-skia`, `iced_tiny_skia`, `iced_graphics`,
//! `iced_core`, `iced_widget` and `cosmic-text`, and set
//! `TABLE_PERF_OPTIMISED=1`, which holds it to the budget itself.
//! `counts.rs` is what fails on every machine when a view starts doing
//! more.

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

use super::Page;
use crate::{
    test_support::snapshot::{in_language, settings},
    ui::{
        theme::{self, Element, Theme},
        widgets::{
            gallery::table::{columns, samples, Sample},
            table::{self, Align, Column, Message, Mode, Order, Rows, Sort, State, Table, Width},
        },
    },
};

const WINDOW: Size = Size::new(1100.0, 720.0);

/// What each part of a frame took.
#[derive(Debug, Clone, Copy, Default)]
struct Frame {
    view: Duration,
    layout: Duration,
    draw: Duration,
    pixels: Duration,
}

impl Frame {
    fn total(self) -> Duration {
        self.own() + self.pixels
    }

    /// The table's share: everything but the renderer's pixels.
    fn own(self) -> Duration {
        self.view + self.layout + self.draw
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

    /// One frame of `view`, timed part by part, and the messages its
    /// first redraw produced.
    fn frame<'a>(&mut self, view: impl FnOnce() -> Element<'a, Page>) -> (Frame, Vec<Page>) {
        let mut took = Frame::default();
        let start = Instant::now();
        let page = container(view())
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
}

/// The frame in the middle of `frames`, by what it took in all.
fn median(mut frames: Vec<Frame>) -> Frame {
    frames.sort_by_key(|frame| frame.total());
    frames[frames.len() / 2]
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn report(what: &str, frame: Frame) {
    println!(
        "{what:<58} view {:>7.3} ms  layout {:>7.3} ms  draw {:>7.3} ms  pixels {:>7.3} ms  = {:>7.3} ms",
        ms(frame.view),
        ms(frame.layout),
        ms(frame.draw),
        ms(frame.pixels),
        ms(frame.total()),
    );
}

/// Eight columns over the same rows: the ticket's table.
fn eight() -> Vec<Column<'static, Sample, Page>> {
    let mut all = columns();
    all.push(
        Column::new("id", "Id", |row: &Sample| {
            table::text(row.id.to_string()).into()
        })
        .width(Width::Fixed(80.0))
        .align(Align::End)
        .sort_by(|a, b| a.id.cmp(&b.id)),
    );
    all.push(Column::new("again", "Name", |row: &Sample| {
        table::text(row.name.clone()).into()
    }));
    all.push(Column::new("tag", "Tag", |row: &Sample| {
        table::text(row.image).into()
    }));
    all
}

fn all(wide: bool) -> Vec<Column<'static, Sample, Page>> {
    if wide {
        eight()
    } else {
        columns()
    }
}

fn read(rows: &[Sample], revision: Option<u64>) -> Rows<'_, Sample, u32> {
    let rows = Rows::new(rows, |row| row.id);
    match revision {
        Some(revision) => rows.revision(revision),
        None => rows,
    }
}

fn table<'a>(
    state: &'a State<u32>,
    rows: &[Sample],
    wide: bool,
    revision: Option<u64>,
) -> Element<'a, Page> {
    Table::new(state, all(wide), read(rows, revision), Page::Table)
        .row_menu(|_| vec![])
        .fill()
        .view()
}

/// The ticket's budget for the table's own work in a frame, five times
/// over in a build that is not optimised.
fn budget() -> Duration {
    let budget = Duration::from_millis(2);
    if std::env::var_os("TABLE_PERF_OPTIMISED").is_some() {
        budget
    } else {
        budget * 5
    }
}

fn within_budget(what: &str, took: Duration) {
    assert!(
        took <= budget(),
        "{what}: {:.3} ms of the table's own work, over {:.0} ms",
        ms(took),
        ms(budget())
    );
}

/// Frames of a table that stays put, then frames that each scroll by a
/// notch of the wheel: the middle one of each.
fn measure(
    bench: &mut Bench,
    state: &mut State<u32>,
    rows: &[Sample],
    wide: bool,
    revision: Option<u64>,
) -> (Frame, Frame) {
    const FRAMES: usize = 40;
    // The first frames tell the table its room and fill the caches.
    for _ in 0..3 {
        let (_, messages) = bench.frame(|| table(state, rows, wide, revision));
        for message in messages {
            if let Page::Table(message) = message {
                state.update(message, &read(rows, revision), &all(wide));
            }
        }
    }
    let still = (0..FRAMES)
        .map(|_| bench.frame(|| table(state, rows, wide, revision)).0)
        .collect();
    let scrolling = (0..FRAMES)
        .map(|_| {
            let start = Instant::now();
            let to = Message::Scrolled(state.offset() + 60.0);
            state.update(to, &read(rows, revision), &all(wide));
            let update = start.elapsed();
            let (mut frame, _) = bench.frame(|| table(state, rows, wide, revision));
            frame.view += update;
            frame
        })
        .collect();
    (median(still), median(scrolling))
}

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn a_frame_of_a_hundred_thousand_rows() {
    let _english = in_language("en");
    let mut bench = Bench::new();
    let multi = || State::new().selecting(Mode::Multi);
    let by_name = || multi().sorted(Sort::by("name", Order::Ascending));
    println!(
        "\nA window of {} by {}, software renderer:",
        WINDOW.width, WINDOW.height
    );

    let few = samples(100);
    let (still_few, scrolling_few) = measure(&mut bench, &mut multi(), &few, false, None);
    report("100 rows, 5 columns, still", still_few);
    report("100 rows, 5 columns, a scroll step", scrolling_few);

    let many = samples(100_000);
    let (still, scrolling) = measure(&mut bench, &mut multi(), &many, false, None);
    report("100,000 rows, 5 columns, still", still);
    report("100,000 rows, 5 columns, a scroll step", scrolling);
    let (still, scrolling) = measure(&mut bench, &mut by_name(), &many, false, None);
    report("100,000 rows, sorted, checked every view, still", still);
    report(
        "100,000 rows, sorted, checked every view, a scroll step",
        scrolling,
    );
    let (still_fast, scrolling) = measure(&mut bench, &mut by_name(), &many, false, Some(1));
    report("100,000 rows, sorted, with a revision, still", still_fast);
    report(
        "100,000 rows, sorted, with a revision, a scroll step",
        scrolling,
    );

    within_budget("100,000 rows, a scroll step", scrolling.own());
    within_budget(
        "100,000 rows sorted with a revision, still",
        still_fast.own(),
    );
    // What makes it a virtual list: a thousand times the rows is not a
    // thousand times the table's work.
    let (_, scrolling_plain) = measure(&mut bench, &mut multi(), &many, false, None);
    within_budget(
        "100,000 rows unsorted, a scroll step",
        scrolling_plain.own(),
    );
    assert!(
        scrolling_plain.own() < scrolling_few.own() * 4,
        "100,000 rows cost {:?} of the table's work a frame, 100 rows {:?}",
        scrolling_plain.own(),
        scrolling_few.own()
    );
}

#[test]
#[ignore = "a measurement: run it alone with --nocapture and read what it prints"]
fn the_budget_of_twenty_thousand_rows_of_eight_columns() {
    let _english = in_language("en");
    let mut bench = Bench::new();
    let mut rows = samples(20_000);
    let by_name = || {
        State::new()
            .selecting(Mode::Multi)
            .sorted(Sort::by("name", Order::Ascending))
    };
    println!(
        "\n20,000 rows of 8 columns, {} by {}:",
        WINDOW.width, WINDOW.height
    );

    let mut state = by_name();
    let (still, scrolling) = measure(&mut bench, &mut state, &rows, true, None);
    report("sorted, checked every view, still", still);
    report("sorted, checked every view, a scroll step", scrolling);
    let mut fast = by_name();
    let (still, scrolling_fast) = measure(&mut bench, &mut fast, &rows, true, Some(1));
    report("sorted, with a revision, still", still);
    report("sorted, with a revision, a scroll step", scrolling_fast);

    // One row changes (its CPU): what `view` costs for the next frame.
    let mut changed = |state: &mut State<u32>, revision: Option<u64>| {
        let views = (0..40_u32)
            .map(|turn| {
                rows[(turn * 97) as usize].cpu += 1;
                let revision = revision.map(|revision| revision + u64::from(turn));
                let start = Instant::now();
                drop(table(state, &rows, true, revision));
                start.elapsed()
            })
            .collect::<Vec<_>>();
        let mut views = views;
        views.sort();
        views[views.len() / 2]
    };
    let checked = changed(&mut state, None);
    let stamped = changed(&mut fast, Some(100));
    println!(
        "a changed row, view alone: checked every view {:.3} ms, with a new revision {:.3} ms",
        ms(checked),
        ms(stamped)
    );
    println!(
        "a scroll step is {:.1} % of a 60 fps frame (checked) and {:.1} % (with a revision); \
         the table's own work is {:.1} % and {:.1} %",
        ms(scrolling.total()) / (1000.0 / 60.0) * 100.0,
        ms(scrolling_fast.total()) / (1000.0 / 60.0) * 100.0,
        ms(scrolling.own()) / (1000.0 / 60.0) * 100.0,
        ms(scrolling_fast.own()) / (1000.0 / 60.0) * 100.0,
    );
    within_budget("20,000 rows of 8 columns, a scroll step", scrolling.own());
    within_budget("the same with a revision", scrolling_fast.own());
    within_budget("a changed row, view alone", checked);
    within_budget("a changed row with a new revision, view alone", stamped);
}
