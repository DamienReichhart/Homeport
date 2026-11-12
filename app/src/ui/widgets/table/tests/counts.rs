//! How much work a view is, counted: the cells built, the rows compared,
//! filtered and asked for their key. A hundred thousand rows cost a
//! screen of cells, and the passes over the whole list are the ones the
//! table says it makes: these numbers don't depend on the machine, so a
//! view that starts building or sorting everything fails here.

use std::cell::Cell;

use super::Page;
use crate::ui::widgets::{
    gallery::table::{samples, Sample},
    table::{self, state::Motion, Column, Message, Mode, Order, Rows, Sort, State, Table},
};

const ROWS: usize = 100_000;
/// A body of 720 px shows 21 rows at most (the first and the last in
/// part), and 4 more are built on each side.
const IN_VIEW: usize = 29;

#[derive(Default)]
struct Counts {
    cells: Cell<usize>,
    compared: Cell<usize>,
    filtered: Cell<usize>,
    keyed: Cell<usize>,
}

impl Counts {
    fn take(&self) -> [usize; 4] {
        [
            self.cells.take(),
            self.compared.take(),
            self.filtered.take(),
            self.keyed.take(),
        ]
    }
}

fn bump(count: &Cell<usize>) {
    count.set(count.get() + 1);
}

fn columns(counts: &Counts) -> Vec<Column<'_, Sample, Page>> {
    vec![
        Column::new("name", "Name", |row: &Sample| {
            bump(&counts.cells);
            table::text(row.name.clone()).into()
        })
        .sort_by(|a, b| {
            bump(&counts.compared);
            a.name.cmp(&b.name)
        }),
        Column::new("memory", "Memory", |row: &Sample| {
            bump(&counts.cells);
            table::text(row.memory.to_string()).into()
        }),
    ]
}

/// What a table is asked to do with its rows.
#[derive(Clone, Copy, Default)]
struct Asked {
    filter: bool,
    revision: Option<u64>,
}

fn read<'r>(rows: &'r [Sample], counts: &'r Counts, asked: Asked) -> Rows<'r, Sample, u32> {
    let rows = Rows::new(rows, |row| {
        bump(&counts.keyed);
        row.id
    });
    let rows = match asked.revision {
        Some(revision) => rows.revision(revision),
        None => rows,
    };
    if asked.filter {
        rows.filter(|row| {
            bump(&counts.filtered);
            row.id % 3 != 0
        })
    } else {
        rows
    }
}

/// One view, and what it cost: cells, comparisons, filtered, keyed.
fn view(state: &State<u32>, rows: &[Sample], counts: &Counts, asked: Asked) -> [usize; 4] {
    counts.take();
    drop(
        Table::new(
            state,
            columns(counts),
            read(rows, counts, asked),
            Page::Table,
        )
        .view(),
    );
    counts.take()
}

fn scroll(state: &mut State<u32>, rows: &[Sample], counts: &Counts, asked: Asked) {
    let to = Message::Scrolled(state.offset() + 60.0);
    state.update(to, &read(rows, counts, asked), &columns(counts));
}

#[test]
fn an_unsorted_list_costs_a_screen_of_cells_and_no_pass() {
    let rows = samples(u32::try_from(ROWS).unwrap());
    let counts = Counts::default();
    let mut state = State::new().selecting(Mode::Multi);
    let asked = Asked::default();
    view(&state, &rows, &counts, asked);
    for _ in 0..3 {
        scroll(&mut state, &rows, &counts, asked);
        let [cells, compared, filtered, keyed] = view(&state, &rows, &counts, asked);
        assert!(cells <= 2 * IN_VIEW, "{cells} cells built");
        assert_eq!((compared, filtered), (0, 0));
        assert!(keyed <= IN_VIEW + 4, "{keyed} rows asked for their key");
    }
}

#[test]
fn a_sorted_list_is_checked_in_one_pass_and_not_at_all_with_a_revision() {
    let rows = samples(u32::try_from(ROWS).unwrap());
    let counts = Counts::default();
    let mut state = State::new().sorted(Sort::by("name", Order::Ascending));
    let asked = Asked::default();
    let [_, sorting, ..] = view(&state, &rows, &counts, asked);
    assert!(
        sorting > ROWS,
        "the first view sorts: {sorting} comparisons"
    );
    for _ in 0..3 {
        scroll(&mut state, &rows, &counts, asked);
        let [cells, compared, _, keyed] = view(&state, &rows, &counts, asked);
        assert!(cells <= 2 * IN_VIEW, "{cells} cells built");
        // One comparison per neighbour says the order still holds.
        assert!(compared < ROWS, "{compared} comparisons to check the order");
        assert!(keyed <= IN_VIEW + 4, "{keyed} rows asked for their key");
    }
    // The page says when its rows change: nothing is looked at in between.
    let asked = Asked {
        revision: Some(1),
        ..asked
    };
    view(&state, &rows, &counts, asked);
    for _ in 0..3 {
        scroll(&mut state, &rows, &counts, asked);
        let [cells, compared, _, keyed] = view(&state, &rows, &counts, asked);
        assert!(cells <= 2 * IN_VIEW, "{cells} cells built");
        assert_eq!(compared, 0);
        assert!(keyed <= IN_VIEW + 4, "{keyed} rows asked for their key");
    }
    // Nor by the keyboard, which goes through the order that was kept.
    counts.take();
    state.update(
        Message::Move {
            to: Motion::Down,
            extend: false,
        },
        &read(&rows, &counts, asked),
        &columns(&counts),
    );
    let [_, compared, _, keyed] = counts.take();
    assert_eq!(compared, 0);
    assert!(keyed <= 4, "{keyed} rows asked for their key by an arrow");
}

#[test]
fn a_filtered_list_is_filtered_once_a_view_and_not_at_all_with_a_revision() {
    let rows = samples(u32::try_from(ROWS).unwrap());
    let counts = Counts::default();
    let mut state = State::new().sorted(Sort::by("name", Order::Ascending));
    let asked = Asked {
        filter: true,
        revision: None,
    };
    view(&state, &rows, &counts, asked);
    scroll(&mut state, &rows, &counts, asked);
    let [cells, compared, filtered, _] = view(&state, &rows, &counts, asked);
    assert!(cells <= 2 * IN_VIEW, "{cells} cells built");
    assert_eq!(filtered, ROWS, "every row is asked once whether it passes");
    assert!(compared < ROWS, "{compared} comparisons to check the order");
    let asked = Asked {
        filter: true,
        revision: Some(1),
    };
    view(&state, &rows, &counts, asked);
    scroll(&mut state, &rows, &counts, asked);
    let [cells, compared, filtered, keyed] = view(&state, &rows, &counts, asked);
    assert!(cells <= 2 * IN_VIEW, "{cells} cells built");
    assert_eq!((compared, filtered), (0, 0));
    assert!(keyed <= IN_VIEW + 4, "{keyed} rows asked for their key");
}

#[test]
fn a_selection_costs_a_pass_over_the_keys_and_none_with_a_revision() {
    let rows = samples(u32::try_from(ROWS).unwrap());
    let counts = Counts::default();
    let mut state = State::new().selecting(Mode::Multi);
    let asked = Asked::default();
    view(&state, &rows, &counts, asked);
    let toggle = Message::Toggle { key: 7, index: 7 };
    state.update(toggle, &read(&rows, &counts, asked), &columns(&counts));
    let [.., keyed] = view(&state, &rows, &counts, asked);
    // Which of the selected rows are still there, and how many show.
    assert!(
        keyed <= ROWS + IN_VIEW + 4,
        "{keyed} rows asked for their key"
    );
    let asked = Asked {
        revision: Some(1),
        ..asked
    };
    view(&state, &rows, &counts, asked);
    scroll(&mut state, &rows, &counts, asked);
    let [.., keyed] = view(&state, &rows, &counts, asked);
    assert!(keyed <= IN_VIEW + 4, "{keyed} rows asked for their key");
}
