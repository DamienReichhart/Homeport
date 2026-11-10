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
            engine: homeport_hosts::EngineKind::Docker,
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
        podman_version: None,
        containerd_version: None,
        incus_version: None,
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
