//! What the table's state does with the messages it is sent.

use super::*;
use crate::ui::widgets::{
    gallery::table::grouped,
    overlay::menu,
    table::{build::focus_in, column_items, grid::Focus, window::Window, ROW_HEIGHT},
};

#[test]
fn a_press_selects_ctrl_adds_and_shift_takes_the_range() {
    let rows = samples(10);
    let mut state = multi();
    seen(&state, &rows);
    let none = Modifiers::default();
    assert_eq!(
        tell(&mut state, press(2, 2, none), &rows),
        Some(Event::Selected)
    );
    assert_eq!(selected(&state), [2]);
    tell(&mut state, press(5, 5, Modifiers::COMMAND), &rows);
    assert_eq!(selected(&state), [2, 5]);
    // Shift: from the last row pressed to this one, and nothing else.
    tell(&mut state, press(8, 8, Modifiers::SHIFT), &rows);
    assert_eq!(selected(&state), [5, 6, 7, 8]);
    tell(&mut state, press(3, 3, Modifiers::SHIFT), &rows);
    assert_eq!(
        selected(&state),
        [3, 4, 5],
        "the range turns round its anchor"
    );
    tell(&mut state, press(5, 5, Modifiers::COMMAND), &rows);
    assert_eq!(selected(&state), [3, 4]);
    tell(&mut state, press(7, 7, none), &rows);
    assert_eq!(selected(&state), [7], "a plain press starts over");
}

#[test]
fn a_range_follows_the_order_the_rows_show_in() {
    let rows = samples(10);
    // Largest memory first: not the order of the keys.
    let mut state = multi().sorted(Sort::by("memory", Order::Descending));
    seen(&state, &rows);
    let shown: Vec<u32> = {
        let mut by_memory = rows.clone();
        by_memory.sort_by_key(|row| std::cmp::Reverse(row.memory));
        by_memory.iter().map(|row| row.id).collect()
    };
    tell(&mut state, press(1, shown[1], Modifiers::default()), &rows);
    tell(&mut state, press(4, shown[4], Modifiers::SHIFT), &rows);
    let mut expected = shown[1..=4].to_vec();
    expected.sort_unstable();
    assert_eq!(selected(&state), expected);
}

#[test]
fn the_selection_and_the_focused_row_outlive_a_list_read_again() {
    let mut rows = samples(10);
    let mut state = multi();
    seen(&state, &rows);
    tell(&mut state, press(4, 4, Modifiers::default()), &rows);
    tell(&mut state, press(6, 6, Modifiers::COMMAND), &rows);
    assert_eq!(state.focused(), Some(6));
    // Read again: two rows gone from the top, one new at the end.
    rows.drain(..2);
    rows.push(Sample {
        id: 99,
        ..rows[0].clone()
    });
    seen(&state, &rows);
    assert_eq!(selected(&state), [4, 6], "the same rows, by key");
    // The keyboard goes on from the same row, now two places up.
    tell(&mut state, go(Motion::Down), &rows);
    assert_eq!(state.focused(), Some(7));
}

#[test]
fn a_row_that_left_the_list_is_let_go_of_without_the_page_doing_anything() {
    let mut rows = samples(10);
    let mut state = multi();
    seen(&state, &rows);
    tell(&mut state, press(4, 4, Modifiers::default()), &rows);
    tell(&mut state, press(6, 6, Modifiers::COMMAND), &rows);
    tell(&mut state, Message::MenuFor(None), &rows);
    assert_eq!(menu_of(&state), Some(6));
    // The list is read again without two of them.
    rows.retain(|row| row.id != 4 && row.id != 6);
    seen(&state, &rows);
    assert!(selected(&state).is_empty(), "no longer selected");
    assert_eq!(state.focused(), None, "no longer under the keyboard");
    assert_eq!(menu_of(&state), None, "its menu is closed");
    // A range starts anew: the anchor went with its row.
    tell(&mut state, press(3, 5, Modifiers::SHIFT), &rows);
    assert_eq!(selected(&state), [5]);
}

#[test]
fn a_selected_row_the_search_hides_stays_selected_and_no_range_starts_from_it() {
    let rows = samples(10);
    let mut state = multi();
    seen(&state, &rows);
    tell(&mut state, press(2, 2, Modifiers::default()), &rows);
    // The search now shows the odd rows only: 1, 3, 5, 7, 9.
    let odd = || source(&rows).filter(|row| row.id % 2 == 1);
    seen_as(&state, odd());
    assert_eq!(selected(&state), [2], "hidden, not gone");
    // Shift with a press: the row the range started at doesn't show, so
    // the range is the row pressed.
    state.update(press(3, 7, Modifiers::SHIFT), &odd(), &columns());
    assert_eq!(selected(&state), [7]);
}

#[test]
fn the_keyboard_moves_by_rows_and_pages_and_scrolls_the_row_into_view() {
    let rows = samples(1000);
    let mut state = multi();
    seen(&state, &rows);
    // Ten rows show.
    tell(&mut state, Message::Viewport(10.0 * ROW_HEIGHT), &rows);
    let mut at = |to| {
        tell(&mut state, go(to), &rows);
        (state.focused().expect("a focused row"), state.offset())
    };
    assert_eq!(at(Motion::Down), (0, 0.0), "the first row");
    assert_eq!(at(Motion::Down), (1, 0.0));
    assert_eq!(at(Motion::PageDown), (11, 2.0 * ROW_HEIGHT));
    assert_eq!(at(Motion::End), (999, 990.0 * ROW_HEIGHT));
    assert_eq!(at(Motion::Down), (999, 990.0 * ROW_HEIGHT));
    assert_eq!(at(Motion::PageUp), (989, 989.0 * ROW_HEIGHT));
    assert_eq!(at(Motion::Up), (988, 988.0 * ROW_HEIGHT));
    assert_eq!(at(Motion::Home), (0, 0.0));
    assert_eq!(at(Motion::Up), (0, 0.0));
    assert!(state.selection().is_empty(), "moving selects nothing");
}

#[test]
fn shift_with_an_arrow_extends_and_space_enter_and_escape_do_their_part() {
    let rows = samples(20);
    let mut state = multi();
    seen(&state, &rows);
    let extend = |to| Message::Move { to, extend: true };
    tell(&mut state, press(3, 3, Modifiers::default()), &rows);
    tell(&mut state, extend(Motion::Down), &rows);
    tell(&mut state, extend(Motion::Down), &rows);
    assert_eq!(selected(&state), [3, 4, 5]);
    tell(&mut state, extend(Motion::Up), &rows);
    assert_eq!(selected(&state), [3, 4]);
    tell(&mut state, extend(Motion::Home), &rows);
    assert_eq!(selected(&state), [0, 1, 2, 3]);
    // Space takes the focused row out, Enter opens it.
    tell(&mut state, Message::ToggleFocused, &rows);
    assert_eq!(selected(&state), [1, 2, 3]);
    assert_eq!(
        tell(&mut state, Message::OpenFocused, &rows),
        Some(Event::Opened(0))
    );
    assert_eq!(
        tell(&mut state, Message::SelectAll, &rows),
        Some(Event::Selected)
    );
    assert_eq!(state.selection().len(), 20);
    assert_eq!(
        tell(&mut state, Message::Clear, &rows),
        Some(Event::Selected)
    );
    assert_eq!(tell(&mut state, Message::Clear, &rows), None);
}

#[test]
fn select_all_touches_the_rows_that_show_and_leaves_the_others_as_they_were() {
    let rows = samples(20);
    let mut state = multi();
    seen(&state, &rows);
    tell(&mut state, Message::Toggle { key: 5, index: 5 }, &rows);
    let even = || source(&rows).filter(|row| row.id.is_multiple_of(2));
    seen_as(&state, even());
    state.update(Message::ToggleAll(true), &even(), &columns());
    assert_eq!(state.selection().len(), 11, "the ten that show, and row 5");
    assert!(state.selection().contains(&5));
    state.update(Message::ToggleAll(false), &even(), &columns());
    assert_eq!(selected(&state), [5], "only what shows is deselected");
}

#[test]
fn one_row_at_a_time_or_none_when_the_table_says_so() {
    let rows = samples(5);
    let mut single = State::new().selecting(Mode::Single);
    seen(&single, &rows);
    tell(&mut single, press(1, 1, Modifiers::default()), &rows);
    tell(&mut single, press(3, 3, Modifiers::SHIFT), &rows);
    assert_eq!(selected(&single), [3], "no range");
    tell(&mut single, press(2, 2, Modifiers::COMMAND), &rows);
    assert_eq!(selected(&single), [2]);
    assert_eq!(tell(&mut single, Message::SelectAll, &rows), None);

    let mut none = State::new();
    seen(&none, &rows);
    assert_eq!(
        tell(&mut none, press(1, 1, Modifiers::default()), &rows),
        None
    );
    assert!(none.selection().is_empty());
    assert_eq!(none.focused(), Some(1));
    // A double press opens; with `opening_on_press`, the first one does.
    assert_eq!(
        tell(&mut none, Message::Opened(1), &rows),
        Some(Event::Opened(1))
    );
    let mut opens = State::new().opening_on_press();
    seen(&opens, &rows);
    assert_eq!(
        tell(&mut opens, press(1, 1, Modifiers::default()), &rows),
        Some(Event::Opened(1))
    );
    assert_eq!(tell(&mut opens, Message::Opened(1), &rows), None);
}

/// Scrolls to the 50th row and 5 px, then adds and removes rows above.
fn stays_on_its_rows(revision: Option<u64>) {
    let mut rows = samples(200);
    let mut state = multi();
    fn read(rows: &[Sample], revision: Option<u64>, turn: u64) -> Rows<'_, Sample, u32> {
        match revision {
            Some(revision) => source(rows).revision(revision + turn),
            None => source(rows),
        }
    }
    seen_as(&state, read(&rows, revision, 0));
    tell(&mut state, Message::Viewport(10.0 * ROW_HEIGHT), &rows);
    tell(
        &mut state,
        Message::Scrolled(50.0 * ROW_HEIGHT + 5.0),
        &rows,
    );
    seen_as(&state, read(&rows, revision, 0));
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
    seen_as(&state, read(&rows, revision, 1));
    assert_eq!(state.offset(), 53.0 * ROW_HEIGHT + 5.0);
    // Ten rows go from the top.
    rows.drain(..10);
    seen_as(&state, read(&rows, revision, 2));
    assert_eq!(state.offset(), 43.0 * ROW_HEIGHT + 5.0);
    // The list gets shorter than where it was scrolled: it ends there.
    rows.truncate(20);
    seen_as(&state, read(&rows, revision, 3));
    assert_eq!(state.offset(), 10.0 * ROW_HEIGHT);
}

#[test]
fn the_view_stays_on_its_rows_when_some_are_added_or_removed_above() {
    stays_on_its_rows(None);
}

#[test]
fn the_view_stays_on_its_rows_after_a_scroll_when_the_page_gives_a_revision() {
    stays_on_its_rows(Some(7));
}

#[test]
fn another_order_leaves_the_view_where_it_is_scrolled() {
    let rows = samples(200);
    let mut state = multi();
    seen(&state, &rows);
    tell(&mut state, Message::Viewport(10.0 * ROW_HEIGHT), &rows);
    tell(&mut state, Message::Scrolled(50.0 * ROW_HEIGHT), &rows);
    seen(&state, &rows);
    let sort = Message::Sort {
        column: "memory",
        descending_first: true,
        additive: false,
    };
    assert_eq!(tell(&mut state, sort, &rows), Some(Event::Arranged));
    seen(&state, &rows);
    assert_eq!(state.offset(), 50.0 * ROW_HEIGHT);
    assert_eq!(state.sort().keys(), [("memory", Order::Descending)]);
}

#[test]
fn a_resized_column_is_kept_for_the_page_to_keep_and_reset_by_its_edge() {
    let rows = samples(3);
    let mut state = multi();
    let resize = |width| Message::Resize {
        column: "name",
        width,
    };
    assert_eq!(tell(&mut state, resize(240.0), &rows), None);
    assert_eq!(tell(&mut state, resize(260.0), &rows), None);
    assert_eq!(
        tell(&mut state, Message::Resized, &rows),
        Some(Event::Arranged)
    );
    assert_eq!(state.widths(), [("name", 260.0)]);
    let mut again = State::<u32>::new().with_widths(state.widths().to_vec());
    assert_eq!(again.widths(), [("name", 260.0)]);
    assert_eq!(
        tell(&mut again, Message::ResetWidth("name"), &rows),
        Some(Event::Arranged)
    );
    assert!(again.widths().is_empty());
}

#[test]
fn a_revision_spares_the_work_until_it_changes() {
    let mut rows = samples(50);
    let state = multi().sorted(Sort::by("memory", Order::Descending));
    let first_shown = |rows: &[Sample], revision| {
        seen_as(&state, source(rows).revision(revision));
        let derived = state.derived.borrow();
        rows[derived.order[0] as usize].id
    };
    let largest = first_shown(&rows, 1);
    // The rows change behind the same revision: the order is not looked at.
    rows[7].memory = 9999;
    assert_eq!(first_shown(&rows, 1), largest);
    assert_eq!(first_shown(&rows, 2), 7);
}

#[test]
fn the_menu_key_brings_its_row_into_view_before_its_menu_opens() {
    let rows = samples(500);
    let mut state = multi();
    seen(&state, &rows);
    tell(&mut state, Message::Viewport(10.0 * ROW_HEIGHT), &rows);
    tell(&mut state, press(3, 3, Modifiers::default()), &rows);
    // The wheel takes the view far from the focused row.
    tell(&mut state, Message::Scrolled(300.0 * ROW_HEIGHT), &rows);
    tell(&mut state, Message::MenuFor(None), &rows);
    assert_eq!(menu_of(&state), Some(3));
    assert_eq!(state.offset(), 3.0 * ROW_HEIGHT, "the row shows again");
}

#[test]
fn a_key_behind_a_list_read_again_goes_through_the_list_as_it_is_now() {
    let mut rows = samples(10);
    let mut state = multi().sorted(Sort::by("memory", Order::Descending));
    seen(&state, &rows);
    let by_memory = |rows: &[Sample]| {
        let mut sorted = rows.to_vec();
        sorted.sort_by_key(|row| std::cmp::Reverse(row.memory));
        sorted.iter().map(|row| row.id).collect::<Vec<_>>()
    };
    let shown = by_memory(&rows);
    tell(&mut state, press(0, shown[0], Modifiers::default()), &rows);
    // The second row leaves the list, and a Down that was queued comes
    // before any view of the new list.
    rows.retain(|row| row.id != shown[1]);
    tell(&mut state, go(Motion::Down), &rows);
    assert_eq!(state.focused(), Some(by_memory(&rows)[1]));
    // Select all, queued the same way behind a search, takes what passes.
    rows.pop();
    let odd = source(&rows).filter(|row| row.id % 2 == 1);
    state.update(Message::SelectAll, &odd, &columns());
    assert!(selected(&state).iter().all(|id| id % 2 == 1));
    assert!(!state.selection().is_empty());
}

#[test]
fn a_marked_row_is_scrolled_to_once_it_is_there() {
    let rows = samples(300);
    let mut state = multi();
    seen(&state, &rows[..100]);
    tell(
        &mut state,
        Message::Viewport(10.0 * ROW_HEIGHT),
        &rows[..100],
    );
    // The page marks a row that was just made and is not in the list yet.
    state.mark(Some(250));
    seen(&state, &rows[..100]);
    assert_eq!(state.offset(), 0.0, "it waits for the row");
    seen(&state, &rows);
    assert_eq!(
        state.offset(),
        241.0 * ROW_HEIGHT,
        "the row shows, at the end"
    );
    // Once: the user scrolls away and stays there.
    tell(&mut state, Message::Scrolled(0.0), &rows);
    seen(&state, &rows);
    assert_eq!(state.offset(), 0.0);
    assert_eq!(state.marked(), Some(&250));
    assert!(state.selection().is_empty(), "marked is not selected");
}

#[test]
fn rows_are_nested_under_their_groups_which_close_and_open() {
    let rows = samples(8);
    let mut state = multi();
    let group = || grouped(&rows);
    seen_as(&state, group());
    assert_eq!(
        lines(&state, &rows),
        [
            "[blog]", "cache-3", "api-7", "[shop]", "web-1", "proxy-4", "worker-5", "auth-8",
            "db-2", "queue-6"
        ],
        "the groups by name, then the rows of none"
    );
    let pressed = Message::GroupPressed {
        name: "shop".to_owned(),
        index: 3,
    };
    state.update(pressed, &group(), &columns());
    seen_as(&state, group());
    assert_eq!(
        lines(&state, &rows),
        ["[blog]", "cache-3", "api-7", "[shop]", "db-2", "queue-6"]
    );
    assert!(state.is_collapsed("shop"));
    assert_eq!(state.focused_group().as_deref(), Some("shop"));
    // The checkbox of a group selects its rows, shown or not.
    let select = |selected| Message::SelectGroup {
        name: "shop".to_owned(),
        selected,
    };
    assert_eq!(
        state.update(select(true), &group(), &columns()),
        Some(Event::Selected)
    );
    assert_eq!(selected(&state), [0, 3, 4, 7]);
    state.update(select(false), &group(), &columns());
    assert!(state.selection().is_empty());
}

#[test]
fn the_keyboard_goes_over_the_headers_of_groups_and_closes_them() {
    let rows = samples(8);
    let mut state = multi();
    let group = || grouped(&rows);
    seen_as(&state, group());
    let mut key = |message| state.update(message, &group(), &columns());
    key(go(Motion::Down));
    key(go(Motion::Down));
    key(go(Motion::Down));
    key(go(Motion::Down));
    assert_eq!(state.focused_group().as_deref(), Some("shop"));
    assert_eq!(state.focused(), None);
    // Space and Enter close and open it; the arrows across do one each.
    let mut key = |message| state.update(message, &group(), &columns());
    key(Message::ToggleFocused);
    assert!(state.is_collapsed("shop"));
    let mut key = |message| state.update(message, &group(), &columns());
    key(Message::OpenFocused);
    assert!(!state.is_collapsed("shop"));
    let mut key = |message| state.update(message, &group(), &columns());
    key(Message::Collapse {
        name: "shop".to_owned(),
        collapsed: true,
    });
    // Down from a closed group is the next line that shows.
    key(go(Motion::Down));
    assert_eq!(state.focused(), Some(1));
    // A range over a header takes the rows and not the header.
    let mut key = |message| state.update(message, &group(), &columns());
    key(Message::Move {
        to: Motion::Home,
        extend: true,
    });
    assert_eq!(selected(&state), [1, 2, 6]);
}

#[test]
fn a_row_whose_group_closes_lets_go_of_the_keyboard() {
    let rows = samples(8);
    let mut state = multi();
    let group = || grouped(&rows);
    seen_as(&state, group());
    state.update(press(1, 2, Modifiers::default()), &group(), &columns());
    assert_eq!(state.focused(), Some(2));
    let close = Message::Collapse {
        name: "blog".to_owned(),
        collapsed: true,
    };
    state.update(close, &group(), &columns());
    seen_as(&state, group());
    assert_eq!(state.focused(), None);
    assert_eq!(selected(&state), [2], "closed, its rows stay selected");
}

#[test]
fn letters_go_to_the_next_row_that_starts_with_them() {
    let rows = samples(40);
    let mut state = multi();
    seen(&state, &rows);
    tell(&mut state, Message::Viewport(10.0 * ROW_HEIGHT), &rows);
    let mut typed = |letters: &str| {
        tell(&mut state, Message::Typed(letters.to_owned()), &rows);
        state.focused().map(|id| rows[id as usize].name.clone())
    };
    assert_eq!(typed("w").as_deref(), Some("web-1"));
    assert_eq!(typed("w").as_deref(), Some("worker-5"), "the next one");
    assert_eq!(typed("we").as_deref(), Some("web-9"), "narrowed down");
    assert_eq!(typed("web-3").as_deref(), Some("web-33"));
    assert_eq!(typed("zz").as_deref(), Some("web-33"), "nothing: it stays");
    assert_eq!(typed("A").as_deref(), Some("api-39"), "whatever the case");
    assert!(state.offset() > 0.0, "and the row is brought into view");
}

#[test]
fn a_column_is_hidden_and_shown_again_from_the_menu_of_the_columns() {
    let rows = samples(3);
    let mut state = multi();
    let checked = |state: &State<u32>| -> Vec<(String, bool)> {
        column_items(state, &columns(), Page::Table)
            .into_iter()
            .filter_map(|item| match item {
                menu::Item::Action { label, checked, .. } => Some((label, checked?)),
                _ => None,
            })
            .collect()
    };
    let all = checked(&state);
    assert_eq!(all.len(), 4, "the name can't be hidden: {all:?}");
    assert!(all.iter().all(|(_, shown)| *shown));
    let hide = Message::Hide {
        column: "image",
        hidden: true,
    };
    assert_eq!(tell(&mut state, hide, &rows), Some(Event::Arranged));
    assert_eq!(state.hidden(), ["image"]);
    assert_eq!(checked(&state)[0], ("Image".to_owned(), false));
    let again = State::<u32>::new().with_hidden(state.hidden().to_vec());
    assert!(again.is_hidden("image"));
    let show = Message::Hide {
        column: "image",
        hidden: false,
    };
    tell(&mut state, show, &rows);
    assert!(state.hidden().is_empty());
}

#[test]
fn the_ring_is_on_the_focused_line_and_nowhere_when_that_line_is_out_of_view() {
    let view = Window { first: 10, end: 30 };
    assert_eq!(focus_in(None, view), Focus::None, "no line yet: the rows");
    assert_eq!(focus_in(Some(12), view), Focus::At(12));
    assert_eq!(
        focus_in(Some(3), view),
        Focus::Away,
        "scrolled away: not the rows as a whole"
    );
    assert_eq!(focus_in(Some(30), view), Focus::Away);
}
