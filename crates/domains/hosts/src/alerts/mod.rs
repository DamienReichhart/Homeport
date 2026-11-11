//! Which status changes and Engine events deserve an OS notification: a
//! container that died or turned unhealthy, a Host that became unreachable
//! or came back. Pure: the caller passes the time, so every rule is tested
//! without an Engine.

use std::collections::HashMap;

use crate::{
    messages as msg,
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
                    title: msg::ALERT_HOST_BACK_TITLE.localized(),
                    body: msg::ALERT_HOST_BACK.with("host", host_name).localized(),
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
                    title: msg::ALERT_HOST_UNREACHABLE_TITLE.localized(),
                    body: msg::ALERT_HOST_UNREACHABLE
                        .with("host", host_name)
                        .with("reason", error.localized_message())
                        .localized(),
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
                msg::ALERT_OUT_OF_MEMORY_TITLE,
                msg::ALERT_OUT_OF_MEMORY
                    .with("container", name.as_str())
                    .with("host", host_name),
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
                        msg::ALERT_STOPPED_TITLE,
                        msg::ALERT_STOPPED
                            .with("container", name.as_str())
                            .with("host", host_name)
                            .with("code", code),
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
                        msg::ALERT_UNHEALTHY_TITLE,
                        msg::ALERT_UNHEALTHY
                            .with("container", name.as_str())
                            .with("host", host_name),
                    )
                })
            }
        };
        let (kind, title, body) = alert.filter(|_| fresh)?;
        self.raise(host_id, &event.actor_id, kind, now)?;
        Some(Alert {
            kind,
            title: title.localized(),
            body: body.localized(),
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
mod tests;
