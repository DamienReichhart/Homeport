//! The task that keeps one Host connected: connect, watch, retry.

use std::{
    collections::HashMap,
    sync::{Arc, Weak},
    time::Duration,
};

use bollard::{query_parameters::EventsOptionsBuilder, Docker};
use futures_util::StreamExt;
use serde::Deserialize;

use homeport_core::error::{AppError, ErrorCode};
use homeport_engine::swarm;

use crate::messages as msg;
use homeport_transport::{self as transport, http, ConnectError, Dialer};

use super::{Client, EngineEvent, HostRegistry, HostStatus};
use crate::{events, EngineKind, HostId};

/// How far back a Host without any kept event asks its Engine for events
/// on connecting. The Engine only holds its latest few hundred anyway.
const FIRST_LOOK_BACK_NANOS: i64 = 24 * 3600 * 1_000_000_000;

const PING_INTERVAL: Duration = Duration::from_secs(15);

pub use homeport_core::backoff::next_backoff;

pub(super) async fn supervise(registry: Weak<HostRegistry>, id: HostId) {
    let mut backoff = 1;
    loop {
        let Some(reg) = registry.upgrade() else {
            return;
        };
        let host = match reg.store.get(&id) {
            Ok(host) => host,
            Err(_) => return reg.remove(&id),
        };
        if !host.enabled {
            return reg.set_status(&id, HostStatus::Disabled, None);
        }
        // Only a restored backup from a newer Homeport can hold another kind.
        if host.engine != EngineKind::Docker {
            let error = super::unsupported(&host.name, host.engine.label())
                .with_hint(msg::ENGINE_NOT_CONNECTABLE_HINT);
            return reg.set_status(&id, HostStatus::Invalid { error }, None);
        }
        reg.set_status(&id, HostStatus::Connecting, None);

        let secrets = if host.connection.needs_secrets() {
            match reg.store.secrets(&id) {
                Ok(secrets) => secrets,
                Err(e) if e.is(ErrorCode::CredentialsLost) => {
                    return reg.set_status(&id, HostStatus::CredentialsLost, None)
                }
                Err(error) if error.is(ErrorCode::KeychainUnavailable) => {
                    return reg.set_status(&id, HostStatus::KeychainUnavailable { error }, None)
                }
                Err(error) => return reg.set_status(&id, HostStatus::Invalid { error }, None),
            }
        } else {
            None
        };

        let error = match transport::connect(
            &host.name,
            &host.connection,
            secrets,
            host.host_key.as_deref(),
        )
        .await
        {
            Ok(connected) => match describe(&connected.docker).await {
                Ok(status) => {
                    backoff = 1;
                    let docker = connected.docker.clone();
                    // Podman sends events bollard can't read; read its own way.
                    let podman = status
                        .podman_version()
                        .is_some()
                        .then(|| connected.dialer.clone());
                    reg.set_status(&id, status, Some(Client::Docker(connected)));
                    drop(reg);
                    watch(&registry, &id, docker, podman).await;
                    AppError::new(
                        ErrorCode::ConnectionLost,
                        msg::CONNECTION_LOST.with("name", &host.name),
                    )
                    .with_hint(msg::CONNECTION_LOST_HINT)
                }
                Err(error) => error,
            },
            Err(ConnectError::Unreachable(error)) => error,
            Err(other) => return reg.set_status(&id, other.into(), None),
        };

        let Some(reg) = registry.upgrade() else {
            return;
        };
        reg.set_status(
            &id,
            HostStatus::Unreachable {
                error,
                retry_in_secs: backoff,
            },
            None,
        );
        drop(reg);
        tokio::time::sleep(Duration::from_secs(backoff.into())).await;
        backoff = next_backoff(backoff);
    }
}

async fn describe(docker: &Docker) -> Result<HostStatus, AppError> {
    let v = docker.version().await?;
    Ok(HostStatus::Connected {
        podman_version: podman_version(&v),
        engine_version: v.version.unwrap_or_default(),
        api_version: v.api_version.unwrap_or_default(),
        os: v.os.unwrap_or_default(),
        arch: v.arch.unwrap_or_default(),
        swarm: membership(docker).await,
    })
}

/// Podman's version, from the "Podman Engine" component its `/version`
/// lists beside the Docker-compatible fields; `None` for any other Engine.
fn podman_version(v: &bollard::models::SystemVersion) -> Option<String> {
    v.components
        .as_ref()?
        .iter()
        .find(|c| c.name.eq_ignore_ascii_case("podman engine"))
        .map(|c| c.version.clone())
        .filter(|version| !version.is_empty())
}

/// The Host's place in a Swarm; an Engine that can't say counts as outside one.
async fn membership(docker: &Docker) -> Option<swarm::SwarmMembership> {
    swarm::membership(docker).await.unwrap_or_else(|e| {
        log::debug!("couldn't read swarm membership: {e}");
        None
    })
}

/// One event as the Engine writes it, read leniently: Podman sends kinds
/// (`pod`, `system`) that bollard's event type refuses, which would end the
/// stream.
#[derive(Deserialize)]
struct RawEvent {
    #[serde(rename = "Type", default)]
    kind: String,
    #[serde(rename = "Action", default)]
    action: String,
    #[serde(rename = "Actor", default)]
    actor: Option<RawActor>,
    #[serde(default)]
    time: i64,
    #[serde(rename = "timeNano", default)]
    time_nano: Option<i64>,
}

#[derive(Deserialize)]
struct RawActor {
    #[serde(rename = "ID", default)]
    id: String,
    #[serde(rename = "Attributes", default)]
    attributes: Option<HashMap<String, String>>,
}

/// One line of the Engine's event stream; `None` for one that isn't an event.
fn parse_event(line: &[u8]) -> Option<EngineEvent> {
    let raw: RawEvent = serde_json::from_slice(line)
        .map_err(|e| log::debug!("skipping an event Homeport can't read: {e}"))
        .ok()?;
    let actor = raw.actor;
    Some(EngineEvent {
        time: raw.time,
        time_nano: raw.time_nano.unwrap_or(raw.time * 1_000_000_000),
        kind: raw.kind,
        action: raw.action,
        actor_id: actor.as_ref().map(|a| a.id.clone()).unwrap_or_default(),
        attributes: actor.and_then(|a| a.attributes).unwrap_or_default(),
    })
}

/// Forwards Engine events until the stream breaks or a ping fails. Asks
/// first for those since the last one kept, to fill the gap while Homeport
/// was closed or disconnected; repeats of kept ones are dropped on writing.
/// A Podman Host's events are read through `podman`, its dialer, line by
/// line, skipping what isn't an event.
async fn watch(
    registry: &Weak<HostRegistry>,
    id: &str,
    docker: Docker,
    podman: Option<Arc<dyn Dialer>>,
) {
    let since = registry.upgrade().map(|reg| {
        let last = reg.events.last_time_nano(id).unwrap_or_else(|e| {
            log::warn!("couldn't read the last kept event of {id}: {e}");
            None
        });
        events::since(last.unwrap_or_else(|| events::now_nano() - FIRST_LOOK_BACK_NANOS))
    });
    let podman_events = async {
        let Some(dialer) = &podman else {
            return;
        };
        let mut path = "/events".to_owned();
        if let Some(since) = &since {
            path.push_str(&format!("?since={}", http::encode(since)));
        }
        let mut lines = match http::lines(&**dialer, http::Request::get(path)).await {
            Ok(lines) => lines,
            Err(e) => return log::debug!("couldn't read the events of {id}: {e}"),
        };
        while let Some(Ok(line)) = lines.next().await {
            let Some(reg) = registry.upgrade() else {
                return;
            };
            if let Some(event) = parse_event(&line) {
                reg.record_event(id, event);
            }
        }
    };
    let events = async {
        if podman.is_some() {
            return podman_events.await;
        }
        let mut options = EventsOptionsBuilder::default();
        if let Some(since) = &since {
            options = options.since(since);
        }
        let mut stream = docker.events(Some(options.build()));
        while let Some(Ok(msg)) = stream.next().await {
            let Some(reg) = registry.upgrade() else {
                return;
            };
            let actor = msg.actor.unwrap_or_default();
            let kind = msg.typ.map(|t| t.to_string()).unwrap_or_default();
            if kind == "node" {
                reg.set_swarm(id, membership(&docker).await);
            }
            reg.record_event(
                id,
                EngineEvent {
                    time: msg.time.unwrap_or_default(),
                    time_nano: msg
                        .time_nano
                        .unwrap_or_else(|| msg.time.unwrap_or_default() * 1_000_000_000),
                    kind,
                    action: msg.action.unwrap_or_default(),
                    actor_id: actor.id.unwrap_or_default(),
                    attributes: actor.attributes.unwrap_or_default(),
                },
            );
        }
    };
    let pings = async {
        loop {
            tokio::time::sleep(PING_INTERVAL).await;
            match tokio::time::timeout(transport::CONNECT_TIMEOUT, docker.ping()).await {
                Ok(Ok(_)) => {}
                _ => return,
            }
            // A Swarm joined or left from a terminal sends no event on a worker.
            let swarm = membership(&docker).await;
            match registry.upgrade() {
                Some(reg) => reg.set_swarm(id, swarm),
                None => return,
            }
        }
    };
    tokio::select! {
        _ = events => {}
        _ = pings => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(json: serde_json::Value) -> bollard::models::SystemVersion {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn podman_says_so_in_its_components() {
        // Trimmed from Podman 5.8's GET /version.
        let podman = version(serde_json::json!({
            "Platform": { "Name": "linux/amd64/fedora-43" },
            "Components": [
                { "Name": "Podman Engine", "Version": "5.8.1", "Details": { "APIVersion": "5.8.1" } },
                { "Name": "Conmon", "Version": "conmon version 2.1.13", "Details": {} }
            ],
            "Version": "5.8.1",
            "ApiVersion": "1.41"
        }));
        assert_eq!(podman_version(&podman).as_deref(), Some("5.8.1"));
    }

    #[test]
    fn podmans_own_kinds_of_event_are_read() {
        // As Podman 4.9 and 5.x write them to /events.
        let pod = br#"{"status":"create","id":"c56f","Type":"pod","Action":"create","Actor":{"ID":"c56f","Attributes":{"name":"web"}},"scope":"local","time":1794162522,"timeNano":1794162522479016754}"#;
        let event = parse_event(pod).unwrap();
        assert_eq!(
            (event.kind.as_str(), event.action.as_str()),
            ("pod", "create")
        );
        assert_eq!(event.actor_id, "c56f");
        assert_eq!(
            event.attributes.get("name").map(String::as_str),
            Some("web")
        );
        assert_eq!(event.time_nano, 1_794_162_522_479_016_754);

        let system = br#"{"Type":"system","Action":"refresh","time":1794162500}"#;
        let event = parse_event(system).unwrap();
        assert_eq!(event.kind, "system");
        assert_eq!(event.time_nano, 1_794_162_500_000_000_000);
        assert!(event.actor_id.is_empty() && event.attributes.is_empty());

        assert!(parse_event(b"not json").is_none());
    }

    #[test]
    fn docker_has_no_podman_component() {
        let docker = version(serde_json::json!({
            "Components": [
                { "Name": "Engine", "Version": "29.0.1", "Details": {} },
                { "Name": "containerd", "Version": "v2.1.4", "Details": {} }
            ],
            "Version": "29.0.1",
            "ApiVersion": "1.52"
        }));
        assert_eq!(podman_version(&docker), None);
        assert_eq!(podman_version(&version(serde_json::json!({}))), None);
    }
}
