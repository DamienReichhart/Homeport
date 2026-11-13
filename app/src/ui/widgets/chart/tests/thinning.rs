//! Which points a long line keeps: the ends, a point per bucket, the
//! peaks, and the same points when another sample comes.

use crate::ui::widgets::chart::lttb::pick;

/// A slow wave with one sample far above it, a sample a second from
/// `start`.
fn line(start: usize, count: usize, spike: usize) -> Vec<(f64, f64)> {
    (start..start + count)
        .map(|n| {
            // Counts are small.
            #[allow(clippy::cast_precision_loss)]
            let x = n as f64;
            (x, if n == spike { 100.0 } else { (x / 50.0).sin() })
        })
        .collect()
}

/// The points kept of `points` with buckets `width` seconds wide.
fn kept(points: &[(f64, f64)], width: f64) -> Vec<usize> {
    // Times are small whole numbers.
    #[allow(clippy::cast_possible_truncation)]
    pick(
        points.len(),
        |n| (points[n].0 / width).floor() as i64,
        |n| points[n],
    )
}

#[test]
fn a_line_with_a_point_a_bucket_keeps_every_point() {
    let points = line(0, 10, 99);
    assert_eq!(kept(&points, 1.0), (0..10).collect::<Vec<_>>());
    assert_eq!(kept(&points, 0.25), (0..10).collect::<Vec<_>>());
    assert_eq!(kept(&points[..0], 5.0), Vec::<usize>::new());
    assert_eq!(kept(&points[..2], 5.0), [0, 1]);
}

#[test]
fn a_long_line_keeps_a_point_a_bucket_in_order_with_both_ends() {
    let points = line(0, 3600, 9999);
    for width in [2.0, 4.0, 10.0, 36.0, 1000.0] {
        let kept = kept(&points, width);
        // 3,598 seconds between the ends, in buckets of `width`.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let buckets = (3598.0 / width).ceil() as usize + 1;
        assert!(
            kept.len() <= buckets + 2 && kept.len() + 3 >= buckets,
            "{} kept for {buckets} buckets",
            kept.len()
        );
        assert_eq!((kept[0], kept[kept.len() - 1]), (0, 3599));
        assert!(
            kept.windows(2).all(|pair| pair[0] < pair[1]),
            "in order, none twice"
        );
    }
}

/// What an average or every n-th point would lose.
#[test]
fn a_peak_of_one_sample_is_kept() {
    for spike in [1, 777, 1800, 3598] {
        let points = line(0, 3600, spike);
        assert!(
            kept(&points, 18.0).contains(&spike),
            "the spike at {spike} is gone"
        );
    }
}

#[test]
fn equal_values_and_equal_times_thin_without_a_fault() {
    let flat = vec![(0.0, 5.0); 1000];
    assert_eq!(kept(&flat, 10.0).len(), 3);
    let level: Vec<_> = (0..1000).map(|n| (f64::from(n), 5.0)).collect();
    let kept = kept(&level, 10.0);
    assert!(kept.windows(2).all(|pair| pair[0] < pair[1]));
}

/// A full ring drops its oldest sample for each new one. The points
/// kept of what stays are the same: buckets stand on time, not on how
/// many samples are before them, so the line does not shimmer.
#[test]
fn another_sample_leaves_the_points_kept_where_they_were() {
    let times = |start: usize| -> Vec<f64> {
        let points = line(start, 3600, 99_999);
        kept(&points, 4.0)
            .into_iter()
            .map(|n| points[n].0)
            .collect()
    };
    let before = times(1000);
    for shift in 1..=5 {
        let after = times(1000 + shift);
        let moved = before
            .iter()
            .filter(|time| **time > 1020.0 && **time < 4580.0 && !after.contains(time))
            .count();
        assert_eq!(moved, 0, "{moved} points moved after {shift} more samples");
    }
}
