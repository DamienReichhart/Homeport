//! A table's columns, as data: what each is called, how wide it may be,
//! how it sorts and what it shows for a row. And the arithmetic of their
//! widths, which is the same whichever way the language reads.

use std::cmp::Ordering;

use crate::ui::theme::Element;

/// A column's name in the code: what the sort and the widths a page keeps
/// are filed under.
pub type ColumnId = &'static str;

/// Narrower than this, a column can't be told from its neighbours.
pub const MIN_WIDTH: f32 = 48.0;
/// `px-2`: the room on each side of a cell.
pub const CELL_PADDING: f32 = 8.0;

/// How wide a column is before anyone resizes it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Width {
    /// So many pixels.
    Fixed(f32),
    /// A share of what the fixed columns leave, as CSS's `flex-grow`.
    Fill(u16),
}

/// Where a cell's content goes in its column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    /// Where text starts: the left, or the right in a right-to-left language.
    #[default]
    Start,
    Center,
    /// Where text ends: numbers (`text-end`).
    End,
}

type Compare<'a, T> = Box<dyn Fn(&T, &T) -> Ordering + 'a>;
type Cell<'a, T, Message> = Box<dyn Fn(&T) -> Element<'a, Message> + 'a>;
type Typed<'a, T> = Box<dyn Fn(&T) -> String + 'a>;

/// One column of a table over rows of `T`.
pub struct Column<'a, T, Message> {
    pub(super) id: ColumnId,
    pub(super) header: String,
    pub(super) width: Width,
    pub(super) min: f32,
    pub(super) max: f32,
    pub(super) align: Align,
    pub(super) resizable: bool,
    pub(super) compare: Option<Compare<'a, T>>,
    pub(super) descending_first: bool,
    pub(super) typed: Option<Typed<'a, T>>,
    pub(super) hideable: bool,
    pub(super) cell: Cell<'a, T, Message>,
}

impl<'a, T, Message> Column<'a, T, Message> {
    /// The column `id`, titled `header` (a catalogue's message: the columns
    /// are built in `view`, so it is in the language of the moment), which
    /// shows `cell` for a row. It takes a share of the table's width until
    /// told otherwise.
    pub fn new(
        id: ColumnId,
        header: impl Into<String>,
        cell: impl Fn(&T) -> Element<'a, Message> + 'a,
    ) -> Self {
        Self {
            id,
            header: header.into(),
            width: Width::Fill(1),
            min: MIN_WIDTH,
            max: f32::INFINITY,
            align: Align::Start,
            resizable: true,
            compare: None,
            descending_first: false,
            typed: None,
            hideable: true,
            cell: Box::new(cell),
        }
    }

    #[must_use]
    pub fn width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    /// The narrowest and the widest it gets, shared or resized.
    #[must_use]
    pub fn between(mut self, min: f32, max: f32) -> Self {
        self.min = min.max(0.0);
        self.max = max.max(self.min);
        self
    }

    #[must_use]
    pub fn min_width(mut self, min: f32) -> Self {
        self.min = min.max(0.0);
        self.max = self.max.max(self.min);
        self
    }

    #[must_use]
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Whether the edge of its header can be dragged; it can unless told.
    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Makes the column sortable: its header becomes a button, and
    /// `compare` says which of two rows comes first when ascending.
    #[must_use]
    pub fn sort_by(mut self, compare: impl Fn(&T, &T) -> Ordering + 'a) -> Self {
        self.compare = Some(Box::new(compare));
        self
    }

    /// The first press on its header sorts the largest first, as TanStack
    /// does for a column of numbers.
    #[must_use]
    pub fn descending_first(mut self) -> Self {
        self.descending_first = true;
        self
    }

    /// Makes it the column typing looks in: letters typed while the rows
    /// have focus go to the next row whose `text` starts with them. The
    /// letters are then the table's, not a page shortcut's.
    #[must_use]
    pub fn typed(mut self, text: impl Fn(&T) -> String + 'a) -> Self {
        self.typed = Some(Box::new(text));
        self
    }

    /// Whether the user may hide it; they may unless told (the name of a
    /// row stays).
    #[must_use]
    pub fn hideable(mut self, hideable: bool) -> Self {
        self.hideable = hideable;
        self
    }

    pub fn id(&self) -> ColumnId {
        self.id
    }

    /// What its header says.
    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn is_hideable(&self) -> bool {
        self.hideable
    }

    pub(super) fn track(&self, resized: Option<f32>) -> Track {
        Track {
            width: self.width,
            min: self.min,
            max: self.max,
            resized,
            resizable: self.resizable,
            align: self.align,
            padding: CELL_PADDING,
        }
    }
}

/// What the layout knows of a column.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Track {
    pub width: Width,
    pub min: f32,
    pub max: f32,
    /// The width the user gave it, which wins over `width`.
    pub resized: Option<f32>,
    pub resizable: bool,
    pub align: Align,
    /// The room on each side of what a cell shows (`px-2`).
    pub padding: f32,
}

impl Track {
    /// A column of exactly `width` that nobody resizes: the checkboxes'.
    pub fn exactly(width: f32) -> Self {
        Self {
            width: Width::Fixed(width),
            min: width,
            max: width,
            resized: None,
            resizable: false,
            align: Align::Center,
            padding: 0.0,
        }
    }

    fn clamp(&self, width: f32) -> f32 {
        width.clamp(self.min, self.max.max(self.min))
    }
}

impl Track {
    /// The least it takes: its minimum, or its whole width when it is
    /// fixed and nobody resized it.
    fn floor(&self) -> f32 {
        match (self.resized, self.width) {
            (None, Width::Fixed(width)) => self.clamp(width),
            _ => self.min,
        }
    }
}

/// The widest the column `index` may be dragged in a table `available`
/// pixels wide: what the least of every other column leaves, so no drag
/// pushes a column, or the edge being dragged, out of the table.
pub fn room(tracks: &[Track], index: usize, available: f32) -> f32 {
    let others: f32 = tracks
        .iter()
        .enumerate()
        .filter(|(other, _)| *other != index)
        .map(|(_, track)| track.floor())
        .sum();
    let track = &tracks[index];
    track.clamp((available - others).max(track.min))
}

/// The width of every column in a table `available` pixels wide. Fixed
/// columns take theirs; resized ones take what the user gave them as far
/// as the others' least leaves room; the rest share what is left by their
/// portions, none going under its minimum or over its maximum. When the
/// least of every column doesn't fit, each keeps its least and the table
/// scrolls sideways.
pub fn resolve(tracks: &[Track], available: f32) -> Vec<f32> {
    let floors: f32 = tracks.iter().map(Track::floor).sum();
    // What the resized columns may take over their least, the first first.
    let mut spare = (available - floors).max(0.0);
    let mut widths: Vec<Option<f32>> = tracks
        .iter()
        .map(|track| match (track.resized, track.width) {
            (Some(width), _) => {
                let more = (track.clamp(width) - track.min).clamp(0.0, spare);
                spare -= more;
                Some(track.min + more)
            }
            (None, Width::Fixed(width)) => Some(track.clamp(width)),
            (None, Width::Fill(_)) => None,
        })
        .collect();
    let portion = |track: &Track| match track.width {
        Width::Fill(portion) => f32::from(portion.max(1)),
        Width::Fixed(_) => 0.0,
    };
    // A share that hits a bound is settled there, and the others share
    // again what that leaves: at most one round per column.
    loop {
        let taken: f32 = widths.iter().flatten().sum();
        let open: f32 = tracks
            .iter()
            .zip(&widths)
            .filter(|(_, width)| width.is_none())
            .map(|(track, _)| portion(track))
            .sum();
        if open <= 0.0 {
            break;
        }
        let unit = (available - taken).max(0.0) / open;
        let mut bound = None;
        for (index, track) in tracks.iter().enumerate() {
            if widths[index].is_some() {
                continue;
            }
            let share = unit * portion(track);
            let kept = track.clamp(share);
            // The one furthest from its share is settled first.
            let off = (kept - share).abs();
            if off > 0.0 && bound.is_none_or(|(_, _, worst)| off > worst) {
                bound = Some((index, kept, off));
            }
        }
        match bound {
            Some((index, kept, _)) => widths[index] = Some(kept),
            None => {
                for (index, track) in tracks.iter().enumerate() {
                    if widths[index].is_none() {
                        widths[index] = Some(unit * portion(track));
                    }
                }
            }
        }
    }
    let mut widths: Vec<f32> = widths
        .into_iter()
        .map(|width| width.unwrap_or(0.0).round())
        .collect();
    // Whole pixels each, and the pixels the rounding made or lost go to
    // the last column that shares: together they are the table's width,
    // not a pixel more (which would be a pixel to scroll sideways).
    let off = available.round() - widths.iter().sum::<f32>();
    let shares = |track: &Track| track.resized.is_none() && matches!(track.width, Width::Fill(_));
    if off != 0.0 && off.abs() <= widths.len() as f32 {
        let last = tracks
            .iter()
            .rposition(|track| shares(track) && track.max > track.min);
        if let Some(last) = last {
            let fixed = widths[last] + off;
            if fixed >= tracks[last].min && fixed <= tracks[last].max {
                widths[last] = fixed;
            }
        }
    }
    widths
}

/// Where each column starts, from the left of the table: the first column
/// is on the left, or on the right when `rtl`, in a table `total` wide.
pub fn offsets(widths: &[f32], total: f32, rtl: bool) -> Vec<f32> {
    let mut at = 0.0;
    widths
        .iter()
        .map(|width| {
            let start = at;
            at += width;
            if rtl {
                total - start - width
            } else {
                start
            }
        })
        .collect()
}

/// The width of a column that was `from` wide once the edge it ends at was
/// dragged `dx` pixels to the right: wider, or narrower when `rtl` (its
/// end is its left edge), and within its bounds.
pub fn resized(track: &Track, from: f32, dx: f32, rtl: bool) -> f32 {
    let grown = if rtl { -dx } else { dx };
    track.clamp(from + grown).round()
}
