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

/// The width of every column in a table `available` pixels wide. Fixed and
/// resized columns take theirs; the others share what is left by their
/// portions, none going under its minimum or over its maximum. When the
/// minimums don't fit, the columns are wider than the table together and
/// the last ones are cut.
pub fn resolve(tracks: &[Track], available: f32) -> Vec<f32> {
    let mut widths: Vec<Option<f32>> = tracks
        .iter()
        .map(|track| match (track.resized, track.width) {
            (Some(width), _) | (None, Width::Fixed(width)) => Some(track.clamp(width)),
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
    widths
        .into_iter()
        .map(|width| width.unwrap_or(0.0).round())
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fill(portion: u16) -> Track {
        Track {
            width: Width::Fill(portion),
            min: MIN_WIDTH,
            max: f32::INFINITY,
            resized: None,
            resizable: true,
            align: Align::Start,
            padding: CELL_PADDING,
        }
    }

    fn fixed(width: f32) -> Track {
        Track {
            width: Width::Fixed(width),
            ..fill(1)
        }
    }

    #[test]
    fn fixed_columns_take_theirs_and_the_others_share_the_rest() {
        let tracks = [Track::exactly(32.0), fill(2), fixed(120.0), fill(1)];
        assert_eq!(resolve(&tracks, 752.0), [32.0, 400.0, 120.0, 200.0]);
        // Nothing to share: only what is fixed.
        assert_eq!(resolve(&[fixed(100.0), fixed(50.0)], 800.0), [100.0, 50.0]);
        assert_eq!(resolve(&[], 800.0), Vec::<f32>::new());
    }

    #[test]
    fn a_share_stops_at_its_bounds_and_the_others_take_what_that_leaves() {
        let capped = Track {
            max: 100.0,
            ..fill(1)
        };
        assert_eq!(resolve(&[capped, fill(1)], 600.0), [100.0, 500.0]);
        let floored = Track {
            min: 300.0,
            ..fill(1)
        };
        assert_eq!(
            resolve(&[floored, fill(1), fill(1)], 600.0),
            [300.0, 150.0, 150.0]
        );
        // Too narrow for the minimums: each keeps its own, and they overflow.
        assert_eq!(resolve(&[fill(1), fill(1)], 60.0), [48.0, 48.0]);
    }

    #[test]
    fn a_width_the_user_gave_wins_and_stays_within_the_bounds() {
        let dragged = Track {
            resized: Some(250.0),
            ..fill(1)
        };
        assert_eq!(resolve(&[dragged, fill(1)], 600.0), [250.0, 350.0]);
        let too_small = Track {
            resized: Some(10.0),
            ..fixed(120.0)
        };
        assert_eq!(resolve(&[too_small], 600.0), [48.0]);
    }

    #[test]
    fn a_drag_resizes_within_the_bounds_and_the_other_way_round_in_rtl() {
        let track = Track {
            max: 300.0,
            ..fill(1)
        };
        assert_eq!(resized(&track, 200.0, 40.0, false), 240.0);
        assert_eq!(resized(&track, 200.0, -40.0, false), 160.0);
        assert_eq!(resized(&track, 200.0, -500.0, false), 48.0, "its minimum");
        assert_eq!(resized(&track, 200.0, 500.0, false), 300.0, "its maximum");
        // Right to left, the column ends on its left: dragging left widens.
        assert_eq!(resized(&track, 200.0, -40.0, true), 240.0);
        assert_eq!(resized(&track, 200.0, 40.0, true), 160.0);
        assert_eq!(resized(&Track::exactly(32.0), 32.0, 90.0, false), 32.0);
    }

    #[test]
    fn columns_start_on_the_left_or_on_the_right_in_rtl() {
        let widths = [32.0, 400.0, 120.0];
        assert_eq!(offsets(&widths, 600.0, false), [0.0, 32.0, 432.0]);
        // Mirrored: the first column ends at the right edge, and what the
        // columns leave is on the left.
        assert_eq!(offsets(&widths, 600.0, true), [568.0, 168.0, 48.0]);
    }
}
