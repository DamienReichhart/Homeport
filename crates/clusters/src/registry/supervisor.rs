//! The task that keeps one Cluster connected: connect, check, retry.

use std::{sync::Weak, time::Duration};

use homeport_core::{
    backoff::next_backoff,
    error::{AppError, ErrorCode},
};

use super::{ClusterClient, ClusterRegistry, ClusterStatus};
use crate::{
    classify, messages as msg,
    model::{Cluster, ClusterId, ClusterKind, ClusterSecrets},
};

const CHECK_INTERVAL: Duration = Duration::from_secs(15);

pub(super) async fn supervise(registry: Weak<ClusterRegistry>, id: ClusterId) {
    let mut backoff = 1;
    loop {
        let Some(reg) = registry.upgrade() else {
            return;
        };
        let cluster = match reg.store.get(&id) {
            Ok(cluster) => cluster,
            Err(_) => return reg.remove(&id),
        };
        if !cluster.enabled {
            return reg.set_status(&id, ClusterStatus::Disabled, None);
        }
        reg.set_status(&id, ClusterStatus::Connecting, None);

        let secrets = match reg.store.secrets(&id) {
            Ok(secrets) => secrets,
            Err(e) if e.is(ErrorCode::CredentialsLost) => {
                return reg.set_status(&id, ClusterStatus::CredentialsLost, None)
            }
            Err(error) if error.is(ErrorCode::KeychainUnavailable) => {
                return reg.set_status(&id, ClusterStatus::KeychainUnavailable { error }, None)
            }
            Err(error) => return reg.set_status(&id, ClusterStatus::Invalid { error }, None),
        };

        let (error, was_connected) = match reg.connect(&cluster, &secrets).await {
            Ok((client, status)) => {
                backoff = 1;
                reg.set_status(&id, status, Some(client.clone()));
                drop(reg);
                (check(&registry, &cluster, &secrets, client).await, true)
            }
            Err(error) => (error, false),
        };

        let Some(reg) = registry.upgrade() else {
            return;
        };
        // An exec plugin's token refused while connected: connecting again
        // asks the plugin for a new one before giving up.
        if was_connected && error.is(ErrorCode::ClusterUnauthorized) && has_exec_plugin(&secrets) {
            continue;
        }
        let needs_user = match cluster.kind {
            ClusterKind::Kubernetes => classify::needs_user(&error),
            ClusterKind::Nomad => homeport_nomad::error::needs_user(&error),
        };
        if needs_user {
            let status = if error.is(ErrorCode::ClusterUnauthorized)
                || error.is(ErrorCode::ExecPluginInteractive)
                || error.is(ErrorCode::NomadTokenInvalid)
            {
                ClusterStatus::Unauthorized { error }
            } else {
                ClusterStatus::Invalid { error }
            };
            return reg.set_status(&id, status, None);
        }
        reg.set_status(
            &id,
            ClusterStatus::Unreachable {
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

/// Whether a Kubernetes Cluster's credentials come from an exec plugin.
fn has_exec_plugin(secrets: &ClusterSecrets) -> bool {
    matches!(secrets, ClusterSecrets::Kubernetes(s) if s.exec.is_some())
}

/// Asks the API its version every 15 s until it stops answering, and
/// returns why. Credentials about to expire are renewed on the way.
async fn check(
    registry: &Weak<ClusterRegistry>,
    cluster: &Cluster,
    secrets: &ClusterSecrets,
    mut client: ClusterClient,
) -> AppError {
    loop {
        tokio::time::sleep(CHECK_INTERVAL).await;
        let Some(reg) = registry.upgrade() else {
            return lost(&cluster.name);
        };
        match reg.check(cluster, secrets, &client).await {
            Ok(Some(renewed)) => client = renewed,
            Ok(None) => {}
            Err(error) => return error,
        }
    }
}

fn lost(name: &str) -> AppError {
    AppError::new(
        ErrorCode::ConnectionLost,
        msg::CONNECTION_LOST.with("name", name),
    )
    .with_hint(msg::CONNECTION_LOST_HINT)
}
