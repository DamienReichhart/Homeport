//! A task's logs, streamed from Nomad: the servers forward the request to
//! the client node running the allocation, which answers with framed JSON
//! (`{"Offset": …, "Data": "<base64>"}`, and `{}` as a heartbeat). Each
//! of stdout and stderr is a stream of its own; both go to one sink.

use base64::Engine as _;
use reqwest::Method;
use serde::Deserialize;
use tokio::sync::mpsc;

use homeport_core::{
    error::AppError,
    logs::{LineSplitter, LogEvent, LogLine, LogOptions, BATCH_EVERY, BATCH_LINES},
};

use crate::{client::segment, messages as msg, Nomad};

/// Bytes read back from the end of a log per line of history asked for.
const BYTES_PER_LINE: u64 = 256;
/// The most history read back from the end of a log.
const MAX_HISTORY: u64 = 2 * 1024 * 1024;

/// A frame of Nomad's log stream.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
struct Frame {
    data: Option<String>,
}

/// Turns the bytes of Nomad's framed log stream into the log's own bytes,
/// keeping a frame split across chunks until it's complete.
#[derive(Default)]
pub struct FrameDecoder {
    pending: Vec<u8>,
}

impl FrameDecoder {
    pub fn push(&mut self, bytes: &[u8]) -> Vec<u8> {
        self.pending.extend_from_slice(bytes);
        let mut out = vec![];
        let mut frames = serde_json::Deserializer::from_slice(&self.pending).into_iter::<Frame>();
        let mut used = 0;
        loop {
            match frames.next() {
                Some(Ok(frame)) => {
                    used = frames.byte_offset();
                    if let Some(data) = frame.data.filter(|d| !d.is_empty()) {
                        match base64::engine::general_purpose::STANDARD.decode(data) {
                            Ok(bytes) => out.extend(bytes),
                            Err(e) => log::debug!("a log frame that isn't base64: {e}"),
                        }
                    }
                }
                // A frame cut in the middle: wait for the rest.
                Some(Err(e)) if e.is_eof() => break,
                Some(Err(e)) => {
                    log::debug!("a log frame that isn't JSON: {e}");
                    used = self.pending.len();
                    break;
                }
                None => {
                    used = self.pending.len();
                    break;
                }
            }
        }
        self.pending.drain(..used);
        out
    }
}

/// The query of a log request for `task`'s `stream` with `options`.
fn query(task: &str, stream: &str, options: &LogOptions) -> Vec<(&'static str, String)> {
    let mut query = vec![
        ("task", task.to_owned()),
        ("type", stream.to_owned()),
        ("follow", options.follow.to_string()),
    ];
    match options.tail {
        Some(lines) => {
            query.push(("origin", "end".into()));
            query.push((
                "offset",
                (u64::from(lines) * BYTES_PER_LINE)
                    .min(MAX_HISTORY)
                    .to_string(),
            ));
        }
        None => {
            query.push(("origin", "start".into()));
            query.push(("offset", "0".into()));
        }
    }
    query
}

/// What each stream sends the batcher.
enum Read {
    Line(&'static str, String),
    Failed(AppError),
    Done,
}

impl Nomad {
    /// Sends `task`'s stdout and stderr, as `options` asks, to `sink` in
    /// batches, until both end, the sink refuses, or the task is cancelled
    /// (which closes the requests). History from the end of a log starts at
    /// a whole line and keeps at most `tail` lines.
    pub async fn stream_logs(
        &self,
        alloc_id: String,
        task: String,
        options: LogOptions,
        sink: impl Fn(LogEvent) -> bool,
    ) {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let streams: Vec<&'static str> = [("stdout", options.stdout), ("stderr", options.stderr)]
            .into_iter()
            .filter_map(|(name, wanted)| wanted.then_some(name))
            .collect();
        let mut readers = vec![];
        for stream in &streams {
            let nomad = self.clone();
            let (alloc_id, task, options, tx) =
                (alloc_id.clone(), task.clone(), options.clone(), tx.clone());
            let stream = *stream;
            readers.push(tokio::spawn(async move {
                nomad
                    .read_log(&alloc_id, &task, stream, &options, &tx)
                    .await;
                let _ = tx.send(Read::Done);
            }));
        }
        drop(tx);
        // Abort the readers, and so their requests, however this ends.
        let _guard = AbortOnDrop(readers);

        let mut open = streams.len();
        let mut batch: Vec<LogLine> = vec![];
        let mut first = true;
        let mut tick = tokio::time::interval(BATCH_EVERY);
        while open > 0 {
            tokio::select! {
                read = rx.recv() => match read {
                    Some(Read::Line(stream, text)) => batch.push(LogLine {
                        source: None,
                        stream: stream.into(),
                        text,
                    }),
                    Some(Read::Failed(error)) => {
                        let _ = sink(LogEvent::Error { error });
                        return;
                    }
                    Some(Read::Done) | None => open -= 1,
                },
                _ = tick.tick() => {
                    if batch.is_empty() {
                        continue;
                    }
                    // The history read back holds about `tail` lines of each
                    // stream: keep that many of each.
                    if first {
                        if let Some(tail) = options.tail {
                            batch = last_of_each(batch, tail as usize);
                        }
                        first = false;
                    }
                    for chunk in std::mem::take(&mut batch).chunks(BATCH_LINES) {
                        if !sink(LogEvent::Lines { lines: chunk.to_vec() }) {
                            return;
                        }
                    }
                }
            }
        }
        // Logs that ended before the first batch went are history too.
        if first {
            if let Some(tail) = options.tail {
                batch = last_of_each(batch, tail as usize);
            }
        }
        for chunk in batch.chunks(BATCH_LINES) {
            if !sink(LogEvent::Lines {
                lines: chunk.to_vec(),
            }) {
                return;
            }
        }
        let _ = sink(LogEvent::End);
    }

    /// Reads one of `task`'s streams into `tx`, line by line.
    async fn read_log(
        &self,
        alloc_id: &str,
        task: &str,
        stream: &'static str,
        options: &LogOptions,
        tx: &mpsc::UnboundedSender<Read>,
    ) {
        let request = self.streaming(
            Method::GET,
            &format!("/v1/client/fs/logs/{}", segment(alloc_id)),
            &query(task, stream, options),
        );
        let mut response = match self.send(request).await {
            Ok(response) => response,
            Err(e) => {
                let error = self.allocation_error(
                    e,
                    alloc_id,
                    Some(task),
                    msg::LOGS_DENIED,
                    msg::LOGS_DENIED_HINT,
                );
                let _ = tx.send(Read::Failed(error));
                return;
            }
        };
        let mut frames = FrameDecoder::default();
        let mut lines = LineSplitter::default();
        // History from the end starts mid-line: drop that part line.
        let mut partial = options.tail.is_some();
        loop {
            match response.chunk().await {
                Ok(Some(chunk)) => {
                    for line in lines.push(&frames.push(&chunk)) {
                        if std::mem::take(&mut partial) {
                            continue;
                        }
                        if tx.send(Read::Line(stream, line)).is_err() {
                            return;
                        }
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let error = crate::error::unreachable(self.name(), self.address(), &e);
                    let _ = tx.send(Read::Failed(error));
                    return;
                }
            }
        }
        if let Some(rest) = lines.finish() {
            let _ = tx.send(Read::Line(stream, rest));
        }
    }
}

/// The last `tail` lines of each stream in `lines`, in their order.
fn last_of_each(lines: Vec<LogLine>, tail: usize) -> Vec<LogLine> {
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut kept: Vec<LogLine> = lines
        .into_iter()
        .rev()
        .filter(|line| {
            let count = seen.entry(line.stream.clone()).or_default();
            *count += 1;
            *count <= tail
        })
        .collect();
    kept.reverse();
    kept
}

/// Aborts its tasks when dropped.
struct AbortOnDrop(Vec<tokio::task::JoinHandle<()>>);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        for task in &self.0 {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::client::testing::agent;

    fn frame(text: &str) -> String {
        format!(
            r#"{{"File":"alloc/logs/server.stdout.0","Offset":1,"Data":"{}"}}"#,
            base64::engine::general_purpose::STANDARD.encode(text)
        )
    }

    #[test]
    fn frames_split_anywhere_decode_to_the_logs_bytes() {
        let stream = format!("{}{{}}{}\n", frame("hello\nwor"), frame("ld\n"));
        let mut decoder = FrameDecoder::default();
        let mut out = vec![];
        for byte in stream.as_bytes() {
            out.extend(decoder.push(&[*byte]));
        }
        assert_eq!(String::from_utf8(out).unwrap(), "hello\nworld\n");
    }

    #[test]
    fn history_from_the_end_is_asked_in_bytes() {
        let options = LogOptions {
            tail: Some(100),
            timestamps: false,
            since: None,
            stdout: true,
            stderr: false,
            follow: true,
        };
        let q = query("server", "stdout", &options);
        assert!(q.contains(&("origin", "end".into())));
        assert!(q.contains(&("offset", "25600".into())));
        assert!(q.contains(&("follow", "true".into())));
    }

    #[tokio::test]
    async fn both_streams_reach_the_sink_and_then_the_end() {
        let fake = agent(vec![(
            "GET /v1/client/fs/logs/a1",
            200,
            vec![],
            format!(
                "{}{}",
                frame("cut-off part\nfirst\nsecond\n"),
                frame("third\n")
            ),
        )])
        .await;
        let events = Arc::new(Mutex::new(vec![]));
        let seen = events.clone();
        fake.client(None)
            .stream_logs(
                "a1".into(),
                "server".into(),
                LogOptions {
                    tail: Some(2),
                    timestamps: false,
                    since: None,
                    stdout: true,
                    stderr: true,
                    follow: false,
                },
                move |e| {
                    seen.lock().unwrap().push(e);
                    true
                },
            )
            .await;
        let events = events.lock().unwrap();
        let lines: Vec<(String, String)> = events
            .iter()
            .flat_map(|e| match e {
                LogEvent::Lines { lines } => lines
                    .iter()
                    .map(|l| (l.stream.clone(), l.text.clone()))
                    .collect(),
                _ => vec![],
            })
            .collect();
        // The cut-off line is dropped; each stream keeps the last two of the rest.
        for stream in ["stdout", "stderr"] {
            let of: Vec<&str> = lines
                .iter()
                .filter(|(s, _)| s == stream)
                .map(|(_, text)| text.as_str())
                .collect();
            assert_eq!(of, ["second", "third"], "{stream}");
        }
        assert!(matches!(events.last(), Some(LogEvent::End)));
        let sent = fake.received();
        assert!(sent.iter().any(|r| r.target.contains("type=stderr")));
    }

    #[tokio::test]
    async fn a_token_without_read_logs_says_so() {
        let fake = agent(vec![(
            "GET /v1/client/fs/logs/a1",
            403,
            vec![],
            "Permission denied".into(),
        )])
        .await;
        let events = Arc::new(Mutex::new(vec![]));
        let seen = events.clone();
        fake.client(Some("t"))
            .stream_logs(
                "a1".into(),
                "server".into(),
                LogOptions {
                    tail: None,
                    timestamps: false,
                    since: None,
                    stdout: true,
                    stderr: false,
                    follow: true,
                },
                move |e| {
                    seen.lock().unwrap().push(e);
                    true
                },
            )
            .await;
        let events = events.lock().unwrap();
        let Some(LogEvent::Error { error }) = events.last() else {
            panic!("{events:?}");
        };
        assert_eq!(error.key, "errors.nomad.logsDenied");
    }
}
