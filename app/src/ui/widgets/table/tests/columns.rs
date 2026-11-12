//! The arithmetic of the columns' widths.

use crate::ui::widgets::table::column::{
    offsets, resized, resolve, room, Align, Track, Width, CELL_PADDING, MIN_WIDTH,
};

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
fn shares_are_whole_pixels_that_add_up_to_the_table() {
    // 718 px in portions of 3, 2, 5 and 2: 179.5 each for the twos.
    let tracks = [fill(3), fill(2), fill(5), fill(2)];
    let widths = resolve(&tracks, 718.0);
    assert_eq!(widths.iter().sum::<f32>(), 718.0, "{widths:?}");
    assert!(widths.iter().all(|width| width.fract() == 0.0));
    for available in [301.0, 333.0, 500.0, 719.0, 1001.0] {
        let total: f32 = resolve(&tracks, available).iter().sum();
        assert_eq!(total, available, "no pixel to scroll sideways");
    }
}

#[test]
fn the_pixel_of_the_rounding_goes_to_a_column_that_has_room_for_it() {
    // 301 px: the last column keeps its least, 150, and the two others
    // share 151, 75.5 each: rounded, a pixel too many, which the last one
    // can't give.
    let least = Track {
        min: 150.0,
        ..fill(1)
    };
    let widths = resolve(&[fill(1), fill(1), least], 301.0);
    assert_eq!(widths, [76.0, 75.0, 150.0]);
    assert_eq!(
        widths.iter().sum::<f32>(),
        301.0,
        "no pixel to scroll sideways"
    );
    // Nor can one at its most take the pixel that is missing.
    let most = Track {
        max: 100.0,
        ..fill(4)
    };
    let widths = resolve(&[fill(1), fill(1), most], 301.0);
    assert_eq!(widths.iter().sum::<f32>(), 301.0, "{widths:?}");
}

#[test]
fn a_resized_column_takes_no_more_than_the_others_least_leaves() {
    // 600 px: the checkboxes take 32 and the other column at least 48.
    let dragged = |width| Track {
        resized: Some(width),
        ..fill(1)
    };
    let tracks = [Track::exactly(32.0), dragged(5000.0), fill(1)];
    assert_eq!(room(&tracks, 1, 600.0), 520.0);
    assert_eq!(resolve(&tracks, 600.0), [32.0, 520.0, 48.0]);
    // Two of them: the first is served first, and nothing overflows.
    let both = [dragged(400.0), dragged(400.0), fixed(100.0)];
    assert_eq!(resolve(&both, 600.0), [400.0, 100.0, 100.0]);
    assert_eq!(resolve(&both, 600.0).iter().sum::<f32>(), 600.0);
    // No room at all: every column keeps its least, and the table
    // scrolls sideways rather than cutting one out.
    let tight = [dragged(400.0), fixed(100.0), fill(1)];
    assert_eq!(resolve(&tight, 150.0), [48.0, 100.0, 48.0]);
    assert_eq!(room(&tight, 0, 150.0), 48.0);
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
