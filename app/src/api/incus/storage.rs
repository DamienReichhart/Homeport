//! Incus storage pools and their volumes.

use homeport_core::error::AppResult;
use homeport_incus::storage::{self, IncusStoragePool, IncusVolume, VolumeRequest};

use crate::app::AppState;

/// Every storage pool, with how full it is.
pub async fn incus_storage_pools(
    state: &AppState,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusStoragePool>> {
    storage::pools(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// The volumes of a pool in `project`, with what uses each.
pub async fn incus_storage_volumes(
    state: &AppState,
    host_id: String,
    project: String,
    pool: String,
) -> AppResult<Vec<IncusVolume>> {
    storage::volumes(&state.hosts.registry.incus(&host_id)?, &project, &pool).await
}

pub async fn incus_volume_create(
    state: &AppState,
    host_id: String,
    project: String,
    pool: String,
    request: VolumeRequest,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    storage::create_volume(&incus, &project, &pool, &request).await
}

/// Deletes a custom volume nothing uses.
pub async fn incus_volume_delete(
    state: &AppState,
    host_id: String,
    project: String,
    pool: String,
    name: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    storage::delete_volume(&incus, &project, &pool, &name).await
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
    async fn storage_needs_a_connected_host() {
        let app = testing::app();
        let state = &app.state;
        let pool = || "default".to_owned();
        not_connected(incus_storage_pools(state, host(), project()).await);
        not_connected(incus_storage_volumes(state, host(), project(), pool()).await);
        let request = VolumeRequest {
            name: "data".into(),
            size: Some("10GiB".into()),
            content_type: "filesystem".into(),
        };
        not_connected(incus_volume_create(state, host(), project(), pool(), request).await);
        not_connected(incus_volume_delete(state, host(), project(), pool(), "data".into()).await);
    }
}
