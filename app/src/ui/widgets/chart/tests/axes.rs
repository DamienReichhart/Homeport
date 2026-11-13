//! Where the axes put things: the range of the values, the ticks and
//! what they say in the app's language, the plot in the room it has in
//! either direction, the runs of a line, and the tooltip.

use iced::{Point, Size};
use jiff::tz::TimeZone;

use crate::{
    test_support::snapshot::in_language,
    ui::widgets::chart::{
        plot::{enter, runs},
        scale::{range, step, ticks, time_step, time_ticks},
        scene::{Ask, Scene},
        tip::{self, Tip},
        Look, State, Unit,
    },
};

/// Every character seven pixels wide: what a label takes, without a font.
fn measure(text: &str) -> f32 {
    // A label is a few characters.
    #[allow(clippy::cast_precision_loss)]
    let wide = text.chars().count() as f32;
    wide * 7.0
}

static UTC: TimeZone = TimeZone::UTC;

fn ask(size: (f32, f32), hidden: &[bool], unit: Unit, rtl: bool) -> Ask<'_> {
    Ask {
        size: Size::new(size.0, size.1),
        hidden,
        window: None,
        unit,
        rtl,
        zone: &UTC,
        measure: &measure,
    }
}

/// A chart of one gauge sampled each second from a round minute.
fn gauge(values: &[Option<f64>]) -> State {
    let mut state = State::new(1, 10_000);
    for (n, value) in values.iter().enumerate() {
        // Counts are small.
        #[allow(clippy::cast_precision_loss)]
        state.push(1_777_629_600.0 + n as f64, &[*value]);
    }
    state
}

fn labels(ticks: &[crate::ui::widgets::chart::scene::Tick]) -> Vec<&str> {
    ticks.iter().map(|tick| tick.label.as_str()).collect()
}

#[test]
fn the_values_go_from_zero_to_a_tenth_over_the_highest() {
    // uPlot's `range: [0, max > 0 ? max * 1.1 : 1]`.
    assert_eq!(range(Some((20.0, 100.0))), (0.0, 110.00000000000001));
    assert_eq!(range(Some((0.0, 0.0))), (0.0, 1.0));
    assert_eq!(range(None), (0.0, 1.0));
    // All equal: a flat line under the top, never a range of no height.
    let (low, high) = range(Some((42.0, 42.0)));
    assert!(low == 0.0 && high > 42.0);
    // Under zero the range opens downward the same way.
    assert_eq!(range(Some((-10.0, -10.0))), (-11.0, 0.0));
    assert_eq!(range(Some((-10.0, 20.0))), (-11.0, 22.0));
    // The greatest number there is stays a number.
    assert!(range(Some((0.0, f64::MAX))).1.is_finite());
}

#[test]
fn ticks_stand_on_round_numbers_of_their_unit() {
    assert_eq!(step(110.0, 5, false), 50.0);
    assert_eq!(step(1.0, 5, false), 0.2);
    assert_eq!(step(0.003, 4, false), 0.001);
    assert_eq!(ticks((0.0, 110.0), 5, false), [0.0, 50.0, 100.0]);
    assert_eq!(ticks((-11.0, 22.0), 4, false), [-10.0, 0.0, 10.0, 20.0]);
    // Sizes step by powers of two of the unit they are said in.
    let mib = 1024.0 * 1024.0;
    assert_eq!(step(300.0 * mib, 4, true), 128.0 * mib);
    assert_eq!(step(3.0 * 1024.0 * mib, 4, true), 1024.0 * mib);
    assert_eq!(step(900.0, 4, true), 256.0);
    // Under a byte there is no power of two to stand on.
    assert_eq!(step(0.5, 5, true), 0.1);
    // A range of nothing, or of no number, has a step all the same.
    assert_eq!(step(0.0, 5, false), 1.0);
    assert_eq!(step(f64::NAN, 5, true), 1.0);
    assert!(ticks((0.0, f64::INFINITY), 5, false).is_empty());
}

#[test]
fn time_ticks_stand_on_round_times_of_the_clock_they_are_read_on() {
    assert_eq!(time_step(300.0, 5), 60.0);
    assert_eq!(time_step(3600.0, 6), 600.0);
    assert_eq!(time_step(1.0, 2), 1.0);
    assert_eq!(time_step(86_400.0 * 400.0, 4), 86_400.0 * 100.0);
    // Five minutes from 10:00:07: a tick at each whole minute.
    let (step, ticks) = time_ticks((607.0, 907.0), 5, 0.0);
    assert_eq!(
        (step, ticks),
        (60.0, vec![660.0, 720.0, 780.0, 840.0, 900.0])
    );
    // A clock half an hour ahead of UTC has its hours on UTC's half hours.
    let (step, ticks) = time_ticks((0.0, 10_800.0), 3, 1800.0);
    assert_eq!((step, ticks), (3600.0, vec![1800.0, 5400.0, 9000.0]));
}

#[test]
fn the_axes_say_their_values_in_the_chart_s_unit_and_language() {
    let state = gauge(&[Some(100.0 * 1024.0 * 1024.0), Some(300.0 * 1024.0 * 1024.0)]);
    {
        let _english = in_language("en");
        let scene = Scene::build(
            state.samples(),
            &ask((600.0, 160.0), &[], Unit::Bytes, false),
        );
        assert_eq!(labels(&scene.y_ticks), ["0 B", "128.0 MB", "256.0 MB"]);
        assert_eq!(Unit::BytesPerSecond.format(1536.0), "1.5 KB/s");
        assert_eq!(Unit::BitsPerSecond.format(1500.0), "1.5 kbit/s");
        assert_eq!(Unit::BitsPerSecond.format(999.0), "999 bit/s");
        assert_eq!(Unit::BitsPerSecond.format(2.5e9), "2.5 Gbit/s");
        assert_eq!(Unit::BitsPerSecond.format(f64::NAN), "0 bit/s");
        assert_eq!(Unit::Percent.format(3.456), "3.5%");
        assert_eq!(Unit::Number.format(12_345.0), "12,345");
        assert_eq!(
            Unit::Custom(|cores| format!("{cores} cores")).format(2.0),
            "2 cores"
        );
    }
    let _french = in_language("fr");
    let scene = Scene::build(
        state.samples(),
        &ask((600.0, 160.0), &[], Unit::Bytes, false),
    );
    assert_eq!(labels(&scene.y_ticks), ["0 o", "128,0 Mo", "256,0 Mo"]);
    assert_eq!(Unit::BytesPerSecond.format(1536.0), "1,5 Ko/s");
    assert_eq!(Unit::BitsPerSecond.format(1500.0), "1,5 kbit/s");
    // The clock is the chart's, and said as the language says a time.
    assert_eq!(labels(&scene.x_ticks), ["10:00:00", "10:00:01"]);
}

#[test]
fn the_y_axis_is_where_text_starts_and_time_runs_left_to_right() {
    let _english = in_language("en");
    let state = gauge(&[Some(10.0), Some(20.0), Some(30.0)]);
    let ltr = Scene::build(
        state.samples(),
        &ask((600.0, 160.0), &[], Unit::Number, false),
    );
    let rtl = Scene::build(
        state.samples(),
        &ask((600.0, 160.0), &[], Unit::Number, true),
    );
    // The plot is as wide either way, against the other side.
    assert_eq!(ltr.plot.width, rtl.plot.width);
    assert!(ltr.plot.x > rtl.plot.x);
    assert_eq!(ltr.plot.x + ltr.plot.width, 600.0 - rtl.plot.x);
    // Labels end at the axis on the left, and start at it on the right.
    for tick in &ltr.y_ticks {
        assert!(tick.start >= 0.0 && tick.start + tick.width < ltr.plot.x);
    }
    for tick in &rtl.y_ticks {
        assert!(tick.start > rtl.plot.x + rtl.plot.width && tick.start + tick.width <= 600.0);
    }
    // Time runs the same way in both: the first sample is on the left.
    let (first, last) = (1_777_629_600.0, 1_777_629_602.0);
    for scene in [&ltr, &rtl] {
        assert_eq!(scene.px(first), scene.plot.x);
        assert_eq!(scene.px(last), scene.plot.x + scene.plot.width);
        assert!(
            (scene.time_at(scene.plot.x + scene.plot.width / 2.0) - (first + 1.0)).abs() < 1e-6
        );
        // Greater values are higher up.
        assert!(scene.py(30.0) < scene.py(10.0));
        assert_eq!(scene.py(0.0), scene.plot.y + scene.plot.height);
    }
}

#[test]
fn a_chart_without_samples_or_with_one_has_a_scene() {
    let _english = in_language("en");
    let empty = State::new(1, 10);
    let scene = Scene::build(
        empty.samples(),
        &ask((600.0, 160.0), &[], Unit::Bytes, false),
    );
    assert!(scene.x_ticks.is_empty() && scene.y_ticks.is_empty());
    assert!(scene.plot.width > 500.0 && scene.plot.height > 100.0);
    // One sample: the second that ends with it, the sample at its end.
    let one = gauge(&[Some(5.0)]);
    let scene = Scene::build(
        one.samples(),
        &ask((600.0, 160.0), &[], Unit::Number, false),
    );
    assert_eq!(scene.x, (1_777_629_599.0, 1_777_629_600.0));
    assert_eq!(scene.y, (0.0, 5.5));
    assert!(scene.px(1_777_629_600.0).is_finite() && scene.py(5.0).is_finite());
    // Nothing but gaps: the range of nothing, and no coordinate off the scale.
    let gaps = gauge(&[None, Some(f64::NAN), None]);
    let scene = Scene::build(
        gaps.samples(),
        &ask((600.0, 160.0), &[], Unit::Number, false),
    );
    assert_eq!(scene.y, (0.0, 1.0));
    // No room at all draws nothing and divides by nothing.
    let scene = Scene::build(one.samples(), &ask((0.0, 0.0), &[], Unit::Number, false));
    assert!(scene.px(1_777_629_600.0).is_finite() && scene.py(5.0).is_finite());
    assert!(scene.time_at(10.0).is_finite());
}

#[test]
fn a_hidden_series_no_longer_counts_in_the_range() {
    let _english = in_language("en");
    let mut state = State::new(2, 10);
    state.push(1.0, &[Some(1000.0), Some(10.0)]);
    state.push(2.0, &[Some(2000.0), Some(20.0)]);
    let all = Scene::build(
        state.samples(),
        &ask((600.0, 160.0), &[false, false], Unit::Number, false),
    );
    let one = Scene::build(
        state.samples(),
        &ask((600.0, 160.0), &[true, false], Unit::Number, false),
    );
    assert_eq!(all.y, (0.0, 2200.0));
    assert_eq!(one.y, (0.0, 22.0));
}

#[test]
fn labels_that_do_not_fit_are_left_out_whole() {
    let _english = in_language("en");
    let state = gauge(&[Some(1e9), Some(2e9)]);
    // "1,000,000,000" is 91 px of a chart of 150: over its share.
    let scene = Scene::build(
        state.samples(),
        &ask((150.0, 160.0), &[], Unit::Number, false),
    );
    assert!(scene.y_ticks.len() > 1, "the ticks stay");
    assert!(scene.y_ticks.iter().all(|tick| tick.label.is_empty()));
    assert!(scene.plot.width > 130.0);
    // No label is ever shortened: each says its value whole or nothing.
    let roomy = Scene::build(
        state.samples(),
        &ask((600.0, 160.0), &[], Unit::Number, false),
    );
    assert!(labels(&roomy.y_ticks).contains(&"2,000,000,000"));
    // A time label moved in from the chart's edge sends away the one it
    // would stand on; none overlaps another.
    let scene = Scene::build(
        state.samples(),
        &ask((230.0, 160.0), &[], Unit::Percent, true),
    );
    let shown: Vec<_> = scene
        .x_ticks
        .iter()
        .filter(|tick| !tick.label.is_empty())
        .collect();
    assert!(!shown.is_empty());
    for pair in shown.windows(2) {
        assert!(pair[0].start + pair[0].width < pair[1].start, "{pair:?}");
    }
    for tick in shown {
        assert!(
            tick.start >= 0.0 && tick.start + tick.width <= 230.0,
            "{tick:?}"
        );
    }
}

#[test]
fn a_line_stops_at_a_gap_and_where_the_stream_stopped() {
    let mut state = State::new(1, 100).breaking_after(5.0);
    for (time, value) in [
        (0.0, Some(1.0)),
        (1.0, Some(2.0)),
        (2.0, None),
        (3.0, Some(3.0)),
        (4.0, Some(f64::NAN)),
        (5.0, Some(4.0)),
        (6.0, Some(5.0)),
        // Nothing for a minute.
        (66.0, Some(6.0)),
        (67.0, Some(7.0)),
    ] {
        state.push(time, &[value]);
    }
    // Two samples, one alone, two, and two after the silence.
    assert_eq!(runs(&state, 0, 0), [(0, 2), (3, 4), (5, 7), (7, 9)]);
    // From the fourth sample on.
    assert_eq!(runs(&state, 0, 3), [(3, 4), (5, 7), (7, 9)]);
    // A chart that joins across silences has one run where there were two.
    let mut joined = State::new(1, 100);
    for time in [0.0, 1.0, 61.0] {
        joined.push(time, &[Some(1.0)]);
    }
    assert_eq!(runs(&joined, 0, 0), [(0, 3)]);
    assert!(runs(&State::new(1, 4), 0, 0).is_empty());
}

#[test]
fn a_line_that_comes_into_the_plot_starts_at_its_edge() {
    let mut line = vec![
        Point::new(0.0, 10.0),
        Point::new(20.0, 30.0),
        Point::new(40.0, 30.0),
    ];
    enter(&mut line, 10.0);
    assert_eq!(
        line,
        [
            Point::new(10.0, 20.0),
            Point::new(20.0, 30.0),
            Point::new(40.0, 30.0)
        ]
    );
    // Already inside: as it is.
    let mut inside = vec![Point::new(10.0, 1.0), Point::new(20.0, 2.0)];
    enter(&mut inside, 10.0);
    assert_eq!(inside.len(), 2);
    // A sample alone before the edge shows nothing.
    let mut alone = vec![Point::new(5.0, 1.0)];
    enter(&mut alone, 10.0);
    assert!(alone.is_empty());
}

fn tip_of(state: &State, labels: &[(String, Look)], index: usize, room: (f32, f32)) -> Tip {
    Tip::build(
        state.samples(),
        &tip::Ask {
            key: 0,
            index,
            labels,
            hidden: &state.hidden,
            unit: Unit::Percent,
            zone: &TimeZone::UTC,
            dated: false,
            room: Size::new(room.0, room.1),
            measure: &measure,
        },
    )
}

fn named(count: usize) -> Vec<(String, Look)> {
    (0..count)
        .map(|n| (format!("worker-{}", n + 1), Look::of(n)))
        .collect()
}

#[test]
fn the_tooltip_says_when_and_what_each_series_that_shows_was() {
    let _english = in_language("en");
    let mut state = State::new(3, 10);
    state.push(1_777_629_600.0, &[Some(12.5), None, Some(150.0)]);
    state.push(1_777_629_601.0, &[Some(13.0), Some(1.0), Some(151.0)]);
    let labels = named(3);
    let tip = tip_of(&state, &labels, 0, (400.0, 150.0));
    assert_eq!(tip.time, "10:00:00\u{202f}AM");
    let said: Vec<_> = tip
        .rows
        .iter()
        .map(|row| (row.label.as_str(), row.value.as_str()))
        .collect();
    // A gap is said as one, not as zero.
    assert_eq!(
        said,
        [
            ("worker-1", "12.5%"),
            ("worker-2", tip::NONE),
            ("worker-3", "150%")
        ]
    );
    // A hidden series is not in it.
    state.update(crate::ui::widgets::chart::Message::Toggle(0));
    let tip = tip_of(&state, &labels, 1, (400.0, 150.0));
    assert_eq!(
        tip.rows.iter().map(|row| row.series).collect::<Vec<_>>(),
        [1, 2]
    );
    assert_eq!(tip.rows[0].value, "1.0%");
}

#[test]
fn a_tooltip_taller_than_the_chart_goes_into_columns_and_stays_inside() {
    let _english = in_language("en");
    let mut state = State::new(10, 10);
    state.push(1.0, &[Some(1.0); 10]);
    let labels = named(10);
    // Room for the time and five rows: two columns of five.
    let tip = tip_of(&state, &labels, 0, (700.0, 2.0 * tip::PAD + 6.0 * tip::ROW));
    assert_eq!((tip.rows.len(), tip.per_column), (10, 5));
    assert!(tip.size.height <= 2.0 * tip::PAD + 6.0 * tip::ROW);
    assert!(tip.size.width <= 700.0);
    // Tall enough: one column.
    let tip = tip_of(&state, &labels, 0, (700.0, 400.0));
    assert_eq!(tip.per_column, 10);
    // Too narrow for the names: they are cut with an ellipsis (the
    // legend has them whole), the values are not.
    let long: Vec<_> = (0..10)
        .map(|n| {
            (
                format!("a-container-with-a-very-long-name-{n}"),
                Look::of(n),
            )
        })
        .collect();
    let tip = tip_of(&state, &long, 0, (200.0, 400.0));
    assert!(tip.size.width <= 200.0, "{}", tip.size.width);
    assert!(tip
        .rows
        .iter()
        .all(|row| row.label.ends_with('…') && row.value == "1.0%"));
    // Cut by characters, not bytes: a name in another script.
    let arabic = vec![("حاوية-باسم-طويل-جدا-جدا-جدا-جدا".to_owned(), Look::of(0))];
    let mut one = State::new(1, 4);
    one.push(1.0, &[Some(1.0)]);
    let tip = tip_of(&one, &arabic, 0, (120.0, 400.0));
    assert!(tip.rows[0].label.ends_with('…') && tip.rows[0].label.chars().count() > 1);
}

#[test]
fn the_tooltip_stands_beside_the_crosshair_inside_the_chart() {
    // After the crosshair the way the language reads, where it fits.
    assert_eq!(tip::place(100.0, 150.0, 600.0, false), 112.0);
    assert_eq!(tip::place(500.0, 150.0, 600.0, true), 338.0);
    // Else on the other side.
    assert_eq!(tip::place(500.0, 150.0, 600.0, false), 338.0);
    assert_eq!(tip::place(100.0, 150.0, 600.0, true), 112.0);
    // A chart narrower than both sides: inside it all the same.
    let x = tip::place(100.0, 150.0, 200.0, false);
    assert!((0.0..=50.0).contains(&x));
    assert_eq!(tip::place(10.0, 300.0, 200.0, false), 0.0);
}
