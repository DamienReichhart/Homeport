//! The Inspect tab (`InspectTab`): the inspect document as readable
//! sections, with the document itself one tab away.

use std::sync::Arc;

use iced::Task;

use crate::ui::{
    features::docker::containers::detail::{
        model::inspect::{Details, Inspected},
        view::inspect as view,
    },
    shell::Context,
    widgets::{
        data::json,
        feedback::error_message,
        table::{self, Rows},
    },
};

/// A table of the sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Checks,
    Environment,
    Mounts,
    Ports,
    Networks,
    Labels,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// "Details" or "Raw JSON".
    Show(usize),
    Table(Section, table::Message<usize>),
    Json(json::Message),
    /// What was asked of the failure shown in place of the document.
    Failure(error_message::Event),
}

/// What the page is asked to do about the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Read it again.
    Retry,
    /// Show the Details of why it couldn't be read.
    Details,
}

/// The sort, the widths and the scroll of each section's table.
#[derive(Debug, Default)]
pub struct Tables {
    pub checks: table::State<usize>,
    pub environment: table::State<usize>,
    pub mounts: table::State<usize>,
    pub ports: table::State<usize>,
    pub networks: table::State<usize>,
    pub labels: table::State<usize>,
}

#[derive(Default)]
pub struct Inspect {
    /// Which of the two views shows: the sections, or the raw document.
    pub raw: bool,
    pub tables: Box<Tables>,
    /// The raw view's tree, once a document was read.
    pub json: Option<json::State>,
    /// The read the tree was made from.
    seen: Option<Arc<Inspected>>,
}

impl std::fmt::Debug for Inspect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The tree of a document is not worth printing.
        f.debug_struct("Inspect")
            .field("raw", &self.raw)
            .field("read", &self.seen.is_some())
            .finish_non_exhaustive()
    }
}

impl Inspect {
    /// The document was read, or read again: the raw view takes it.
    /// What was open in the tree stays open, by path. Nothing is done
    /// while it is the read the tree already holds.
    pub fn take(&mut self, read: Option<&Arc<Inspected>>) -> Task<Message> {
        let Some(read) = read else {
            return Task::none();
        };
        if self
            .seen
            .as_ref()
            .is_some_and(|seen| Arc::ptr_eq(seen, read))
        {
            return Task::none();
        }
        self.seen = Some(read.clone());
        let document = read.document.clone();
        match &mut self.json {
            Some(json) => json.set_document(document).map(Message::Json),
            None => {
                self.json = Some(json::State::new(document));
                Task::none()
            }
        }
    }

    /// The sections of the document that was read last.
    fn details(&self) -> Option<&Details> {
        self.seen.as_ref().map(|read| &read.details)
    }

    fn table(&mut self, section: Section, message: table::Message<usize>) {
        let Some(read) = self.seen.clone() else {
            return;
        };
        let details = &read.details;
        let tables = &mut self.tables;
        // No row opens and none is selected: only what the table itself
        // keeps (its widths, its scroll) changes.
        match section {
            Section::Checks => {
                let none = vec![];
                let checks = details.health.as_ref().map_or(&none, |h| &h.checks);
                let rows = Rows::new(checks, |check| check.at);
                tables.checks.update(message, &rows, &view::checks());
            }
            Section::Environment => {
                let rows = Rows::new(&details.env, |pair| pair.at);
                tables
                    .environment
                    .update(message, &rows, &view::pairs(false));
            }
            Section::Mounts => {
                let rows = Rows::new(&details.mounts, |mount| mount.at);
                tables.mounts.update(message, &rows, &view::mounts());
            }
            Section::Ports => {
                let rows = Rows::new(&details.ports, |port| port.at);
                tables.ports.update(message, &rows, &view::ports());
            }
            Section::Networks => {
                let rows = Rows::new(&details.networks, |network| network.at);
                tables.networks.update(message, &rows, &view::networks());
            }
            Section::Labels => {
                let rows = Rows::new(&details.labels, |pair| pair.at);
                tables.labels.update(message, &rows, &view::pairs(true));
            }
        }
    }

    pub fn update(
        &mut self,
        message: Message,
        ctx: &Context<'_>,
    ) -> (Task<Message>, Option<Event>) {
        match message {
            Message::Show(index) => self.raw = index == 1,
            Message::Table(section, message) => {
                if self.details().is_some() {
                    self.table(section, message);
                }
            }
            Message::Json(message) => {
                if let Some(json) = &mut self.json {
                    let (task, event) = json.update(message);
                    if let Some(json::Event::Copy(text)) = event {
                        ctx.copy(text);
                    }
                    return (task.map(Message::Json), None);
                }
            }
            Message::Failure(error_message::Event::Retry) => {
                return (Task::none(), Some(Event::Retry));
            }
            Message::Failure(error_message::Event::Details) => {
                return (Task::none(), Some(Event::Details));
            }
            Message::Failure(error_message::Event::Copy(command)) => ctx.copy(command),
        }
        (Task::none(), None)
    }
}
