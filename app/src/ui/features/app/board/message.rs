//! What happens on the Board.

use crate::ui::{
    domain::containers::{actions, model::Usage, Bulk},
    shell::router::Route,
    widgets::{feedback::error_message, form::select, overlay::menu, table},
};

#[derive(Debug, Clone)]
pub enum Message {
    Table(table::Message<String>),
    Search(String),
    /// The menu of the Hosts to show.
    HostsMenu(menu::Event),
    /// A Host of that menu is shown or no longer.
    ToggleHost(String),
    StateSelect(select::Message),
    StackSelect(select::Message),
    ClearFilters,
    /// A container's button, its menu, or the dialog one of them opened.
    Actions(actions::Message),
    /// A button of the selection's bar.
    Bulk(Bulk),
    /// A report of a Host's usage stream, with the Host's id.
    Usage((String, Usage)),
    /// "Open host", beside a Host that isn't shown.
    Go(Route),
    /// What was asked of the failure of this Host's list.
    Failure(String, error_message::Event),
}
