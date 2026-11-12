//! A line as a page gives it to the viewer, and what the viewer says of
//! itself between two lines.

use std::sync::Arc;

/// Where a line was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Stream {
    #[default]
    Stdout,
    Stderr,
    /// A container with a terminal: one stream for both.
    Console,
}

impl Stream {
    /// From the name the backend gives it (`stdout`, `stderr`, `console`).
    pub fn named(name: &str) -> Self {
        match name {
            "stderr" => Self::Stderr,
            "console" => Self::Console,
            _ => Self::Stdout,
        }
    }

    pub(super) const fn bits(self) -> u8 {
        match self {
            Self::Stdout => 0,
            Self::Stderr => 1,
            Self::Console => 2,
        }
    }

    pub(super) const fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            1 => Self::Stderr,
            2 => Self::Console,
            _ => Self::Stdout,
        }
    }
}

/// A line of a log, as it came: its escape sequences still in it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Line {
    pub text: String,
    pub stream: Stream,
    /// Which of several sources wrote it (a task of a service, a Pod of a
    /// workload): shown before the line, each source in its colour.
    pub source: Option<Arc<str>>,
    /// The text starts with the time it was written, then a space: what
    /// the Timestamps toggle shows and hides.
    pub timestamped: bool,
}

impl Line {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn stream(mut self, stream: Stream) -> Self {
        self.stream = stream;
        self
    }

    #[must_use]
    pub fn source(mut self, source: impl Into<Arc<str>>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// The stream was asked for timestamps: the text starts with one.
    #[must_use]
    pub fn timestamped(mut self, timestamped: bool) -> Self {
        self.timestamped = timestamped;
        self
    }
}

impl From<homeport_core::logs::LogLine> for Line {
    fn from(line: homeport_core::logs::LogLine) -> Self {
        Self {
            stream: Stream::named(&line.stream),
            source: line.source.map(Arc::from),
            text: line.text,
            timestamped: false,
        }
    }
}

/// What the viewer writes between the lines of the log, in the app's
/// language at the time it is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Marker {
    /// The source said it has no more ("— end of logs —").
    Ended,
    /// The window was behind and this many lines were never shown.
    Skipped(u64),
}

/// How many cells of a terminal `text` takes: two for the wide characters
/// of East Asian scripts and emoji, none for what combines with the
/// character before, one for the rest.
pub fn cells(text: &str) -> usize {
    if text.is_ascii() {
        return text.len();
    }
    text.chars().map(cell_width).sum()
}

/// The cells of one character (see [`cells`]).
pub fn cell_width(c: char) -> usize {
    match c as u32 {
        0..=0x7e => 1,
        // Combining marks, zero-width spaces and joiners, variation selectors.
        0x0300..=0x036f | 0x200b..=0x200f | 0x2060 | 0xfe00..=0xfe0f | 0xfeff => 0,
        // Hangul Jamo, CJK and its punctuation, kana, Hangul, full-width
        // forms, emoji.
        0x1100..=0x115f
        | 0x2e80..=0x303e
        | 0x3041..=0x33ff
        | 0x3400..=0x4dbf
        | 0x4e00..=0x9fff
        | 0xa000..=0xa4cf
        | 0xac00..=0xd7a3
        | 0xf900..=0xfaff
        | 0xfe30..=0xfe4f
        | 0xff00..=0xff60
        | 0xffe0..=0xffe6
        | 0x1f300..=0x1f64f
        | 0x1f900..=0x1f9ff
        | 0x20000..=0x3fffd => 2,
        _ => 1,
    }
}

/// The byte of `text` at which its cell `cell` starts, or its length when
/// it has fewer cells.
pub fn byte_at_cell(text: &str, cell: usize) -> usize {
    if text.is_ascii() {
        return cell.min(text.len());
    }
    let mut at = 0;
    for (index, c) in text.char_indices() {
        let width = cell_width(c);
        if at + width > cell && width > 0 {
            return index;
        }
        at += width;
    }
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_from_the_backend_keeps_its_stream_and_its_source() {
        let line = Line::from(homeport_core::logs::LogLine {
            source: Some("web.1".to_owned()),
            stream: "stderr".to_owned(),
            text: "warning: slow".to_owned(),
        });
        assert_eq!(line.stream, Stream::Stderr);
        assert_eq!(line.source.as_deref(), Some("web.1"));
        assert_eq!(Stream::named("console"), Stream::Console);
        assert_eq!(Stream::named("anything"), Stream::Stdout);
        for stream in [Stream::Stdout, Stream::Stderr, Stream::Console] {
            assert_eq!(Stream::from_bits(stream.bits()), stream);
        }
    }

    #[test]
    fn cells_count_wide_and_combining_characters() {
        assert_eq!(cells("listening on :80"), 16);
        assert_eq!(cells("日本語"), 6);
        assert_eq!(cells("e\u{301}"), 1, "an accent combines");
        assert_eq!(cells("مرحبا"), 5);
        assert_eq!(byte_at_cell("abc", 2), 2);
        assert_eq!(byte_at_cell("abc", 9), 3);
        // The second cell of a wide character is still that character.
        assert_eq!(byte_at_cell("日本", 1), 0);
        assert_eq!(byte_at_cell("日本", 2), 3);
        assert_eq!(byte_at_cell("e\u{301}x", 1), 3);
    }
}
