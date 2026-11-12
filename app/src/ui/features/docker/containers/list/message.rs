//! What happens on the Containers page.

use crate::ui::{
    domain::{
        containers::{actions, model::Usage, Bulk},
        freshness,
    },
    shell::router::Route,
    widgets::{feedback::error_message, table},
};

#[derive(Debug, Clone)]
pub enum Message {
    Table(table::Message<String>),
    Search(String),
    OnlyRunning(bool),
    /// A container's button, its menu, or the dialog one of them opened.
    Actions(actions::Message),
    /// A button of the selection's bar.
    Bulk(Bulk),
    /// "Check for newer images".
    Check(freshness::Message),
    /// A report of the Host's usage stream, with the Host's id.
    Usage((String, Usage)),
    /// "Run container": the button, `n` or the palette.
    Run,
    /// A published port was pressed.
    OpenPort(u16),
    /// A link of a row: its Stack, its pod, its Quadlet unit.
    Go(Route),
    /// What was asked of the failure shown in place of the list.
    Failure(error_message::Event),
}
