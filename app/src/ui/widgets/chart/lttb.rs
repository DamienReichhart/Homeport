//! Largest-Triangle-Three-Buckets (Steinarsson, 2013): which points of a
//! long line to keep so that it looks the same drawn with far fewer.
//!
//! The points between the first and the last are cut into as many
//! buckets as points are wanted, and of each bucket the one kept is the
//! one that makes the largest triangle with the point kept before it
//! and the average of the bucket after: a peak stays a peak, which an
//! average or every n-th point would flatten or miss.

/// The indexes, in order, of the `keep` points to draw of the `count`
/// points `at` gives (`at(i)` is the i-th point, `x` increasing). Every
/// index when `count` is no more than `keep`, or `keep` is under three.
pub fn pick(count: usize, keep: usize, at: impl Fn(usize) -> (f64, f64)) -> Vec<usize> {
    if keep >= count || keep < 3 {
        return (0..count).collect();
    }
    let mut kept = Vec::with_capacity(keep);
    kept.push(0);
    // Counts of points are far below 2^53.
    #[allow(clippy::cast_precision_loss)]
    let every = (count - 2) as f64 / (keep - 2) as f64;
    // Where the bucket `n` starts: the first point is its own.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let start = |bucket: usize| ((bucket as f64 * every).floor() as usize + 1).min(count - 1);
    let mut last = 0;
    for bucket in 0..keep - 2 {
        let (from, to) = (start(bucket), start(bucket + 1).max(start(bucket) + 1));
        // The bucket after, averaged; the last point stands for the one
        // after the last bucket.
        let (next_from, next_to) = if bucket + 1 < keep - 2 {
            (to, start(bucket + 2).max(to + 1).min(count))
        } else {
            (count - 1, count)
        };
        let (mut sum_x, mut sum_y) = (0.0, 0.0);
        for index in next_from..next_to {
            let (x, y) = at(index);
            sum_x += x;
            sum_y += y;
        }
        #[allow(clippy::cast_precision_loss)]
        let span = (next_to - next_from) as f64;
        let (mean_x, mean_y) = (sum_x / span, sum_y / span);
        let (ax, ay) = at(last);
        let mut best = (from, -1.0);
        for index in from..to.min(count - 1) {
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
