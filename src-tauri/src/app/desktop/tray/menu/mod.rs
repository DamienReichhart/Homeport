//! What the tray's menu holds, from the Hosts and their statuses: pure, so
//! it's tested without a desktop.

use homeport_hosts::{registry::HostStatus, service::HostView};

use crate::messages as msg;

/// One entry of the tray's menu, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// "<name> — <status>". `reconnect` adds a submenu with **Reconnect**
    /// for an Unreachable Host.
    Host {
        id: String,
        label: String,
        reconnect: bool,
    },
    Separator,
    Open,
    AddHost,
    Quit,
}

/// The menu and the icon's tooltip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayMenu {
    pub tooltip: String,
    pub items: Vec<Item>,
}

/// What choosing a menu item does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Opens the window on that Host's Overview.
    ChooseHost(String),
    Reconnect(String),
    Open,
    AddHost,
    Quit,
}

const HOST: &str = "host:";
const RECONNECT: &str = "reconnect:";
const OPEN: &str = "open";
const ADD_HOST: &str = "add-host";
const QUIT: &str = "quit";

impl Action {
    /// The menu item id that triggers this action.
    pub fn id(&self) -> String {
        match self {
            Self::ChooseHost(id) => format!("{HOST}{id}"),
            Self::Reconnect(id) => format!("{RECONNECT}{id}"),
            Self::Open => OPEN.to_owned(),
            Self::AddHost => ADD_HOST.to_owned(),
            Self::Quit => QUIT.to_owned(),
        }
    }

    /// The action of a menu item id, `None` for one that does nothing.
    pub fn parse(id: &str) -> Option<Self> {
        if let Some(host) = id.strip_prefix(HOST) {
            return Some(Self::ChooseHost(host.to_owned()));
        }
        if let Some(host) = id.strip_prefix(RECONNECT) {
            return Some(Self::Reconnect(host.to_owned()));
        }
        match id {
            OPEN => Some(Self::Open),
            ADD_HOST => Some(Self::AddHost),
            QUIT => Some(Self::Quit),
            _ => None,
        }
    }
}

/// The words for a Host's status, as the Hosts page puts them.
fn status_label(status: &HostStatus) -> String {
    match status {
        HostStatus::Connected { .. } => msg::TRAY_CONNECTED,
        HostStatus::Connecting => msg::TRAY_CONNECTING,
        HostStatus::Unreachable { .. } => msg::TRAY_UNREACHABLE,
        HostStatus::Disabled => msg::TRAY_DISABLED,
        _ => msg::TRAY_NEEDS_ATTENTION,
    }
    .localized()
}

/// The tray's menu for `hosts`, in the order they're listed.
pub fn menu(hosts: &[HostView]) -> TrayMenu {
    let connected = hosts
        .iter()
        .filter(|h| matches!(h.status, HostStatus::Connected { .. }))
        .count();
    let tooltip = match hosts.len() {
        0 => msg::TRAY_NO_HOSTS.localized(),
        1 => msg::TRAY_CONNECTED_ONE
            .with("connected", connected)
            .localized(),
        n => msg::TRAY_CONNECTED_MANY
            .with("connected", connected)
            .with("total", n)
            .localized(),
    };
    let mut items: Vec<Item> = hosts
        .iter()
        .map(|h| Item::Host {
            id: h.host.id.clone(),
            label: msg::TRAY_HOST
                .with("host", h.host.name.as_str())
                .with("status", status_label(&h.status))
                .localized(),
            reconnect: matches!(h.status, HostStatus::Unreachable { .. }),
        })
        .collect();
    if !items.is_empty() {
        items.push(Item::Separator);
    }
    items.extend([Item::Open, Item::AddHost, Item::Separator, Item::Quit]);
    TrayMenu { tooltip, items }
}

#[cfg(test)]
mod tests;
