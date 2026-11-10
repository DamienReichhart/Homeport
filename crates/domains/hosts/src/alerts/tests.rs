use homeport_core::error::{AppError, ErrorCode};

use super::*;

const HOST: &str = "h1";
const NAME: &str = "prod-server";
const T: i64 = 1_700_000_000;

fn event(action: &str, container: &str, at: i64, attributes: &[(&str, &str)]) -> EngineEvent {
    let mut attributes: HashMap<String, String> = attributes
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    attributes.insert("name".into(), container.into());
    EngineEvent {
        time: at,
        time_nano: at * 1_000_000_000,
        kind: "container".into(),
        action: action.into(),
        actor_id: format!("{container}-0123456789abcdef"),
        attributes,
    }
}

fn died(container: &str, at: i64, code: &str) -> EngineEvent {
    event("die", container, at, &[("exitCode", code)])
}

fn connected() -> HostStatus {
    HostStatus::Connected {
        engine_version: "28.0".into(),
        api_version: "1.48".into(),
        os: "linux".into(),
        arch: "x86_64".into(),
        podman_version: None,
        containerd_version: None,
        incus_version: None,
        swarm: None,
    }
}

fn unreachable() -> HostStatus {
    HostStatus::Unreachable {
        error: AppError::new(
            ErrorCode::ConnectionLost,
            crate::messages::CONNECTION_LOST.with("name", NAME),
        ),
        retry_in_secs: 4,
    }
}

fn kinds(alerts: &[Option<Alert>]) -> Vec<Option<AlertKind>> {
    alerts.iter().map(|a| a.as_ref().map(|a| a.kind)).collect()
}

#[test]
fn a_crash_is_an_alert_naming_the_container_and_host() {
    let mut alerts = Alerts::new();
    let alert = alerts
        .event(HOST, NAME, &died("web-1", T, "137"), T)
        .unwrap();
    assert_eq!(alert.kind, AlertKind::ContainerDied);
    assert_eq!(alert.body, "web-1 stopped on prod-server (exit code 137)");
}

#[test]
fn a_clean_exit_is_not_an_alert() {
    let mut alerts = Alerts::new();
    assert_eq!(alerts.event(HOST, NAME, &died("job", T, "0"), T), None);
}

#[test]
fn stopping_a_container_is_not_an_alert() {
    let mut alerts = Alerts::new();
    // `docker stop`: kill, then die (143, or 137 after the timeout), then stop.
    let stop = [
        event("kill", "web-1", T, &[("signal", "15")]),
        died("web-1", T + 10, "137"),
        event("stop", "web-1", T + 10, &[]),
    ];
    let raised: Vec<_> = stop
        .iter()
        .map(|e| alerts.event(HOST, NAME, e, T + 10))
        .collect();
    assert_eq!(kinds(&raised), [None, None, None]);
    // Started again, then it crashes by itself.
    alerts.event(HOST, NAME, &event("start", "web-1", T + 20, &[]), T + 20);
    let crash = alerts.event(HOST, NAME, &died("web-1", T + 30, "1"), T + 30);
    assert_eq!(crash.map(|a| a.kind), Some(AlertKind::ContainerDied));
}

#[test]
fn running_out_of_memory_is_one_alert() {
    let mut alerts = Alerts::new();
    let oom = alerts
        .event(HOST, NAME, &event("oom", "db", T, &[]), T)
        .unwrap();
    assert_eq!(oom.body, "db ran out of memory on prod-server");
    assert_eq!(alerts.event(HOST, NAME, &died("db", T, "137"), T), None);
}

#[test]
fn a_restart_loop_alerts_once_every_five_minutes() {
    let mut alerts = Alerts::new();
    let mut raised = Vec::new();
    for i in 0..10 {
        let at = T + i * 40;
        alerts.event(HOST, NAME, &event("start", "web-1", at, &[]), at);
        raised.push(alerts.event(HOST, NAME, &died("web-1", at + 1, "1"), at + 1));
    }
    assert_eq!(raised.iter().flatten().count(), 2, "{raised:?}");
    // Another container still alerts.
    assert!(alerts
        .event(HOST, NAME, &died("api", T + 50, "2"), T + 50)
        .is_some());
}

#[test]
fn events_from_before_now_are_history() {
    let mut alerts = Alerts::new();
    assert_eq!(
        alerts.event(HOST, NAME, &died("web-1", T, "1"), T + 3600),
        None
    );
}

#[test]
fn turning_unhealthy_is_an_alert_once() {
    let mut alerts = Alerts::new();
    let health = |status: &str, at: i64| event(&format!("health_status: {status}"), "api", at, &[]);
    alerts.event(HOST, NAME, &event("start", "api", T, &[]), T);
    let first = alerts.event(HOST, NAME, &health("unhealthy", T + 30), T + 30);
    assert_eq!(first.unwrap().body, "api is unhealthy on prod-server");
    // Still unhealthy: the Engine reports it again, no new alert.
    assert_eq!(
        alerts.event(HOST, NAME, &health("unhealthy", T + 400), T + 400),
        None
    );
    // Healthy, then unhealthy again after the quiet time.
    alerts.event(HOST, NAME, &health("healthy", T + 500), T + 500);
    let again = alerts.event(HOST, NAME, &health("unhealthy", T + 700), T + 700);
    assert_eq!(again.map(|a| a.kind), Some(AlertKind::ContainerUnhealthy));
}

#[test]
fn an_outage_alerts_once_then_says_when_the_host_is_back() {
    let mut alerts = Alerts::new();
    let mut raised = vec![alerts.status(HOST, NAME, &connected(), T)];
    // Retries cycle between connecting and unreachable.
    for i in 1..5 {
        raised.push(alerts.status(HOST, NAME, &unreachable(), T + i * 10));
        raised.push(alerts.status(HOST, NAME, &HostStatus::Connecting, T + i * 10 + 1));
    }
    raised.push(alerts.status(HOST, NAME, &connected(), T + 600));
    let raised: Vec<_> = raised.into_iter().flatten().collect();
    assert_eq!(raised.len(), 2, "{raised:?}");
    assert_eq!(raised[0].kind, AlertKind::HostUnreachable);
    assert!(raised[0].body.starts_with("prod-server is unreachable"));
    assert_eq!(raised[1].kind, AlertKind::HostBack);
    assert_eq!(raised[1].body, "prod-server is connected again.");
}

#[test]
fn a_host_that_never_connected_is_not_an_outage() {
    let mut alerts = Alerts::new();
    // At startup, before it ever connected.
    assert_eq!(alerts.status(HOST, NAME, &HostStatus::Connecting, T), None);
    assert_eq!(alerts.status(HOST, NAME, &unreachable(), T + 1), None);
    // Connecting for the first time isn't "back".
    assert_eq!(alerts.status(HOST, NAME, &connected(), T + 2), None);
}

#[test]
fn a_disabled_host_never_alerts() {
    let mut alerts = Alerts::new();
    alerts.status(HOST, NAME, &connected(), T);
    assert_eq!(
        alerts.status(HOST, NAME, &HostStatus::Disabled, T + 1),
        None
    );
    // Enabled again, it fails to connect: not an outage of a connected Host.
    assert_eq!(alerts.status(HOST, NAME, &unreachable(), T + 2), None);
}
