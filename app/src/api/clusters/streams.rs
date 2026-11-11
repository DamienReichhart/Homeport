//! What streams from a Cluster's Pods: their logs, terminals in their
//! containers, and their Nodes' and their CPU and memory.

use homeport_core::{
    error::AppResult,
    logs::{LogEvent, LogOptions},
    streams::StreamId,
    terminal::ExecEvent,
};
use homeport_kubernetes::{
    metrics::{MetricsEvent, MetricsTarget},
    streams::{
        logs::{PodLogSource, PodLogTarget},
        terminal::{PodExecOptions, TerminalId},
    },
};

use super::on;
use crate::app::AppState;

/// Streams the logs of one Pod, or of several in one stream with each
/// line labelled, until cancelled or they end.
pub async fn clusters_pod_logs_subscribe(
    state: &AppState,
    id: String,
    namespace: String,
    sources: Vec<PodLogSource>,
    target: PodLogTarget,
    options: LogOptions,
    on_event: impl Fn(LogEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let on = on(state, &id)?;
    Ok(state
        .streams
        .spawn(on.stream_logs(namespace, sources, target, options, on_event)))
}

/// Polls the Metrics API for `target` every 15 s while its page is open,
/// until cancelled; says once and ends when metrics-server isn't there.
pub async fn clusters_metrics_subscribe(
    state: &AppState,
    id: String,
    target: MetricsTarget,
    on_event: impl Fn(MetricsEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let on = on(state, &id)?;
    Ok(state.streams.spawn(on.stream_metrics(target, on_event)))
}

/// Opens a terminal in a Pod's container, like `kubectl exec -it`.
pub async fn clusters_exec_start(
    state: &AppState,
    id: String,
    namespace: String,
    pod: String,
    options: PodExecOptions,
    on_event: impl Fn(ExecEvent) -> bool + Send + Sync + 'static,
) -> AppResult<TerminalId> {
    let (terminal, process) = on(state, &id)?.exec(&namespace, &pod, options).await?;
    Ok(state.terminals.pods.open(terminal, process, on_event))
}

pub async fn clusters_exec_input(
    state: &AppState,
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

pub async fn clusters_exec_resize(
    state: &AppState,
    terminal: TerminalId,
    cols: u16,
    rows: u16,
) -> AppResult<()> {
    state.terminals.pods.get(terminal)?.resize(cols, rows).await
}

/// Ends a Pod terminal and its process.
pub fn clusters_exec_close(state: &AppState, terminal: TerminalId) {
    state.terminals.pods.close(terminal);
}

#[cfg(test)]
mod tests {
    use super::super::tests::{gone, unknown};
    use super::*;
    use crate::app::testing::app;

    #[tokio::test]
    async fn nothing_streams_from_an_unknown_cluster() {
        let app = app();
        let state = &app.state;
        let sources = vec![PodLogSource {
            pod: "web-0".into(),
            label: None,
        }];
        let options = LogOptions {
            tail: Some(10),
            timestamps: false,
            since: None,
            stdout: true,
            stderr: true,
            follow: true,
        };
        let logs = clusters_pod_logs_subscribe(
            state,
            unknown(),
            "default".into(),
            sources,
            PodLogTarget::default(),
            options,
            |_| panic!("an unknown Cluster's Pod logged"),
        );
        assert!(gone(logs.await));
        let metrics = clusters_metrics_subscribe(state, unknown(), MetricsTarget::Nodes, |_| {
            panic!("an unknown Cluster has metrics")
        });
        assert!(gone(metrics.await));
        let exec = clusters_exec_start(
            state,
            unknown(),
            "default".into(),
            "web-0".into(),
            PodExecOptions::default(),
            |_| panic!("an unknown Cluster's terminal wrote"),
        );
        assert!(gone(exec.await));
    }

    #[tokio::test]
    async fn a_terminal_that_ended_takes_no_input() {
        let app = app();
        let state = &app.state;
        assert!(gone(clusters_exec_input(state, 3, "ls\n".into()).await));
        assert!(gone(clusters_exec_resize(state, 3, 80, 24).await));
        // Closing it again does nothing.
        clusters_exec_close(state, 3);
    }
}
