//! A Host's Engine as a whole: what it runs, the disk it uses, and pruning.

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_engine::{
    containers::stats::{self, ContainerUsage},
    system::{self, DiskUsage, EngineInfo, PruneSelection, SystemPruneReport},
};

use crate::app::AppState;

pub async fn system_info(state: &AppState, host_id: String) -> AppResult<EngineInfo> {
    system::info(&state.hosts.registry.docker(&host_id)?).await
}

pub async fn system_disk_usage(state: &AppState, host_id: String) -> AppResult<DiskUsage> {
    system::disk_usage(&state.hosts.registry.docker(&host_id)?).await
}

pub async fn system_prune(
    state: &AppState,
    host_id: String,
    selection: PruneSelection,
) -> AppResult<SystemPruneReport> {
    system::prune(&state.hosts.registry.docker(&host_id)?, selection).await
}

/// Usage of every running container on a Host, refreshed every two seconds.
pub async fn usage_subscribe(
    state: &AppState,
    host_id: String,
    on_usage: impl Fn(Vec<ContainerUsage>) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state.streams.spawn(stats::overview(docker, on_usage)))
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, host};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_host_that_isnt_connected_is_refused() {
        let app = app();
        let s = &app.state;
        let everything = PruneSelection {
            containers: true,
            images: true,
            all_images: true,
            networks: true,
            volumes: true,
            build_cache: true,
        };
        let results = [
            ("info", code(system_info(s, host()).await)),
            ("disk_usage", code(system_disk_usage(s, host()).await)),
            ("prune", code(system_prune(s, host(), everything).await)),
            ("usage", code(usage_subscribe(s, host(), |_| true).await)),
        ];
        assert_all(ErrorCode::HostNotConnected, &results);
    }
}
