//! The table drawn in each of its states, and what its state does with
//! the messages it is sent.

use iced::{keyboard::Modifiers, widget::container, Length};

use super::{state::Motion, Event, Message, Mode, Order, Sort, State, ROW_HEIGHT};
use crate::{
    test_support::snapshot::assert_snapshot,
    ui::{
        theme::{self, Element, Theme},
        widgets::gallery::{
            self,
            table::{samples, view, Sample, Show},
        },
    },
};

type Page = gallery::Message;

const SIZE: (f32, f32) = (860.0, 420.0);

fn on_page(part: Element<'_, Page>) -> Element<'_, Page> {
    container(part)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page)
        .into()
}

fn key(row: &Sample) -> u32 {
    row.id
}

fn multi() -> State<u32> {
    State::new().selecting(Mode::Multi)
}

/// Sorted by memory, the largest first, with three rows selected.
fn arranged(rows: &[Sample]) -> State<u32> {
    let mut state = multi().sorted(Sort::by("memory", Order::Descending));
    for id in [3, 4, 9] {
        state.update(Message::Toggle { key: id, index: 0 }, rows, key);
    }
    state
}

fn press(index: usize, key: u32, modifiers: Modifiers) -> Message<u32> {
    Message::Pressed {
        key,
        index,
        modifiers,
    }
}

fn selected(state: &State<u32>) -> Vec<u32> {
    let mut keys: Vec<u32> = state.selection().keys().copied().collect();
    keys.sort_unstable();
    keys
}

/// Draws the table once so its state knows the rows, as a page does
/// before any message can come.
fn seen(state: &State<u32>, rows: &[Sample]) {
    drop(view(state, rows, Show::Rows));
}

#[test]
fn populated_with_a_selection_and_a_sorted_column() {
    let rows = samples(12);
    let state = arranged(&rows);
    assert_snapshot("table-populated-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state, &rows, Show::Rows))
    });
}

#[test]
fn populated_on_dark() {
    let rows = samples(12);
    let state = arranged(&rows);
    assert_snapshot("table-populated-dark-en", "en", &Theme::Dark, SIZE, || {
        on_page(view(&state, &rows, Show::Rows))
    });
}

#[test]
fn populated_in_arabic_is_mirrored() {
    let rows = samples(12);
    let state = arranged(&rows);
    assert_snapshot("table-populated-ar", "ar", &Theme::Light, SIZE, || {
        on_page(view(&state, &rows, Show::Rows))
    });
}

#[test]
fn a_long_list_shows_its_scrollbar() {
    let rows = samples(100_000);
    let mut state = multi().sorted(Sort::by("name", Order::Ascending));
    seen(&state, &rows);
    state.update(Message::Viewport(330.0), &rows, key);
    state.update(Message::Scrolled(50_000.0 * ROW_HEIGHT), &rows, key);
    assert_snapshot("table-scrolled-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state, &rows, Show::Rows))
    });
}

#[test]
fn empty_loading_and_failed_stand_for_the_rows() {
    let state = multi();
    for (name, show) in [
        ("table-empty-en", Show::Empty),
        ("table-loading-en", Show::Loading),
        ("table-failed-en", Show::Failed),
    ] {
        assert_snapshot(name, "en", &Theme::Light, SIZE, || {
            on_page(view(&state, &[], show))
        });
    }
}

#[test]
fn a_press_selects_ctrl_adds_and_shift_takes_the_range() {
    let rows = samples(10);
    let mut state = multi();
    seen(&state, &rows);
    let none = Modifiers::default();
    assert_eq!(
        state.update(press(2, 2, none), &rows, key),
        Some(Event::Selected)
    );
    assert_eq!(selected(&state), [2]);
    state.update(press(5, 5, Modifiers::COMMAND), &rows, key);
    assert_eq!(selected(&state), [2, 5]);
    // Shift: from the last row pressed to this one, and nothing else.
    state.update(press(8, 8, Modifiers::SHIFT), &rows, key);
    assert_eq!(selected(&state), [5, 6, 7, 8]);
    state.update(press(3, 3, Modifiers::SHIFT), &rows, key);
    assert_eq!(
        selected(&state),
        [3, 4, 5],
        "the range turns round its anchor"
    );
    state.update(press(5, 5, Modifiers::COMMAND), &rows, key);
    assert_eq!(selected(&state), [3, 4]);
    state.update(press(7, 7, none), &rows, key);
    assert_eq!(selected(&state), [7], "a plain press starts over");
}

#[test]
fn a_range_follows_the_order_the_rows_show_in() {
    let rows = samples(10);
    // Largest memory first: 9, 8, 7... is not the order of the keys.
    let mut state = multi().sorted(Sort::by("memory", Order::Descending));
    seen(&state, &rows);
    let shown: Vec<u32> = {
        let mut by_memory = rows.clone();
        by_memory.sort_by_key(|row| std::cmp::Reverse(row.memory));
        by_memory.iter().map(|row| row.id).collect()
    };
    state.update(press(1, shown[1], Modifiers::default()), &rows, key);
    state.update(press(4, shown[4], Modifiers::SHIFT), &rows, key);
    let mut expected = shown[1..=4].to_vec();
    expected.sort_unstable();
    assert_eq!(selected(&state), expected);
}

#[test]
fn the_selection_and_the_focused_row_outlive_a_list_read_again() {
    let mut rows = samples(10);
    let mut state = multi();
    seen(&state, &rows);
    state.update(press(4, 4, Modifiers::default()), &rows, key);
    state.update(press(6, 6, Modifiers::COMMAND), &rows, key);
    assert_eq!(state.focused(), Some(&6));
    // Read again: two rows gone from the top, one new at the end.
    rows.drain(..2);
    rows.push(Sample {
        id: 99,
        ..rows[0].clone()
    });
    seen(&state, &rows);
    assert_eq!(selected(&state), [4, 6], "the same rows, by key");
    // The keyboard goes on from the same row, now two places up.
    state.update(
        Message::Move {
            to: Motion::Down,
            extend: false,
        },
        &rows,
        key,
    );
    assert_eq!(state.focused(), Some(&7));
    // A selected row that left is forgotten when the page prunes.
    rows.retain(|row| row.id != 4);
    state.prune(|id| rows.iter().any(|row| row.id == *id));
    assert_eq!(selected(&state), [6]);
}

#[test]
fn the_keyboard_moves_by_rows_and_pages_and_scrolls_the_row_into_view() {
    let rows = samples(1000);
    let mut state = multi();
    seen(&state, &rows);
    // Ten rows show.
    state.update(Message::Viewport(10.0 * ROW_HEIGHT), &rows, key);
    let go = |state: &mut State<u32>, to| {
        state.update(Message::Move { to, extend: false }, &rows, key);
        (*state.focused().expect("a focused row"), state.offset())
    };
    assert_eq!(go(&mut state, Motion::Down), (0, 0.0), "the first row");
    assert_eq!(go(&mut state, Motion::Down), (1, 0.0));
    assert_eq!(go(&mut state, Motion::PageDown), (11, 2.0 * ROW_HEIGHT));
    assert_eq!(go(&mut state, Motion::End), (999, 990.0 * ROW_HEIGHT));
    assert_eq!(go(&mut state, Motion::Down), (999, 990.0 * ROW_HEIGHT));
    assert_eq!(go(&mut state, Motion::PageUp), (989, 989.0 * ROW_HEIGHT));
    assert_eq!(go(&mut state, Motion::Up), (988, 988.0 * ROW_HEIGHT));
    assert_eq!(go(&mut state, Motion::Home), (0, 0.0));
    assert_eq!(go(&mut state, Motion::Up), (0, 0.0));
    assert!(state.selection().is_empty(), "moving selects nothing");
}

#[test]
fn shift_with_an_arrow_extends_and_space_enter_and_escape_do_their_part() {
    let rows = samples(20);
    let mut state = multi();
    seen(&state, &rows);
    let extend = |to| Message::Move { to, extend: true };
    state.update(press(3, 3, Modifiers::default()), &rows, key);
    state.update(extend(Motion::Down), &rows, key);
    state.update(extend(Motion::Down), &rows, key);
    assert_eq!(selected(&state), [3, 4, 5]);
    state.update(extend(Motion::Up), &rows, key);
    assert_eq!(selected(&state), [3, 4]);
    state.update(extend(Motion::Home), &rows, key);
    assert_eq!(selected(&state), [0, 1, 2, 3]);
    // Space takes the focused row out, Enter opens it.
    state.update(Message::ToggleFocused, &rows, key);
    assert_eq!(selected(&state), [1, 2, 3]);
    assert_eq!(
        state.update(Message::OpenFocused, &rows, key),
        Some(Event::Opened(0))
    );
    assert_eq!(
        state.update(Message::SelectAll, &rows, key),
        Some(Event::Selected)
    );
    assert_eq!(state.selection().len(), 20);
    assert_eq!(
        state.update(Message::Clear, &rows, key),
        Some(Event::Selected)
    );
    assert_eq!(state.update(Message::Clear, &rows, key), None);
}

#[test]
fn select_all_takes_the_rows_that_show_and_no_other() {
    let rows = samples(20);
    let mut state = multi();
    let even = |row: &Sample| row.id.is_multiple_of(2);
    let filtered = || {
        super::Table::new(&state, gallery::table::columns(), &rows, key, Page::Table)
            .filter(even)
            .view()
    };
    drop(filtered());
    state.update(Message::ToggleAll(true), &rows, key);
    assert_eq!(state.selection().len(), 10);
    assert!(selected(&state).iter().all(|id| id.is_multiple_of(2)));
    state.update(Message::ToggleAll(false), &rows, key);
    assert!(state.selection().is_empty());
}

#[test]
fn one_row_at_a_time_or_none_when_the_table_says_so() {
    let rows = samples(5);
    let mut single = State::new().selecting(Mode::Single);
    seen(&single, &rows);
    single.update(press(1, 1, Modifiers::default()), &rows, key);
    single.update(press(3, 3, Modifiers::SHIFT), &rows, key);
    assert_eq!(selected(&single), [3], "no range");
    single.update(press(2, 2, Modifiers::COMMAND), &rows, key);
    assert_eq!(selected(&single), [2]);
    assert_eq!(single.update(Message::SelectAll, &rows, key), None);

    let mut none = State::new();
    seen(&none, &rows);
    assert_eq!(
        none.update(press(1, 1, Modifiers::default()), &rows, key),
        None
    );
    assert!(none.selection().is_empty());
    assert_eq!(none.focused(), Some(&1));
    // A double press opens; with `opening_on_press`, the first one does.
    assert_eq!(
        none.update(Message::Opened(1), &rows, key),
        Some(Event::Opened(1))
    );
    let mut opens = State::new().opening_on_press();
    seen(&opens, &rows);
    assert_eq!(
        opens.update(press(1, 1, Modifiers::default()), &rows, key),
        Some(Event::Opened(1))
    );
    assert_eq!(opens.update(Message::Opened(1), &rows, key), None);
}

#[test]
fn the_view_stays_on_its_rows_when_some_are_added_or_removed_above() {
    let mut rows = samples(200);
    let mut state = multi();
    seen(&state, &rows);
    state.update(Message::Viewport(10.0 * ROW_HEIGHT), &rows, key);
    state.update(Message::Scrolled(50.0 * ROW_HEIGHT + 5.0), &rows, key);
    seen(&state, &rows);
    // Three rows are added before everything: row 50 is now the 53rd.
    for id in 1000..1003 {
        rows.insert(
            0,
            Sample {
                id,
                ..rows[0].clone()
            },
        );
    }
    seen(&state, &rows);
    assert_eq!(state.offset(), 53.0 * ROW_HEIGHT + 5.0);
    // Ten rows go from the top.
    rows.drain(..10);
    seen(&state, &rows);
    assert_eq!(state.offset(), 43.0 * ROW_HEIGHT + 5.0);
    // The list gets shorter than where it was scrolled: it ends there.
    rows.truncate(20);
    seen(&state, &rows);
    assert_eq!(state.offset(), 10.0 * ROW_HEIGHT);
}

#[test]
fn another_order_leaves_the_view_where_it_is_scrolled() {
    let rows = samples(200);
    let mut state = multi();
    seen(&state, &rows);
    state.update(Message::Viewport(10.0 * ROW_HEIGHT), &rows, key);
    state.update(Message::Scrolled(50.0 * ROW_HEIGHT), &rows, key);
    seen(&state, &rows);
    let sort = Message::Sort {
        column: "memory",
        descending_first: true,
        additive: false,
    };
    assert_eq!(state.update(sort, &rows, key), Some(Event::Arranged));
    seen(&state, &rows);
    assert_eq!(state.offset(), 50.0 * ROW_HEIGHT);
    assert_eq!(state.sort().keys(), [("memory", Order::Descending)]);
}

#[test]
fn a_resized_column_is_kept_for_the_page_to_keep() {
    let rows = samples(3);
    let mut state = multi();
    let resize = |width| Message::Resize {
        column: "name",
        width,
    };
    assert_eq!(state.update(resize(240.0), &rows, key), None);
    assert_eq!(state.update(resize(260.0), &rows, key), None);
    assert_eq!(
        state.update(Message::Resized, &rows, key),
        Some(Event::Arranged)
    );
    assert_eq!(state.widths(), [("name", 260.0)]);
    let again = State::<u32>::new().with_widths(state.widths().to_vec());
    assert_eq!(again.widths(), [("name", 260.0)]);
}

#[test]
fn a_revision_spares_the_work_until_it_changes() {
    let mut rows = samples(50);
    let state = multi().sorted(Sort::by("memory", Order::Descending));
    let first_shown = |rows: &[Sample], revision| {
        drop(
            super::Table::new(&state, gallery::table::columns(), rows, key, Page::Table)
                .revision(revision)
                .view(),
        );
        let derived = state.derived.borrow();
        rows[derived.order[0] as usize].id
    };
    let largest = first_shown(&rows, 1);
    // The rows change behind the same revision: the order is not looked at.
    rows[7].memory = 9999;
    assert_eq!(first_shown(&rows, 1), largest);
    assert_eq!(first_shown(&rows, 2), 7);
}
