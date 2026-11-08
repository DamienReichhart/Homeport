//! The task that keeps one Cluster connected: connect, check, retry.

use std::{sync::Weak, time::Duration};

use homeport_core::{
    backoff::next_backoff,
    error::{AppError, ErrorCode},
};

use super::{ClusterRegistry, ClusterStatus};
use crate::{classify, messages as msg, model::ClusterId};

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

        let error = match reg.connect(&cluster, &secrets).await {
            Ok((client, status)) => {
                backoff = 1;
                reg.set_status(&id, status, Some(client.clone()));
                drop(reg);
                check(&registry, &cluster, &secrets, client).await
            }
            Err(error) => error,
        };

        let Some(reg) = registry.upgrade() else {
            return;
        };
        if classify::needs_user(&error) {
            let status = if error.is(ErrorCode::ClusterUnauthorized) {
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

/// Asks the API server its version every 15 s until it stops answering, and
/// returns why. Exec credentials about to expire are renewed on the way.
async fn check(
    registry: &Weak<ClusterRegistry>,
    cluster: &crate::model::Cluster,
    secrets: &crate::model::ClusterSecrets,
    mut client: kube::Client,
) -> AppError {
    loop {
        tokio::time::sleep(CHECK_INTERVAL).await;
        let Some(reg) = registry.upgrade() else {
            return lost(&cluster.name);
        };
        match reg.renewed(cluster, secrets).await {
            Ok(Some(renewed)) => client = renewed,
            Ok(None) => {}
            Err(error) => return error,
        }
        drop(reg);
        if let Err(error) = super::version(cluster, &client).await {
            return error;
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
