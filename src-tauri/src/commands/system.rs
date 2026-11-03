//! A Host's Engine as a whole: what it runs, the disk it uses, and pruning.

use homeport_core::streams::StreamId;
use homeport_engine::{
    containers::stats::{self, ContainerUsage},
    system::{self, DiskUsage, EngineInfo, PruneSelection, SystemPruneReport},
};
use tauri::{ipc::Channel, State};

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn system_info(state: State<'_, AppState>, host_id: String) -> AppResult<EngineInfo> {
    system::info(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn system_disk_usage(
    state: State<'_, AppState>,
    host_id: String,
) -> AppResult<DiskUsage> {
    system::disk_usage(&state.hosts.registry.docker(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn system_prune(
    state: State<'_, AppState>,
    host_id: String,
    selection: PruneSelection,
) -> AppResult<SystemPruneReport> {
    system::prune(&state.hosts.registry.docker(&host_id)?, selection).await
}

/// Usage of every running container on a Host, refreshed every two seconds.
#[tauri::command]
#[specta::specta]
pub async fn usage_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    on_usage: Channel<Vec<ContainerUsage>>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(stats::overview(docker, move |u| on_usage.send(u).is_ok())))
}
