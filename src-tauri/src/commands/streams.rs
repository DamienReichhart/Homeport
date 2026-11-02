use tauri::{ipc::Channel, State};

use crate::{
    error::AppResult,
    services::{
        docker::logs::{self, LogEvent, LogOptions, LogSource},
        streams::StreamId,
    },
    state::AppState,
};

/// Stops a stream started by any `*_subscribe` command.
#[tauri::command]
#[specta::specta]
pub fn stream_cancel(state: State<'_, AppState>, id: StreamId) {
    state.streams.cancel(id);
}

#[tauri::command]
#[specta::specta]
pub fn logs_subscribe(
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
