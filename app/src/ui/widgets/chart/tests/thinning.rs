//! Which points a long line keeps: the ends, a point per bucket, and
//! the peaks.

use crate::ui::widgets::chart::lttb::pick;

/// A slow wave with one sample far above it.
fn line(count: usize, spike: usize) -> Vec<(f64, f64)> {
    (0..count)
        .map(|n| {
            // Counts are small.
            #[allow(clippy::cast_precision_loss)]
            let x = n as f64;
            (x, if n == spike { 100.0 } else { (x / 50.0).sin() })
        })
        .collect()
}

#[test]
fn a_line_no_longer_than_asked_keeps_every_point() {
    let points = line(10, 99);
    assert_eq!(pick(10, 10, |n| points[n]), (0..10).collect::<Vec<_>>());
    assert_eq!(pick(10, 50, |n| points[n]), (0..10).collect::<Vec<_>>());
    assert_eq!(pick(0, 5, |n| points[n]), Vec::<usize>::new());
    // Fewer than three can't be thinned: a bucket needs two ends.
    assert_eq!(pick(10, 2, |n| points[n]).len(), 10);
}

#[test]
fn a_long_line_keeps_as_many_points_as_asked_in_order_with_both_ends() {
    let points = line(3600, 9999);
    for keep in [3, 4, 100, 1500, 3599] {
        let kept = pick(points.len(), keep, |n| points[n]);
        assert_eq!(kept.len(), keep, "{keep} asked for");
        assert_eq!((kept[0], kept[keep - 1]), (0, 3599));
        assert!(
            kept.windows(2).all(|pair| pair[0] < pair[1]),
            "{keep}: in order, none twice"
        );
    }
}

/// What an average or every n-th point would lose.
#[test]
fn a_peak_of_one_sample_is_kept() {
    for spike in [1, 777, 1800, 3598] {
        let points = line(3600, spike);
        let kept = pick(points.len(), 200, |n| points[n]);
        assert!(kept.contains(&spike), "the spike at {spike} is gone");
    }
}

#[test]
fn equal_values_and_equal_times_thin_without_a_fault() {
    let flat = vec![(0.0, 5.0); 1000];
    let kept = pick(flat.len(), 10, |n| flat[n]);
    assert_eq!(kept.len(), 10);
    assert!(kept.windows(2).all(|pair| pair[0] < pair[1]));
}
