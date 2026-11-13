//! The Health tab: what it reads, when, its folded output, and how it
//! looks.

use homeport_engine::containers::health::{Health, HealthConfig, HealthResult};

use super::*;
use crate::{
    test_support::snapshot::{assert_snapshot, in_language},
    ui::{
        features::docker::containers::detail::{queries, tabs::health},
        shell::page::testing::Asked,
        theme::Theme,
        widgets::basic::technical,
    },
};

/// `unhealthy` of the React test, with a run whose output is long.
fn unhealthy() -> Health {
    let long: Vec<String> = (1..=9).map(|n| format!("attempt {n}: refused")).collect();
    Health {
        status: "unhealthy".to_owned(),
        failing_streak: 4,
        config: HealthConfig {
            command: "curl -fsS localhost/health || exit 1".to_owned(),
            interval_s: Some(10),
            timeout_s: None,
            retries: Some(3),
            start_period_s: None,
        },
        log: vec![
            HealthResult {
                start: "2026-11-04T10:00:20.000Z".to_owned(),
                end: "2026-11-04T10:00:22.500Z".to_owned(),
                exit_code: 1,
                output: long.join("\n"),
            },
            HealthResult {
                start: "2026-11-04T10:00:10.000Z".to_owned(),
                end: "2026-11-04T10:00:10.300Z".to_owned(),
                exit_code: 7,
                output: "curl: (7) Failed to connect to localhost port 80".to_owned(),
            },
            HealthResult {
                start: "2026-11-04T10:00:00Z".to_owned(),
                end: "2026-11-04T10:00:00.1Z".to_owned(),
                exit_code: 0,
                output: String::new(),
            },
        ],
    }
}

/// The page of `web`, whose list says `status`, on its Health tab.
fn with(status: &str, read: Option<Health>) -> Rig {
    let mut list = sample();
    list[0].health = Some(status.to_owned());
    let mut rig = Rig::on(
        connected_host(),
        route(WEB, Some("health")).into(),
        Some(list),
    );
    let key = queries::health_key("h1", WEB, Some(status));
    rig.bench.data.set(&key, read);
    rig.refresh();
    rig
}

#[test]
fn an_unhealthy_container_shows_its_failing_output_and_exit_codes() {
    let _english = in_language("en");
    let rig = with("unhealthy", Some(unhealthy()));
    for said in [
        "exit 7",
        "every 10s, timeout 30s, 3 retries, start period 0s",
        "No output.",
        "· took 300 ms",
        "· took 2.5 s",
        "Show 3 more lines",
    ] {
        assert!(rig.shows_in((828.0, 1400.0), said), "{said}");
    }
    assert!(rig.shows_in(
        (828.0, 1400.0),
        &technical("curl: (7) Failed to connect to localhost port 80")
    ));
    assert!(rig.shows(&technical("unhealthy")));
}

#[test]
fn a_long_output_is_folded_and_unfolded() {
    let mut rig = with("unhealthy", Some(unhealthy()));
    let start = "2026-11-04T10:00:20.000Z".to_owned();
    rig.send(Message::Health(health::Message::Fold(start.clone())));
    assert!(rig.state.health.open.contains(&start));
    {
        let _english = in_language("en");
        assert!(rig.shows_in((828.0, 1400.0), "Show less"));
    }
    rig.send(Message::Health(health::Message::Fold(start.clone())));
    assert!(!rig.state.health.open.contains(&start));
}

#[test]
fn the_healthcheck_is_read_again_when_its_status_changes_and_once_a_check() {
    let mut rig = with("healthy", None);
    let _ = rig.bench.took();
    let mut list = sample();
    list[0].health = Some("unhealthy".to_owned());
    rig.list(list);
    assert_eq!(
        rig.bench.watched(),
        [queries::health_key("h1", WEB, Some("unhealthy"))]
    );
    rig.send(Message::Health(health::Message::Poll));
    assert_eq!(
        rig.bench.took(),
        [Asked::Invalidate(queries::health_key(
            "h1",
            WEB,
            Some("unhealthy")
        ))]
    );
    let check = unhealthy();
    assert_eq!(health::every(Some(&check)).as_secs(), 10);
    assert_eq!(health::every(None).as_secs(), 30);
}

#[test]
fn a_container_with_no_healthcheck_says_so() {
    let _english = in_language("en");
    let rig = with("healthy", None);
    assert!(rig.shows("This container has no healthcheck."));
}

#[test]
fn a_healthy_container_whose_check_never_ran() {
    let mut never = unhealthy();
    never.status = "healthy".to_owned();
    never.failing_streak = 0;
    never.log.clear();
    let rig = with("healthy", Some(never));
    {
        let _english = in_language("en");
        assert!(rig.shows("No check has run yet."));
    }
    assert_snapshot(
        "container-health-empty-en",
        "en",
        &Theme::Light,
        SIZE,
        || rig.view(),
    );
}

#[test]
fn the_health_tab_drawn() {
    let rig = with("unhealthy", Some(unhealthy()));
    let tall = (828.0, 1000.0);
    assert_snapshot("container-health-en", "en", &Theme::Light, tall, || {
        rig.view()
    });
    assert_snapshot("container-health-ar", "ar", &Theme::Light, tall, || {
        rig.view()
    });
    assert_snapshot("container-health-dark-en", "en", &Theme::Dark, tall, || {
        rig.view()
    });
}
