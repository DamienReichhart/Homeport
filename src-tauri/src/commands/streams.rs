use homeport_core::streams::StreamId;
use tauri::{ipc::Channel, State};

use homeport_engine::{
    images::{self, PullEvent},
    logs::{self, LogEvent, LogOptions, LogSource},
    stats::{self, ContainerUsage, StatsSample},
};

use crate::{error::AppResult, state::AppState};

/// Stops a stream started by any `*_subscribe` command.
#[tauri::command]
#[specta::specta]
pub fn stream_cancel(state: State<'_, AppState>, id: StreamId) {
    state.streams.cancel(id);
}

#[tauri::command]
#[specta::specta]
pub async fn logs_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    sources: Vec<LogSource>,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(logs::stream(docker, sources, options, move |e| {
            on_event.send(e).is_ok()
        })))
}

#[tauri::command]
#[specta::specta]
pub async fn stats_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    on_sample: Channel<StatsSample>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(stats::stream(docker, container_id, move |s| {
            on_sample.send(s).is_ok()
        })))
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

#[tauri::command]
#[specta::specta]
pub async fn images_pull_subscribe(
    state: State<'_, AppState>,
    host_id: String,
    reference: String,
    on_event: Channel<PullEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(images::pull(docker, reference, move |e| {
            on_event.send(e).is_ok()
        })))
}
