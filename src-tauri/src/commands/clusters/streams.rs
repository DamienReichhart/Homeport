//! What streams from a Cluster's Pods: their logs, terminals in their
//! containers, and their Nodes' and their CPU and memory.

use tauri::{ipc::Channel, State};

use homeport_clusters::{
    logs::{PodLogSource, PodLogTarget},
    metrics::{MetricsEvent, MetricsTarget},
    terminal::{PodExecOptions, TerminalId},
};
use homeport_core::{
    logs::{LogEvent, LogOptions},
    streams::StreamId,
    terminal::ExecEvent,
};

use super::on;
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

/// Polls the Metrics API for `target` every 15 s while its page is open,
/// until cancelled; says once and ends when metrics-server isn't there.
#[tauri::command]
#[specta::specta]
pub async fn clusters_metrics_subscribe(
    state: State<'_, AppState>,
    id: String,
    target: MetricsTarget,
    on_event: Channel<MetricsEvent>,
) -> AppResult<StreamId> {
    let on = on(&state, &id)?;
    Ok(state
        .streams
        .spawn(on.stream_metrics(target, move |e| on_event.send(e).is_ok())))
}

/// Opens a terminal in a Pod's container, like `kubectl exec -it`.
#[tauri::command]
#[specta::specta]
pub async fn clusters_exec_start(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    pod: String,
    options: PodExecOptions,
    on_event: Channel<ExecEvent>,
) -> AppResult<TerminalId> {
    let (terminal, process) = on(&state, &id)?.exec(&namespace, &pod, options).await?;
    Ok(state
        .terminals
        .pods
        .open(terminal, process, move |e| on_event.send(e).is_ok()))
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_exec_input(
    state: State<'_, AppState>,
    terminal: TerminalId,
    data: String,
) -> AppResult<()> {
    state
        .terminals
        .pods
        .get(terminal)?
        .write(data.as_bytes())
        .await
}

#[tauri::command]
#[specta::specta]
pub async fn clusters_exec_resize(
    state: State<'_, AppState>,
    terminal: TerminalId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state.terminals.pods.get(terminal)?.resize(cols, rows).await
}

/// Ends a Pod terminal and its process.
#[tauri::command]
#[specta::specta]
pub fn clusters_exec_close(state: State<'_, AppState>, terminal: TerminalId) {
    state.terminals.pods.close(terminal);
}
