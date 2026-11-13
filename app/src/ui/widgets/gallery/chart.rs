//! The time-series chart, on what a container and a Cluster give it:
//! one series, two counters, ten series of an hour of samples, and a
//! stream that stops, restarts and sends what is no number.

use std::time::Duration;

use iced::{widget::Row, Length, Subscription};
use jiff::tz::TimeZone;

use super::{Gallery, Message};
use crate::ui::{
    theme::{
        metrics::{space, ButtonSize},
        Element,
    },
    widgets::{
        basic::{
            button::{self, Variant},
            card,
        },
        chart::{self, Chart, Series, State, Tone, Unit},
        stack,
    },
};

/// 2026-05-01 10:00:00 UTC: when the gallery's samples start.
pub const START: f64 = 1_777_629_600.0;
/// Five minutes at a sample a second: `STATS_WINDOW`.
pub const WINDOW: usize = 300;
/// An hour at a sample a second, for ten series: the ticket's load.
pub const HOUR: usize = 3600;
pub const MANY: usize = 10;

const MIB: f64 = 1024.0 * 1024.0;

/// The gallery's charts and whether samples keep coming.
pub struct Charts {
    pub cpu: State,
    pub memory: State,
    pub network: State,
    pub many: State,
    pub ragged: State,
    pub live: bool,
    /// How many samples each chart was given.
    pub written: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Cpu(chart::Message),
    Memory(chart::Message),
    Network(chart::Message),
    Many(chart::Message),
    Ragged(chart::Message),
    Live(bool),
    Tick,
}

/// A number between 0 and 1 that is the same for the same `n`: the
/// noise of the samples, without a clock or a generator to seed.
fn noise(n: usize) -> f64 {
    let mixed = (n as u64)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .rotate_left(29)
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);
    // The top 53 bits make a fraction exactly.
    #[allow(clippy::cast_precision_loss)]
    let unit = (mixed >> 11) as f64 / (1u64 << 53) as f64;
    unit
}

/// A wave of `period` samples with noise on it, between 0 and 1.
#[allow(clippy::cast_precision_loss)]
fn wave(n: usize, period: f64, seed: usize) -> f64 {
    let turn = (n as f64 / period + seed as f64 * 0.37) * std::f64::consts::TAU;
    (0.5 + 0.35 * turn.sin() + 0.15 * (noise(n * 31 + seed) - 0.5)).clamp(0.0, 1.0)
}

#[allow(clippy::cast_precision_loss)]
fn at(n: usize) -> f64 {
    START + n as f64
}

fn cpu_sample(n: usize) -> [Option<f64>; 1] {
    [Some(wave(n, 90.0, 1) * 180.0)]
}

fn memory_sample(n: usize) -> [Option<f64>; 1] {
    #[allow(clippy::cast_precision_loss)]
    let grown = 180.0 + n as f64 * 0.4;
    [Some((grown + wave(n, 40.0, 2) * 24.0) * MIB)]
}

/// Two counters; the container restarts at the 200th sample, so both
/// start over from nothing.
fn network_sample(n: usize) -> [Option<f64>; 2] {
    let since = if n >= 200 { n - 200 } else { n };
    let total = |seed: usize, pace: f64| {
        (0..=since)
            .map(|step| wave(step + n - since, 60.0, seed) * pace)
            .sum::<f64>()
    };
    [Some(total(3, 900_000.0)), Some(total(4, 120_000.0))]
}

fn many_sample(n: usize) -> [Option<f64>; MANY] {
    std::array::from_fn(|series| {
        #[allow(clippy::cast_precision_loss)]
        let level = 20.0 + series as f64 * 14.0;
        let turn = n as f64 / (240.0 + 35.0 * series as f64) + series as f64 * 0.37;
        let drift = 20.0 * (turn * std::f64::consts::TAU).sin();
        // A spike now and then, which thinning the line must keep.
        let spike = if (n + series * 97).is_multiple_of(613) {
            30.0
        } else {
            0.0
        };
        Some(level + drift + spike + 3.0 * noise(n * 31 + series))
    })
}

pub fn cpu(samples: usize) -> State {
    let mut state = State::new(1, WINDOW);
    (0..samples).for_each(|n| state.push(at(n), &cpu_sample(n)));
    state
}

pub fn memory(samples: usize) -> State {
    let mut state = State::new(1, WINDOW);
    (0..samples).for_each(|n| state.push(at(n), &memory_sample(n)));
    state
}

pub fn network(samples: usize) -> State {
    let mut state = State::new(2, WINDOW).counter(0).counter(1);
    (0..samples).for_each(|n| state.push(at(n), &network_sample(n)));
    state
}

/// Ten series of `samples` samples each.
pub fn many(samples: usize) -> State {
    let mut state = State::new(MANY, HOUR);
    (0..samples).for_each(|n| state.push(at(n), &many_sample(n)));
    state
}

/// What a stream that misbehaves leaves: a series with a hole where
/// samples were missing and one sample alone after it, a series of
/// equal values with a `NaN` and an infinite value among them, and a
/// minute during which nothing came at all.
pub fn ragged() -> State {
    let mut state = State::new(2, WINDOW).breaking_after(5.0);
    for n in (0..120).chain(180..300) {
        let first = match n {
            40..=69 | 72..=90 => None,
            _ => Some(30.0 + wave(n, 50.0, 5) * 40.0),
        };
        let second = match n {
            100 => Some(f64::NAN),
            101 => Some(f64::INFINITY),
            _ => Some(42.0),
        };
        state.push(at(n), &[first, second]);
    }
    state
}

impl Default for Charts {
    fn default() -> Self {
        Self {
            cpu: cpu(WINDOW),
            memory: memory(WINDOW),
            network: network(WINDOW),
            many: many(HOUR),
            ragged: ragged(),
            live: false,
            written: WINDOW,
        }
    }
}

impl Charts {
    pub fn update(&mut self, event: Event) {
        match event {
            Event::Cpu(message) => self.cpu.update(message),
            Event::Memory(message) => self.memory.update(message),
            Event::Network(message) => self.network.update(message),
            Event::Many(message) => self.many.update(message),
            Event::Ragged(message) => self.ragged.update(message),
            Event::Live(live) => self.live = live,
            Event::Tick => {
                let n = self.written;
                self.written += 1;
                self.cpu.push(at(n), &cpu_sample(n));
                self.memory.push(at(n), &memory_sample(n));
                self.network.push(at(n), &network_sample(n));
                self.many
                    .push(at(HOUR - WINDOW + n), &many_sample(HOUR - WINDOW + n));
            }
        }
    }
}

fn chart<'a>(
    state: &'a State,
    series: Vec<Series>,
    unit: Unit,
    event: fn(chart::Message) -> Event,
) -> Chart<'a, Message> {
    Chart::new(state, series, move |message| Message::Chart(event(message)))
        .unit(unit)
        // The same picture wherever it is drawn.
        .zone(TimeZone::UTC)
}

/// One series, as a container's CPU: ink, and no legend beyond its name.
pub fn cpu_view(state: &State) -> Element<'_, Message> {
    let series = vec![Series::new("CPU").tone(Tone::Ink)];
    chart(state, series, Unit::Percent, Event::Cpu).view()
}

/// An area, in bytes.
pub fn memory_view(state: &State) -> Element<'_, Message> {
    let series = vec![Series::new("Used").tone(Tone::Ink).filled()];
    chart(state, series, Unit::Bytes, Event::Memory).view()
}

/// Two counters as rates, the brand's way: ink, and grey dashed.
pub fn network_view(state: &State) -> Element<'_, Message> {
    let series = vec![
        Series::new("Received").tone(Tone::Ink),
        Series::new("Sent").tone(Tone::Grey).dashed(),
    ];
    chart(state, series, Unit::BytesPerSecond, Event::Network).view()
}

/// Ten series in the looks the kit gives them.
pub fn many_view(state: &State) -> Element<'_, Message> {
    let series = (1..=MANY)
        .map(|n| Series::new(format!("worker-{n}")))
        .collect();
    chart(state, series, Unit::Percent, Event::Many)
        .height(220.0)
        .view()
}

pub fn ragged_view(state: &State) -> Element<'_, Message> {
    let series = vec![Series::new("Stops and starts"), Series::new("Always 42")];
    chart(state, series, Unit::Number, Event::Ragged).view()
}

/// A chart nothing was pushed to yet.
pub fn empty_view(state: &State) -> Element<'_, Message> {
    chart(state, vec![Series::new("Waiting")], Unit::Bytes, Event::Cpu).view()
}

impl Gallery {
    pub(super) fn chart_subscription(&self) -> Subscription<Message> {
        if self.charts.live && self.page == super::Page::Chart {
            iced::time::every(Duration::from_secs(1)).map(|_| Message::Chart(Event::Tick))
        } else {
            Subscription::none()
        }
    }

    pub(super) fn charts(&self) -> Element<'_, Message> {
        let charts = &self.charts;
        let live = button::text("A sample a second")
            .variant(if charts.live {
                Variant::Secondary
            } else {
                Variant::Outline
            })
            .size(ButtonSize::Sm)
            .on_press(Message::Chart(Event::Live(!charts.live)));
        let titled = |title: &'static str, body| -> Element<'_, Message> {
            card(body).title(title).view().into()
        };
        let row = |a, b| -> Element<'_, Message> {
            Row::from_vec(crate::i18n::direction().ordered(vec![a, b]))
                .spacing(space::S4)
                .width(Length::Fill)
                .into()
        };
        stack(vec![
            live.into(),
            row(
                titled("CPU", cpu_view(&charts.cpu)),
                titled("Memory", memory_view(&charts.memory)),
            ),
            row(
                titled("Network", network_view(&charts.network)),
                titled("Gaps", ragged_view(&charts.ragged)),
            ),
            titled("Ten series, an hour", many_view(&charts.many)),
        ])
        .spacing(space::S4)
        .into()
    }
}
