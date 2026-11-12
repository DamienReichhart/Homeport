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
/// Where a tab goes: the next multiple of this many columns.
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

impl Parser {
    /// The style text is in now: what the next line starts with.
    pub fn style(&self) -> Style {
        self.style
    }

    /// Reads one line (or a piece of one): its text is added to `text`
    /// without its escape sequences, tabs as spaces, and the runs that are
    /// not plain to `runs`, counted from `line_start` in `text`.
    pub fn feed(&mut self, input: &str, text: &mut String, line_start: usize, runs: &mut Vec<Run>) {
        let first_run = runs.len();
        // A carriage return takes the line back to its start: what comes
        // after it overwrites, so only that is kept. One at the very end
        // (`\r\n`) changes nothing.
        let mut returned = false;
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
                    self.overwrite(&mut returned, text, line_start, runs, first_run);
                    self.print(&input[at..at + plain], text, line_start, runs, first_run);
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
                    '\r' => returned = true,
                    '\t' => {
                        self.overwrite(&mut returned, text, line_start, runs, first_run);
                        let column = text[line_start..].chars().count();
                        let spaces = TAB - column % TAB;
                        self.print(&" ".repeat(spaces), text, line_start, runs, first_run);
                    }
                    c if c.is_control() => {}
                    c => {
                        self.overwrite(&mut returned, text, line_start, runs, first_run);
                        let mut bytes = [0; 4];
                        self.print(c.encode_utf8(&mut bytes), text, line_start, runs, first_run);
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

    /// What a carriage return left to do, now that text follows it.
    fn overwrite(
        &self,
        returned: &mut bool,
        text: &mut String,
        line_start: usize,
        runs: &mut Vec<Run>,
        first_run: usize,
    ) {
        if std::mem::take(returned) {
            text.truncate(line_start);
            runs.truncate(first_run);
        }
    }

    fn print(
        &self,
        printed: &str,
        text: &mut String,
        line_start: usize,
        runs: &mut Vec<Run>,
        first_run: usize,
    ) {
        // A line of a log is far below 4 GiB.
        #[allow(clippy::cast_possible_truncation)]
        let at = (text.len() - line_start) as u32;
        text.push_str(printed);
        if self.style.is_plain() || printed.is_empty() {
            return;
        }
        #[allow(clippy::cast_possible_truncation)]
        let end = (text.len() - line_start) as u32;
        match runs[first_run..].last_mut() {
            Some(last) if last.end == at && last.style == self.style => last.end = end,
            _ => runs.push(Run {
                start: at,
                end,
                style: self.style,
            }),
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
            let mut own = group.split(':').map(number);
            let code = own.next().flatten().unwrap_or(0);
            let own: Vec<Option<u16>> = own.collect();
            match code {
                38 | 48 => {
                    let colour = if own.is_empty() {
                        extended(&mut groups.by_ref().map(number))
                    } else {
                        colon_colour(&own)
                    };
                    match (colour, code) {
                        (Some(colour), 38) => self.style.fg = colour,
                        (Some(colour), _) => self.style.bg = colour,
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

/// A parameter: empty is absent, and what is not a number is too.
fn number(text: &str) -> Option<u16> {
    text.parse().ok()
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
