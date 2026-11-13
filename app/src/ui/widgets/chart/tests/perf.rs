//! What a frame of the chart costs with ten series of an hour of
//! samples (3,600 each), measured: the ticket's "10 series × 3,600
//! points at 60 fps".
//!
//! The numbers that count are those of the build that ships, the
//! workspace's release profile (`opt-level = "s"`, LTO, one codegen
//! unit):
//!
//! `cargo test --release -p homeport-app --lib chart::tests::perf --
//! --ignored --nocapture --test-threads=1`
//!
//! It prints them and holds the chart's own share of a frame (`push`,
//! `view`, layout and draw: the scene, the points picked, the paths
//! built; not the renderer's pixels) to the budgets below. Without
//! `--release` the build has its debug assertions and is not optimised:
//! it says so, and is given eight times each budget.
//!
//! The budgets, of a frame of 16.7 ms at sixty a second:
//!
//! - a frame in which nothing changed, and one in which only the
//!   pointer moved over the plot (the crosshair and its tooltip, on the
//!   lines kept from the last draw): 1 ms;
//! - a frame after a sample was appended, which scans the samples for
//!   the range, picks the points of every series again and builds
//!   their paths: 4 ms, a quarter of the frame. A stream gives one
//!   sample a second; the test gives one every frame, sixty times that;
//! - the worst of those frames: 8 ms, half a frame.
//!
//! The pixels are the software renderer's and are printed, not held:
//! the window draws with wgpu, which these tests cannot run.

use std::time::{Duration, Instant};

use iced::Point;

use super::{worked, Bench, Took};
use crate::{
    test_support::snapshot::in_language,
    ui::widgets::{
        chart::State,
        gallery::chart::{self as sample, HOUR, MANY},
    },
};

const WINDOW: (f32, f32) = (1100.0, 420.0);
const STILL: Duration = Duration::from_millis(1);
const APPEND: Duration = Duration::from_millis(4);
const WORST: Duration = Duration::from_millis(8);
const FRAMES: usize = 120;

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// Whether this is the build that ships: the release profile, which has
/// no debug assertions. Nothing else is taken for it.
fn optimised() -> bool {
    !cfg!(debug_assertions)
}

fn build() -> &'static str {
    if optimised() {
        "the release profile"
    } else {
        "NOT OPTIMISED (a debug build: budgets are eight times over)"
    }
}

fn within(what: &str, took: Duration, budget: Duration) {
    let allowed = if optimised() { budget } else { budget * 8 };
    assert!(
        took <= allowed,
        "{what}: {:.3} ms of the chart's own work, over {:.0} ms",
        ms(took),
        ms(allowed)
    );
}

/// One frame's cost: the sample appended before it, and the frame.
#[derive(Clone, Copy)]
struct Cost {
    push: Duration,
    took: Took,
}

impl Cost {
    fn own(self) -> Duration {
        self.push + self.took.own()
    }
}

fn report(what: &str, costs: &mut [Cost]) -> (Cost, Cost) {
    costs.sort_by_key(|cost| cost.own());
    let (median, worst) = (costs[costs.len() / 2], costs[costs.len() - 1]);
    println!(
        "{what:<46} push {:>6.3}  view {:>6.3}  layout {:>6.3}  draw {:>6.3}  = own {:>6.3} ms \
         (worst {:>6.3})   pixels {:>7.3} ms",
        ms(median.push),
        ms(median.took.view),
        ms(median.took.layout),
        ms(median.took.draw),
        ms(median.own()),
        ms(worst.own()),
        ms(median.took.pixels),
    );
    (median, worst)
}

/// Ten series of an hour, drawn once so every cache is filled.
fn settled() -> (Bench, State) {
    let state = sample::many(HOUR);
    assert_eq!(
        (state.samples().len(), state.samples().series()),
        (HOUR, MANY)
    );
    let mut bench = Bench::new(WINDOW);
    for _ in 0..3 {
        bench.frame(|| sample::many_view(&state), None, true);
    }
    (bench, state)
}

#[test]
#[ignore = "timing: run it alone, with --release for the numbers that count"]
fn a_frame_of_ten_series_of_an_hour_fits_sixty_a_second() {
    let _english = in_language("en");
    println!(
        "ten series × 3,600 samples, a {} × {} window, {}",
        WINDOW.0,
        WINDOW.1,
        build()
    );
    let (mut bench, mut state) = settled();

    // Nothing changed: the lines are the last draw's.
    let before = worked();
    let mut still: Vec<Cost> = (0..FRAMES)
        .map(|_| Cost {
            push: Duration::ZERO,
            took: bench.frame(|| sample::many_view(&state), None, true),
        })
        .collect();
    assert_eq!(worked(), before, "a still frame worked nothing out");
    let (median, _) = report("a frame that changes nothing", &mut still);
    within("a frame that changes nothing", median.own(), STILL);

    // The pointer crosses the plot: a crosshair and a tooltip a frame.
    let before = worked();
    let mut pointed: Vec<Cost> = (0..FRAMES)
        .map(|n| {
            // Frames are counted on two hands.
            #[allow(clippy::cast_precision_loss)]
            let at = Point::new(120.0 + n as f32 * 7.0, 140.0);
            Cost {
                push: Duration::ZERO,
                took: bench.frame(|| sample::many_view(&state), Some(at), true),
            }
        })
        .collect();
    assert_eq!(worked(), before, "the pointer worked nothing out");
    let (median, worst) = report("the pointer moving over the plot", &mut pointed);
    within("the pointer moving over the plot", median.own(), STILL);
    within("the worst frame under the pointer", worst.own(), WORST);

    // A sample every frame, with the pointer on the plot: everything
    // is worked out again, each frame.
    let before = worked();
    let mut appended: Vec<Cost> = (0..FRAMES)
        .map(|n| {
            #[allow(clippy::cast_precision_loss)]
            let time = sample::START + (HOUR + n) as f64;
            let start = Instant::now();
            state.push(time, &[Some(60.0); MANY]);
            let push = start.elapsed();
            let at = Some(Point::new(500.0, 140.0));
            Cost {
                push,
                took: bench.frame(|| sample::many_view(&state), at, true),
            }
        })
        .collect();
    let after = worked();
    // The path it names was taken: a scene and ten series a frame.
    assert_eq!(
        (after.0 - before.0, after.1 - before.1),
        (FRAMES, FRAMES * MANY)
    );
    assert_eq!(state.samples().len(), HOUR, "the ring is full and stays so");
    let (median, worst) = report("a sample appended, every frame", &mut appended);
    within("a frame after a sample", median.own(), APPEND);
    within("the worst frame after a sample", worst.own(), WORST);
    let pushes: Duration = appended.iter().map(|cost| cost.push).sum();
    println!(
        "appending a sample to the ten full rings: {:.4} ms",
        ms(pushes) / appended.len() as f64
    );
}

/// What thinning saves: the same frames with every sample drawn would
/// build ten paths of 3,600 points; they build ten of at most the
/// plot's width.
#[test]
#[ignore = "timing: run it alone, with --release for the numbers that count"]
fn an_hour_costs_little_more_than_five_minutes() {
    let _english = in_language("en");
    println!("{}", build());
    let mut costs = vec![];
    for samples in [300, HOUR] {
        let mut state = sample::many(samples);
        let mut bench = Bench::new(WINDOW);
        bench.frame(|| sample::many_view(&state), None, true);
        let mut frames: Vec<Cost> = (0..FRAMES)
            .map(|n| {
                #[allow(clippy::cast_precision_loss)]
                let time = sample::START + (samples + n) as f64;
                let start = Instant::now();
                state.push(time, &[Some(60.0); MANY]);
                Cost {
                    push: start.elapsed(),
                    took: bench.frame(|| sample::many_view(&state), None, true),
                }
            })
            .collect();
        let (median, _) = report(
            &format!("a sample appended to {samples} × ten"),
            &mut frames,
        );
        costs.push(median.own());
    }
    // Twelve times the samples; the points drawn are 3.4 times as many.
    assert!(
        costs[1] <= costs[0] * 8,
        "an hour: {:.3} ms, five minutes: {:.3} ms",
        ms(costs[1]),
        ms(costs[0])
    );
}
