//! Container logs, streamed in batches. Several containers can share one
//! stream (a Stack's services), each line tagged with its source.

use std::time::Duration;

use bollard::{container::LogOutput, query_parameters::LogsOptionsBuilder, Docker};
use futures_util::{stream::select_all, StreamExt};
use serde::{Deserialize, Serialize};

const BATCH_LINES: usize = 500;
const BATCH_EVERY: Duration = Duration::from_millis(50);

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogOptions {
    /// Lines of history to start with; `None` for all of it.
    pub tail: Option<u32>,
    pub timestamps: bool,
    /// Only lines newer than this Unix time (seconds).
    pub since: Option<i32>,
    pub stdout: bool,
    pub stderr: bool,
    pub follow: bool,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogSource {
    pub container_id: String,
    /// Shown in front of each line when several sources share a stream.
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    pub source: Option<String>,
    /// `stdout`, `stderr` or `console` (TTY containers).
    pub stream: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LogEvent {
    Lines {
        lines: Vec<LogLine>,
    },
    Error {
        error: homeport_core::error::AppError,
    },
    End,
}

/// Splits a byte stream into lines, keeping partial lines until completed so
/// multi-byte characters and TTY chunks are never cut.
#[derive(Default)]
pub struct LineSplitter {
    pending: Vec<u8>,
}

impl LineSplitter {
    pub fn push(&mut self, bytes: &[u8]) -> Vec<String> {
        self.pending.extend_from_slice(bytes);
        let mut lines = Vec::new();
        while let Some(end) = self.pending.iter().position(|&b| b == b'\n') {
            let line: Vec<u8> = self.pending.drain(..=end).collect();
            lines.push(decode(&line[..line.len() - 1]));
        }
        lines
    }

    /// Whatever is left after the stream ends.
    pub fn finish(&mut self) -> Option<String> {
        (!self.pending.is_empty()).then(|| decode(&std::mem::take(&mut self.pending)))
    }
}

fn decode(line: &[u8]) -> String {
    let line = line.strip_suffix(b"\r").unwrap_or(line);
    String::from_utf8_lossy(line).into_owned()
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

    #[test]
    fn lines_survive_arbitrary_chunking() {
        let mut s = LineSplitter::default();
        assert!(s.push(b"hel").is_empty());
        assert_eq!(s.push(b"lo\r\nwor"), ["hello"]);
        // "é" is two bytes; split it across chunks.
        assert_eq!(s.push(&[b'l', b'd', 0xC3]), Vec::<String>::new());
        assert_eq!(s.push(&[0xA9, b'\n', b'\n']), ["worldé", ""]);
        assert_eq!(s.finish(), None);
        s.push(b"tail");
        assert_eq!(s.finish().as_deref(), Some("tail"));
    }

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
