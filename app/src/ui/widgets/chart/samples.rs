//! The samples of a chart: one time and one value per series for each,
//! kept in rings, so the newest is appended and the oldest dropped
//! without moving the others.
//!
//! What may come from a stream is settled here, once, so nothing that
//! draws has to ask again:
//!
//! - a value that is missing, `NaN` or infinite is a **gap**: nothing is
//!   drawn there, and the line stops before it and starts again after;
//! - a **counter** (bytes received since the container started) is kept
//!   as its rate since its last reading; the first reading has none,
//!   and neither has one after the counter went down (the container
//!   restarted): a gap, never a negative rate. A sample without a
//!   reading is a gap, and the next rate is taken across it;
//! - a sample whose **time is not after** the newest one is left out,
//!   as React's `rates` left out a point that did not move time
//!   forward: one late sample, or one stamped zero, costs nothing of
//!   what is kept. A clock set back for good stops the chart until it
//!   has caught up; a page that knows it was calls `State::reset`;
//! - a sample whose time is not a number is left out.

use std::collections::VecDeque;

/// How a series' values are read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Kind {
    /// The value is what is drawn: a share of CPU, bytes of memory.
    #[default]
    Gauge,
    /// The value only grows (bytes received): its rate per second
    /// between two samples is what is drawn.
    Counter,
}

/// The samples kept, oldest first.
#[derive(Debug, Clone)]
pub struct Samples {
    cap: usize,
    /// Seconds, strictly increasing.
    times: VecDeque<f64>,
    /// One ring per series, as long as `times`; `NaN` is a gap.
    values: Vec<VecDeque<f64>>,
    kinds: Vec<Kind>,
    /// Each counter's last reading and when, to take the next rate from.
    last: Vec<Option<(f64, f64)>>,
}

impl Samples {
    /// Room for `cap` samples (one at least) of `series` series.
    pub fn new(series: usize, cap: usize) -> Self {
        let cap = cap.max(1);
        Self {
            cap,
            times: VecDeque::with_capacity(cap),
            values: (0..series).map(|_| VecDeque::with_capacity(cap)).collect(),
            kinds: vec![Kind::Gauge; series],
            last: vec![None; series],
        }
    }

    pub fn set_kind(&mut self, series: usize, kind: Kind) {
        if let Some(slot) = self.kinds.get_mut(series) {
            *slot = kind;
        }
    }

    pub fn len(&self) -> usize {
        self.times.len()
    }

    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }

    pub fn series(&self) -> usize {
        self.values.len()
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    /// When the sample at `index` was taken.
    pub fn time(&self, index: usize) -> Option<f64> {
        self.times.get(index).copied()
    }

    pub fn first_time(&self) -> Option<f64> {
        self.times.front().copied()
    }

    pub fn last_time(&self) -> Option<f64> {
        self.times.back().copied()
    }

    /// What `series` was at the sample `index`; `None` for a gap.
    pub fn value(&self, series: usize, index: usize) -> Option<f64> {
        self.values
            .get(series)?
            .get(index)
            .copied()
            .filter(|value| value.is_finite())
    }

    /// The first sample at or after `time`.
    pub fn index_from(&self, time: f64) -> usize {
        self.times.partition_point(|at| *at < time)
    }

    /// The sample nearest `time`, among those from `from` on.
    pub fn nearest(&self, time: f64, from: usize) -> Option<usize> {
        if from >= self.len() {
            return None;
        }
        let after = self.index_from(time).clamp(from, self.len() - 1);
        let before = after.saturating_sub(1).max(from);
        let far = |index: usize| (self.times[index] - time).abs();
        Some(if far(before) <= far(after) {
            before
        } else {
            after
        })
    }

    /// Appends a sample: `time` in seconds and one value per series (a
    /// series that has none there, or was given none, has a gap). Returns
    /// whether anything changed.
    pub fn push(&mut self, time: f64, values: &[Option<f64>]) -> bool {
        if !time.is_finite() || self.last_time().is_some_and(|newest| time <= newest) {
            return false;
        }
        if self.times.len() == self.cap {
            self.pop_front();
        }
        self.times.push_back(time);
        for (series, ring) in self.values.iter_mut().enumerate() {
            let read = values
                .get(series)
                .copied()
                .flatten()
                .filter(|value| value.is_finite());
            let value = match self.kinds[series] {
                Kind::Gauge => read,
                Kind::Counter => {
                    let rate = match (self.last[series], read) {
                        (Some((then, before)), Some(now)) if now >= before && time > then => {
                            Some((now - before) / (time - then))
                        }
                        _ => None,
                    };
                    // A reading that is missing leaves the last one
                    // standing: the next rate is taken from it.
                    if let Some(now) = read {
                        self.last[series] = Some((time, now));
                    }
                    rate
                }
            };
            ring.push_back(value.unwrap_or(f64::NAN));
        }
        true
    }

    /// Drops the samples older than `time`. Returns whether any was.
    pub fn drop_before(&mut self, time: f64) -> bool {
        let count = self.index_from(time);
        for _ in 0..count {
            self.pop_front();
        }
        count > 0
    }

    /// Forgets every sample and every counter's last reading.
    pub fn clear(&mut self) {
        self.times.clear();
        for ring in &mut self.values {
            ring.clear();
        }
        self.last.fill(None);
    }

    fn pop_front(&mut self) {
        self.times.pop_front();
        for ring in &mut self.values {
            ring.pop_front();
        }
    }
}
