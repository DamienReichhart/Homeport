//! What a frame works out, counted, and what the keyboard and the
//! pointer do to the legend, on a window that lasts.
//!
//! The counts are numbers no machine changes: how many times the
//! samples were scanned for a scene, how many series had their points
//! picked, and how many points were drawn.

use iced::{keyboard::key::Named, Point};

use super::{paths, worked, Bench};
use crate::{
    test_support::{snapshot::in_language, stage::Stage},
    ui::{
        theme::Theme,
        widgets::{
            chart::{Message, State},
            gallery::{
                self,
                chart::{self as sample, Event, HOUR, MANY},
            },
        },
    },
};

const WINDOW: (f32, f32) = (900.0, 360.0);
/// Somewhere on the plot of that window.
const ON_PLOT: Point = Point::new(400.0, 120.0);

/// What a frame of the ten series worked out.
fn frame(bench: &mut Bench, state: &State, pointer: Option<Point>) -> (usize, usize, usize) {
    let before = worked();
    bench.frame(|| sample::many_view(state), pointer, false);
    let after = worked();
    (after.0 - before.0, after.1 - before.1, after.2 - before.2)
}

/// The ticket's "not recomputed per frame for unchanged data".
#[test]
fn a_frame_that_changes_nothing_scans_no_sample_and_picks_no_point() {
    let _english = in_language("en");
    let mut state = sample::many(HOUR);
    let mut bench = Bench::new(WINDOW);
    // The first frame works everything out, once.
    let (scenes, series, points) = frame(&mut bench, &state, None);
    assert_eq!((scenes, series), (1, MANY));
    // Ten lines of 3,600 samples, thinned to a point a pixel of a
    // plot under 852 px wide: far fewer than the 36,000 there are.
    assert!(
        (MANY * 700..=MANY * 852).contains(&points),
        "{points} points drawn"
    );
    // The same again, and with the pointer moving over the plot.
    assert_eq!(frame(&mut bench, &state, None), (0, 0, 0));
    for x in [300.0, 301.0, 420.0, 700.0] {
        let moved = frame(&mut bench, &state, Some(Point::new(x, ON_PLOT.y)));
        assert_eq!(moved, (0, 0, 0), "the pointer at {x}");
    }
    // A sample: everything once more, and no more than once.
    state.push(sample::START + 3600.0, &[Some(50.0); MANY]);
    let (scenes, series, _) = frame(&mut bench, &state, Some(ON_PLOT));
    assert_eq!((scenes, series), (1, MANY));
    assert_eq!(frame(&mut bench, &state, Some(ON_PLOT)), (0, 0, 0));
    // A series hidden: nine are drawn.
    state.update(Message::Toggle(3));
    let (scenes, series, _) = frame(&mut bench, &state, None);
    assert_eq!((scenes, series), (1, MANY - 1));
}

#[test]
fn what_changes_how_the_chart_looks_draws_it_again() {
    let _english = in_language("en");
    let state = sample::many(600);
    let mut bench = Bench::new(WINDOW);
    assert_eq!(frame(&mut bench, &state, None).1, MANY);
    assert_eq!(frame(&mut bench, &state, None).1, 0);
    // The other theme: other colours.
    bench.theme = Theme::Dark;
    assert_eq!(frame(&mut bench, &state, None).1, MANY);
    assert_eq!(frame(&mut bench, &state, None).1, 0);
    // Another language: other labels, and the axis on the other side.
    drop(_english);
    let _arabic = in_language("ar");
    assert_eq!(frame(&mut bench, &state, None), (1, MANY, 600 * MANY));
    assert_eq!(frame(&mut bench, &state, None), (0, 0, 0));
    // Another size.
    let mut wider = Bench::new((1200.0, 360.0));
    wider.cache = bench.cache.take();
    assert_eq!(frame(&mut wider, &state, None).1, MANY);
}

/// The ticket's "10 series × 3,600 points at 60 fps", as what a frame
/// is given to do: numbers no machine changes, held on every
/// `cargo test`. (How long that takes is `perf.rs`, which is timed and
/// ignored.) After a sample: one pass over the samples, ten series
/// thinned, a line each of no more points than the plot has pixels.
/// Between two samples: nothing.
#[test]
fn a_frame_of_ten_series_of_an_hour_is_given_a_bounded_amount_to_draw() {
    let _english = in_language("en");
    let mut state = sample::many(HOUR);
    let mut bench = Bench::new(WINDOW);
    frame(&mut bench, &state, None);
    // The plot of a 900 px window is under 852 px wide.
    let most = MANY * 856;
    for n in 0..20 {
        // Counts are small.
        #[allow(clippy::cast_precision_loss)]
        state.push(sample::START + (HOUR + n) as f64, &[Some(50.0); MANY]);
        let lines = paths();
        let (scenes, series, points) = frame(&mut bench, &state, Some(ON_PLOT));
        assert_eq!(
            (scenes, series, paths() - lines),
            (1, MANY, MANY),
            "frame {n}"
        );
        assert!(points <= most, "{points} points after a sample");
        // The frames until the next sample, pointer moving or not.
        let lines = paths();
        assert_eq!(frame(&mut bench, &state, Some(ON_PLOT)), (0, 0, 0));
        assert_eq!(frame(&mut bench, &state, None), (0, 0, 0));
        assert_eq!(paths(), lines);
    }
}

/// Gaps do not undo the bound: a series whose every other sample is
/// missing is 1,800 samples each alone, which were 1,800 dots. They are
/// a dot a pixel at the most.
#[test]
fn a_series_cut_up_by_gaps_draws_a_mark_a_pixel_at_the_most() {
    let _english = in_language("en");
    let mut state = State::new(MANY, HOUR);
    for n in 0..HOUR {
        let value = (n % 2 == 0).then_some(50.0);
        #[allow(clippy::cast_precision_loss)]
        state.push(sample::START + n as f64, &[value; MANY]);
    }
    let mut bench = Bench::new(WINDOW);
    let lines = paths();
    let (_, series, points) = frame(&mut bench, &state, None);
    let marks = paths() - lines;
    assert_eq!(series, MANY);
    assert!(marks <= MANY * 856, "{marks} marks");
    assert!(points <= MANY * 856, "{points} points");
    // And they are drawn: a series of dots is still there to see.
    assert!(marks >= MANY * 400, "{marks} marks");
    // Short runs between gaps too: three samples, then one missing.
    let mut state = State::new(1, HOUR);
    for n in 0..HOUR {
        #[allow(clippy::cast_precision_loss)]
        state.push(sample::START + n as f64, &[(n % 4 != 3).then_some(50.0)]);
    }
    let lines = paths();
    let (_, _, points) = frame(&mut bench, &state, None);
    assert!(
        paths() - lines <= 856 && points <= 3 * 856,
        "{points} points"
    );
}

/// Few enough samples are all drawn: nothing is thinned that fits.
#[test]
fn a_short_line_keeps_every_sample() {
    let _english = in_language("en");
    let state = sample::cpu(300);
    let mut bench = Bench::new(WINDOW);
    let before = worked();
    bench.frame(|| sample::cpu_view(&state), None, false);
    assert_eq!(worked().2 - before.2, 300);
}

type Sent = gallery::Message;

fn toggles(messages: &[Sent]) -> Vec<usize> {
    messages
        .iter()
        .filter_map(|message| match message {
            Sent::Chart(Event::Network(Message::Toggle(index))) => Some(*index),
            _ => None,
        })
        .collect()
}

/// The kit's focus conventions: each entry of the legend is a button,
/// reached with Tab in the order the series are, pressed with Enter or
/// Space, with a ring while the keyboard has it there.
#[test]
fn the_legend_is_reached_with_tab_and_pressed_with_enter_or_space() {
    let _english = in_language("en");
    let mut state = sample::network(300);
    let mut stage = Stage::new(WINDOW);
    let messages = stage.frame(sample::network_view(&state), |frame| {
        frame.tab();
        assert!(frame.is_focused("Received"));
        assert!(frame.shows_focus());
        frame.tap_key(Named::Enter);
    });
    assert_eq!(toggles(&messages), [0]);
    state.update(Message::Toggle(0));
    assert!(state.is_hidden(0));
    // Focus stays on the entry it pressed; Space shows the series again.
    let messages = stage.frame(sample::network_view(&state), |frame| {
        assert!(frame.is_focused("Received"));
        frame.tap_key(Named::Space);
    });
    assert_eq!(toggles(&messages), [0]);
    state.update(Message::Toggle(0));
    // Tab goes on to the next series, and Shift+Tab back.
    let messages = stage.frame(sample::network_view(&state), |frame| {
        frame.tab();
        assert!(frame.is_focused("Sent"));
        frame.tap_key(Named::Enter);
        frame.tab_back();
        assert!(frame.is_focused("Received"));
    });
    assert_eq!(toggles(&messages), [1]);
}

#[test]
fn a_press_on_a_legend_entry_hides_its_series() {
    let _english = in_language("en");
    let state = sample::network(300);
    let mut stage = Stage::new(WINDOW);
    let messages = stage.frame(sample::network_view(&state), |frame| frame.click("Sent"));
    assert_eq!(toggles(&messages), [1]);
    // The chart itself sends nothing: the pointer on it only draws.
    let messages = stage.frame(sample::network_view(&state), |frame| {
        frame.point_at(ON_PLOT);
        frame.press();
    });
    assert!(messages.is_empty());
}

/// An entry says what a press does, in the app's language, and names
/// its series whole.
#[test]
fn a_legend_entry_says_what_pressing_it_does() {
    let _english = in_language("en");
    let mut state = sample::network(300);
    let mut stage = Stage::new(WINDOW);
    stage.frame(sample::network_view(&state), |frame| {
        // Keyboard focus shows a tooltip at once.
        frame.tab();
        frame.redraw();
        assert!(
            frame.texts().contains(&"Hide Received".to_owned()),
            "{:?}",
            frame.texts()
        );
    });
    state.update(Message::Toggle(0));
    stage.frame(sample::network_view(&state), |frame| {
        frame.redraw();
        assert!(
            frame.texts().contains(&"Show Received".to_owned()),
            "{:?}",
            frame.texts()
        );
    });
}
