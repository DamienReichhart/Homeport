//! The task that keeps one Host connected: connect, watch, retry.

use std::{sync::Weak, time::Duration};

use bollard::{query_parameters::EventsOptionsBuilder, Docker};
use futures_util::StreamExt;

use homeport_core::error::{AppError, ErrorCode};
use homeport_engine::swarm;

use crate::messages as msg;
use homeport_transport::{self as transport, ConnectError};

use super::{EngineEvent, HostRegistry, HostStatus};
use crate::{events, HostId};

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
                    reg.set_status(&id, status, Some(connected));
                    drop(reg);
                    watch(&registry, &id, docker).await;
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
        engine_version: v.version.unwrap_or_default(),
        api_version: v.api_version.unwrap_or_default(),
        os: v.os.unwrap_or_default(),
        arch: v.arch.unwrap_or_default(),
        swarm: membership(docker).await,
    })
}

/// The Host's place in a Swarm; an Engine that can't say counts as outside one.
async fn membership(docker: &Docker) -> Option<swarm::SwarmMembership> {
    swarm::membership(docker).await.unwrap_or_else(|e| {
        log::debug!("couldn't read swarm membership: {e}");
        None
    })
}

/// Forwards Engine events until the stream breaks or a ping fails. Asks
/// first for those since the last one kept, to fill the gap while Homeport
/// was closed or disconnected; repeats of kept ones are dropped on writing.
async fn watch(registry: &Weak<HostRegistry>, id: &str, docker: Docker) {
    let since = registry.upgrade().map(|reg| {
        let last = reg.events.last_time_nano(id).unwrap_or_else(|e| {
            log::warn!("couldn't read the last kept event of {id}: {e}");
            None
        });
        events::since(last.unwrap_or_else(|| events::now_nano() - FIRST_LOOK_BACK_NANOS))
    });
    let events = async {
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
