//! The table under the pointer and the keyboard, on a tree that lasts
//! from one view to the next as the window keeps it: what moves, what
//! scrolls, who has focus and whether it shows.

use iced::{
    advanced::mouse,
    keyboard::{self, key::Named, Key, Modifiers},
    widget::container,
    Event, Length, Point, Rectangle,
};

use super::{Event as Outcome, Mode, Order, Sort, State, HEADER_HEIGHT, ROW_HEIGHT};
use crate::{
    test_support::{
        snapshot::{in_language, InLanguage},
        stage::{Frame, Stage},
    },
    ui::{
        theme,
        widgets::gallery::{
            self,
            table::{samples, view, Sample, Show},
        },
    },
};

type Page = gallery::Message;

const WINDOW: (f32, f32) = (1100.0, 720.0);
/// Where the table starts: the page's padding, then the selection bar's
/// room and the gap under it.
const TOP: f32 = 24.0 + 28.0 + 8.0;

/// A page with one table, on a window that lasts.
struct Bench {
    stage: Stage,
    state: State<u32>,
    rows: Vec<Sample>,
    _english: InLanguage,
}

impl Bench {
    fn new(rows: u32) -> Self {
        let english = in_language("en");
        let mut bench = Self {
            stage: Stage::new(WINDOW),
            state: State::new().selecting(Mode::Multi),
            rows: samples(rows),
            _english: english,
        };
        // The first view tells the table how much room its rows have.
        bench.frame(|_| {});
        bench
    }

    /// Draws the page, lets `act` use it, and gives the table what that
    /// produced. Returns what the table made of it.
    fn frame(&mut self, act: impl FnOnce(&mut Frame<'_, '_, Page>)) -> Vec<Outcome<u32>> {
        let page = container(view(&self.state, &self.rows, Show::Rows))
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::container::page);
        let messages = self.stage.frame(page, act);
        messages
            .into_iter()
            .filter_map(|message| match message {
                Page::Table(message) => self.state.update(message, &self.rows, |row| row.id),
                _ => None,
            })
            .collect()
    }

    /// One key, and what the table made of it.
    fn key(&mut self, key: impl Into<Key>, modifiers: Modifiers) -> Vec<Outcome<u32>> {
        let key = key.into();
        self.frame(|frame| frame.send(press_of(key, modifiers)))
    }

    fn selected(&self) -> Vec<u32> {
        let mut keys: Vec<u32> = self.state.selection().keys().copied().collect();
        keys.sort_unstable();
        keys
    }

    /// Tab until the rows have focus.
    fn tab_to_rows(&mut self) {
        self.frame(|frame| {
            for _ in 0..12 {
                frame.tab();
                if has_rows_focused(frame) {
                    return;
                }
            }
            panic!("Tab never reached the rows");
        });
    }
}

fn press_of(key: Key, modifiers: Modifiers) -> [Event; 2] {
    let pressed = keyboard::Event::KeyPressed {
        key: key.clone(),
        modified_key: key.clone(),
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers,
        text: None,
        repeat: false,
    };
    let released = keyboard::Event::KeyReleased {
        key: key.clone(),
        modified_key: key,
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers,
    };
    [Event::Keyboard(pressed), Event::Keyboard(released)]
}

/// The rows are the one thing that takes focus and is taller than a row.
fn has_rows_focused(frame: &mut Frame<'_, '_, Page>) -> bool {
    frame
        .focused()
        .is_some_and(|bounds| bounds.height > 2.0 * HEADER_HEIGHT)
}

fn row_of(frame: &mut Frame<'_, '_, Page>, name: &str) -> Rectangle {
    frame
        .find(name)
        .unwrap_or_else(|| panic!("the row {name} is not in view"))
}

const NONE: Modifiers = Modifiers::empty();

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
        assert_eq!(bench.state.focused(), Some(&expected));
    }
    assert!(bench.state.offset() > 0.0, "the 30th row was out of view");
    bench.frame(|frame| {
        let row = row_of(frame, "queue-30");
        assert!(row.y + row.height <= WINDOW.1 - 24.0, "it shows whole");
        assert!(frame.find("web-1").is_none(), "the first rows are gone");
        assert!(has_rows_focused(frame) && frame.shows_focus());
    });
    bench.key(Named::End, NONE);
    assert_eq!(bench.state.focused(), Some(&999));
    bench.frame(|frame| {
        row_of(frame, "auth-1000");
    });
    bench.key(Named::PageUp, NONE);
    let page = bench.state.focused().copied().expect("a focused row");
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
    assert_eq!(bench.state.focused(), Some(&1));
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
fn a_press_selects_without_a_ring_and_a_double_press_opens() {
    let mut bench = Bench::new(40);
    let outcome = bench.frame(|frame| {
        frame.click("cache-3");
        assert!(has_rows_focused(frame));
        assert!(!frame.shows_focus(), "a press shows no ring");
    });
    assert_eq!(outcome, [Outcome::Selected]);
    assert_eq!(bench.selected(), [2]);
    assert_eq!(bench.state.focused(), Some(&2));
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
    assert_eq!(bench.state.focused(), Some(&7));
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
        // Sorted by name: the first row is api-15, not web-1.
        let first = row_of(frame, "api-15");
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
fn the_menu_key_opens_the_rows_menu_and_escape_gives_the_row_its_focus_back() {
    let mut bench = Bench::new(40);
    bench.tab_to_rows();
    bench.key(Named::ArrowDown, NONE);
    bench.key(Named::ArrowDown, NONE);
    bench.key(Named::ContextMenu, NONE);
    assert!(bench.state.menu.is_some());
    bench.frame(|frame| {
        assert!(frame.is_focused("Start"), "the menu has the keyboard");
        // The arrows are the menu's now: the row doesn't move.
        frame.tap_key(Named::ArrowDown);
        assert!(frame.is_focused("Stop"));
    });
    assert_eq!(bench.state.focused(), Some(&1));
    bench.key(Named::Escape, NONE);
    assert!(bench.state.menu.is_none());
    bench.frame(|frame| {
        assert!(frame.find("Start").is_none(), "the menu is closed");
        assert!(has_rows_focused(frame), "focus is back on the rows");
        assert!(frame.shows_focus(), "with its ring");
    });
    // And the keyboard goes on from the same row.
    bench.key(Named::ArrowDown, NONE);
    assert_eq!(bench.state.focused(), Some(&2));
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
    assert!(bench.state.menu_of(&2).is_some());
    assert!(
        bench.state.selection().is_empty(),
        "the row was not pressed"
    );
    let mut chosen = vec![];
    let page = container(view(&bench.state, &bench.rows, Show::Rows)).padding(24);
    for message in bench.stage.frame(page, |frame| frame.click("Stop")) {
        match message {
            Page::Table(message) => {
                bench.state.update(message, &bench.rows, |row| row.id);
            }
            other => chosen.push(other),
        }
    }
    assert!(bench.state.menu.is_none());
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
    assert!(bench.state.menu_of(&5).is_some());
    assert_eq!(bench.state.focused(), Some(&5));
    bench.frame(|frame| assert!(frame.find("Remove").is_some()));
}

#[test]
fn a_sorted_list_keeps_its_place_when_it_starts_sorted() {
    let _english = in_language("en");
    let rows = samples(40);
    let state = State::<u32>::new().sorted(Sort::by("cpu", Order::Descending));
    let mut stage = Stage::new(WINDOW);
    stage.frame(view(&state, &rows, Show::Rows), |frame| {
        // 37·26+11 = 973: the busiest row comes first.
        let first = frame.find("97.3 %").expect("the largest CPU");
        assert!(first.y < 36.0 + HEADER_HEIGHT + ROW_HEIGHT);
    });
}
