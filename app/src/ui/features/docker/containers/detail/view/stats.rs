//! The Stats tab: four charts, each under its name and the figure of the
//! moment.

use iced::Length;

use super::after_dot;
use crate::{
    i18n::format,
    t,
    ui::{
        features::docker::containers::detail::{
            model::zone,
            tabs::stats::{Graph, Message, Stats},
        },
        theme::{metrics::space, Element},
        widgets::{
            basic::{card, in_reading_order},
            chart::{self, Chart, Series, Tone, Unit},
            feedback::error_message,
            line, paragraph, stack,
        },
    },
};

/// The window's width from which the charts stand two a row
/// (`lg:grid-cols-2`).
pub const TWO_COLUMNS_FROM: f32 = 1024.0;

/// A chart in its card: its name, the figure of the moment, its lines.
fn graph<'a>(
    title: String,
    now: Option<String>,
    state: &'a chart::State,
    series: Vec<Series>,
    unit: Unit,
    graph: Graph,
) -> Element<'a, Message> {
    let chart = Chart::new(state, series, move |message| Message::Chart(graph, message))
        .unit(unit)
        .zone(zone())
        .view();
    let card = card(chart).title(title);
    match now {
        // The catalogue's figures start with their dot: they read after
        // the chart's name in either direction.
        Some(now) => card.aside(in_reading_order(&now)),
        None => card,
    }
    .into()
}

/// Two charts side by side, each half the row.
fn pair<'a>(first: Element<'a, Message>, second: Element<'a, Message>) -> Element<'a, Message> {
    line(vec![first, second])
        .spacing(space::S4)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Top)
        .into()
}

#[allow(clippy::cast_precision_loss)]
pub fn view(stats: &Stats, running: bool, window_width: Option<f32>) -> Element<'_, Message> {
    if !running {
        return paragraph(t!("containers.stats.startFirst"))
            .style(crate::ui::theme::text::muted)
            .into();
    }
    let last = stats.last.as_ref();
    let bytes = |count: u64| format::bytes(count as f64);
    let cpu = graph(
        t!("containers.stats.cpu"),
        last.map(|s| after_dot(&format::percent(s.cpu_percent))),
        &stats.cpu,
        vec![Series::new(t!("containers.stats.cpu")).tone(Tone::Ink)],
        Unit::Percent,
        Graph::Cpu,
    );
    let memory = graph(
        t!("containers.stats.memory"),
        last.map(|s| {
            t!(
                "containers.stats.memoryNow",
                used = bytes(s.memory_usage),
                limit = bytes(s.memory_limit)
            )
        }),
        &stats.memory,
        vec![Series::new(t!("containers.stats.used")).tone(Tone::Ink)],
        Unit::Bytes,
        Graph::Memory,
    );
    let network = graph(
        t!("containers.stats.network"),
        last.map(|s| {
            t!(
                "containers.stats.networkNow",
                received = bytes(s.net_rx),
                sent = bytes(s.net_tx)
            )
        }),
        &stats.network,
        vec![
            Series::new(t!("containers.stats.received")).tone(Tone::Ink),
            Series::new(t!("containers.stats.sent"))
                .tone(Tone::Grey)
                .dashed(),
        ],
        Unit::BytesPerSecond,
        Graph::Network,
    );
    let disk = graph(
        t!("containers.stats.disk"),
        last.map(|s| t!("containers.stats.processes", count = s.pids)),
        &stats.disk,
        vec![
            Series::new(t!("containers.stats.read"))
                .tone(Tone::Grey)
                .dashed(),
            Series::new(t!("containers.stats.written")).tone(Tone::Ink),
        ],
        Unit::BytesPerSecond,
        Graph::Disk,
    );
    let wide = window_width.is_none_or(|width| width >= TWO_COLUMNS_FROM);
    let mut parts: Vec<Element<'_, Message>> = vec![];
    // A stream that couldn't start is said over the charts.
    parts.extend(
        stats
            .failure
            .as_ref()
            .map(|failure| error_message(&failure.error, true, Message::Failure)),
    );
    if wide {
        parts.extend([pair(cpu, memory), pair(network, disk)]);
    } else {
        parts.extend([cpu, memory, network, disk]);
    }
    stack(parts).spacing(space::S4).into()
}
