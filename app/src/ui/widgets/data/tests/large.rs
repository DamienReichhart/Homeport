//! A large document stays quick: what a view builds and what an update
//! reads are counted, in numbers no machine changes.

use crate::{
    test_support::{snapshot::in_language, stage::Stage, tasks::messages},
    ui::widgets::{
        basic::technical,
        data::json::{self, state::SLICE, Document, Message, State},
        gallery::data::large,
    },
};

/// 100,000 objects of five values: 800,001 nodes, about 9 MB of text.
const ITEMS: usize = 100_000;

fn cells() -> usize {
    json::built::CELLS.with(std::cell::Cell::get)
}

fn view(state: &State) -> crate::ui::theme::Element<'_, Message> {
    json::View::new(state, |message| message)
        .max_height(400.0)
        .view()
}

/// The rows built are those in view, whatever the document holds: the
/// same for a thousand objects as for a hundred thousand.
#[test]
fn a_view_builds_the_rows_in_view_of_a_document_of_any_size() {
    let _english = in_language("en");
    let mut built = vec![];
    for items in [1000, ITEMS] {
        let mut state = State::new(Document::parse(&large(items)).unwrap());
        assert_eq!(state.rows(), items + 1);
        let mut stage = Stage::new((900.0, 600.0));
        // The first frame tells the table its room; the next is as
        // every frame after it.
        for message in stage.frame(view(&state), |_| {}) {
            let _ = state.update(message);
        }
        let before = cells();
        stage.frame(view(&state), |_| {});
        built.push(cells() - before);
    }
    assert_eq!(built[0], built[1], "as many cells built for either");
    // Two cells a row, for the rows of 400 px and a few around them.
    assert!((20..80).contains(&built[1]), "{} cells", built[1]);
}

/// A search reads a slice at an `update`, however large the document:
/// the window draws between two of them.
#[tokio::test(start_paused = true)]
async fn a_search_reads_a_large_document_a_slice_at_a_time() {
    let mut state = State::new(Document::parse(&large(ITEMS)).unwrap());
    let nodes = state.document().len();
    assert_eq!(nodes, 1 + ITEMS * 8);
    let (task, _) = state.update(Message::Typed("container-4242".to_owned()));
    let mut pending = messages(task).await;
    let mut updates = 0;
    while let Some(message) = pending.pop() {
        let before = state.read();
        let (task, _) = state.update(message);
        assert!(
            state.read() - before <= SLICE,
            "an update read more than a slice"
        );
        updates += 1;
        pending.extend(messages(task).await);
    }
    assert_eq!(state.read(), nodes, "every node was read");
    assert_eq!(updates, nodes.div_ceil(SLICE), "one slice an update");
    // container-4242 and container-42420 to -42429.
    assert_eq!(state.matches(), (Some(0), 11));
    // Opening the match's object laid the rows out once more: one more
    // object's five values show.
    assert_eq!(state.rows(), ITEMS + 1 + 5);
}

/// Going to a match scrolls the rows to it.
#[tokio::test(start_paused = true)]
async fn the_match_the_search_is_on_is_scrolled_into_view() {
    let _english = in_language("en");
    let mut state = State::new(Document::parse(&large(1000)).unwrap());
    let mut stage = Stage::new((900.0, 600.0));
    // The view was drawn before anything is typed: the table knows its room.
    for message in stage.frame(view(&state), |_| {}) {
        let _ = state.update(message);
    }
    let mut pending = vec![Message::Typed("container-777".to_owned())];
    while let Some(message) = pending.pop() {
        let (task, _) = state.update(message);
        pending.extend(messages(task).await);
    }
    assert_eq!(state.matches(), (Some(0), 1));
    let mut seen = false;
    stage.frame(view(&state), |frame| {
        assert!(
            frame.find(&technical("\"container-1\"")).is_none(),
            "still at the top"
        );
        seen = frame.find(&technical("\"container-777\"")).is_some();
    });
    assert!(seen, "the 778th object's name is not in view");
}
