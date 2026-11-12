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

/// A character of a text, the byte it starts at and the cells it takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placed {
    pub byte: usize,
    pub c: char,
    pub width: usize,
}

/// The characters of `text` with the cells each takes in a terminal, by
/// Unicode's own tables (`unicode-width`): two for East Asian wide
/// characters and emoji, none for what combines with the character
/// before (accents, Arabic and Indic vowel signs, joiners), one for the
/// rest. A character followed by the emoji selector takes two, and what a
/// joiner joins to the character before takes none.
pub fn widths(text: &str) -> impl Iterator<Item = Placed> + '_ {
    let mut chars = text.char_indices().peekable();
    let mut joined = false;
    std::iter::from_fn(move || {
        let (byte, c) = chars.next()?;
        let own = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        let width = if std::mem::replace(&mut joined, c == '\u{200d}') {
            0
        } else if own == 1 && chars.peek().is_some_and(|(_, next)| *next == '\u{fe0f}') {
            2
        } else {
            own
        };
        Some(Placed { byte, c, width })
    })
}

/// How many cells of a terminal `text` takes (see [`widths`]).
pub fn cells(text: &str) -> usize {
    if text.is_ascii() {
        return text.len();
    }
    widths(text).map(|placed| placed.width).sum()
}

/// Whether a character of `text` takes two cells: such a line can't be
/// cut into rows by a division.
pub fn has_wide(text: &str) -> bool {
    !text.is_ascii() && widths(text).any(|placed| placed.width > 1)
}

/// The byte of `text` at which its cell `cell` starts, or its length when
/// it has fewer cells.
pub fn byte_at_cell(text: &str, cell: usize) -> usize {
    if text.is_ascii() {
        return cell.min(text.len());
    }
    let mut at = 0;
    for placed in widths(text) {
        if at + placed.width > cell && placed.width > 0 {
            return placed.byte;
        }
        at += placed.width;
    }
    text.len()
}

/// The longest timestamp a line may start with, in bytes.
const STAMP_MAX: usize = 64;

/// The timestamp a line of a stream asked for timestamps starts with,
/// and the rest: what stands before the first space, when it starts with
/// a digit and is printable ASCII of a timestamp's length. A line that is
/// nothing but its timestamp has an empty message.
pub fn split_stamp(text: &str) -> Option<(&str, &str)> {
    let end = text.find(' ').unwrap_or(text.len());
    let stamp = &text[..end];
    let is_stamp = (1..=STAMP_MAX).contains(&end)
        && stamp.as_bytes()[0].is_ascii_digit()
        && stamp.bytes().all(|byte| byte.is_ascii_graphic());
    is_stamp.then(|| (stamp, text.get(end + 1..).unwrap_or("")))
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
        // Emoji past the first blocks, and the dingbats that are emoji.
        for wide in ["🚀", "✅", "🟠", "🫠", "⏳"] {
            assert_eq!(cells(wide), 2, "{wide}");
        }
        // Signs that combine in other scripts: Arabic, Devanagari, Thai.
        assert_eq!(cells("مَرْحَبًا"), 5);
        assert_eq!(cells("क\u{94d}"), 1, "a virama joins");
        assert_eq!(cells("ที่"), 1);
        // A character made of several joined ones is the first; one told
        // to show as an emoji is wide; a flag is two.
        assert_eq!(cells("👨\u{200d}👩\u{200d}👧"), 2);
        assert_eq!(cells("☀"), 1);
        assert_eq!(cells("☀\u{fe0f}"), 2);
        assert_eq!(cells("🇫🇷"), 2);
        assert!(has_wide("a🚀") && !has_wide("été") && !has_wide("abc"));
        assert_eq!(byte_at_cell("abc", 2), 2);
        assert_eq!(byte_at_cell("abc", 9), 3);
        // The second cell of a wide character is still that character.
        assert_eq!(byte_at_cell("日本", 1), 0);
        assert_eq!(byte_at_cell("日本", 2), 3);
        assert_eq!(byte_at_cell("e\u{301}x", 1), 3);
    }

    #[test]
    fn a_timestamp_is_what_starts_with_a_digit_before_the_first_space() {
        assert_eq!(
            split_stamp("2026-05-01T10:00:00.123456789Z started"),
            Some(("2026-05-01T10:00:00.123456789Z", "started"))
        );
        // Nothing after it, with or without the space.
        assert_eq!(
            split_stamp("2026-05-01T10:00:00Z "),
            Some(("2026-05-01T10:00:00Z", ""))
        );
        assert_eq!(
            split_stamp("2026-05-01T10:00:00Z"),
            Some(("2026-05-01T10:00:00Z", ""))
        );
        // A word is not a timestamp, nor is what holds an escape sequence.
        assert_eq!(split_stamp("no stamp here"), None);
        assert_eq!(split_stamp("1\x1b[31m red"), None);
        assert_eq!(split_stamp(""), None);
        assert_eq!(split_stamp(&format!("{} x", "1".repeat(65))), None);
    }
}
