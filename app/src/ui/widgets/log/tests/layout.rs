//! What a layout shows and what it costs, on a window that lasts: where
//! the way back to the latest line goes, what a row holds, and how much
//! of a very long line is walked.

use super::{
    driven::{row_of, Bench},
    numbered,
};
use crate::ui::widgets::log::Line;

#[test]
fn the_way_back_to_the_latest_line_covers_no_line_in_either_direction() {
    for language in ["en", "ar"] {
        let mut bench = Bench::in_the_language(language, numbered(0, 200));
        bench.frame(|frame| {
            let row = row_of(frame, "line 190");
            frame.point_at(row.center());
            frame.scroll(-2.0);
        });
        let label = crate::t!("ui.log.latest");
        let rows = bench.rows();
        bench.frame(|frame| {
            let button = frame.find(&label).expect("the way back shows");
            for text in &rows {
                let row = row_of(frame, text);
                assert!(
                    row.y + row.height <= button.y,
                    "{language}: {text} at {row:?} is under the button at {button:?}"
                );
            }
        });
    }
}

#[test]
fn a_line_a_carriage_return_restarted_shows_whole_with_or_without_its_timestamp() {
    let line =
        Line::new("2026-05-01T10:00:00Z pulling 10%\rpulling layer 2 of 5").timestamped(true);
    let mut bench = Bench::new([line]);
    bench.frame(|frame| assert!(frame.find("pulling layer 2 of 5").is_some()));
    bench.frame(|frame| frame.click("Timestamps"));
    bench.frame(|frame| {
        assert!(frame
            .find("2026-05-01T10:00:00Z pulling layer 2 of 5")
            .is_some());
    });
}

#[test]
fn no_row_holds_more_cells_than_the_viewer_has() {
    use crate::ui::widgets::log::line::cells;
    // Wide characters from an odd cell, so one would straddle each row's end.
    let wide = Line::new(format!("a{}", "日".repeat(400)));
    let mixed = Line::new("ok 🚀 done ✅ ".repeat(60));
    let mut bench = Bench::new([wide, mixed]);
    let cols = bench.state.metrics.cols;
    let mut said = vec![];
    bench.frame(|frame| said = frame.texts());
    let rows: Vec<&String> = said
        .iter()
        .filter(|text| text.contains('日') || text.contains('🚀'))
        .collect();
    assert!(rows.len() > 6, "{} rows", rows.len());
    for row in rows {
        assert!(cells(row) <= cols, "{} cells in {cols}: {row}", cells(row));
    }
}

/// How much a layout walks of a line of a megabyte, counted.
#[test]
fn a_megabyte_line_with_a_hundred_thousand_matches_is_laid_out_by_what_shows() {
    use crate::ui::widgets::log::{index::WALKED, Message, Scroll};
    let walked = || WALKED.with(std::cell::Cell::get);
    // One accent, so no cell of it is a byte.
    let text = format!("é{}", "abcdefghi ".repeat(100_000));
    let bytes = text.len();
    let before = walked();
    let mut bench = Bench::new([Line::new(text)]);
    let (cols, rows) = (bench.state.metrics.cols, bench.state.metrics.rows);
    super::drive(&mut bench.state, Message::Query("abc".to_owned()));
    bench.frame(|_| {});
    assert_eq!(bench.state.matches().1, 100_000);
    // The first layouts mark the line: its characters, once or twice.
    let first = walked() - before;
    assert!(first <= 3 * bytes, "{first} characters walked for {bytes}");
    // From then on a layout walks what it shows, wherever that is: a
    // screen of cells and a mark's distance for each of its ends.
    let screen = 4 * (cols * rows + 4 * 128);
    for to in [
        Scroll::Rows(-1),
        Scroll::To(0.5),
        Scroll::Rows(7),
        Scroll::Start,
    ] {
        let _ = bench.state.update(Message::Scroll(to));
        let before = walked();
        bench.frame(|_| {});
        let step = walked() - before;
        assert!(
            step <= screen,
            "{step} characters walked for {to:?}, over {screen}"
        );
    }
    // Stepping through its matches walks no more.
    let before = walked();
    for _ in 0..50 {
        let _ = bench.state.update(Message::Next);
    }
    assert!(walked() - before <= 50 * 4 * 128, "{}", walked() - before);
}
