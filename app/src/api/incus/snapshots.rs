//! Snapshots of Incus Instances.

use homeport_core::error::AppResult;
use homeport_incus::instances::snapshots::{self, IncusSnapshot, SnapshotRequest};

use crate::app::AppState;

/// The snapshots of an Instance, oldest first.
pub async fn incus_snapshots(
    state: &AppState,
    host_id: String,
    project: String,
    instance: String,
) -> AppResult<Vec<IncusSnapshot>> {
    snapshots::list(&state.hosts.registry.incus(&host_id)?, &project, &instance).await
}

/// Takes a snapshot of an Instance, stateful and expiring if asked.
pub async fn incus_snapshot_create(
    state: &AppState,
    host_id: String,
    project: String,
    instance: String,
    request: SnapshotRequest,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::create(&incus, &project, &instance, &request).await
}

/// Restores an Instance to one of its snapshots; what changed since is lost.
pub async fn incus_snapshot_restore(
    state: &AppState,
    host_id: String,
    project: String,
    instance: String,
    snapshot: String,
    stateful: bool,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::restore(&incus, &project, &instance, &snapshot, stateful).await
}

pub async fn incus_snapshot_delete(
    state: &AppState,
    host_id: String,
    project: String,
    instance: String,
    snapshot: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    snapshots::delete(&incus, &project, &instance, &snapshot).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::incus::tests::{not_connected, NO_HOST},
        app::testing,
    };

    fn host() -> String {
        NO_HOST.to_owned()
    }

    fn project() -> String {
        "default".to_owned()
    }

    #[tokio::test]
    async fn snapshots_need_a_connected_host() {
        let app = testing::app();
        let state = &app.state;
        let instance = || "web".to_owned();
        let snapshot = || "snap0".to_owned();
        not_connected(incus_snapshots(state, host(), project(), instance()).await);
        let request = SnapshotRequest {
            name: String::new(),
            stateful: false,
            expires_at: None,
        };
        not_connected(incus_snapshot_create(state, host(), project(), instance(), request).await);
        not_connected(
            incus_snapshot_restore(state, host(), project(), instance(), snapshot(), false).await,
        );
        not_connected(
            incus_snapshot_delete(state, host(), project(), instance(), snapshot()).await,
        );
    }
}
