//! Where everything of a chart goes in the room it has: the plot, the
//! two ranges, the ticks and what they say. It is worked out when the
//! samples, the series that show, the room or the language change, and
//! kept in between (`State::scene`): a frame that only moves the
//! pointer reads it and scans no sample.

use iced::{Rectangle, Size};

use super::{
    samples::Samples,
    scale::{self, DAY},
    unit::Unit,
};
use crate::{i18n, ui::theme::metrics::text as size};

/// The size of what the axes say.
pub const FONT: f32 = size::XS;
/// A tick's mark, and the room between it and its label.
pub const TICK: f32 = 4.0;
const GAP: f32 = 4.0;
/// Over the plot, so the top label is whole; and at its far side.
const TOP: f32 = 8.0;
const FAR: f32 = 8.0;
/// Under the plot: the marks, the gap and a line of text.
const BOTTOM: f32 = TICK + GAP + 16.0;
/// The least room a tick of each axis takes.
const ROW: f32 = 28.0;
const BETWEEN: f32 = 16.0;
/// The most of the chart's width its y axis may take; past it the
/// labels are left out whole rather than cut (the tooltip says the
/// values).
const AXIS_SHARE: f32 = 0.45;

/// One tick: where it is along its axis, in the canvas, and its label
/// with how wide it is drawn and where its left end is.
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    pub at: f32,
    pub label: String,
    pub width: f32,
    pub start: f32,
}

/// What the scene is made from, apart from the samples.
pub struct Ask<'a> {
    pub size: Size,
    pub hidden: &'a [bool],
    /// How many seconds the time axis shows, ending at the newest sample.
    pub window: Option<f64>,
    /// How far apart two samples may be and still be joined.
    pub break_after: Option<f64>,
    pub unit: Unit,
    pub rtl: bool,
    /// The clock the times are said on.
    pub zone: &'a jiff::tz::TimeZone,
    /// How wide a label is drawn.
    pub measure: &'a dyn Fn(&str) -> f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    /// Where the lines are drawn.
    pub plot: Rectangle,
    /// The times at the plot's left and right edges: time runs left to
    /// right in every language, as on the React chart.
    pub x: (f64, f64),
    pub y: (f64, f64),
    /// The first sample in view: none before it is drawn or pointed at.
    pub from: usize,
    /// For each series, what it is at the plot's left edge when its
    /// line comes in from a sample before the window: where that line
    /// starts.
    pub entering: Vec<Option<f64>>,
    pub x_ticks: Vec<Tick>,
    pub y_ticks: Vec<Tick>,
    /// Whether the y axis is on the right: where text starts in Arabic.
    pub rtl: bool,
}

impl Scene {
    pub fn build(samples: &Samples, ask: &Ask<'_>) -> Self {
        scans::count();
        let (x, from) = time_range(samples, ask.window);
        let entering = entering(samples, ask, x.0, from);
        let y = scale::range(extent(samples, ask.hidden, from, &entering));
        // A unit written whole has room for one of it: an axis of a
        // third of a byte would have nothing to say but zero.
        let y = (y.0, y.1.max(y.0 + ask.unit.least_step()));
        let height = (ask.size.height - TOP - BOTTOM).max(0.0);
        // A tick every 28 px at most: a handful.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let rows = ((height / ROW).floor() as usize).clamp(2, 8);
        // Each tick's value, what it says and how wide that is drawn.
        let mut labels: Vec<(f64, String, f32)> = if samples.is_empty() {
            vec![]
        } else {
            // The step that gives as many ticks as there are rows, or a
            // finer one if that leaves the axis with zero alone.
            let step = (rows..rows + 4)
                .map(|count| {
                    scale::step_from(
                        y.1 - y.0,
                        count,
                        ask.unit.is_binary(),
                        ask.unit.least_step(),
                    )
                })
                .find(|step| scale::ticks_by(y, *step).len() >= 2)
                .unwrap_or(y.1 - y.0);
            let mut said: Vec<(f64, String, f32)> = vec![];
            for value in scale::ticks_by(y, step) {
                let label = ask.unit.tick(value, step);
                // No two ticks say the same: whatever a unit rounds
                // to, the second of two equal labels is left out.
                if said.iter().all(|(_, other, _)| *other != label) {
                    let width = (ask.measure)(&label);
                    said.push((value, label, width));
                }
            }
            said
        };
        let widest = labels.iter().map(|tick| tick.2).fold(0.0, f32::max);
        let mut axis = widest + GAP + TICK;
        if axis > ask.size.width * AXIS_SHARE {
            for tick in &mut labels {
                tick.1.clear();
                tick.2 = 0.0;
            }
            axis = TICK;
        }
        let width = (ask.size.width - axis - FAR).max(0.0);
        let left = if ask.rtl { FAR } else { axis };
        let plot = Rectangle::new((left, TOP).into(), Size::new(width, height));
        let mut scene = Self {
            plot,
            x,
            y,
            from,
            entering,
            x_ticks: vec![],
            y_ticks: vec![],
            rtl: ask.rtl,
        };
        scene.y_ticks = labels
            .into_iter()
            .map(|(value, label, width)| Tick {
                at: scene.py(value).round(),
                // Against the axis, on the side text starts.
                start: if ask.rtl {
                    plot.x + plot.width + TICK + GAP
                } else {
                    plot.x - TICK - GAP - width
                },
                label,
                width,
            })
            .collect();
        if !samples.is_empty() {
            scene.x_ticks = time_ticks(&scene, ask);
        }
        scene
    }

    /// Where the time `at` is across the canvas.
    pub fn px(&self, at: f64) -> f32 {
        self.plot.x + self.plot.width * unit_of(at, self.x)
    }

    /// Where the value `at` is down the canvas.
    pub fn py(&self, at: f64) -> f32 {
        self.plot.y + self.plot.height * (1.0 - unit_of(at, self.y))
    }

    /// The time under the canvas' `x`.
    pub fn time_at(&self, x: f32) -> f64 {
        if self.plot.width <= 0.0 {
            return self.x.1;
        }
        let share = f64::from(((x - self.plot.x) / self.plot.width).clamp(0.0, 1.0));
        self.x.0 + (self.x.1 - self.x.0) * share
    }
}

/// `at` between `low` and `high`, as a share; a value far outside is
/// held near the edges, so no coordinate is ever infinite.
#[allow(clippy::cast_possible_truncation)]
fn unit_of(at: f64, (low, high): (f64, f64)) -> f32 {
    if high <= low {
        return 0.0;
    }
    (((at - low) / (high - low)).clamp(-1.0, 2.0)) as f32
}

/// The times shown, and the first sample among them.
fn time_range(samples: &Samples, window: Option<f64>) -> ((f64, f64), usize) {
    let (Some(first), Some(last)) = (samples.first_time(), samples.last_time()) else {
        return ((0.0, 1.0), 0);
    };
    match window {
        Some(window) if window > 0.0 => {
            let start = last - window;
            ((start, last), samples.index_from(start))
        }
        // One sample is a point at the right edge of a second.
        _ if first >= last => ((last - 1.0, last), 0),
        _ => ((first, last), 0),
    }
}

/// The least and the greatest value of the series that show, from the
/// sample `from` on: the one pass over the samples a scene makes.
fn extent(
    samples: &Samples,
    hidden: &[bool],
    from: usize,
    entering: &[Option<f64>],
) -> Option<(f64, f64)> {
    let mut extent: Option<(f64, f64)> = None;
    for series in 0..samples.series() {
        if hidden.get(series).copied().unwrap_or(false) {
            continue;
        }
        // What shows of the series: where its line comes in, and its
        // samples in view. The sample before the window does not.
        let edge = entering.get(series).copied().flatten();
        let inside = (from..samples.len()).filter_map(|index| samples.value(series, index));
        for value in edge.into_iter().chain(inside) {
            {
                extent = Some(match extent {
                    Some((least, most)) => (least.min(value), most.max(value)),
                    None => (value, value),
                });
            }
        }
    }
    extent
}

/// What each series is at `start`, the plot's left edge, on the line
/// from the last sample before it to the first in view: worked out
/// from the samples themselves, however old the one before is.
fn entering(samples: &Samples, ask: &Ask<'_>, start: f64, from: usize) -> Vec<Option<f64>> {
    let ends = from
        .checked_sub(1)
        .and_then(|before| Some((before, samples.time(before)?, samples.time(from)?)));
    (0..samples.series())
        .map(|series| {
            let (before, then, first) = ends?;
            // The stream stopped in between: nothing comes in.
            if ask.break_after.is_some_and(|most| first - then > most) || first <= then {
                return None;
            }
            let (was, is) = (samples.value(series, before)?, samples.value(series, from)?);
            Some(was + (is - was) * ((start - then) / (first - then)))
        })
        .collect()
}

fn time_ticks(scene: &Scene, ask: &Ask<'_>) -> Vec<Tick> {
    let formats = &i18n::current().formats;
    // Seconds since 1970 fit an i64 for any time a chart is given.
    #[allow(clippy::cast_possible_truncation)]
    let say = |at: f64, step: f64| {
        let at = at.round() as i64;
        if step >= DAY {
            formats.date(at, ask.zone)
        } else {
            formats.time(at, ask.zone)
        }
    };
    #[allow(clippy::cast_possible_truncation)]
    let offset = jiff::Timestamp::from_second(scene.x.1 as i64)
        .map(|at| f64::from(ask.zone.to_offset(at).seconds()))
        .unwrap_or(0.0);
    let sample = (ask.measure)(&say(scene.x.1, 1.0));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let count = (scene.plot.width / (sample + BETWEEN)).floor() as usize;
    if count == 0 || sample <= 0.0 {
        return vec![];
    }
    let (step, times) = scale::time_ticks(scene.x, count, offset);
    // A tick that is a date stands on that day's midnight, which is
    // not a whole number of days from the last one when the clocks
    // changed in between.
    let times = if step >= DAY {
        midnights(scene.x, step, ask.zone)
    } else {
        times
    };
    // Where the label before ends: one at the chart's edge is moved
    // inside it, and the next is left out if that puts it in its way.
    let mut taken = f32::NEG_INFINITY;
    times
        .into_iter()
        .map(|time| {
            let at = scene.px(time).round();
            let mut label = say(time, step);
            let mut width = (ask.measure)(&label);
            let start = (at - width / 2.0).clamp(0.0, (ask.size.width - width).max(0.0));
            if start < taken + GAP {
                label.clear();
                width = 0.0;
            } else {
                taken = start + width;
            }
            Tick {
                at,
                label,
                width,
                start,
            }
        })
        .collect()
}

/// The midnights, on the clock of `zone`, of every `step / DAY`-th day
/// between `from` and `to`: each with the offset its own day has.
fn midnights((from, to): (f64, f64), step: f64, zone: &jiff::tz::TimeZone) -> Vec<f64> {
    use jiff::{civil::date, Span, Timestamp};
    // Seconds since 1970 fit an i64, and a step is days.
    #[allow(clippy::cast_possible_truncation)]
    let (every, start) = ((step / DAY).round().max(1.0) as i64, from.floor() as i64);
    let Ok(start) = Timestamp::from_second(start) else {
        return vec![];
    };
    let epoch = date(1970, 1, 1);
    let first = i64::from((start.to_zoned(zone.clone()).date() - epoch).get_days());
    // Whole multiples of the step, counted in days of the calendar.
    let first = first.div_euclid(every) * every;
    (0..66)
        .filter_map(|n| {
            let day = epoch
                .checked_add(Span::new().days(first + n * every))
                .ok()?;
            // A second count is far below 2^53.
            #[allow(clippy::cast_precision_loss)]
            let at = day.to_zoned(zone.clone()).ok()?.timestamp().as_second() as f64;
            (at >= from && at <= to).then_some(at)
        })
        .collect()
}

/// How many scenes this thread has built: a test holds a frame that
/// changes nothing to none.
pub(super) mod scans {
    #[cfg(test)]
    thread_local! {
        pub static BUILT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    pub fn count() {
        #[cfg(test)]
        BUILT.with(|built| built.set(built.get() + 1));
    }
}
