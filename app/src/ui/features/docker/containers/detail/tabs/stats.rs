//! The Stats tab (`StatsTab`, `useStats`): the last five minutes of a
//! running container's CPU, memory, network and disk, a sample a second.
//!
//! The stream has a number, as the logs': what a stream that was let go
//! of still sends changes nothing.

use std::sync::Arc;

use iced::Subscription;

use homeport_engine::containers::stats::StatsSample;

use crate::{
    app::AppState,
    data::streams::{self, shapes, Update},
    ui::{
        shell::Context,
        widgets::{
            chart,
            feedback::{error_message, Surfaced},
        },
    },
};

/// Five minutes of samples at about one a second (`STATS_WINDOW`).
pub const WINDOW: usize = 300;

/// One of the tab's four charts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Graph {
    Cpu,
    Memory,
    Network,
    Disk,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// What the stream of this number sent.
    Samples(u64, Update<StatsSample>),
    /// The legend of a chart.
    Chart(Graph, chart::Message),
    /// What was asked of the failure shown over the charts.
    Failure(error_message::Event),
}

#[derive(Debug)]
pub struct Stats {
    pub cpu: chart::State,
    pub memory: chart::State,
    /// Received and sent: counters, drawn as their rates.
    pub network: chart::State,
    /// Read and written: counters, drawn as their rates.
    pub disk: chart::State,
    /// The latest sample, for the figures beside the charts' names.
    pub last: Option<StatsSample>,
    /// The number of the stream that feeds the charts.
    generation: u64,
    /// Why the stream couldn't start.
    pub failure: Option<Surfaced>,
}

/// The moment a sample came, in seconds since 1970 (`Date.now() / 1000`).
#[allow(clippy::cast_precision_loss)]
fn now() -> f64 {
    jiff::Timestamp::now().as_millisecond() as f64 / 1000.0
}

impl Stats {
    pub fn new() -> Self {
        Self {
            cpu: chart::State::new(1, WINDOW),
            memory: chart::State::new(1, WINDOW),
            network: chart::State::new(2, WINDOW).counter(0).counter(1),
            disk: chart::State::new(2, WINDOW).counter(0).counter(1),
            last: None,
            generation: 1,
            failure: None,
        }
    }

    /// The number of the stream that feeds the charts.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Starts over, for a tab that comes back: no sample, no counter's
    /// last reading, and another stream.
    pub fn restart(&mut self) {
        self.generation += 1;
        for chart in [
            &mut self.cpu,
            &mut self.memory,
            &mut self.network,
            &mut self.disk,
        ] {
            chart.reset();
        }
        self.last = None;
        self.failure = None;
    }

    /// A sample that came at `time`, in seconds since 1970.
    #[allow(clippy::cast_precision_loss)]
    pub fn push(&mut self, time: f64, sample: StatsSample) {
        let count = |bytes: u64| Some(bytes as f64);
        self.cpu.push(time, &[Some(sample.cpu_percent)]);
        self.memory.push(time, &[count(sample.memory_usage)]);
        self.network
            .push(time, &[count(sample.net_rx), count(sample.net_tx)]);
        self.disk
            .push(time, &[count(sample.block_read), count(sample.block_write)]);
        self.last = Some(sample);
    }

    /// The container's stats stream, while the page returns it.
    pub fn subscription(
        &self,
        state: &Arc<AppState>,
        host_id: &str,
        container_id: &str,
    ) -> Subscription<Message> {
        let params = (host_id.to_owned(), container_id.to_owned(), self.generation);
        streams::subscription(state, params, |source| {
            let (host_id, container_id, _) = source.params.clone();
            shapes::stats(source.state.clone(), host_id, container_id)
        })
        .with(self.generation)
        .map(|(generation, update)| Message::Samples(generation, update))
    }

    pub fn update(&mut self, message: Message, ctx: &Context<'_>) {
        match message {
            // A stream that was let go of may still have sent this.
            Message::Samples(generation, _) if generation != self.generation => {}
            Message::Samples(_, Update::Batch { items, .. }) => {
                // A sample has the time it came at. Several in one batch
                // (the window was behind) came together: the chart keeps
                // one sample a moment, the latest.
                if let Some(sample) = items.into_iter().last() {
                    self.failure = None;
                    self.push(now(), sample);
                }
            }
            Message::Samples(_, Update::Failed(error)) => {
                self.failure = Some(Surfaced::new(error, "stats_subscribe"));
            }
            Message::Samples(_, Update::Started(_) | Update::Ended) => {}
            Message::Chart(graph, message) => match graph {
                Graph::Cpu => self.cpu.update(message),
                Graph::Memory => self.memory.update(message),
                Graph::Network => self.network.update(message),
                Graph::Disk => self.disk.update(message),
            },
            Message::Failure(error_message::Event::Retry) => self.restart(),
            Message::Failure(error_message::Event::Details) => {
                if let Some(failure) = &self.failure {
                    ctx.details(failure.clone());
                }
            }
            Message::Failure(error_message::Event::Copy(command)) => ctx.copy(command),
        }
    }
}
