//! The rows in view, built from the grid: each cut into pieces of one
//! look, the text of each shaped once and kept.
//!
//! Two caches keep a frame cheap. A row whose cells are the same as a row
//! of the last layout (wherever it was: the view scrolled) is taken as it
//! was, pieces and all, so only the rows a program changed are looked at
//! again. And the shaped text of a piece is kept by what it says
//! ([`Glyphs`]), so text that comes back (a prompt, `yes`, the frame of a
//! full-screen program) is never shaped twice.
//!
//! Text the monospaced face has (ASCII and Latin-1) is one piece per run
//! of a look; any other character is a piece of its own, placed at its
//! cell, so what a fallback font makes of its width never moves the text
//! after it. Box drawing and block elements are drawn as shapes
//! (`boxes.rs`).

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::LazyLock,
};

use alacritty_terminal::{
    grid::{Dimensions, Row as GridRow},
    index::{Column, Line},
    term::{
        cell::{Cell, Flags},
        color::Colors,
    },
    vte::ansi::{Color as Wanted, CursorShape},
};
use iced::{
    advanced::text::{self, Paragraph as _},
    Pixels, Size,
};
use regex::Regex;

use super::{boxes, Geometry, Raw};
use crate::ui::{
    theme::font,
    widgets::{
        log::ansi::{Attrs, Style},
        terminal::{colors, state::State},
    },
};

/// What a piece of a row is drawn as.
#[derive(Clone)]
pub enum Kind {
    /// Cells with nothing to write: only what is behind them, or a line
    /// under them.
    Blank,
    /// Text, shaped, set this far into its first cell (a character of a
    /// fallback font, centred in its cells).
    Text { paragraph: Raw, inset: f32 },
    /// A box-drawing character or a block, drawn as shapes.
    Drawn(char),
}

/// Cells of one look, at a cell of their row.
#[derive(Clone)]
pub struct Piece {
    pub col: usize,
    pub cells: usize,
    pub style: Style,
    pub kind: Kind,
}

/// A link on a row: the cells `from..to` open `uri`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub from: usize,
    pub to: usize,
    pub uri: String,
}

/// A row in view.
pub struct Row {
    /// Which line of the grid it shows.
    pub line: Line,
    /// What its cells hold, as a number: a row with the same is the same.
    signature: u64,
    pub pieces: Vec<Piece>,
    pub links: Vec<Link>,
    /// What it says, for whoever looks for a text.
    pub text: String,
}

/// The cursor as the layout found it.
pub struct Cursor {
    /// Its row among those in view, its cell, and how many cells it covers.
    pub row: usize,
    pub col: usize,
    pub cells: usize,
    pub shape: CursorShape,
    /// The character under it, to be drawn in the cursor's accent.
    pub glyph: Option<Raw>,
    pub drawn: Option<char>,
}

/// What a layout came to.
#[derive(Default)]
pub struct Laid {
    pub rows: Vec<Row>,
    pub cursor: Option<Cursor>,
    /// How far up the scrollback the view is, and how many lines it has.
    pub offset: usize,
    pub history: usize,
}

/// Shaped text by what it says, at one size.
#[derive(Default)]
pub struct Glyphs {
    size: f32,
    height: f32,
    plain: HashMap<String, (Raw, u64)>,
    italic: HashMap<String, (Raw, u64)>,
    /// The layout that last used each, to let go of the others.
    generation: u64,
    /// How many pieces of text were shaped: what a test counts.
    pub shaped: usize,
}

/// How many shaped pieces are kept before those the last layout did not
/// use are let go of.
const KEPT: usize = 4_096;

impl Glyphs {
    fn begin(&mut self, size: f32, height: f32) {
        if (self.size - size).abs() > f32::EPSILON || (self.height - height).abs() > f32::EPSILON {
            self.plain.clear();
            self.italic.clear();
            self.size = size;
            self.height = height;
        }
        self.generation += 1;
    }

    fn end(&mut self) {
        let now = self.generation;
        for map in [&mut self.plain, &mut self.italic] {
            if map.len() > KEPT {
                map.retain(|_, (_, used)| *used == now);
            }
        }
    }

    /// `content` shaped, from the cache when it was shaped before.
    fn get(&mut self, content: &str, italic: bool) -> Raw {
        let now = self.generation;
        let map = if italic {
            &mut self.italic
        } else {
            &mut self.plain
        };
        if let Some((paragraph, used)) = map.get_mut(content) {
            *used = now;
            return paragraph.clone();
        }
        self.shaped += 1;
        let paragraph = shape(content, italic, self.size, self.height);
        map.insert(content.to_owned(), (paragraph.clone(), now));
        paragraph
    }
}

/// A piece of a row as text that never wraps.
pub fn shape(content: &str, italic: bool, size: f32, height: f32) -> Raw {
    let ascii = content.is_ascii();
    // Room for any glyph, and finite: text that reads right to left is
    // placed against the end of what it is given.
    #[allow(clippy::cast_precision_loss)]
    let room = if ascii {
        f32::INFINITY
    } else {
        (content.chars().count() + 1) as f32 * 2.0 * height
    };
    let mut paragraph = Raw::with_text(text::Text {
        content,
        bounds: Size::new(room, height),
        size: Pixels(size),
        line_height: text::LineHeight::Absolute(Pixels(height)),
        font: if italic {
            font::MONO_ITALIC
        } else {
            font::MONO
        },
        // A terminal's text starts on the left whatever the app's language.
        align_x: text::Alignment::Left,
        align_y: iced::alignment::Vertical::Top,
        shaping: if ascii {
            text::Shaping::Basic
        } else {
            text::Shaping::Advanced
        },
        wrapping: text::Wrapping::None,
    });
    if !ascii {
        let width = paragraph.min_width().ceil();
        paragraph.resize(Size::new(width, height));
    }
    paragraph
}

/// Whether the monospaced face draws `c` (its character map was read):
/// such text goes in runs.
fn in_face(c: char) -> bool {
    matches!(c, ' '..='~' | '\u{a0}'..='\u{ac}' | '\u{ae}'..='\u{ff}')
}

fn hash_colour<H: Hasher>(colour: Wanted, hasher: &mut H) {
    match colour {
        Wanted::Named(named) => (0_u8, named as u16).hash(hasher),
        Wanted::Spec(rgb) => (1_u8, rgb.r, rgb.g, rgb.b).hash(hasher),
        Wanted::Indexed(index) => (2_u8, index).hash(hasher),
    }
}

/// What a row's cells hold, as a number.
fn signature(row: &GridRow<Cell>, cols: usize, palette: u64) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    palette.hash(&mut hasher);
    for col in 0..cols {
        let cell = &row[Column(col)];
        cell.c.hash(&mut hasher);
        cell.flags.bits().hash(&mut hasher);
        hash_colour(cell.fg, &mut hasher);
        hash_colour(cell.bg, &mut hasher);
        if let Some(extra) = cell.zerowidth() {
            extra.hash(&mut hasher);
        }
        if let Some(link) = cell.hyperlink() {
            link.uri().hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// The colours a program set, as a number: a row drawn in them is drawn
/// again when they change.
fn palette_signature(colours: &Colors) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for index in 0..alacritty_terminal::term::color::COUNT {
        if let Some(rgb) = colours[index] {
            (index, rgb.r, rgb.g, rgb.b).hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// A run of cells being gathered into a piece.
struct Run {
    col: usize,
    cells: usize,
    style: Style,
    text: String,
}

/// Whether cells of `style` with nothing written show anything.
fn shows_blank(style: Style) -> bool {
    style.bg != crate::ui::widgets::log::ansi::Colour::Default
        || style.attrs.has(Attrs::INVERSE)
        || style.attrs.has(Attrs::UNDERLINE)
        || style.attrs.has(Attrs::STRIKE)
}

fn flush(run: Option<Run>, glyphs: &mut Glyphs, pieces: &mut Vec<Piece>) {
    let Some(run) = run else { return };
    let written = run.text.trim_end_matches(' ');
    let kind = if written.is_empty() {
        if !shows_blank(run.style) {
            return;
        }
        Kind::Blank
    } else {
        Kind::Text {
            paragraph: glyphs.get(written, run.style.attrs.has(Attrs::ITALIC)),
            inset: 0.0,
        }
    };
    pieces.push(Piece {
        col: run.col,
        cells: run.cells,
        style: run.style,
        kind,
    });
}

/// Web and mail links in text: what a terminal underlines and opens.
static LINKS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:https?://|mailto:)[^\s<>"'`{}|\\^\x00-\x1f]+"#).expect("a valid pattern")
});

/// The links written in a row's text, by cells, and those a program
/// named with OSC 8.
fn links(text: &str, cols_of: &[usize], row: &GridRow<Cell>, cols: usize) -> Vec<Link> {
    let mut found = vec![];
    let mut col = 0;
    while col < cols {
        let Some(link) = row[Column(col)].hyperlink() else {
            col += 1;
            continue;
        };
        let from = col;
        while col < cols && row[Column(col)].hyperlink().as_ref() == Some(&link) {
            col += 1;
        }
        found.push(Link {
            from,
            to: col,
            uri: link.uri().to_owned(),
        });
    }
    if !(text.contains("://") || text.contains("mailto:")) {
        return found;
    }
    for written in LINKS.find_iter(text) {
        let mut uri = written.as_str();
        // What ends a sentence or closes a bracket is not the link's.
        loop {
            let trimmed = uri.trim_end_matches(['.', ',', ';', ':', '!', '?', '\'', '"']);
            let unbalanced = |open: char, close: char| {
                trimmed.ends_with(close)
                    && trimmed.matches(open).count() < trimmed.matches(close).count()
            };
            let trimmed = if unbalanced('(', ')') || unbalanced('[', ']') {
                &trimmed[..trimmed.len() - 1]
            } else {
                trimmed
            };
            if trimmed.len() == uri.len() {
                break;
            }
            uri = trimmed;
        }
        let start = written.start();
        let end = start + uri.len();
        let (Some(&from), Some(&last)) = (cols_of.get(start), cols_of.get(end - 1)) else {
            continue;
        };
        if found
            .iter()
            .any(|link| link.from < last + 1 && from < link.to)
        {
            continue;
        }
        found.push(Link {
            from,
            to: last + 1,
            uri: uri.to_owned(),
        });
    }
    found
}

/// The pieces of a grid row of `cols` cells.
fn build(
    row: &GridRow<Cell>,
    line: Line,
    cols: usize,
    colours: &Colors,
    glyphs: &mut Glyphs,
    cell: f32,
) -> Row {
    let mut pieces = vec![];
    let mut text = String::with_capacity(cols);
    // The cell of each byte of `text`.
    let mut cols_of = Vec::with_capacity(cols);
    let mut run: Option<Run> = None;
    for col in 0..cols {
        let grid_cell = &row[Column(col)];
        let flags = grid_cell.flags;
        if flags.contains(Flags::WIDE_CHAR_SPACER) {
            continue;
        }
        let cells = if flags.contains(Flags::WIDE_CHAR) {
            2
        } else {
            1
        };
        let style = colors::style(grid_cell, colours);
        let blank = flags.intersects(Flags::HIDDEN | Flags::LEADING_WIDE_CHAR_SPACER)
            || grid_cell.c == '\0'
            || grid_cell.c == '\t';
        let c = if blank { ' ' } else { grid_cell.c };
        let marks = grid_cell.zerowidth().filter(|_| !blank).unwrap_or_default();
        text.push(c);
        text.extend(marks);
        cols_of.resize(text.len(), col);
        if marks.is_empty() && in_face(c) {
            if let Some(open) = run.as_mut().filter(|open| open.style == style) {
                open.cells += 1;
                open.text.push(c);
                continue;
            }
            flush(run.take(), glyphs, &mut pieces);
            run = Some(Run {
                col,
                cells: 1,
                style,
                text: c.to_string(),
            });
            continue;
        }
        flush(run.take(), glyphs, &mut pieces);
        let kind = if marks.is_empty() && boxes::is_drawn(c) {
            Kind::Drawn(c)
        } else {
            let mut written = c.to_string();
            written.extend(marks);
            let paragraph = glyphs.get(&written, style.attrs.has(Attrs::ITALIC));
            // Centred in its cells, however wide the fallback font drew it.
            #[allow(clippy::cast_precision_loss)]
            let room = cells as f32 * cell;
            let inset = ((room - paragraph.min_width()) / 2.0).max(0.0).floor();
            Kind::Text { paragraph, inset }
        };
        pieces.push(Piece {
            col,
            cells,
            style,
            kind,
        });
    }
    flush(run, glyphs, &mut pieces);
    let links = links(&text, &cols_of, row, cols);
    Row {
        line,
        signature: 0,
        pieces,
        links,
        text: text.trim_end().to_owned(),
    }
}

/// Lays out the rows `state` shows, from what the last layout had.
pub fn lay(state: &State, previous: Laid, glyphs: &mut Glyphs, geometry: &Geometry) -> Laid {
    let term = state.term();
    let grid = term.grid();
    let offset = grid.display_offset();
    let cols = term.columns();
    let screen = term.screen_lines();
    let colours = term.colors();
    let palette = palette_signature(colours);
    glyphs.begin(state.font_size(), geometry.cell.height);
    let mut kept: HashMap<u64, Vec<Row>> = HashMap::with_capacity(previous.rows.len());
    for row in previous.rows {
        kept.entry(row.signature).or_default().push(row);
    }
    let mut rows = Vec::with_capacity(screen);
    for index in 0..screen {
        let line =
            Line(i32::try_from(index).unwrap_or(i32::MAX) - i32::try_from(offset).unwrap_or(0));
        let grid_row = &grid[line];
        let signature = signature(grid_row, cols, palette);
        let row = match kept.get_mut(&signature).and_then(Vec::pop) {
            Some(mut row) => {
                row.line = line;
                row
            }
            None => {
                let mut row = build(grid_row, line, cols, colours, glyphs, geometry.cell.width);
                row.signature = signature;
                row
            }
        };
        rows.push(row);
    }
    let cursor = cursor(state, &rows, glyphs);
    glyphs.end();
    Laid {
        rows,
        cursor,
        offset,
        history: term.history_size(),
    }
}

/// Where the cursor is drawn, if it shows in view.
fn cursor(state: &State, rows: &[Row], glyphs: &mut Glyphs) -> Option<Cursor> {
    let term = state.term();
    let content = term.renderable_content();
    let point = content.cursor.point;
    if content.cursor.shape == CursorShape::Hidden {
        return None;
    }
    let row = usize::try_from(point.line.0 + i32::try_from(content.display_offset).ok()?).ok()?;
    rows.get(row)?;
    let cell = &term.grid()[point];
    let cells = if cell.flags.contains(Flags::WIDE_CHAR) {
        2
    } else {
        1
    };
    let shown = !cell.flags.intersects(Flags::HIDDEN) && cell.c != ' ' && cell.c != '\0';
    let drawn = (shown && boxes::is_drawn(cell.c)).then_some(cell.c);
    let glyph = (shown && drawn.is_none()).then(|| {
        let mut written = cell.c.to_string();
        written.extend(cell.zerowidth().unwrap_or_default());
        glyphs.get(&written, cell.flags.contains(Flags::ITALIC))
    });
    Some(Cursor {
        row,
        col: point.column.0,
        cells,
        shape: content.cursor.shape,
        glyph,
        drawn,
    })
}
