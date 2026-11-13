//! The Processes tab (`ProcessesTab`): what runs in a running container
//! (`docker top`), read again every five seconds while the tab shows.

use std::{sync::Arc, time::Duration};

use homeport_engine::containers::Processes;

use crate::ui::widgets::{feedback::error_message, table};

/// How often the list is read again (`refetchInterval: 5000`).
pub const EVERY: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub enum Message {
    Table(table::Message<usize>),
    /// It is time to read the list again.
    Poll,
    /// What was asked of the failure shown in place of the list.
    Failure(error_message::Event),
}

/// A process: its place in the Engine's list and its cells.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub at: usize,
    pub cells: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Tab {
    pub table: table::State<usize>,
    /// The list's titles and rows, as last read.
    pub titles: Vec<String>,
    pub rows: Vec<Row>,
    /// The read they were made from.
    seen: Option<Arc<Processes>>,
}

impl Tab {
    /// Takes a list that was read: nothing is done while it is the one
    /// the rows were made from.
    pub fn take(&mut self, read: Option<&Arc<Processes>>) {
        let Some(read) = read else { return };
        if self
            .seen
            .as_ref()
            .is_some_and(|seen| Arc::ptr_eq(seen, read))
        {
            return;
        }
        self.titles.clone_from(&read.titles);
        self.rows = read
            .processes
            .iter()
            .enumerate()
            .map(|(at, cells)| Row {
                at,
                cells: cells.clone(),
            })
            .collect();
        self.seen = Some(read.clone());
    }
}
