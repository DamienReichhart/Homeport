//! The table under the keyboard: Tab, the arrows, the keys of the rows,
//! and who has focus and whether it shows.

use iced::{
    keyboard::{key::Named, Key, Modifiers},
    Point,
};

use super::bench::{has_rows_focused, hold, press_of, row_of, Bench, NONE, WINDOW};
use crate::{
    test_support::{snapshot::in_language, stage::Stage},
    ui::widgets::{
        gallery::table::{samples, source, view, Show},
        table::{Event as Outcome, Order, Sort, State, HEADER_HEIGHT, ROW_HEIGHT},
    },
};

#[test]
fn tab_reaches_the_headers_then_the_rows_as_one_stop_and_leaves() {
    let mut bench = Bench::new(40);
    bench.frame(|frame| {
        frame.tab();
        let checkbox = frame.focused().expect("the checkbox over the rows");
        assert!(checkbox.width <= 16.0 && frame.shows_focus());
        frame.tab();
        assert!(frame.is_focused("Name"), "a header that sorts is a button");
        for header in ["Image", "Status", "CPU", "Memory"] {
            frame.tab();
            assert!(frame.is_focused(header));
        }
        frame.tab();
        assert!(
            has_rows_focused(frame),
            "then the rows, all of them at once"
        );
        assert!(frame.shows_focus());
        frame.tab();
        assert!(frame.focused().is_none(), "Tab leaves: no stop in a row");
    });
}

#[test]
fn the_arrows_move_the_focused_row_and_scroll_it_into_view() {
    let mut bench = Bench::new(1000);
    bench.tab_to_rows();
    assert_eq!(bench.state.focused(), None);
    for expected in 0..30 {
        bench.key(Named::ArrowDown, NONE);
        assert_eq!(bench.state.focused(), Some(expected));
    }
    assert!(bench.state.offset() > 0.0, "the 30th row was out of view");
    bench.frame(|frame| {
        let row = row_of(frame, "queue-30");
        assert!(row.y + row.height <= WINDOW.1 - 24.0, "it shows whole");
        assert!(frame.find("web-1").is_none(), "the first rows are gone");
        assert!(has_rows_focused(frame) && frame.shows_focus());
    });
    bench.key(Named::End, NONE);
    assert_eq!(bench.state.focused(), Some(999));
    bench.frame(|frame| {
        row_of(frame, "auth-1000");
    });
    bench.key(Named::PageUp, NONE);
    let page = bench.state.focused().expect("a focused row");
    assert!((975..990).contains(&page), "a screen of rows up: {page}");
    bench.key(Named::Home, NONE);
    assert_eq!(bench.state.offset(), 0.0);
    assert!(bench.state.selection().is_empty(), "moving selects nothing");
}

#[test]
fn shift_extends_space_toggles_ctrl_a_selects_all_and_escape_clears() {
    let mut bench = Bench::new(40);
    bench.tab_to_rows();
    bench.key(Named::ArrowDown, NONE);
    bench.key(Named::ArrowDown, NONE);
    assert_eq!(bench.state.focused(), Some(1));
    bench.key(Named::ArrowDown, Modifiers::SHIFT);
    bench.key(Named::ArrowDown, Modifiers::SHIFT);
    assert_eq!(bench.selected(), [1, 2, 3]);
    bench.key(Named::ArrowUp, Modifiers::SHIFT);
    assert_eq!(bench.selected(), [1, 2]);
    bench.key(Named::Space, NONE);
    assert_eq!(bench.selected(), [1], "Space takes the focused row out");
    let outcome = bench.key(Key::Character("a".into()), Modifiers::COMMAND);
    assert_eq!(outcome, [Outcome::Selected]);
    assert_eq!(bench.state.selection().len(), 40);
    assert_eq!(bench.key(Named::Enter, NONE), [Outcome::Opened(2)]);
    bench.key(Named::Escape, NONE);
    assert!(bench.state.selection().is_empty());
    bench.frame(|frame| assert!(has_rows_focused(frame), "the rows keep focus"));
}

#[test]
fn the_menu_key_opens_the_rows_menu_and_escape_gives_the_row_its_focus_back() {
    let mut bench = Bench::new(40);
    bench.tab_to_rows();
    bench.key(Named::ArrowDown, NONE);
    bench.key(Named::ArrowDown, NONE);
    bench.key(Named::ContextMenu, NONE);
    assert_eq!(bench.menu(), Some(1));
    bench.frame(|frame| {
        assert!(frame.is_focused("Start"), "the menu has the keyboard");
        // The arrows are the menu's now: the row doesn't move.
        frame.tap_key(Named::ArrowDown);
        assert!(frame.is_focused("Stop"));
    });
    assert_eq!(bench.state.focused(), Some(1));
    bench.key(Named::Escape, NONE);
    assert_eq!(bench.menu(), None);
    bench.frame(|frame| {
        assert!(frame.find("Start").is_none(), "the menu is closed");
        assert!(has_rows_focused(frame), "focus is back on the rows");
        assert!(frame.shows_focus(), "with its ring");
    });
    // And the keyboard goes on from the same row.
    bench.key(Named::ArrowDown, NONE);
    assert_eq!(bench.state.focused(), Some(2));
}

#[test]
fn a_table_that_starts_sorted_shows_its_rows_in_that_order() {
    let _english = in_language("en");
    let rows = samples(40);
    let state = State::<u32>::new().sorted(Sort::by("cpu", Order::Descending));
    let mut stage = Stage::new(WINDOW);
    stage.frame(view(&state, source(&rows), Show::Rows), |frame| {
        // (37·id + 11) mod 1000, the largest first: ids 26, 25, 24, 23...
        let tops: Vec<f32> = ["97.3 %", "93.6 %", "89.9 %", "86.2 %", "82.5 %"]
            .iter()
            .map(|cpu| frame.find(cpu).unwrap_or_else(|| panic!("no {cpu}")).y)
            .collect();
        assert!(tops[0] < 36.0 + HEADER_HEIGHT + ROW_HEIGHT, "the first row");
        for pair in tops.windows(2) {
            assert_eq!(pair[1] - pair[0], ROW_HEIGHT, "one row under the other");
        }
        // And the arrow on its header says so.
        let cpu = frame.find("CPU").expect("the CPU header");
        assert!(cpu.y < 36.0 + HEADER_HEIGHT);
    });
}

#[test]
fn a_modifier_held_for_a_press_shows_no_ring_and_a_key_of_the_rows_does() {
    let mut bench = Bench::new(40);
    bench.frame(|frame| {
        frame.click("cache-3");
        // Ctrl goes down to add another row to the selection.
        frame.send(press_of(Named::Control.into(), Modifiers::CTRL));
        hold(frame, Modifiers::CTRL);
        assert!(has_rows_focused(frame));
        assert!(!frame.shows_focus(), "a modifier is not the keyboard");
        frame.send(press_of(Named::ArrowDown.into(), NONE));
        assert!(frame.shows_focus(), "an arrow is");
    });
}

#[test]
fn a_header_keeps_its_focus_from_the_very_first_view() {
    let mut bench = Bench::fresh("en", 12, WINDOW);
    // The first view of all: the press gives the header focus.
    let outcome = bench.frame(|frame| frame.click("Name"));
    assert_eq!(outcome, [Outcome::Arranged]);
    bench.frame(|frame| assert!(frame.is_focused("Name")));
}

#[test]
fn a_header_keeps_its_focus_while_the_rows_give_way_and_columns_go() {
    let mut bench = Bench::new(12);
    bench.frame(|frame| {
        frame.tab();
        frame.tab();
        assert!(frame.is_focused("Name"));
    });
    // The list is read again (skeletons), comes back empty, then whole.
    for show in [Show::Loading, Show::Empty, Show::Failed, Show::Rows] {
        bench.show = show;
        bench.frame(|frame| assert!(frame.is_focused("Name"), "{show:?}"));
    }
    // A column goes and comes back: the others keep what they had.
    bench.state.set_hidden("image", true);
    bench.frame(|frame| {
        assert!(frame.find("Image").is_none());
        assert!(frame.is_focused("Name"));
    });
    bench.state.set_hidden("image", false);
    bench.frame(|frame| assert!(frame.is_focused("Name") && frame.find("Image").is_some()));
}

#[test]
fn letters_go_to_the_row_that_starts_with_them() {
    let mut bench = Bench::new(40);
    bench.tab_to_rows();
    bench.key(Key::Character("w".into()), NONE);
    assert_eq!(bench.state.focused(), Some(0), "web-1");
    bench.key(Key::Character("o".into()), NONE);
    assert_eq!(bench.state.focused(), Some(4), "worker-5");
    bench.frame(|frame| assert!(frame.shows_focus()));
    // With Ctrl a letter is not typing: Ctrl+A selects all.
    bench.key(Key::Character("a".into()), Modifiers::COMMAND);
    assert_eq!(bench.state.focused(), Some(4));
    assert_eq!(bench.state.selection().len(), 40);
}

#[test]
fn a_group_closes_and_opens_from_the_keyboard_and_under_a_press() {
    let mut bench = Bench::fresh("en", 8, WINDOW);
    bench.grouped = true;
    bench.frame(|_| {});
    bench.frame(|frame| {
        assert!(frame.find("cache-3").is_some());
        frame.click("blog");
    });
    assert!(bench.state.is_collapsed("blog"));
    assert_eq!(bench.state.focused_group().as_deref(), Some("blog"));
    bench.frame(|frame| assert!(frame.find("cache-3").is_none(), "its rows are gone"));
    // The arrow towards where text ends opens it, the other closes it.
    bench.key(Named::ArrowRight, NONE);
    assert!(!bench.state.is_collapsed("blog"));
    bench.key(Named::ArrowLeft, NONE);
    assert!(bench.state.is_collapsed("blog"));
    bench.key(Named::Space, NONE);
    assert!(!bench.state.is_collapsed("blog"));
    bench.key(Named::Enter, NONE);
    assert!(bench.state.is_collapsed("blog"));
    bench.key(Named::Enter, NONE);
    bench.key(Named::ArrowDown, NONE);
    assert_eq!(bench.state.focused(), Some(2), "cache-3, its first row");
    // The checkbox of a group takes all its rows.
    bench.frame(|frame| {
        let shop = row_of(frame, "shop");
        frame.point_at(Point::new(24.0 + 1.0 + 16.0, shop.center_y()));
        frame.press();
    });
    assert_eq!(bench.selected(), [0, 3, 4, 7]);
    assert!(
        !bench.state.is_collapsed("shop"),
        "its checkbox doesn't close it"
    );
}

#[test]
fn right_to_left_the_arrows_of_a_group_are_the_other_way_round() {
    let mut bench = Bench::fresh("ar", 8, WINDOW);
    bench.grouped = true;
    bench.frame(|_| {});
    bench.frame(|frame| frame.click("blog"));
    assert!(bench.state.is_collapsed("blog"));
    bench.key(Named::ArrowLeft, NONE);
    assert!(!bench.state.is_collapsed("blog"), "towards where text ends");
    bench.key(Named::ArrowRight, NONE);
    assert!(bench.state.is_collapsed("blog"));
}
