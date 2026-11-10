//! Container logs, streamed in batches. Several containers can share one
//! stream (a Stack's services), each line tagged with its source.

use bollard::{container::LogOutput, query_parameters::LogsOptionsBuilder, Docker};
use futures_util::{stream::select_all, StreamExt};
use serde::Deserialize;

pub use homeport_core::logs::{LineSplitter, LogEvent, LogLine, LogOptions};
use homeport_core::logs::{BATCH_EVERY, BATCH_LINES};

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogSource {
    pub container_id: String,
    /// Shown in front of each line when several sources share a stream.
    pub label: Option<String>,
}

/// Streams logs of `sources` to `sink` until they end, the sink is gone
/// (returns `false`) or the task is cancelled.
pub async fn stream(
    docker: Docker,
    sources: Vec<LogSource>,
    options: LogOptions,
    sink: impl Fn(LogEvent) -> bool,
) {
    let mut builder = LogsOptionsBuilder::default()
        .stdout(options.stdout)
        .stderr(options.stderr)
        .timestamps(options.timestamps)
        .follow(options.follow)
        .tail(&options.tail.map_or("all".to_string(), |t| t.to_string()));
    if let Some(since) = options.since {
        builder = builder.since(since);
    }
    let logs_options = builder.build();

    let streams = sources.iter().enumerate().map(|(index, source)| {
        docker
            .logs(&source.container_id, Some(logs_options.clone()))
            .map(move |item| (index, item))
            .boxed()
    });
    let mut merged = select_all(streams);
    let mut splitters: Vec<(LineSplitter, String)> = sources
        .iter()
        .map(|_| (LineSplitter::default(), String::new()))
        .collect();
    let labels: Vec<Option<String>> = sources.into_iter().map(|s| s.label).collect();

    let mut batch = Vec::new();
    let mut ticker = tokio::time::interval(BATCH_EVERY);
    loop {
        tokio::select! {
            item = merged.next() => match item {
                Some((index, Ok(output))) => {
                    let (stream_name, bytes) = match output {
                        LogOutput::StdOut { message } => ("stdout", message),
                        LogOutput::StdErr { message } => ("stderr", message),
                        LogOutput::Console { message } => ("console", message),
                        LogOutput::StdIn { .. } => continue,
                    };
                    let (splitter, last_stream) = &mut splitters[index];
                    *last_stream = stream_name.to_owned();
                    batch.extend(splitter.push(&bytes).into_iter().map(|text| LogLine {
                        source: labels[index].clone(),
                        stream: stream_name.to_owned(),
                        text,
                    }));
                    if batch.len() >= BATCH_LINES && !sink(LogEvent::Lines { lines: std::mem::take(&mut batch) }) {
                        return;
                    }
                }
                Some((_, Err(e))) => {
                    sink(LogEvent::Error { error: homeport_core::error::AppError::from(e).logged("log stream") });
                    break;
                }
                None => break,
            },
            _ = ticker.tick() => {
                if !batch.is_empty() && !sink(LogEvent::Lines { lines: std::mem::take(&mut batch) }) {
                    return;
                }
            }
        }
    }
    for (index, (splitter, stream_name)) in splitters.iter_mut().enumerate() {
        if let Some(text) = splitter.finish() {
            batch.push(LogLine {
                source: labels[index].clone(),
                stream: stream_name.clone(),
                text,
            });
        }
    }
    if !batch.is_empty() {
        sink(LogEvent::Lines { lines: batch });
    }
    sink(LogEvent::End);
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::{containers, testing};

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn logs_keep_stdout_and_stderr_apart() {
        let docker = testing::docker();
        let c = testing::container(
            &docker,
            &testing::unique("logs"),
            &["sh", "-c", "echo one; echo two >&2"],
        )
        .await;
        containers::act(&docker, &c.id, containers::ContainerAction::Start)
            .await
            .unwrap();
        docker
            .wait_container(
                &c.id,
                None::<bollard::query_parameters::WaitContainerOptions>,
            )
            .next()
            .await;

        let events = Arc::new(Mutex::new(Vec::new()));
        let sink = {
            let events = events.clone();
            move |e| {
                events.lock().unwrap().push(e);
                true
            }
        };
        let options = LogOptions {
            tail: None,
            timestamps: false,
            since: None,
            stdout: true,
            stderr: true,
            follow: false,
        };
        let source = LogSource {
            container_id: c.id.clone(),
            label: Some("app".into()),
        };
        stream(docker.clone(), vec![source], options, sink).await;

        let events = events.lock().unwrap();
        let lines: Vec<_> = events
            .iter()
            .flat_map(|e| match e {
                LogEvent::Lines { lines } => lines.clone(),
                _ => vec![],
            })
            .map(|l| (l.stream, l.text, l.source))
            .collect();
        assert!(lines.contains(&("stdout".into(), "one".into(), Some("app".into()))));
        assert!(lines.contains(&("stderr".into(), "two".into(), Some("app".into()))));
        assert_eq!(events.last(), Some(&LogEvent::End));
    }
}
