//! What a page keeps of a chart: its samples, which series are hidden,
//! and what was worked out of them for the last draw.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use iced::widget::canvas;

use super::{
    samples::{Kind, Samples},
    scene::Scene,
    tip::Tip,
};

/// What the chart's own controls send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// A press on the legend's entry for a series: hides it, or shows it
    /// again.
    Toggle(usize),
}

pub struct State {
    pub(super) samples: Samples,
    pub(super) hidden: Vec<bool>,
    pub(super) window: Option<f64>,
    pub(super) break_after: Option<f64>,
    /// Counts every change to what is drawn: samples and hidden series.
    pub(super) revision: u64,
    /// The axes and the lines as they were last drawn: drawn again only
    /// when [`State::drawn`] no longer says what they show.
    pub(super) lines: canvas::Cache,
    pub(super) drawn: Cell<u64>,
    /// The scene of the last draw, and what it was made from
    /// (`Plot::signature`).
    pub(super) scene: RefCell<Option<(u64, Rc<Scene>)>>,
    /// The tooltip of the sample last pointed at.
    pub(super) tip: RefCell<Option<Rc<Tip>>>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("samples", &self.samples.len())
            .field("hidden", &self.hidden)
            .field("window", &self.window)
            .field("revision", &self.revision)
            .finish_non_exhaustive()
    }
}

impl State {
    /// A chart of `series` series that keeps the last `cap` samples.
    pub fn new(series: usize, cap: usize) -> Self {
        Self {
            samples: Samples::new(series, cap),
            hidden: vec![false; series],
            window: None,
            break_after: None,
            revision: 0,
            lines: canvas::Cache::new(),
            drawn: Cell::new(u64::MAX),
            scene: RefCell::new(None),
            tip: RefCell::new(None),
        }
    }

    /// The series at `index` is a counter (bytes received): what is
    /// drawn is its rate per second between two samples.
    #[must_use]
    pub fn counter(mut self, index: usize) -> Self {
        self.samples.set_kind(index, Kind::Counter);
        self
    }

    /// The time axis shows the `seconds` before the newest sample and
    /// moves with it; older samples are let go of. Without it the axis
    /// spans every sample kept.
    #[must_use]
    pub fn window(mut self, seconds: f64) -> Self {
        self.window = (seconds.is_finite() && seconds > 0.0).then_some(seconds);
        self
    }

    /// Two samples more than `seconds` apart are not joined: the stream
    /// stopped in between, and a line across would say it did not.
    #[must_use]
    pub fn breaking_after(mut self, seconds: f64) -> Self {
        self.break_after = (seconds.is_finite() && seconds > 0.0).then_some(seconds);
        self
    }

    /// Appends a sample: when it was taken, in seconds since 1970, and
    /// one value per series (`None`, `NaN` or an infinite value is a
    /// gap). `samples.rs` says what becomes of a counter, of a sample
    /// that comes late and of a time that is no number.
    pub fn push(&mut self, time: f64, values: &[Option<f64>]) {
        if !self.samples.push(time, values) {
            return;
        }
        if let Some(window) = self.window {
            // One sample before the window stays: its line reaches in.
            let keep_from = self.samples.index_from(time - window).saturating_sub(1);
            if let Some(oldest) = self.samples.time(keep_from) {
                self.samples.drop_before(oldest);
            }
        }
        self.changed();
    }

    /// Forgets every sample, and what the counters last read: what a
    /// page does before it follows another container. Which series are
    /// hidden stays, as the legend's choice of what to look at.
    pub fn reset(&mut self) {
        self.samples.clear();
        self.changed();
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(index) => {
                if let Some(hidden) = self.hidden.get_mut(index) {
                    *hidden = !*hidden;
                    self.changed();
                }
            }
        }
    }

    pub fn samples(&self) -> &Samples {
        &self.samples
    }

    pub fn is_hidden(&self, index: usize) -> bool {
        self.hidden.get(index).copied().unwrap_or(false)
    }

    /// The newest value of a series, for a figure beside the chart's
    /// title; `None` while it has none or its newest sample is a gap.
    pub fn latest(&self, series: usize) -> Option<f64> {
        self.samples
            .value(series, self.samples.len().checked_sub(1)?)
    }

    fn changed(&mut self) {
        self.revision = self.revision.wrapping_add(1);
        *self.tip.get_mut() = None;
    }
}
