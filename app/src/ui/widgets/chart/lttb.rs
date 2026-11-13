//! Largest-Triangle-Three-Buckets (Steinarsson, 2013): which points of a
//! long line to keep so that it looks the same drawn with far fewer.
//!
//! The points between the first and the last are cut into buckets, and
//! of each bucket the one kept is the one that makes the largest
//! triangle with the point kept before it and the average of the bucket
//! after: a peak stays a peak, which an average or every n-th point
//! would flatten or miss.
//!
//! The buckets are the caller's: the chart cuts them by time, at
//! multiples of a width that does not move, so a sample is in the same
//! bucket whatever came since. Buckets counted from the first sample in
//! view would all shift by one as each sample comes into a full ring,
//! and the line would shimmer.

/// The indexes, in order, of the points to draw of the `count` points
/// `at` gives (`at(i)` is the i-th point, `x` increasing): the first,
/// the last, and one for each run of points between them that
/// `bucket` puts together.
pub fn pick(
    count: usize,
    bucket: impl Fn(usize) -> i64,
    at: impl Fn(usize) -> (f64, f64),
) -> Vec<usize> {
    if count <= 2 {
        return (0..count).collect();
    }
    // The buckets of the points between the ends, as (start, end).
    let mut groups: Vec<(usize, usize)> = vec![];
    let mut start = 1;
    for index in 2..count {
        if index == count - 1 || bucket(index) != bucket(start) {
            groups.push((start, index));
            start = index;
        }
    }
    if groups.len() + 2 >= count {
        return (0..count).collect();
    }
    let mut kept = Vec::with_capacity(groups.len() + 2);
    kept.push(0);
    let mut last = 0;
    for (place, (from, to)) in groups.iter().copied().enumerate() {
        // The bucket after, averaged; the last point stands for the
        // one after the last bucket.
        let (next_from, next_to) = groups.get(place + 1).copied().unwrap_or((count - 1, count));
        let (mut sum_x, mut sum_y) = (0.0, 0.0);
        for index in next_from..next_to {
            let (x, y) = at(index);
            sum_x += x;
            sum_y += y;
        }
        // A bucket holds a handful of points.
        #[allow(clippy::cast_precision_loss)]
        let span = (next_to - next_from) as f64;
        let (mean_x, mean_y) = (sum_x / span, sum_y / span);
        let (ax, ay) = at(last);
        let mut best = (from, -1.0);
        for index in from..to {
            let (x, y) = at(index);
            let area = ((ax - mean_x) * (y - ay) - (ax - x) * (mean_y - ay)).abs();
            if area > best.1 {
                best = (index, area);
            }
        }
        kept.push(best.0);
        last = best.0;
    }
    kept.push(count - 1);
    kept
}
