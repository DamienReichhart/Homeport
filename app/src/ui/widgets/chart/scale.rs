//! The two axes: how far the values go, and where the ticks stand.

/// The steps a time axis may take, in seconds: the first that leaves
/// room for the labels is used.
const TIME_STEPS: [f64; 22] = [
    1.0,
    2.0,
    5.0,
    10.0,
    15.0,
    30.0,
    60.0,
    120.0,
    300.0,
    600.0,
    900.0,
    1800.0,
    3600.0,
    7200.0,
    10_800.0,
    21_600.0,
    43_200.0,
    86_400.0,
    172_800.0,
    604_800.0,
    1_209_600.0,
    2_592_000.0,
];

/// A day, from which a tick is a date and no longer a time.
pub const DAY: f64 = 86_400.0;

/// The range of the y axis for values between `least` and `most`
/// (`None` when there is none to draw).
///
/// uPlot's, as the React chart asked for it: from zero to a tenth over
/// the highest value, and to 1 when nothing is above zero. So values
/// that are all equal are a flat line a little under the top, never a
/// range of no height. Values under zero, which no React chart had,
/// open the range downward the same way.
pub fn range(extent: Option<(f64, f64)>) -> (f64, f64) {
    let (least, most) = extent.unwrap_or((0.0, 0.0));
    let low = if least < 0.0 {
        (least * 1.1).max(f64::MIN)
    } else {
        0.0
    };
    let high = if most > 0.0 {
        (most * 1.1).min(f64::MAX)
    } else if low < 0.0 {
        0.0
    } else {
        1.0
    };
    (low, high)
}

/// The least of `steps` times a power of `base` that is `at_least`.
fn round_up(at_least: f64, base: f64, steps: &[f64]) -> f64 {
    let unit = base.powf((at_least.ln() / base.ln()).floor());
    let over = at_least / unit;
    let step = steps.iter().copied().find(|step| *step >= over);
    // A rounding at a power's edge can leave `over` a hair past the last.
    step.unwrap_or(base) * unit
}

/// How far apart the ticks of a y axis of `span` are, for `count` of
/// them at most: 1, 2 or 5 times a power of ten, or for sizes
/// (`binary`) a round number of the unit they are said in (256 MB,
/// never 250).
pub fn step(span: f64, count: usize, binary: bool) -> f64 {
    step_from(span, count, binary, 0.0)
}

/// The same, never under `least`: the step of a unit written whole.
pub fn step_from(span: f64, count: usize, binary: bool, least: f64) -> f64 {
    let step = plain_step(span, count, binary);
    if step < least {
        least
    } else {
        step
    }
}

fn plain_step(span: f64, count: usize, binary: bool) -> f64 {
    // Tick counts are single digits.
    #[allow(clippy::cast_precision_loss)]
    let at_least = span / count.max(1) as f64;
    if !at_least.is_finite() || at_least <= 0.0 {
        return 1.0;
    }
    if binary && at_least >= 1.0 {
        round_up(
            at_least,
            1024.0,
            &[1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0],
        )
    } else {
        round_up(at_least, 10.0, &[1.0, 2.0, 5.0])
    }
}

/// The values of the ticks between `low` and `high`, `count` at most.
pub fn ticks(range: (f64, f64), count: usize, binary: bool) -> Vec<f64> {
    ticks_by(range, step(range.1 - range.0, count, binary))
}

/// The multiples of `step` between `low` and `high`.
pub fn ticks_by((low, high): (f64, f64), step: f64) -> Vec<f64> {
    let first = (low / step).ceil();
    let last = (high / step).floor();
    if !(first.is_finite() && last.is_finite()) || last - first > 64.0 {
        return vec![];
    }
    // Between `first` and `last` there are a handful of whole numbers.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    (first as i64..=last as i64)
        .map(|n| n as f64 * step)
        .collect()
}

/// How far apart the ticks of a time axis of `span` seconds are, for
/// `count` of them at most.
pub fn time_step(span: f64, count: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let at_least = span / count.max(1) as f64;
    TIME_STEPS
        .iter()
        .copied()
        .find(|step| *step >= at_least)
        .unwrap_or_else(|| (at_least / DAY).ceil() * DAY)
}

/// The times of the ticks between `from` and `to`: whole multiples of
/// the step on a clock `offset` seconds ahead of UTC, so a tick is at
/// the top of the minute where the chart is read.
pub fn time_ticks((from, to): (f64, f64), count: usize, offset: f64) -> (f64, Vec<f64>) {
    let step = time_step(to - from, count);
    let first = ((from + offset) / step).ceil();
    let last = ((to + offset) / step).floor();
    if !(first.is_finite() && last.is_finite()) || last - first > 64.0 {
        return (step, vec![]);
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    let ticks = (first as i64..=last as i64)
        .map(|n| n as f64 * step - offset)
        .collect();
    (step, ticks)
}
