//! What streams from a Cluster's Pods: their logs.

use tauri::{ipc::Channel, State};

use homeport_clusters::logs::{PodLogSource, PodLogTarget};
use homeport_core::{
    logs::{LogEvent, LogOptions},
    streams::StreamId,
};

use super::clusters::on;
use crate::{app::AppState, error::AppResult};

/// Streams the logs of one Pod, or of several in one stream with each
/// line labelled, until cancelled or they end.
#[tauri::command]
#[specta::specta]
pub async fn clusters_pod_logs_subscribe(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    sources: Vec<PodLogSource>,
    target: PodLogTarget,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let on = on(&state, &id)?;
    Ok(state.streams.spawn(
        on.stream_logs(namespace, sources, target, options, move |e| {
            on_event.send(e).is_ok()
        }),
    ))
}
