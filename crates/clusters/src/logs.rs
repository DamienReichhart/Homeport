//! A Pod's logs, or several Pods' in one stream (a workload's), streamed in
//! batches like a container's: each line tagged with its Pod when there
//! are several.

use std::time::{SystemTime, UNIX_EPOCH};

use futures_util::{
    io::AsyncReadExt,
    stream::{self, select_all, BoxStream},
    AsyncBufRead, StreamExt,
};
use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, LogParams};
use serde::Deserialize;

use homeport_core::{
    error::{AppError, ErrorCode},
    logs::{LineSplitter, LogEvent, LogLine, LogOptions, BATCH_EVERY, BATCH_LINES},
};

use crate::{
    messages as msg,
    resources::{forbidden_on, ApiType, On},
};

/// The bytes read from a log stream at a time.
const CHUNK: usize = 16 * 1024;

/// A Pod to read the logs of.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodLogSource {
    pub pod: String,
    /// Shown in front of each line when several Pods share a stream.
    pub label: Option<String>,
}

/// What to read of each Pod, beyond the options every log viewer has.
#[derive(Debug, Clone, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodLogTarget {
    /// The container; the Pod's only one (or default) without.
    pub container: Option<String>,
    /// The instance that ran before the last restart, like `kubectl logs --previous`.
    pub previous: bool,
}

/// The query of `pods/log` for `options`, `now` being the Unix time.
pub fn log_params(options: &LogOptions, target: &PodLogTarget, now: i64) -> LogParams {
    LogParams {
        container: target.container.clone().filter(|c| !c.is_empty()),
        // A previous instance has ended: there's nothing to follow.
        follow: options.follow && !target.previous,
        previous: target.previous,
        tail_lines: options.tail.map(i64::from),
        since_seconds: options.since.map(|since| (now - i64::from(since)).max(1)),
        timestamps: options.timestamps,
        ..Default::default()
    }
}

/// The Unix time now.
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs().try_into().unwrap_or(i64::MAX))
}

type Chunks = BoxStream<'static, std::io::Result<Vec<u8>>>;

/// A log stream's bytes, a chunk at a time.
fn chunks(reader: impl AsyncBufRead + Send + Unpin + 'static) -> Chunks {
    stream::unfold(reader, |mut reader| async move {
        let mut buf = vec![0; CHUNK];
        match reader.read(&mut buf).await {
            Ok(0) => None,
            Ok(n) => {
                buf.truncate(n);
                Some((Ok(buf), reader))
            }
            Err(e) => Some((Err(e), reader)),
        }
    })
    .boxed()
}

impl On {
    /// Streams the logs of `sources` in `namespace` to `sink` until they
    /// end, the sink is gone (returns `false`) or the task is cancelled.
    pub async fn stream_logs(
        self,
        namespace: String,
        sources: Vec<PodLogSource>,
        target: PodLogTarget,
        options: LogOptions,
        sink: impl Fn(LogEvent) -> bool,
    ) {
        let api: Api<Pod> = Api::namespaced(self.client.clone(), &namespace);
        let params = log_params(&options, &target, now());
        let mut opened = Vec::with_capacity(sources.len());
        for source in sources {
            match api.log_stream(&source.pod, &params).await {
                Ok(reader) => opened.push((source.label, chunks(reader))),
                Err(e) => {
                    let error = self.log_failed(&namespace, &source.pod, &target, &e);
                    if !sink(LogEvent::Error { error }) {
                        return;
                    }
                }
            }
        }
        if !opened.is_empty() {
            pump(opened, sink).await;
        }
    }

    /// Why the logs of `pod` can't be read.
    fn log_failed(
        &self,
        namespace: &str,
        pod: &str,
        target: &PodLogTarget,
        error: &kube::Error,
    ) -> AppError {
        let container = target.container.as_deref().unwrap_or(pod);
        if let kube::Error::Api(status) = error {
            if status.code == 403 {
                return forbidden_on("get", &pod_logs(), Some(namespace))
                    .with_detail(&status.message);
            }
            if status.code == 400 {
                if let Some(error) = refused(pod, container, &status.message) {
                    return error;
                }
            }
        }
        self.failed_on("get", &pod_logs(), Some(namespace), Some(pod), error)
    }
}

/// The `pods/log` subresource, as RBAC names it.
fn pod_logs() -> ApiType {
    ApiType {
        group: String::new(),
        version: "v1".into(),
        kind: "Pod".into(),
        plural: "pods/log".into(),
        namespaced: true,
    }
}

/// What the API server's refusal to give logs means, when it's one of the
/// usual ones: no previous instance, or a container that hasn't started.
pub fn refused(pod: &str, container: &str, message: &str) -> Option<AppError> {
    if message.contains("previous terminated container") {
        return Some(
            AppError::new(
                ErrorCode::NotFound,
                msg::NO_PREVIOUS_LOGS
                    .with("container", container)
                    .with("pod", pod),
            )
            .with_hint(msg::NO_PREVIOUS_LOGS_HINT),
        );
    }
    if let Some((_, why)) = message.split_once("is waiting to start: ") {
        return Some(
            AppError::new(
                ErrorCode::ContainerNotRunning,
                msg::CONTAINER_WAITING
                    .with("container", container)
                    .with("reason", why.trim()),
            )
            .with_hint(msg::CONTAINER_WAITING_HINT),
        );
    }
    None
}

/// Forwards the lines of every stream, tagged with its label, in batches;
/// says when they all ended.
pub async fn pump(sources: Vec<(Option<String>, Chunks)>, sink: impl Fn(LogEvent) -> bool) {
    let labels: Vec<Option<String>> = sources.iter().map(|(l, _)| l.clone()).collect();
    let mut splitters: Vec<LineSplitter> =
        sources.iter().map(|_| LineSplitter::default()).collect();
    let mut merged = select_all(
        sources
            .into_iter()
            .enumerate()
            .map(|(i, (_, chunks))| chunks.map(move |c| (i, c)).boxed()),
    );
    let line = |i: usize, text: String| LogLine {
        source: labels[i].clone(),
        stream: "stdout".into(),
        text,
    };
    let mut batch = Vec::new();
    let mut ticker = tokio::time::interval(BATCH_EVERY);
    loop {
        tokio::select! {
            item = merged.next() => match item {
                Some((i, Ok(bytes))) => {
                    batch.extend(splitters[i].push(&bytes).into_iter().map(|t| line(i, t)));
                    if batch.len() >= BATCH_LINES
                        && !sink(LogEvent::Lines { lines: std::mem::take(&mut batch) })
                    {
                        return;
                    }
                }
                Some((_, Err(e))) => {
                    let error = AppError::from(e).logged("pod log stream");
                    if !sink(LogEvent::Error { error }) {
                        return;
                    }
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
    for (i, splitter) in splitters.iter_mut().enumerate() {
        if let Some(text) = splitter.finish() {
            batch.push(line(i, text));
        }
    }
    if !batch.is_empty() && !sink(LogEvent::Lines { lines: batch }) {
        return;
    }
    sink(LogEvent::End);
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    fn options() -> LogOptions {
        LogOptions {
            tail: Some(100),
            timestamps: true,
            since: Some(1_000),
            stdout: true,
            stderr: true,
            follow: true,
        }
    }

    #[test]
    fn options_become_the_query_of_pods_log() {
        let target = PodLogTarget {
            container: Some("api".into()),
            previous: false,
        };
        let p = log_params(&options(), &target, 1_600);
        assert_eq!(p.container.as_deref(), Some("api"));
        assert!(p.follow && p.timestamps && !p.previous);
        assert_eq!(p.tail_lines, Some(100));
        assert_eq!(p.since_seconds, Some(600));
    }

    #[test]
    fn a_previous_instance_isnt_followed() {
        let target = PodLogTarget {
            container: Some(String::new()),
            previous: true,
        };
        let p = log_params(
            &LogOptions {
                tail: None,
                since: None,
                ..options()
            },
            &target,
            0,
        );
        assert!(p.previous && !p.follow);
        assert_eq!(p.container, None);
        assert_eq!((p.tail_lines, p.since_seconds), (None, None));
    }

    #[test]
    fn the_usual_refusals_are_explained() {
        let e = refused(
            "api-1",
            "api",
            "previous terminated container \"api\" in pod \"api-1\" not found",
        )
        .unwrap();
        assert_eq!(e.code, ErrorCode::NotFound);
        assert_eq!(e.key, "errors.clusters.logs.noPrevious");
        let e = refused(
            "api-1",
            "api",
            "container \"api\" in pod \"api-1\" is waiting to start: CrashLoopBackOff",
        )
        .unwrap();
        assert_eq!(e.code, ErrorCode::ContainerNotRunning);
        assert!(e.message.contains("CrashLoopBackOff"));
        assert!(refused("api-1", "api", "something else").is_none());
    }

    fn source(label: &str, chunks: &[&str]) -> (Option<String>, Chunks) {
        let chunks: Vec<std::io::Result<Vec<u8>>> =
            chunks.iter().map(|c| Ok(c.as_bytes().to_vec())).collect();
        (Some(label.into()), stream::iter(chunks).boxed())
    }

    #[tokio::test]
    async fn several_pods_interleave_their_lines_each_with_its_label() {
        let events = Arc::new(Mutex::new(vec![]));
        let sink = {
            let events = events.clone();
            move |e| {
                events.lock().unwrap().push(e);
                true
            }
        };
        pump(
            vec![
                source("api-1", &["one\ntw", "o\n"]),
                source("api-2", &["three\nfour"]),
            ],
            sink,
        )
        .await;
        let events = events.lock().unwrap();
        let mut lines: Vec<(String, String)> = events
            .iter()
            .flat_map(|e| match e {
                LogEvent::Lines { lines } => lines.clone(),
                _ => vec![],
            })
            .map(|l| (l.source.unwrap(), l.text))
            .collect();
        lines.sort();
        assert_eq!(
            lines,
            [
                ("api-1".to_owned(), "one".to_owned()),
                ("api-1".to_owned(), "two".to_owned()),
                ("api-2".to_owned(), "four".to_owned()),
                ("api-2".to_owned(), "three".to_owned()),
            ]
        );
        assert_eq!(events.last(), Some(&LogEvent::End));
    }
}
