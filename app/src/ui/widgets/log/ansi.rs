//! What a program wrote for a terminal, read for a log: the text, and the
//! colours and emphasis its SGR sequences (`ESC [ … m`) ask for. Every
//! other escape sequence (a cursor move, a title, a hyperlink) is taken
//! out and never drawn.
//!
//! The [`Parser`] is a terminal's state machine cut down to what a log
//! needs, and it is kept from one line to the next: a colour set on a line
//! holds until it is reset, as in a terminal, and a sequence cut by the
//! end of a chunk is finished by the start of the next one. What is
//! malformed is dropped without taking the text after it: a sequence that
//! never ends is given up after [`CSI_MAX`] parameter bytes, a title or a
//! hyperlink after [`STRING_MAX`].

use super::line::cells;

/// A colour a program asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Colour {
    /// The terminal's own: its text, or its background.
    #[default]
    Default,
    /// One of the 256: the 16 of the theme, the 216 of the cube, 24 greys.
    Indexed(u8),
    Rgb(u8, u8, u8),
}

/// Emphasis, as bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Attrs(u8);

impl Attrs {
    pub const BOLD: Self = Self(1);
    pub const DIM: Self = Self(1 << 1);
    pub const ITALIC: Self = Self(1 << 2);
    pub const UNDERLINE: Self = Self(1 << 3);
    pub const INVERSE: Self = Self(1 << 4);
    pub const STRIKE: Self = Self(1 << 5);

    pub const fn has(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub const fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    const fn without(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

/// How a run of text is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Style {
    pub fg: Colour,
    pub bg: Colour,
    pub attrs: Attrs,
}

impl Style {
    pub fn is_plain(self) -> bool {
        self == Self::default()
    }
}

/// The bytes `start..end` of a line's text, drawn in `style`. Text no run
/// covers is plain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    pub start: u32,
    pub end: u32,
    pub style: Style,
}

/// The most parameter bytes of a `ESC [` sequence: past it the sequence is
/// read to its end and dropped.
pub const CSI_MAX: usize = 64;
/// The most bytes of a string sequence (a title, a hyperlink) that never
/// ends: past it, text is text again.
pub const STRING_MAX: usize = 4096;
/// Where a tab goes: the next multiple of this many cells.
const TAB: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Phase {
    #[default]
    Ground,
    /// After `ESC`.
    Escape,
    /// After `ESC` and an intermediate byte: one more byte ends it.
    Intermediate,
    /// In `ESC [ …`.
    Csi,
    /// A `ESC [` sequence that is not SGR's to read: to its last byte.
    CsiIgnored,
    /// In a string (`ESC ]`, `ESC P`, `ESC _`…): to BEL or `ESC \`.
    Text,
    /// After `ESC` in a string.
    TextEscape,
}

/// Reads lines as a terminal would, keeping what one line leaves to the next.
#[derive(Debug, Clone, Default)]
pub struct Parser {
    phase: Phase,
    params: String,
    /// How many bytes the string sequence being read has taken.
    taken: usize,
    style: Style,
}

/// Where a line being read goes.
struct Out<'a> {
    text: &'a mut String,
    runs: &'a mut Vec<Run>,
    /// Where the line starts in `text`: its runs are counted from there.
    line_start: usize,
    /// Where its message starts, after its timestamp: what a carriage
    /// return goes back to, and what tab stops are counted from.
    message_start: usize,
    /// The first run of this line in `runs`.
    first_run: usize,
    /// The cell of the message the next character goes to.
    column: usize,
    /// A carriage return was read and no text since. It takes the line
    /// back to its start: what comes after it overwrites, so only that is
    /// kept. One at the very end (`\r\n`) changes nothing.
    returned: bool,
}

impl Out<'_> {
    fn print(&mut self, printed: &str, style: Style) {
        if std::mem::take(&mut self.returned) {
            self.text.truncate(self.message_start);
            self.runs.truncate(self.first_run);
            self.column = 0;
        }
        // A line of a log is far below 4 GiB.
        #[allow(clippy::cast_possible_truncation)]
        let at = (self.text.len() - self.line_start) as u32;
        self.text.push_str(printed);
        self.column += cells(printed);
        if style.is_plain() || printed.is_empty() {
            return;
        }
        #[allow(clippy::cast_possible_truncation)]
        let end = (self.text.len() - self.line_start) as u32;
        match self.runs[self.first_run..].last_mut() {
            Some(last) if last.end == at && last.style == style => last.end = end,
            _ => self.runs.push(Run {
                start: at,
                end,
                style,
            }),
        }
    }
}

impl Parser {
    /// The style text is in now: what the next line starts with.
    pub fn style(&self) -> Style {
        self.style
    }

    /// Reads one line (or a piece of one) that starts at `line_start` in
    /// `text`: [`Parser::feed_message`] for a line that is all message.
    pub fn feed(&mut self, input: &str, text: &mut String, line_start: usize, runs: &mut Vec<Run>) {
        self.feed_message(input, text, line_start, line_start, runs);
    }

    /// Reads the message of a line: its text is added to `text` without
    /// its escape sequences, tabs as spaces, and the runs that are not
    /// plain to `runs`, counted from `line_start` in `text`. The message
    /// starts at `message_start`, after the line's timestamp when it has
    /// one: a carriage return goes back there, never further.
    pub fn feed_message(
        &mut self,
        input: &str,
        text: &mut String,
        line_start: usize,
        message_start: usize,
        runs: &mut Vec<Run>,
    ) {
        let mut out = Out {
            column: cells(&text[message_start..]),
            first_run: runs.len(),
            text,
            runs,
            line_start,
            message_start,
            returned: false,
        };
        let bytes = input.as_bytes();
        let mut at = 0;
        while at < input.len() {
            if self.phase == Phase::Ground {
                // Most of a line is nothing but text: taken whole, up to
                // the next byte that may be something else.
                let plain = bytes[at..]
                    .iter()
                    .take_while(|byte| is_text(**byte))
                    .count();
                if plain > 0 {
                    out.print(&input[at..at + plain], self.style);
                    at += plain;
                    continue;
                }
            }
            let Some(c) = input[at..].chars().next() else {
                break;
            };
            at += c.len_utf8();
            match self.phase {
                Phase::Ground => match c {
                    '\u{1b}' => self.phase = Phase::Escape,
                    '\u{9b}' => self.start_csi(),
                    '\r' => out.returned = true,
                    '\t' => {
                        // To the next stop, counted in cells of the message.
                        let column = if out.returned { 0 } else { out.column };
                        const SPACES: &str = "        ";
                        out.print(&SPACES[..TAB - column % TAB], self.style);
                    }
                    c if c.is_control() => {}
                    c => {
                        let mut bytes = [0; 4];
                        out.print(c.encode_utf8(&mut bytes), self.style);
                    }
                },
                Phase::Escape => self.escape(c),
                Phase::Intermediate => match c {
                    ' '..='/' => {}
                    '\u{1b}' => self.phase = Phase::Escape,
                    _ => self.phase = Phase::Ground,
                },
                Phase::Csi => self.csi(c),
                Phase::CsiIgnored => match c {
                    '@'..='~' => self.phase = Phase::Ground,
                    '\u{1b}' => self.phase = Phase::Escape,
                    _ => {}
                },
                Phase::Text => self.string(c),
                Phase::TextEscape => {
                    // `ESC \` ends the string; any other escape starts anew.
                    if c == '\\' {
                        self.phase = Phase::Ground;
                    } else {
                        self.escape(c);
                    }
                }
            }
        }
    }

    fn start_csi(&mut self) {
        self.params.clear();
        self.phase = Phase::Csi;
    }

    fn start_string(&mut self) {
        self.taken = 0;
        self.phase = Phase::Text;
    }

    fn escape(&mut self, c: char) {
        self.phase = match c {
            '[' => {
                self.start_csi();
                return;
            }
            // OSC, DCS, SOS, PM and APC: strings.
            ']' | 'P' | 'X' | '^' | '_' => {
                self.start_string();
                return;
            }
            ' '..='/' => Phase::Intermediate,
            '\u{1b}' => Phase::Escape,
            // A sequence of two bytes, or nothing a terminal knows.
            _ => Phase::Ground,
        };
    }

    fn csi(&mut self, c: char) {
        match c {
            '0'..='?' if self.params.len() < CSI_MAX => self.params.push(c),
            '0'..='?' | ' '..='/' => self.phase = Phase::CsiIgnored,
            'm' => {
                self.sgr();
                self.phase = Phase::Ground;
            }
            '@'..='~' => self.phase = Phase::Ground,
            '\u{1b}' => self.phase = Phase::Escape,
            // A control character does not end it; anything else is not
            // a sequence after all.
            c if c.is_control() => {}
            _ => self.phase = Phase::Ground,
        }
    }

    fn string(&mut self, c: char) {
        self.taken += c.len_utf8();
        self.phase = match c {
            '\u{7}' | '\u{9c}' => Phase::Ground,
            '\u{1b}' => Phase::TextEscape,
            _ if self.taken > STRING_MAX => Phase::Ground,
            _ => Phase::Text,
        };
    }

    /// Applies the parameters of an SGR sequence to the style.
    fn sgr(&mut self) {
        // `ESC [ ? … m` and the like are private: not SGR.
        if self.params.starts_with(['<', '=', '>', '?']) {
            return;
        }
        let params = std::mem::take(&mut self.params);
        // No parameter at all is one that is empty: a reset.
        let mut groups = params.split(';');
        while let Some(group) = groups.next() {
            // `38:2::r:g:b` keeps its values to itself; `38;2;r;g;b`
            // takes them from the parameters that follow.
            let mut own = group.split(':');
            let code = match own.next().map_or(Number::Empty, number) {
                Number::Empty => 0,
                Number::Is(code) => code,
                // Not a code any terminal knows: it changes nothing.
                Number::Unreadable => continue,
            };
            let own: Vec<Option<u16>> = own.map(|text| number(text).value()).collect();
            match code {
                // The text's colour, its background's, its underline's.
                38 | 48 | 58 => {
                    let colour = if own.is_empty() {
                        extended(&mut groups.by_ref().map(|text| number(text).value()))
                    } else {
                        colon_colour(&own)
                    };
                    match (colour, code) {
                        (Some(colour), 38) => self.style.fg = colour,
                        (Some(colour), 48) => self.style.bg = colour,
                        // An underline is drawn in the text's colour.
                        (Some(_), _) => {}
                        // Not a colour: what follows can't be trusted.
                        (None, _) => break,
                    }
                }
                // `4:0` is no underline; any other kind is one.
                4 if own.first() == Some(&Some(0)) => {
                    self.style.attrs = self.style.attrs.without(Attrs::UNDERLINE);
                }
                code => self.simple(code),
            }
        }
        self.params = params;
        self.params.clear();
    }

    fn simple(&mut self, code: u16) {
        let style = &mut self.style;
        let on = |style: &mut Style, attr| style.attrs = style.attrs.with(attr);
        let off = |style: &mut Style, attr| style.attrs = style.attrs.without(attr);
        // The matches below keep a code within a byte.
        #[allow(clippy::cast_possible_truncation)]
        let low = (code % 10) as u8;
        match code {
            0 => *style = Style::default(),
            1 => on(style, Attrs::BOLD),
            2 => on(style, Attrs::DIM),
            3 => on(style, Attrs::ITALIC),
            4 | 21 => on(style, Attrs::UNDERLINE),
            7 => on(style, Attrs::INVERSE),
            9 => on(style, Attrs::STRIKE),
            22 => off(style, Attrs::BOLD.with(Attrs::DIM)),
            23 => off(style, Attrs::ITALIC),
            24 => off(style, Attrs::UNDERLINE),
            27 => off(style, Attrs::INVERSE),
            29 => off(style, Attrs::STRIKE),
            30..=37 => style.fg = Colour::Indexed(low),
            39 => style.fg = Colour::Default,
            40..=47 => style.bg = Colour::Indexed(low),
            49 => style.bg = Colour::Default,
            90..=97 => style.fg = Colour::Indexed(8 + low),
            100..=107 => style.bg = Colour::Indexed(8 + low),
            // Blink, conceal, fonts, overline: nothing a log shows.
            _ => {}
        }
    }
}

/// Whether a byte can only be part of text: no control character, and not
/// the first byte of the few two-byte characters among which the 8-bit
/// controls are.
fn is_text(byte: u8) -> bool {
    byte >= 0x20 && byte != 0x7f && byte != 0xc2
}

/// A parameter of a sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Number {
    /// Left out: it stands for 0.
    Empty,
    Is(u16),
    /// Not a number, or one too large to be a code or a colour.
    Unreadable,
}

impl Number {
    fn value(self) -> Option<u16> {
        match self {
            Self::Is(value) => Some(value),
            Self::Empty | Self::Unreadable => None,
        }
    }
}

fn number(text: &str) -> Number {
    if text.is_empty() {
        return Number::Empty;
    }
    text.parse().map_or(Number::Unreadable, Number::Is)
}

fn byte(value: Option<u16>) -> Option<u8> {
    u8::try_from(value?).ok()
}

/// `5;n` or `2;r;g;b`, from the parameters after `38` or `48`.
fn extended(rest: &mut dyn Iterator<Item = Option<u16>>) -> Option<Colour> {
    match rest.next()?? {
        5 => Some(Colour::Indexed(byte(rest.next()?)?)),
        2 => {
            let (r, g, b) = (rest.next()?, rest.next()?, rest.next()?);
            Some(Colour::Rgb(byte(r)?, byte(g)?, byte(b)?))
        }
        _ => None,
    }
}

/// `5:n`, `2:r:g:b` or `2::r:g:b` (with the colour space left empty).
fn colon_colour(own: &[Option<u16>]) -> Option<Colour> {
    match own {
        [Some(5), n] => Some(Colour::Indexed(byte(*n)?)),
        [Some(2), r, g, b] | [Some(2), _, r, g, b] => {
            Some(Colour::Rgb(byte(*r)?, byte(*g)?, byte(*b)?))
        }
        _ => None,
    }
}

/// The text of `input` alone, as a parser that starts anew reads it.
pub fn strip(input: &str) -> String {
    let mut text = String::with_capacity(input.len());
    Parser::default().feed(input, &mut text, 0, &mut vec![]);
    text
}
