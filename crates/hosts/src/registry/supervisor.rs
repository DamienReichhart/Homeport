//! The task that keeps one Host connected: connect, watch, retry.

use std::{sync::Weak, time::Duration};

use bollard::{query_parameters::EventsOptions, Docker};
use futures_util::StreamExt;

use homeport_core::error::{AppError, ErrorCode};
use homeport_transport::{self as transport, ConnectError};

use super::{EngineEvent, HostRegistry, HostStatus};
use crate::HostId;

const PING_INTERVAL: Duration = Duration::from_secs(15);
const MAX_BACKOFF_SECS: u32 = 60;

/// The next wait after `current` seconds: doubling from 1 up to 60.
pub fn next_backoff(current: u32) -> u32 {
    (current.max(1) * 2).min(MAX_BACKOFF_SECS)
}

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
                            format!("Homeport lost the connection to {}.", host.name),
                        )
                        .with_hint("The Docker Engine or the network may have restarted. Homeport keeps retrying.")
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
    })
}

/// Forwards Engine events until the stream breaks or a ping fails.
async fn watch(registry: &Weak<HostRegistry>, id: &str, docker: Docker) {
    let events = async {
        let mut stream = docker.events(None::<EventsOptions>);
        while let Some(Ok(msg)) = stream.next().await {
            let Some(reg) = registry.upgrade() else {
                return;
            };
            let actor = msg.actor.unwrap_or_default();
            reg.record_event(
                id,
                EngineEvent {
                    time: msg.time.unwrap_or_default(),
                    kind: msg.typ.map(|t| t.to_string()).unwrap_or_default(),
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

    #[test]
    fn backoff_doubles_up_to_a_minute() {
        let waits: Vec<_> = std::iter::successors(Some(1), |&b| Some(next_backoff(b)))
            .take(8)
            .collect();
        assert_eq!(waits, [1, 2, 4, 8, 16, 32, 60, 60]);
    }
}
