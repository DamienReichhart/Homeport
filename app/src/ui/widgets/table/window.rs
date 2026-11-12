//! Which rows show. Every row is as tall as the next, so what is in view
//! is arithmetic on how far the table is scrolled: only those rows (and a
//! few around them) are built, whatever the length of the list.

/// The rows `first..end` of the list, in the order they show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Window {
    pub first: usize,
    pub end: usize,
}

impl Window {
    pub fn len(self) -> usize {
        self.end - self.first
    }

    pub fn is_empty(self) -> bool {
        self.end == self.first
    }

    pub fn contains(self, index: usize) -> bool {
        (self.first..self.end).contains(&index)
    }
}

// Row counts are far below 2^24 pixels-per-row away from where an `f32`
// stops counting whole numbers (100,000 rows of 37 px are 3.7 million).
#[allow(clippy::cast_precision_loss)]
fn px(rows: usize) -> f32 {
    rows as f32
}

// A scroll offset is never negative and its row index fits a `usize`.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn rows(px: f32) -> usize {
    px.max(0.0) as usize
}

/// How tall `count` rows are.
pub fn extent(count: usize, row_height: f32) -> f32 {
    px(count) * row_height
}

/// The furthest the rows scroll in a body of `viewport` pixels.
pub fn max_offset(count: usize, row_height: f32, viewport: f32) -> f32 {
    (extent(count, row_height) - viewport).max(0.0)
}

/// `offset` kept within what the rows allow.
pub fn clamp(offset: f32, count: usize, row_height: f32, viewport: f32) -> f32 {
    offset.clamp(0.0, max_offset(count, row_height, viewport))
}

/// Where the row `index` starts, from the top of the list.
pub fn top_of(index: usize, row_height: f32) -> f32 {
    px(index) * row_height
}

/// The rows to build for a body of `viewport` pixels scrolled by `offset`:
/// those in view, and `overscan` more on each side so a scroll step shows
/// rows that are already there.
pub fn window(
    offset: f32,
    viewport: f32,
    row_height: f32,
    count: usize,
    overscan: usize,
) -> Window {
    if count == 0 || row_height <= 0.0 {
        return Window::default();
    }
    let offset = clamp(offset, count, row_height, viewport);
    let first = rows((offset / row_height).floor());
    let end = rows(((offset + viewport.max(0.0)) / row_height).ceil());
    Window {
        first: first.saturating_sub(overscan).min(count),
        end: (end + overscan).min(count),
    }
}

/// The offset that brings the row `index` into view, moving as little as
/// possible (`scrollIntoView({ block: 'nearest' })`): unchanged when the
/// row already shows whole.
pub fn reveal(index: usize, offset: f32, viewport: f32, row_height: f32) -> f32 {
    let top = top_of(index, row_height);
    let bottom = top + row_height;
    if top < offset {
        top
    } else if bottom > offset + viewport {
        (bottom - viewport).max(0.0)
    } else {
        offset
    }
}

/// The row under a point `y` pixels below the top of the body.
pub fn row_at(y: f32, offset: f32, row_height: f32, count: usize) -> Option<usize> {
    if y < 0.0 || row_height <= 0.0 {
        return None;
    }
    let index = rows(((offset + y) / row_height).floor());
    (index < count).then_some(index)
}

/// How many rows a Page Up or a Page Down goes: those that show whole, one
/// at least.
pub fn page(viewport: f32, row_height: f32) -> usize {
    if row_height <= 0.0 {
        return 1;
    }
    rows((viewport / row_height).floor()).max(1)
}

/// The scrollbar's thumb in a track of `track` pixels: where it starts and
/// how long it is. `None` when everything shows.
pub fn thumb(
    offset: f32,
    viewport: f32,
    extent: f32,
    track: f32,
    least: f32,
) -> Option<(f32, f32)> {
    if extent <= viewport || viewport <= 0.0 {
        return None;
    }
    let length = (track * viewport / extent).max(least).min(track);
    let travel = track - length;
    let scrolled = (offset / (extent - viewport)).clamp(0.0, 1.0);
    Some((travel * scrolled, length))
}

/// The offset a thumb dragged to `top` in its track stands for.
pub fn offset_of_thumb(top: f32, viewport: f32, extent: f32, track: f32, least: f32) -> f32 {
    let Some((_, length)) = thumb(0.0, viewport, extent, track, least) else {
        return 0.0;
    };
    let travel = track - length;
    if travel <= 0.0 {
        return 0.0;
    }
    (top / travel).clamp(0.0, 1.0) * (extent - viewport)
}

/// Whether the view follows the row that was at its top, now `moved` rows
/// away, in a list that grew by `grown` rows: it does when rows added or
/// removed above explain the move, so what was looked at stays in place. A
/// row that moved because the order changed is not followed.
pub fn follows(moved: isize, grown: isize) -> bool {
    moved != 0 && moved.signum() == grown.signum() && moved.abs() <= grown.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW: f32 = 37.0;

    #[test]
    fn the_window_is_what_shows_and_a_few_rows_around() {
        // 720 px show rows 0..20 (19.46 rows); four more are built below.
        assert_eq!(
            window(0.0, 720.0, ROW, 100_000, 4),
            Window { first: 0, end: 24 }
        );
        // Scrolled by ten rows and a half: rows 10..30 show.
        let scrolled = window(10.5 * ROW, 720.0, ROW, 100_000, 4);
        assert_eq!(scrolled, Window { first: 6, end: 34 });
        assert!(scrolled.len() < 30, "a constant number of rows");
        // A short list is built whole, and never past its end.
        assert_eq!(window(0.0, 720.0, ROW, 3, 4), Window { first: 0, end: 3 });
        assert!(window(0.0, 720.0, ROW, 0, 4).is_empty());
    }

    #[test]
    fn the_window_ends_with_the_list_however_far_it_was_scrolled() {
        let count = 100_000;
        let last = window(f32::MAX, 720.0, ROW, count, 4);
        assert_eq!(last.end, count);
        assert!(last.contains(count - 1));
        assert!(last.first >= count - 25, "{last:?}");
        // The extent is the rows', so the last row ends where scrolling does.
        assert_eq!(extent(count, ROW), 3_700_000.0);
        assert_eq!(max_offset(count, ROW, 720.0), 3_700_000.0 - 720.0);
        assert_eq!(max_offset(3, ROW, 720.0), 0.0);
        assert_eq!(clamp(-5.0, count, ROW, 720.0), 0.0);
    }

    #[test]
    fn a_row_is_revealed_by_the_shortest_scroll() {
        // In view: nothing moves.
        assert_eq!(reveal(3, 0.0, 370.0, ROW), 0.0);
        // Below: it becomes the last row that shows.
        assert_eq!(reveal(10, 0.0, 370.0, ROW), 11.0 * ROW - 370.0);
        // Above: it becomes the first.
        assert_eq!(reveal(2, 500.0, 370.0, ROW), 2.0 * ROW);
        // Half hidden at the top counts as hidden.
        assert_eq!(reveal(1, 50.0, 370.0, ROW), ROW);
    }

    #[test]
    fn a_point_falls_on_the_row_drawn_there() {
        assert_eq!(row_at(0.0, 0.0, ROW, 10), Some(0));
        assert_eq!(row_at(36.9, 0.0, ROW, 10), Some(0));
        assert_eq!(row_at(37.0, 0.0, ROW, 10), Some(1));
        assert_eq!(row_at(5.0, 100.0, ROW, 10), Some(2));
        assert_eq!(row_at(400.0, 0.0, ROW, 10), None, "under the last row");
        assert_eq!(row_at(-1.0, 0.0, ROW, 10), None);
    }

    #[test]
    fn a_page_is_the_rows_that_show_whole() {
        assert_eq!(page(370.0, ROW), 10);
        assert_eq!(page(380.0, ROW), 10);
        assert_eq!(page(10.0, ROW), 1);
    }

    #[test]
    fn the_thumb_follows_the_offset_and_back() {
        assert_eq!(thumb(0.0, 500.0, 400.0, 500.0, 20.0), None);
        let (top, length) = thumb(0.0, 500.0, 2000.0, 500.0, 20.0).expect("a thumb");
        assert_eq!((top, length), (0.0, 125.0));
        let (top, _) = thumb(1500.0, 500.0, 2000.0, 500.0, 20.0).expect("a thumb");
        assert_eq!(top, 375.0);
        // A very long list keeps a thumb one can grab.
        let (_, length) = thumb(0.0, 500.0, 3_700_000.0, 500.0, 20.0).expect("a thumb");
        assert_eq!(length, 20.0);
        assert_eq!(offset_of_thumb(375.0, 500.0, 2000.0, 500.0, 20.0), 1500.0);
        assert_eq!(offset_of_thumb(9999.0, 500.0, 2000.0, 500.0, 20.0), 1500.0);
    }

    #[test]
    fn the_view_follows_its_top_row_only_past_rows_added_or_removed() {
        assert!(follows(1, 1), "a row was added above");
        assert!(follows(-2, -3), "rows were removed above");
        assert!(!follows(30, 0), "the order changed");
        assert!(!follows(5, 1), "more than what was added explains");
        assert!(!follows(0, 1));
        assert!(!follows(-1, 1));
    }
}
