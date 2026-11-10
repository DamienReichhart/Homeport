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
    /// (which closes the requests). Each stream's history keeps its last
    /// `tail` lines.
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
                    for chunk in std::mem::take(&mut batch).chunks(BATCH_LINES) {
                        if !sink(LogEvent::Lines { lines: chunk.to_vec() }) {
                            return;
                        }
                    }
                }
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

    /// Reads one of `task`'s streams into `tx`, line by line. With a tail,
    /// the history is read whole first and only its last lines sent, then
    /// new lines are followed from the end, so each stream keeps its own
    /// history whatever order the streams answer in.
    async fn read_log(
        &self,
        alloc_id: &str,
        task: &str,
        stream: &'static str,
        options: &LogOptions,
        tx: &mpsc::UnboundedSender<Read>,
    ) {
        let Some(tail) = options.tail else {
            self.read_lines(
                alloc_id,
                task,
                stream,
                &query(task, stream, options),
                tx,
                None,
            )
            .await;
            return;
        };
        let history = LogOptions {
            follow: false,
            ..options.clone()
        };
        let complete = self
            .read_lines(
                alloc_id,
                task,
                stream,
                &query(task, stream, &history),
                tx,
                Some(tail as usize),
            )
            .await;
        if complete && options.follow {
            let mut follow = query(task, stream, options);
            for (key, value) in &mut follow {
                if *key == "offset" {
                    *value = "0".into();
                }
            }
            self.read_lines(alloc_id, task, stream, &follow, tx, None)
                .await;
        }
    }

    /// Reads the log `query` asks for into `tx`: every line, or with `last`
    /// only that many from the end once the answer is complete. `false`
    /// when it failed or the receiver is gone.
    async fn read_lines(
        &self,
        alloc_id: &str,
        task: &str,
        stream: &'static str,
        query: &[(&'static str, String)],
        tx: &mpsc::UnboundedSender<Read>,
        last: Option<usize>,
    ) -> bool {
        let request = self.streaming(
            Method::GET,
            &format!("/v1/client/fs/logs/{}", segment(alloc_id)),
            query,
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
                return false;
            }
        };
        let mut frames = FrameDecoder::default();
        let mut lines = LineSplitter::default();
        let mut kept: std::collections::VecDeque<String> = std::collections::VecDeque::new();
        let mut send = |line: String| -> bool {
            match last {
                // History from the end starts mid-line: keeping the last
                // lines drops that part line along with the older ones.
                Some(n) => {
                    kept.push_back(line);
                    if kept.len() > n {
                        kept.pop_front();
                    }
                    true
                }
                None => tx.send(Read::Line(stream, line)).is_ok(),
            }
        };
        loop {
            match response.chunk().await {
                Ok(Some(chunk)) => {
                    for line in lines.push(&frames.push(&chunk)) {
                        if !send(line) {
                            return false;
                        }
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let error = crate::error::unreachable(self.name(), self.address(), &e);
                    let _ = tx.send(Read::Failed(error));
                    return false;
                }
            }
        }
        if let Some(rest) = lines.finish() {
            if !send(rest) {
                return false;
            }
        }
        kept.into_iter()
            .all(|line| tx.send(Read::Line(stream, line)).is_ok())
    }
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
mod tests;
