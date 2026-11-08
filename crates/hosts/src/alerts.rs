//! Which status changes and Engine events deserve an OS notification: a
//! container that died or turned unhealthy, a Host that became unreachable
//! or came back. Pure: the caller passes the time, so every rule is tested
//! without an Engine.

use std::collections::HashMap;

use crate::{
    registry::{EngineEvent, HostStatus},
    HostId,
};

/// What an alert is about; each can be turned off in Settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertKind {
    ContainerDied,
    ContainerUnhealthy,
    HostUnreachable,
    HostBack,
}

/// One notification to show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alert {
    pub kind: AlertKind,
    pub title: String,
    pub body: String,
}

/// At most one alert per container (or Host) and kind in this many seconds,
/// so a restart loop doesn't flood the desktop.
pub const QUIET_SECS: i64 = 5 * 60;
/// A `die` this soon after a `kill` was asked for (`docker stop`, Stop in
/// Homeport, `compose down`): not a crash.
const STOPPING_SECS: i64 = 5 * 60;
/// Events older than this are history (the gap filled after a restart),
/// not news.
const STALE_SECS: i64 = 2 * 60;

#[derive(Debug, Default)]
struct HostState {
    /// Seen connected since it was last disabled, so a failure is an outage
    /// and not a Host that never connected.
    was_connected: bool,
    /// An unreachable alert was raised for the current outage.
    outage_alerted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Health {
    Unknown,
    Starting,
    Healthy,
    Unhealthy,
}

#[derive(Debug)]
struct ContainerState {
    /// When a `kill` was last asked for, until the container starts again.
    stopping_since: Option<i64>,
    health: Health,
}

impl Default for ContainerState {
    fn default() -> Self {
        Self {
            stopping_since: None,
            health: Health::Unknown,
        }
    }
}

/// The alerts' memory, across Hosts.
#[derive(Debug, Default)]
pub struct Alerts {
    hosts: HashMap<HostId, HostState>,
    containers: HashMap<(HostId, String), ContainerState>,
    /// When each (Host, subject, kind) last raised an alert.
    raised: HashMap<(HostId, String, AlertKind), i64>,
}

impl Alerts {
    pub fn new() -> Self {
        Self::default()
    }

    /// The alert a Host's new status raises, if any. `now` is Unix seconds.
    pub fn status(
        &mut self,
        host_id: &str,
        host_name: &str,
        status: &HostStatus,
        now: i64,
    ) -> Option<Alert> {
        let host = self.hosts.entry(host_id.to_owned()).or_default();
        match status {
            HostStatus::Connected { .. } => {
                host.was_connected = true;
                if !std::mem::take(&mut host.outage_alerted) {
                    return None;
                }
                self.raise(host_id, "", AlertKind::HostBack, now)?;
                Some(Alert {
                    kind: AlertKind::HostBack,
                    title: "Host back".to_owned(),
                    body: format!("{host_name} is connected again."),
                })
            }
            HostStatus::Unreachable { error, .. } => {
                if !host.was_connected || host.outage_alerted {
                    return None;
                }
                self.raise(host_id, "", AlertKind::HostUnreachable, now)?;
                self.hosts
                    .entry(host_id.to_owned())
                    .or_default()
                    .outage_alerted = true;
                Some(Alert {
                    kind: AlertKind::HostUnreachable,
                    title: "Host unreachable".to_owned(),
                    body: format!("{host_name} is unreachable: {}", error.message),
                })
            }
            HostStatus::Disabled => {
                *host = HostState::default();
                None
            }
            _ => None,
        }
    }

    /// The alert an Engine event raises, if any. `now` is Unix seconds.
    pub fn event(
        &mut self,
        host_id: &str,
        host_name: &str,
        event: &EngineEvent,
        now: i64,
    ) -> Option<Alert> {
        if event.kind != "container" {
            return None;
        }
        let key = (host_id.to_owned(), event.actor_id.clone());
        if event.action == "destroy" {
            self.containers.remove(&key);
            return None;
        }
        let container = self.containers.entry(key).or_default();
        let name = event
            .attributes
            .get("name")
            .cloned()
            .unwrap_or_else(|| event.actor_id.chars().take(12).collect());
        let fresh = now - event.time <= STALE_SECS;
        let alert = match event.action.as_str() {
            "kill" | "stop" => {
                container.stopping_since.get_or_insert(event.time);
                None
            }
            "start" | "restart" => {
                container.stopping_since = None;
                container.health = Health::Starting;
                None
            }
            "oom" => Some((
                AlertKind::ContainerDied,
                "Container out of memory",
                format!("{name} ran out of memory on {host_name}"),
            )),
            "die" => {
                let asked = container
                    .stopping_since
                    .take()
                    .is_some_and(|since| event.time - since <= STOPPING_SECS);
                let code = event
                    .attributes
                    .get("exitCode")
                    .and_then(|code| code.parse::<i64>().ok())
                    .unwrap_or(0);
                (!asked && code != 0).then(|| {
                    (
                        AlertKind::ContainerDied,
                        "Container stopped",
                        format!("{name} stopped on {host_name} (exit code {code})"),
                    )
                })
            }
            action => {
                let health = match action.strip_prefix("health_status:").map(str::trim) {
                    Some("healthy") => Health::Healthy,
                    Some("unhealthy") => Health::Unhealthy,
                    Some("starting") => Health::Starting,
                    _ => return None,
                };
                let was = std::mem::replace(&mut container.health, health);
                (health == Health::Unhealthy && was != Health::Unhealthy).then(|| {
                    (
                        AlertKind::ContainerUnhealthy,
                        "Container unhealthy",
                        format!("{name} is unhealthy on {host_name}"),
                    )
                })
            }
        };
        let (kind, title, body) = alert.filter(|_| fresh)?;
        self.raise(host_id, &event.actor_id, kind, now)?;
        Some(Alert {
            kind,
            title: title.to_owned(),
            body,
        })
    }

    /// Records an alert about `subject` unless one of the same kind was
    /// raised in the last [`QUIET_SECS`]; `None` when it's too soon.
    fn raise(&mut self, host_id: &str, subject: &str, kind: AlertKind, now: i64) -> Option<()> {
        let key = (host_id.to_owned(), subject.to_owned(), kind);
        if self
            .raised
            .get(&key)
            .is_some_and(|last| now - last < QUIET_SECS)
        {
            return None;
        }
        self.raised.insert(key, now);
        Some(())
    }
}

#[cfg(test)]
mod tests {
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
        let health =
            |status: &str, at: i64| event(&format!("health_status: {status}"), "api", at, &[]);
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
}
