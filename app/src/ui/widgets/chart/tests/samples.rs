//! What the rings keep: the newest samples, gaps where a value is
//! missing or is no number, rates of counters, and time that only runs
//! forward.

use crate::ui::widgets::chart::{Kind, Message, Samples, State};

fn values(samples: &Samples, series: usize) -> Vec<Option<f64>> {
    (0..samples.len())
        .map(|index| samples.value(series, index))
        .collect()
}

fn times(samples: &Samples) -> Vec<f64> {
    (0..samples.len())
        .filter_map(|index| samples.time(index))
        .collect()
}

#[test]
fn a_full_ring_drops_its_oldest_sample_for_the_newest() {
    let mut samples = Samples::new(1, 3);
    for n in 0..5 {
        assert!(samples.push(f64::from(n), &[Some(f64::from(n) * 10.0)]));
    }
    assert_eq!(times(&samples), [2.0, 3.0, 4.0]);
    assert_eq!(values(&samples, 0), [Some(20.0), Some(30.0), Some(40.0)]);
    assert_eq!((samples.len(), samples.cap()), (3, 3));
}

#[test]
fn a_ring_asked_for_no_room_keeps_one_sample() {
    let mut samples = Samples::new(1, 0);
    samples.push(1.0, &[Some(1.0)]);
    samples.push(2.0, &[Some(2.0)]);
    assert_eq!(values(&samples, 0), [Some(2.0)]);
}

#[test]
fn a_value_that_is_missing_or_no_number_is_a_gap_not_a_zero() {
    let mut samples = Samples::new(2, 10);
    samples.push(1.0, &[Some(5.0), None]);
    samples.push(2.0, &[Some(f64::NAN), Some(f64::INFINITY)]);
    samples.push(3.0, &[Some(f64::NEG_INFINITY), Some(7.0)]);
    // Fewer values than series: the others have none.
    samples.push(4.0, &[Some(6.0)]);
    assert_eq!(values(&samples, 0), [Some(5.0), None, None, Some(6.0)]);
    assert_eq!(values(&samples, 1), [None, None, Some(7.0), None]);
    // A series the chart doesn't have has no value anywhere.
    assert_eq!(samples.value(2, 0), None);
}

#[test]
fn a_sample_whose_time_is_no_number_is_left_out() {
    let mut samples = Samples::new(1, 10);
    assert!(!samples.push(f64::NAN, &[Some(1.0)]));
    assert!(!samples.push(f64::INFINITY, &[Some(1.0)]));
    assert!(samples.is_empty());
}

/// `rates` of `src/features/containers/model/stats.test.ts`: "turns
/// counters into per-second rates and skips resets".
#[test]
fn a_counter_is_its_rate_per_second_and_a_reset_is_a_gap() {
    let mut samples = Samples::new(1, 10);
    samples.set_kind(0, Kind::Counter);
    samples.push(10.0, &[Some(1000.0)]);
    samples.push(12.0, &[Some(3000.0)]);
    samples.push(13.0, &[Some(500.0)]);
    assert_eq!(values(&samples, 0), [None, Some(1000.0), None]);
    // After the reset the rate is taken from the new count.
    samples.push(15.0, &[Some(700.0)]);
    assert_eq!(samples.value(0, 3), Some(100.0));
}

#[test]
fn a_counter_that_misses_a_reading_takes_no_rate_across_it() {
    let mut samples = Samples::new(2, 10);
    samples.set_kind(0, Kind::Counter);
    samples.push(1.0, &[Some(100.0), Some(1.0)]);
    samples.push(2.0, &[None, Some(2.0)]);
    samples.push(3.0, &[Some(400.0), Some(3.0)]);
    samples.push(4.0, &[Some(450.0), Some(4.0)]);
    // 300 bytes in the two seconds since the last reading.
    assert_eq!(values(&samples, 0), [None, None, Some(150.0), Some(50.0)]);
    // The gauge beside it is untouched.
    assert_eq!(
        values(&samples, 1),
        [Some(1.0), Some(2.0), Some(3.0), Some(4.0)]
    );
}

/// One sample that comes late, or is stamped zero, costs nothing of
/// what is kept: React's `rates` left that one point out.
#[test]
fn a_sample_that_does_not_move_time_forward_is_left_out() {
    let mut samples = Samples::new(1, 10);
    samples.set_kind(0, Kind::Counter);
    for (time, count) in [(10.0, 100.0), (11.0, 200.0), (12.0, 300.0)] {
        assert!(samples.push(time, &[Some(count)]));
    }
    for late in [0.0, 11.5, 12.0, -5.0] {
        assert!(!samples.push(late, &[Some(999.0)]), "{late} was taken");
    }
    assert_eq!(times(&samples), [10.0, 11.0, 12.0]);
    assert_eq!(values(&samples, 0), [None, Some(100.0), Some(100.0)]);
    // The next sample in time takes its rate from the last one kept,
    // not from what was left out.
    samples.push(13.0, &[Some(350.0)]);
    assert_eq!(samples.value(0, 3), Some(50.0));
}

#[test]
fn the_nearest_sample_is_found_among_those_in_view() {
    let mut samples = Samples::new(1, 10);
    for time in [10.0, 20.0, 30.0, 40.0] {
        samples.push(time, &[Some(time)]);
    }
    assert_eq!(samples.index_from(20.0), 1);
    assert_eq!(samples.index_from(20.5), 2);
    assert_eq!(samples.nearest(24.0, 0), Some(1));
    assert_eq!(samples.nearest(26.0, 0), Some(2));
    assert_eq!(samples.nearest(-5.0, 0), Some(0));
    assert_eq!(samples.nearest(99.0, 0), Some(3));
    // Not a sample before the first in view.
    assert_eq!(samples.nearest(11.0, 2), Some(2));
    assert_eq!(samples.nearest(11.0, 4), None);
    assert_eq!(Samples::new(1, 4).nearest(1.0, 0), None);
}

#[test]
fn a_window_lets_go_of_what_is_older_but_for_the_sample_that_reaches_in() {
    let mut state = State::new(1, 100).window(10.0);
    for n in 0..30 {
        state.push(f64::from(n), &[Some(f64::from(n))]);
    }
    // 19 is the window's edge; 18 stays, for the line that comes in.
    assert_eq!(state.samples().first_time(), Some(18.0));
    assert_eq!(state.samples().last_time(), Some(29.0));
}

/// What a page does before it follows another container: no sample and
/// no counter's reading is left, so the first rate of the new stream is
/// not taken from the old one's count.
#[test]
fn a_reset_leaves_nothing_of_the_stream_before() {
    let mut state = State::new(1, 100).counter(0);
    state.push(1.0, &[Some(1000.0)]);
    state.push(2.0, &[Some(5000.0)]);
    assert_eq!(state.latest(0), Some(4000.0));
    state.reset();
    assert!(state.samples().is_empty());
    assert_eq!(state.latest(0), None);
    // Another container, whose counter is higher and whose clock is the same.
    state.push(3.0, &[Some(9000.0)]);
    assert_eq!(
        state.latest(0),
        None,
        "no rate from the other stream's count"
    );
    state.push(4.0, &[Some(9100.0)]);
    assert_eq!(state.latest(0), Some(100.0));
}

#[test]
fn the_legend_hides_a_series_and_shows_it_again() {
    let mut state = State::new(2, 10);
    assert!(!state.is_hidden(1));
    state.update(Message::Toggle(1));
    assert!(state.is_hidden(1) && !state.is_hidden(0));
    // What is hidden stays hidden across a reset: it is the reader's choice.
    state.reset();
    assert!(state.is_hidden(1));
    state.update(Message::Toggle(1));
    assert!(!state.is_hidden(1));
    // A series the chart doesn't have changes nothing.
    state.update(Message::Toggle(7));
    assert!(!state.is_hidden(7));
}
