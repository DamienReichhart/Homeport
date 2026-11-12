//! The table drawn in each of its states.

use iced::keyboard::key::Named;

use super::*;
use crate::{
    test_support::snapshot::{assert_snapshot, assert_snapshot_after},
    ui::widgets::{gallery::table::grouped, table::ROW_HEIGHT},
};

#[test]
fn populated_with_a_selection_and_a_sorted_column() {
    let rows = samples(12);
    let state = arranged(&rows);
    assert_snapshot("table-populated-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state, source(&rows), Show::Rows))
    });
}

#[test]
fn populated_on_dark() {
    let rows = samples(12);
    let state = arranged(&rows);
    assert_snapshot("table-populated-dark-en", "en", &Theme::Dark, SIZE, || {
        on_page(view(&state, source(&rows), Show::Rows))
    });
}

#[test]
fn populated_in_arabic_is_mirrored() {
    let rows = samples(12);
    let state = arranged(&rows);
    assert_snapshot("table-populated-ar", "ar", &Theme::Light, SIZE, || {
        on_page(view(&state, source(&rows), Show::Rows))
    });
}

#[test]
fn grouped_under_their_stacks_with_one_closed() {
    let rows = samples(12);
    let mut state = multi();
    seen_as(&state, grouped(&rows));
    let close = Message::Collapse {
        name: "blog".to_owned(),
        collapsed: true,
    };
    state.update(close, &grouped(&rows), &columns());
    tell(&mut state, Message::Toggle { key: 3, index: 0 }, &rows);
    assert_snapshot("table-grouped-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state, grouped(&rows), Show::Rows))
    });
}

#[test]
fn too_narrow_for_its_columns_it_scrolls_sideways() {
    let rows = samples(12);
    let mut state = multi();
    tell(&mut state, Message::ScrolledAcross(90.0), &rows);
    assert_snapshot(
        "table-narrow-en",
        "en",
        &Theme::Light,
        (420.0, 420.0),
        || on_page(view(&state, source(&rows), Show::Rows)),
    );
}

#[test]
fn the_row_the_keyboard_is_on_shows_a_ring_and_the_one_under_the_pointer_a_fill() {
    let rows = samples(12);
    let mut state = arranged(&rows);
    seen(&state, &rows);
    for _ in 0..4 {
        tell(&mut state, go(Motion::Down), &rows);
    }
    assert_eq!(state.focused(), Some(8));
    assert_snapshot_after(
        "table-focused-en",
        "en",
        &Theme::Light,
        SIZE,
        || on_page(view(&state, source(&rows), Show::Rows)),
        |window| {
            // A press gives the rows focus and a key of theirs shows the
            // ring on the row the state says; the pointer then rests two
            // rows down.
            window.click("web-9").unwrap();
            window.tap_key(Named::End);
            let below = window.find("api-7").unwrap();
            window.point_at(below.bounds().center());
        },
    );
}

#[test]
fn a_long_list_shows_its_scrollbar() {
    let rows = samples(100_000);
    let mut state = multi().sorted(Sort::by("name", Order::Ascending));
    seen(&state, &rows);
    tell(&mut state, Message::Viewport(330.0), &rows);
    tell(&mut state, Message::Scrolled(50_000.0 * ROW_HEIGHT), &rows);
    assert_snapshot("table-scrolled-en", "en", &Theme::Light, SIZE, || {
        on_page(view(&state, source(&rows), Show::Rows))
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
            on_page(view(&state, source(&[]), show))
        });
    }
}

#[test]
fn what_a_cell_shows_is_cut_where_its_column_ends() {
    use crate::ui::widgets::{
        basic::{badge, BadgeVariant},
        feedback::status_dot,
        line,
        table::{self, Column, Table, Width},
    };
    let rows = samples(4);
    let state = State::new();
    // A badge and a row of dots, both wider than the 56 px they are given,
    // under a header that is too: none of them runs into the next column.
    let columns = || -> Vec<Column<'_, Sample, Page>> {
        vec![
            Column::new("kind", "Connection kind", |row: &Sample| {
                badge(row.image, BadgeVariant::Secondary)
            })
            .width(Width::Fixed(56.0))
            .resizable(false),
            Column::new("dots", "Health checks", |row: &Sample| {
                let dots = (0..8).map(|_| status_dot(theme::Tone::Danger)).collect();
                let _ = row;
                line(dots).spacing(4).into()
            })
            .width(Width::Fixed(56.0))
            .resizable(false),
            Column::new("name", "Name", |row: &Sample| {
                table::text(row.name.clone()).into()
            }),
        ]
    };
    assert_snapshot("table-cut-en", "en", &Theme::Light, (420.0, 260.0), || {
        on_page(Table::new(&state, columns(), source(&rows), Page::Table).view())
    });
}
