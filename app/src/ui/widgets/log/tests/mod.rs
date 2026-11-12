//! The log viewer's tests: what a terminal's text is read as (`ansi`),
//! what the buffer keeps and drops (`buffer`), where the view is as lines
//! come and go (`follow`), how a program's colours read in
//! each theme (`palette`), what a search finds (`search`), what is
//! selected and copied (`select`), what is drawn (`drawn`), what the
//! pointer and the keyboard do on a window that lasts (`driven`), and how
//! long a frame takes with a million lines (`perf`).

mod ansi;
mod buffer;
mod drawn;
mod driven;
mod follow;
mod layout;
mod palette;
mod perf;
mod scan;
mod search;
mod select;
mod wide;

use iced::futures::executor::block_on;

use super::{Event, Line, Message, Metrics, State};

/// A viewer of eighty cells by ten rows, as its first layout would tell.
fn viewer(cap: usize) -> State {
    let mut state = State::new(cap);
    let _ = state.update(Message::Viewport(Metrics { cols: 80, rows: 10 }));
    state
}

/// `count` lines that say their number, from `from`.
fn numbered(from: usize, count: usize) -> impl Iterator<Item = Line> {
    (from..from + count).map(|n| Line::new(format!("line {n}")))
}

/// Appends lines that need no task.
fn feed(state: &mut State, lines: impl IntoIterator<Item = Line>) {
    let task = state.extend(lines);
    assert!(crate::test_support::tasks::is_none(task));
}

/// What the lines in view say, as a viewer of the state's last layout
/// shows them: the message of each line whose first row is in view.
fn shown(state: &State) -> Vec<String> {
    let metrics = state.metrics;
    let top = state.window(metrics).top;
    let mut lines = vec![];
    let mut rows = 0;
    let mut index = top.index;
    let mut row = top.row;
    while rows < metrics.rows {
        let Some(line) = state.seq_at(index).and_then(|seq| state.buffer.line(seq)) else {
            break;
        };
        if row == 0 {
            lines.push(line.message().to_owned());
        }
        rows += state.rows_of(index, metrics.cols) - row;
        index += 1;
        row = 0;
    }
    lines
}

/// Sends `message` and runs what it starts to its end, answers fed back,
/// as the window does. Returns what the viewer told the page.
fn drive(state: &mut State, message: Message) -> Vec<Event> {
    let mut events = vec![];
    let mut pending = vec![message];
    while let Some(message) = pending.pop() {
        let (task, event) = state.update(message);
        events.extend(event);
        pending.extend(block_on(crate::test_support::tasks::messages(task)));
    }
    events
}
