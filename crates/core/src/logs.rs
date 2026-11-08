//! Lines of logs as every log stream sends them, batched, to the frontend:
//! a container's, a Swarm service's or a Pod's.

use serde::{Deserialize, Serialize};

/// Lines sent together at most: a busy stream is batched.
pub const BATCH_LINES: usize = 500;
/// How often a stream sends what it has read.
pub const BATCH_EVERY: std::time::Duration = std::time::Duration::from_millis(50);

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
    Lines { lines: Vec<LogLine> },
    Error { error: crate::error::AppError },
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
