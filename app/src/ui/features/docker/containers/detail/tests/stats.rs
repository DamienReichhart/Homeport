//! The Stats tab: what its stream's samples become, and how it looks.

use homeport_core::error::{AppError, ErrorCode};

use super::*;
use crate::{
    data::streams::Update,
    i18n::format,
    test_support::{
        fixtures::stats_sample,
        snapshot::{assert_snapshot, in_language},
    },
    ui::{
        features::docker::containers::detail::tabs::stats::{self, Graph, WINDOW},
        shell::page::testing::Asked,
        theme::Theme,
        widgets::{chart, feedback::error_message::Event},
    },
};

fn samples(rig: &Rig, items: Vec<homeport_engine::containers::stats::StatsSample>) -> Message {
    let update = Update::Batch { items, dropped: 0 };
    Message::Stats(stats::Message::Samples(
        rig.state.stats.generation(),
        update,
    ))
}

/// A minute of a container that serves requests: a sample a second.
fn busy() -> Rig {
    let mut rig = Rig::at(WEB, "stats");
    for n in 0..60_u64 {
        let mut sample = stats_sample(n);
        #[allow(clippy::cast_precision_loss)]
        let wave = (n as f64 / 6.0).sin();
        sample.cpu_percent = 12.5 + 8.0 * wave;
        sample.memory_usage = (64 + n) * 1024 * 1024;
        sample.net_rx = 40_000 * n + n * n * 300;
        sample.net_tx = 25_000 * n;
        sample.block_read = 4096 * n;
        sample.block_write = 16_384 * n + 2000 * (n % 7);
        #[allow(clippy::cast_precision_loss)]
        rig.state.stats.push(THEN as f64 + n as f64, sample);
    }
    rig
}

#[test]
fn a_sample_feeds_the_four_charts_and_the_figures_beside_their_names() {
    let _english = in_language("en");
    let mut rig = Rig::at(WEB, "stats");
    assert!(rig.state.stats.last.is_none());
    let first = samples(&rig, vec![stats_sample(1)]);
    rig.send(first);
    let stats = &rig.state.stats;
    assert_eq!(stats.last, Some(stats_sample(1)));
    assert_eq!(stats.cpu.latest(0), Some(12.5));
    assert_eq!(stats.memory.latest(0), Some(64.0 * 1024.0 * 1024.0));
    // Counters are drawn as rates: the first reading has none.
    assert_eq!(stats.network.latest(0), None);
    assert_eq!(WINDOW, 300);
    let mega = format::bytes(64.0 * 1024.0 * 1024.0);
    let giga = format::bytes(1024.0 * 1024.0 * 1024.0);
    let (received, sent) = (format::bytes(1000.0), format::bytes(500.0));
    for figure in [
        "· 12.5%".to_owned(),
        format!("· {mega} of {giga}"),
        format!("· {received} in, {sent} out"),
        "· 3 processes".to_owned(),
    ] {
        assert!(rig.shows(&figure), "{figure}");
    }
}

#[test]
fn a_late_sample_of_a_stream_that_was_let_go_of_changes_nothing() {
    let mut rig = Rig::at(WEB, "stats");
    let late = samples(&rig, vec![stats_sample(1)]);
    // The tab is left and comes back: its charts start over.
    rig.send(Message::Tab(Tab::Logs));
    rig.send(Message::Tab(Tab::Stats));
    rig.send(late);
    assert!(rig.state.stats.last.is_none());
    let on_time = samples(&rig, vec![stats_sample(2)]);
    rig.send(on_time);
    assert_eq!(rig.state.stats.last, Some(stats_sample(2)));
}

#[test]
fn a_tab_that_comes_back_forgets_its_samples_and_keeps_what_its_legend_hides() {
    let mut rig = busy();
    let hide = stats::Message::Chart(Graph::Network, chart::Message::Toggle(1));
    rig.send(Message::Stats(hide));
    assert!(rig.state.stats.network.is_hidden(1));
    rig.send(Message::Tab(Tab::Inspect));
    rig.send(Message::Tab(Tab::Stats));
    assert_eq!(rig.state.stats.cpu.latest(0), None);
    assert!(rig.state.stats.last.is_none());
    assert!(rig.state.stats.network.is_hidden(1));
}

#[test]
fn a_stream_that_cannot_start_is_said_over_the_charts() {
    let _english = in_language("en");
    let mut rig = Rig::at(WEB, "stats");
    let refused = AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    );
    let generation = rig.state.stats.generation();
    let failed = stats::Message::Samples(generation, Update::Failed(refused.clone()));
    rig.send(Message::Stats(failed));
    assert!(rig.shows("This host isn't connected."));
    rig.send(Message::Stats(stats::Message::Failure(Event::Details)));
    assert_eq!(rig.bench.took(), [Asked::Details(refused)]);
    rig.send(Message::Stats(stats::Message::Failure(Event::Retry)));
    assert_eq!(rig.state.stats.generation(), generation + 1);
    assert!(!rig.shows("This host isn't connected."));
}

#[test]
fn a_stopped_container_has_no_stats() {
    let rig = Rig::at("shop-db-1-0123456789abcdef", "stats");
    {
        let _english = in_language("en");
        assert!(rig.shows("Start the container to see its resource usage."));
        assert!(!rig.shows("CPU"));
    }
    assert_snapshot(
        "container-stats-stopped-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

#[test]
fn the_stats_tab_drawn() {
    let rig = busy();
    assert_snapshot("container-stats-en", "en", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot("container-stats-ar", "ar", &Theme::Light, SIZE, || {
        rig.view()
    });
    assert_snapshot("container-stats-dark-en", "en", &Theme::Dark, SIZE, || {
        rig.view()
    });
    // Before the first sample, and in a window too narrow for two charts.
    let mut rig = Rig::at(WEB, "stats");
    assert_snapshot(
        "container-stats-empty-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
    rig.send(Message::Resized(iced::Size::new(900.0, 700.0)));
    let narrow = (628.0, 900.0);
    assert_snapshot(
        "container-stats-narrow-en",
        "en",
        &Theme::Light,
        narrow,
        || rig.view(),
    );
}
