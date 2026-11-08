//! What the tray's menu holds, from the Hosts and their statuses: pure, so
//! it's tested without a desktop.

use homeport_hosts::{registry::HostStatus, service::HostView};

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
fn status_label(status: &HostStatus) -> &'static str {
    match status {
        HostStatus::Connected { .. } => "Connected",
        HostStatus::Connecting => "Connecting",
        HostStatus::Unreachable { .. } => "Unreachable",
        HostStatus::Disabled => "Disabled",
        _ => "Needs attention",
    }
}

/// The tray's menu for `hosts`, in the order they're listed.
pub fn menu(hosts: &[HostView]) -> TrayMenu {
    let connected = hosts
        .iter()
        .filter(|h| matches!(h.status, HostStatus::Connected { .. }))
        .count();
    let tooltip = match hosts.len() {
        0 => "Homeport — no Hosts yet".to_owned(),
        1 => format!("Homeport — {connected} of 1 Host connected"),
        n => format!("Homeport — {connected} of {n} Hosts connected"),
    };
    let mut items: Vec<Item> = hosts
        .iter()
        .map(|h| Item::Host {
            id: h.host.id.clone(),
            label: format!("{} — {}", h.host.name, status_label(&h.status)),
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
mod tests {
    use homeport_core::error::{AppError, ErrorCode};
    use homeport_hosts::{Host, HostConnection};

    use super::*;

    fn host(id: &str, name: &str, status: HostStatus) -> HostView {
        HostView {
            host: Host {
                id: id.into(),
                name: name.into(),
                connection: HostConnection::Local {
                    socket_path: "/var/run/docker.sock".into(),
                },
                enabled: true,
                host_key: None,
                created_at: String::new(),
            },
            status,
        }
    }

    fn connected() -> HostStatus {
        HostStatus::Connected {
            engine_version: "28.0".into(),
            api_version: "1.48".into(),
            os: "linux".into(),
            arch: "x86_64".into(),
            swarm: None,
        }
    }

    fn unreachable() -> HostStatus {
        HostStatus::Unreachable {
            error: AppError::new(
                ErrorCode::ConnectionLost,
                homeport_hosts::messages::CONNECTION_LOST.with("name", "prod-server"),
            ),
            retry_in_secs: 8,
        }
    }

    #[test]
    fn lists_every_host_with_its_status_then_the_app_actions() {
        let menu = menu(&[
            host("a", "laptop", connected()),
            host("b", "prod-server", unreachable()),
            host("c", "nas", HostStatus::Disabled),
            host("d", "lab", HostStatus::CredentialsLost),
        ]);
        assert_eq!(menu.tooltip, "Homeport — 1 of 4 Hosts connected");
        assert_eq!(
            menu.items,
            vec![
                Item::Host {
                    id: "a".into(),
                    label: "laptop — Connected".into(),
                    reconnect: false,
                },
                Item::Host {
                    id: "b".into(),
                    label: "prod-server — Unreachable".into(),
                    reconnect: true,
                },
                Item::Host {
                    id: "c".into(),
                    label: "nas — Disabled".into(),
                    reconnect: false,
                },
                Item::Host {
                    id: "d".into(),
                    label: "lab — Needs attention".into(),
                    reconnect: false,
                },
                Item::Separator,
                Item::Open,
                Item::AddHost,
                Item::Separator,
                Item::Quit,
            ]
        );
    }

    #[test]
    fn without_hosts_only_the_app_actions_are_listed() {
        let menu = menu(&[]);
        assert_eq!(menu.tooltip, "Homeport — no Hosts yet");
        assert_eq!(
            menu.items,
            vec![Item::Open, Item::AddHost, Item::Separator, Item::Quit]
        );
    }

    #[test]
    fn one_host_is_counted_in_the_singular() {
        let menu = menu(&[host("a", "laptop", HostStatus::Connecting)]);
        assert_eq!(menu.tooltip, "Homeport — 0 of 1 Host connected");
    }

    #[test]
    fn menu_ids_round_trip_to_their_action() {
        for action in [
            Action::ChooseHost("h-1".into()),
            Action::Reconnect("h-1".into()),
            Action::Open,
            Action::AddHost,
            Action::Quit,
        ] {
            assert_eq!(Action::parse(&action.id()), Some(action));
        }
        assert_eq!(Action::parse("something-else"), None);
    }
}
