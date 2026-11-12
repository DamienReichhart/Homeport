//! The table under the pointer: presses, the wheel, the scrollbars, the
//! edges of the headers.

use iced::{
    advanced::mouse,
    keyboard::{self, key::Named, Modifiers},
    widget::container,
    Event, Point, Rectangle,
};

use super::{
    bench::{drag, has_rows_focused, hold, row_of, Bench, NONE, TOP, WINDOW},
    Page,
};
use crate::{
    t,
    ui::widgets::{
        gallery::table::{columns, source, view, Show},
        table::{Event as Outcome, Message, Order, State, HEADER_HEIGHT, ROW_HEIGHT},
    },
};

#[test]
fn a_press_selects_without_a_ring_and_a_double_press_opens() {
    let mut bench = Bench::new(40);
    let outcome = bench.frame(|frame| {
        frame.click("cache-3");
        assert!(has_rows_focused(frame));
        assert!(!frame.shows_focus(), "a press shows no ring");
    });
    assert_eq!(outcome, [Outcome::Selected]);
    assert_eq!(bench.selected(), [2]);
    assert_eq!(bench.state.focused(), Some(2));
    // Ctrl adds, Shift takes the range from the last one pressed.
    bench.frame(|frame| {
        frame.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
            Modifiers::COMMAND,
        ))]);
        frame.click("queue-6");
    });
    assert_eq!(bench.selected(), [2, 5]);
    bench.frame(|frame| {
        frame.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
            Modifiers::SHIFT,
        ))]);
        frame.click("api-7");
        frame.send([Event::Keyboard(keyboard::Event::ModifiersChanged(NONE))]);
    });
    assert_eq!(bench.selected(), [5, 6]);
    // The keyboard goes on from the row that was pressed, with its ring.
    bench.key(Named::ArrowDown, NONE);
    assert_eq!(bench.state.focused(), Some(7));
    bench.frame(|frame| assert!(frame.shows_focus()));
    let outcome = bench.frame(|frame| {
        frame.click("web-9");
        frame.press();
    });
    assert_eq!(outcome, [Outcome::Selected, Outcome::Opened(8)]);
}

#[test]
fn a_checkbox_toggles_its_row_and_the_one_over_them_all() {
    let mut bench = Bench::new(8);
    bench.frame(|frame| {
        let row = row_of(frame, "db-2");
        // The checkbox is at the start of the row.
        frame.point_at(Point::new(24.0 + 1.0 + 16.0, row.center_y()));
        frame.press();
    });
    assert_eq!(bench.selected(), [1]);
    assert_eq!(
        bench.state.focused(),
        None,
        "the row itself was not pressed"
    );
    let over_all = Point::new(24.0 + 1.0 + 16.0, TOP + 1.0 + HEADER_HEIGHT / 2.0);
    bench.frame(|frame| {
        frame.point_at(over_all);
        frame.press();
    });
    assert_eq!(bench.state.selection().len(), 8, "some selected: all");
    bench.frame(|frame| {
        frame.point_at(over_all);
        frame.press();
    });
    assert!(bench.state.selection().is_empty(), "all selected: none");
}

#[test]
fn a_header_sorts_and_shift_adds_a_column() {
    let mut bench = Bench::new(40);
    let outcome = bench.frame(|frame| frame.click("Name"));
    assert_eq!(outcome, [Outcome::Arranged]);
    assert_eq!(bench.state.sort().keys(), [("name", Order::Ascending)]);
    bench.frame(|frame| {
        // Sorted by name, digits by their value: api-7 first, not web-1
        // and not api-15.
        let first = row_of(frame, "api-7");
        assert!(first.y < TOP + HEADER_HEIGHT + ROW_HEIGHT);
        frame.click("Name");
    });
    assert_eq!(bench.state.sort().keys(), [("name", Order::Descending)]);
    // Enter and Space on the header that has focus do the same.
    bench.frame(|frame| {
        assert!(frame.is_focused("Name"));
        frame.tap_key(Named::Enter);
    });
    assert!(bench.state.sort().is_empty(), "the third press unsorts");
    bench.frame(|frame| frame.tap_key(Named::Space));
    assert_eq!(bench.state.sort().keys(), [("name", Order::Ascending)]);
    // With Shift, another column sorts after it; numbers start largest first.
    bench.frame(|frame| {
        frame.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
            Modifiers::SHIFT,
        ))]);
        frame.click("Memory");
    });
    assert_eq!(
        bench.state.sort().keys(),
        [("name", Order::Ascending), ("memory", Order::Descending)]
    );
}

#[test]
fn dragging_the_edge_of_a_header_resizes_its_column() {
    let mut bench = Bench::new(40);
    let mut image = Rectangle::default();
    let outcome = bench.frame(|frame| {
        image = frame.find("Image").expect("the Image header");
        // The Name column ends where the Image column starts, a cell's
        // padding before its text.
        let edge = Point::new(image.x - 8.0, image.center_y());
        frame.point_at(edge);
        frame.send([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        frame.point_at(Point::new(edge.x + 60.0, edge.y));
        frame.send([Event::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);
    });
    assert_eq!(outcome, [Outcome::Arranged]);
    assert!(bench.state.sort().is_empty(), "the header was not pressed");
    let [("name", width)] = *bench.state.widths() else {
        panic!("one column resized: {:?}", bench.state.widths());
    };
    bench.frame(|frame| {
        let moved = frame.find("Image").expect("the Image header");
        assert_eq!(moved.x, image.x + 60.0, "the next column starts further");
    });
    // It stops at the column's minimum.
    bench.frame(|frame| {
        let edge = Point::new(image.x + 60.0 - 8.0, image.center_y());
        frame.point_at(edge);
        frame.send([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        frame.point_at(Point::new(edge.x - 2000.0, edge.y));
        frame.send([Event::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);
    });
    assert_eq!(bench.state.widths(), [("name", 120.0)]);
    assert!(width > 120.0);
}

#[test]
fn right_to_left_a_column_ends_on_its_left_and_dragging_left_widens_it() {
    let mut bench = Bench::in_the_language("ar", 40);
    let (name, image) = (t!("containers.column.name"), t!("containers.column.image"));
    let mut before = Rectangle::default();
    bench.frame(|frame| {
        let first = frame.find(&name).expect("the Name header");
        before = frame.find(&image).expect("the Image header");
        assert!(first.x > before.x, "the first column is on the right");
        // The Name column ends where the Image column starts: on the
        // right of its header, a cell's padding after its text.
        let edge = Point::new(before.x + before.width + 8.0, before.center_y());
        frame.point_at(edge);
        frame.send([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        frame.point_at(Point::new(edge.x - 60.0, edge.y));
        frame.send([Event::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);
    });
    assert!(matches!(bench.state.widths(), [("name", _)]));
    bench.frame(|frame| {
        let after = frame.find(&image).expect("the Image header");
        assert_eq!(
            after.x + after.width,
            before.x + before.width - 60.0,
            "the Name column took 60 px from its left"
        );
    });
}

#[test]
fn the_wheel_scrolls_and_only_the_rows_in_view_are_built() {
    let mut bench = Bench::new(100_000);
    bench.frame(|frame| {
        assert!(frame.find("web-1").is_some());
        assert!(frame.find("web-41").is_none(), "far below: not built");
        frame.point_at(Point::new(400.0, 300.0));
        frame.scroll(10.0);
    });
    assert_eq!(bench.state.offset(), 600.0);
    bench.frame(|frame| {
        assert!(frame.find("web-1").is_none(), "scrolled away: not built");
        assert!(frame.find("web-25").is_some());
    });
    // The thumb, dragged to the end of its track, shows the last rows.
    let rail_x = WINDOW.0 - 24.0 - 1.0 - 5.0;
    let top = TOP + 1.0 + HEADER_HEIGHT;
    bench.frame(|frame| {
        frame.point_at(Point::new(rail_x, top + 4.0));
        frame.send([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        frame.point_at(Point::new(rail_x, WINDOW.1));
        frame.send([Event::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);
    });
    bench.frame(|frame| {
        assert!(frame.find("auth-100000").is_some());
    });
}

#[test]
fn a_rows_button_opens_its_menu_and_an_item_closes_it() {
    let mut bench = Bench::new(40);
    bench.frame(|frame| {
        let row = row_of(frame, "cache-3");
        // The button is at the end of the row.
        frame.point_at(Point::new(WINDOW.0 - 24.0 - 1.0 - 22.0, row.center_y()));
        frame.press();
    });
    assert_eq!(bench.menu(), Some(2));
    assert!(
        bench.state.selection().is_empty(),
        "the row was not pressed"
    );
    let mut chosen = vec![];
    let page = container(view(&bench.state, source(&bench.rows), Show::Rows)).padding(24);
    for message in bench.stage.frame(page, |frame| frame.click("Stop")) {
        match message {
            Page::Table(message) => {
                bench
                    .state
                    .update(message, &source(&bench.rows), &columns());
            }
            other => chosen.push(other),
        }
    }
    assert_eq!(bench.menu(), None);
    assert!(matches!(chosen[..], [Page::Chosen("Stop")]), "{chosen:?}");
}

#[test]
fn the_secondary_button_opens_the_menu_of_the_row_under_it() {
    let mut bench = Bench::new(40);
    bench.frame(|frame| {
        let row = row_of(frame, "queue-6");
        frame.point_at(row.center());
        frame.send([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Right,
        ))]);
    });
    assert_eq!(bench.menu(), Some(5));
    assert_eq!(bench.state.focused(), Some(5));
    bench.frame(|frame| assert!(frame.find("Remove").is_some()));
}

#[test]
fn a_column_dragged_past_the_table_stops_at_what_the_others_leave() {
    let mut bench = Bench::new(40);
    bench.frame(|frame| {
        let image = frame.find("Image").expect("the Image header");
        let edge = Point::new(image.x - 8.0, image.center_y());
        drag(frame, edge, Point::new(edge.x + 3000.0, edge.y));
    });
    let [("name", width)] = *bench.state.widths() else {
        panic!("one column resized: {:?}", bench.state.widths());
    };
    assert!(width < WINDOW.0, "not the 3000 px it was dragged: {width}");
    // Every column is still in the table, and so is the edge to drag back.
    let inside = WINDOW.0 - 24.0 - 1.0;
    let mut edge = Point::ORIGIN;
    bench.frame(|frame| {
        let image = frame.find("Image").expect("the Image header");
        let memory = frame.find("Memory").expect("the Memory header");
        assert!(memory.x + memory.width <= inside - 44.0, "before the menus");
        edge = Point::new(image.x - 8.0, image.center_y());
        assert!(edge.x < inside);
        // The rows' menus are still there to press.
        let row = row_of(frame, "cache-3").center_y();
        frame.point_at(Point::new(inside - 22.0, row));
        frame.press();
    });
    assert_eq!(bench.menu(), Some(2));
    bench.frame(|frame| frame.tap_key(Named::Escape));
    // What was kept too wide by another version is held back the same.
    bench.state = State::new().with_widths([("name", 5000.0)]);
    bench.frame(|frame| {
        let memory = frame.find("Memory").expect("the Memory header");
        assert!(memory.x + memory.width <= inside);
    });
}

#[test]
fn two_presses_on_the_edge_of_a_header_give_the_column_its_own_width_back() {
    let mut bench = Bench::new(40);
    let mut before = Rectangle::default();
    bench.frame(|frame| {
        before = frame.find("Image").expect("the Image header");
        let edge = Point::new(before.x - 8.0, before.center_y());
        drag(frame, edge, Point::new(edge.x + 90.0, edge.y));
    });
    assert_eq!(bench.state.widths().len(), 1);
    let outcome = bench.frame(|frame| {
        let image = frame.find("Image").expect("the Image header");
        assert_eq!(image.x, before.x + 90.0);
        frame.point_at(Point::new(image.x - 8.0, image.center_y()));
        frame.press();
        frame.press();
    });
    assert_eq!(outcome.last(), Some(&Outcome::Arranged));
    assert!(bench.state.widths().is_empty());
    assert!(bench.state.sort().is_empty(), "the header was not pressed");
    bench.frame(|frame| assert_eq!(frame.find("Image").map(|image| image.x), Some(before.x)));
}

#[test]
fn a_header_scrolled_out_of_the_table_is_neither_shown_nor_pressed() {
    // A table 420 px wide in the middle of the window: its last columns
    // are laid out past its edge, where the page is.
    let mut bench = Bench::fresh("en", 12, WINDOW);
    bench.side = 340.0;
    bench.frame(|_| {});
    let edge = WINDOW.0 - 340.0;
    let header = TOP + 1.0 + HEADER_HEIGHT / 2.0;
    let outcome = bench.frame(|frame| {
        // Where the Memory header would be, 80 px past the table.
        frame.point_at(Point::new(edge + 80.0, header));
        frame.press();
    });
    assert_eq!(outcome, [], "nothing there to press");
    assert!(bench.state.sort().is_empty());
    // Scrolled to the end, the same header is in the table and sorts.
    bench.frame(|frame| {
        frame.point_at(Point::new(edge - 100.0, header + 100.0));
        hold(frame, Modifiers::SHIFT);
        frame.scroll(20.0);
        hold(frame, NONE);
    });
    assert!(bench.state.across() > 100.0);
    let outcome = bench.frame(|frame| frame.click("Memory"));
    assert_eq!(outcome, [Outcome::Arranged]);
    assert_eq!(bench.state.sort().keys(), [("memory", Order::Descending)]);
}

#[test]
fn columns_too_wide_for_the_table_scroll_sideways_and_keep_their_least_width() {
    let mut bench = Bench::fresh("en", 12, WINDOW);
    bench.side = 340.0;
    bench.frame(|_| {});
    bench.frame(|frame| {
        let name = frame.find("Name").expect("the Name header");
        let image = frame.find("Image").expect("the Image header");
        assert_eq!(image.x - name.x, 120.0, "the name keeps its least width");
        frame.point_at(Point::new(500.0, 300.0));
        hold(frame, Modifiers::SHIFT);
        frame.scroll(1.0);
        hold(frame, NONE);
    });
    assert_eq!(bench.state.across(), 60.0);
    assert_eq!(
        bench.state.offset(),
        0.0,
        "with Shift the wheel goes across"
    );
    // The arrows across scroll while the rows have focus.
    bench.frame(|frame| frame.click("Running"));
    bench.key(Named::ArrowRight, NONE);
    assert_eq!(bench.state.across(), 100.0);
    bench.key(Named::ArrowLeft, NONE);
    bench.key(Named::ArrowLeft, NONE);
    bench.key(Named::ArrowLeft, NONE);
    assert_eq!(bench.state.across(), 0.0, "and stop at the start");
    // The thumb under the rows, dragged to the end, shows the last column.
    let rail = TOP + 1.0 + HEADER_HEIGHT + 12.0 * ROW_HEIGHT + 5.0;
    bench.frame(|frame| {
        drag(frame, Point::new(350.0, rail), Point::new(WINDOW.0, rail));
    });
    // The columns are 574 px wide in the 418 the table has.
    assert_eq!(bench.state.across(), 156.0, "the end, and no further");
    bench.frame(|frame| {
        let memory = frame.find("Memory").expect("the Memory header");
        assert!(memory.x + memory.width < WINDOW.0 - 340.0);
    });
}

#[test]
fn a_drag_ends_when_the_pointer_is_lost() {
    let mut bench = Bench::new(40);
    let outcome = bench.frame(|frame| {
        let image = frame.find("Image").expect("the Image header");
        let edge = Point::new(image.x - 8.0, image.center_y());
        frame.point_at(edge);
        frame.send([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        // The pointer leaves the window and the release is never heard.
        frame.send([Event::Mouse(mouse::Event::CursorLeft)]);
        frame.point_at(Point::new(edge.x + 200.0, edge.y + 200.0));
        frame.point_at(Point::new(edge.x + 300.0, edge.y + 200.0));
    });
    assert_eq!(outcome, [Outcome::Arranged], "the drag ended once");
    assert!(bench.state.widths().is_empty(), "and resized nothing since");
}

#[test]
fn a_drag_whose_column_goes_is_over() {
    let mut bench = Bench::new(12);
    let mut edge = Point::ORIGIN;
    bench.frame(|frame| {
        let memory = frame.find("Memory").expect("the Memory header");
        edge = Point::new(memory.x + memory.width + 8.0, memory.center_y());
        frame.point_at(edge);
        frame.send([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
    });
    // The column is hidden while its edge is held: it was the last one.
    bench.state.set_hidden("memory", true);
    bench.frame(|frame| frame.point_at(Point::new(edge.x - 40.0, edge.y)));
    assert!(bench.state.widths().is_empty());
    bench.state.set_hidden("memory", false);
    bench.frame(|frame| frame.point_at(Point::new(edge.x - 80.0, edge.y)));
    assert!(
        bench.state.widths().is_empty(),
        "back, it is not held any more"
    );
}

#[test]
fn two_presses_on_two_rows_that_passed_under_the_pointer_open_nothing() {
    let mut bench = Bench::new(40);
    let mut at = Point::ORIGIN;
    bench.frame(|frame| {
        at = row_of(frame, "cache-3").center();
        frame.point_at(at);
        frame.press();
    });
    assert_eq!(bench.selected(), [2]);
    // The rows move up by one (a row was removed above, the wheel
    // turned): another row is under the pointer for the second press.
    bench.tell(Message::Scrolled(ROW_HEIGHT));
    let outcome = bench.frame(|frame| {
        frame.point_at(at);
        frame.press();
    });
    assert_eq!(outcome, [Outcome::Selected], "selected, not opened");
    assert_eq!(bench.selected(), [3]);
}

#[test]
fn right_to_left_the_scrollbar_is_on_the_left() {
    let mut bench = Bench::in_the_language("ar", 1000);
    let body = TOP + 1.0 + HEADER_HEIGHT;
    bench.frame(|frame| {
        // Nothing to hold where it is left to right.
        drag(
            frame,
            Point::new(WINDOW.0 - 24.0 - 1.0 - 5.0, body + 4.0),
            Point::new(WINDOW.0 - 24.0 - 1.0 - 5.0, body + 300.0),
        );
    });
    assert_eq!(bench.state.offset(), 0.0);
    bench.frame(|frame| {
        drag(
            frame,
            Point::new(24.0 + 1.0 + 5.0, body + 4.0),
            Point::new(24.0 + 1.0 + 5.0, body + 300.0),
        );
    });
    assert!(bench.state.offset() > 1000.0, "{}", bench.state.offset());
}

#[test]
fn a_hidden_column_leaves_its_room_to_the_others() {
    let mut bench = Bench::new(12);
    let mut status = Rectangle::default();
    bench.frame(|frame| status = frame.find("Status").expect("the Status header"));
    let hide = Message::Hide {
        column: "image",
        hidden: true,
    };
    assert_eq!(bench.tell(hide), Some(Outcome::Arranged));
    bench.frame(|frame| {
        assert!(frame.find("Image").is_none());
        assert!(frame.find("nginx:1.27").is_none(), "nor its cells");
        let moved = frame.find("Status").expect("the Status header");
        assert_eq!(moved.x, status.x, "fixed columns stay; the name grew");
        assert!(row_of(frame, "web-1").x < status.x);
    });
}
