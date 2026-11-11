//! Sample data for tests and snapshots: the same Hosts, containers,
//! networks, files, stats and services the React tests used.

use std::collections::HashMap;

use homeport_core::host::{EngineKind, Host, HostConnection, SshAuth};
use homeport_engine::{
    containers::{
        files::{FileEntry, FileKind},
        stats::StatsSample,
        Container,
    },
    networks::NetworkView,
    swarm::services::{ServiceMode, ServiceView},
};
use homeport_hosts::{registry::HostStatus, service::HostView};

/// A Docker Engine that answers.
pub fn connected() -> HostStatus {
    HostStatus::Connected {
        engine_version: "29.1".to_owned(),
        api_version: "1.52".to_owned(),
        os: "linux".to_owned(),
        arch: "amd64".to_owned(),
        podman_version: None,
        containerd_version: None,
        incus_version: None,
        swarm: None,
    }
}

/// This machine's Engine, connected.
pub fn connected_host() -> HostView {
    HostView {
        host: Host {
            id: "h1".to_owned(),
            name: "local".to_owned(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".to_owned(),
            },
            engine: EngineKind::Docker,
            enabled: true,
            host_key: None,
            created_at: String::new(),
        },
        status: connected(),
    }
}

/// The same local Host, connected to Podman.
pub fn podman_host() -> HostView {
    let mut view = connected_host();
    "podman".clone_into(&mut view.host.name);
    view.status = HostStatus::Connected {
        engine_version: "5.8.1".to_owned(),
        api_version: "1.41".to_owned(),
        os: "linux".to_owned(),
        arch: "amd64".to_owned(),
        podman_version: Some("5.8.1".to_owned()),
        containerd_version: None,
        incus_version: None,
        swarm: None,
    };
    view
}

/// An SSH Host named `name`, with `status`.
pub fn host_view(name: &str, status: HostStatus) -> HostView {
    HostView {
        host: Host {
            id: format!("{name}-id"),
            name: name.to_owned(),
            connection: HostConnection::Ssh {
                host: format!("{name}.example.com"),
                port: 22,
                user: "deploy".to_owned(),
                socket_path: "/var/run/docker.sock".to_owned(),
                auth: SshAuth::Generated,
                public_key: Some(
                    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHomeport homeport@test".to_owned(),
                ),
                bastions: vec![],
            },
            engine: EngineKind::Docker,
            enabled: true,
            host_key: None,
            created_at: String::new(),
        },
        status,
    }
}

/// The second word of `web-nginx-1`, as compose names a service's containers.
fn service_of(name: &str) -> Option<&str> {
    name.split('-').nth(1)
}

/// A container named `name`, of `stack` if any, in `state` (`running`,
/// `exited`…), created a minute before `now`.
pub fn container(name: &str, stack: Option<&str>, state: &str, now: i64) -> Container {
    Container {
        id: format!("{name}-0123456789abcdef"),
        name: name.to_owned(),
        image: format!("{}:latest", service_of(name).unwrap_or(name)),
        image_id: "sha256:abc".to_owned(),
        command: String::new(),
        created: now - 60,
        state: state.to_owned(),
        status: if state == "running" {
            "Up 1 minute".to_owned()
        } else {
            "Exited (0)".to_owned()
        },
        health: None,
        ports: vec![],
        labels: HashMap::new(),
        stack: stack.map(str::to_owned),
        service: stack.and_then(|_| service_of(name)).map(str::to_owned),
        networks: vec![],
    }
}

/// A custom bridge network with `containers` attached.
pub fn network(name: &str, containers: &[&str]) -> NetworkView {
    NetworkView {
        id: format!("{name}-id"),
        name: name.to_owned(),
        driver: "bridge".to_owned(),
        scope: "local".to_owned(),
        internal: false,
        attachable: false,
        subnets: vec!["172.20.0.0/16".to_owned()],
        created: None,
        labels: HashMap::new(),
        stack: None,
        built_in: false,
        containers: containers.iter().map(|c| (*c).to_owned()).collect(),
    }
}

/// A folder, a file and a link in `dir`.
pub fn file_entries(dir: &str) -> Vec<FileEntry> {
    let at = |name: &str| match dir {
        "/" => format!("/{name}"),
        dir => format!("{dir}/{name}"),
    };
    let entry = |name: &str, kind, size, mode: &str| FileEntry {
        name: name.to_owned(),
        path: at(name),
        kind,
        size,
        modified: 1_700_000_000,
        mode: mode.to_owned(),
    };
    vec![
        entry("etc", FileKind::Directory, 0, "drwxr-xr-x"),
        entry("hello.txt", FileKind::File, 2048, "-rw-r--r--"),
        entry("lib", FileKind::Symlink, 7, "lrwxrwxrwx"),
    ]
}

/// A stats sample; counters grow with `n` so rates are positive.
pub fn stats_sample(n: u64) -> StatsSample {
    StatsSample {
        cpu_percent: 12.5,
        memory_usage: 64 * 1024 * 1024,
        memory_limit: 1024 * 1024 * 1024,
        net_rx: 1000 * n,
        net_tx: 500 * n,
        block_read: 0,
        block_write: 0,
        pids: 3,
    }
}

/// A Swarm service, replicated with 3 of 3 running.
pub fn service_view() -> ServiceView {
    ServiceView {
        id: "svc1".to_owned(),
        version: 3,
        name: "web".to_owned(),
        image: "nginx:1.27".to_owned(),
        image_digest: None,
        mode: ServiceMode::Replicated,
        replicas: Some(3),
        running: 3,
        desired: 3,
        ports: vec![],
        update_state: None,
        update_message: None,
        stack: None,
        labels: HashMap::new(),
        created: None,
        updated: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stack_container_is_named_after_its_service() {
        let web = container("shop-web-1", Some("shop"), "running", 1_700_000_060);
        assert_eq!(web.image, "web:latest");
        assert_eq!(web.service.as_deref(), Some("web"));
        assert_eq!(web.created, 1_700_000_000);
        let alone = container("redis", None, "exited", 0);
        assert_eq!(
            (alone.image.as_str(), alone.service),
            ("redis:latest", None)
        );
        assert_eq!(alone.status, "Exited (0)");
    }

    #[test]
    fn files_sit_in_their_folder() {
        assert_eq!(file_entries("/")[1].path, "/hello.txt");
        assert_eq!(file_entries("/var")[0].path, "/var/etc");
    }

    #[test]
    fn the_sample_hosts_are_connected() {
        assert_eq!(connected_host().host.name, "local");
        assert!(matches!(
            podman_host().status,
            HostStatus::Connected {
                podman_version: Some(_),
                ..
            }
        ));
        assert_eq!(host_view("prod", connected()).host.id, "prod-id");
        assert_eq!(network("front", &["web"]).containers, ["web"]);
        assert!(stats_sample(2).net_rx > stats_sample(1).net_rx);
        assert_eq!(service_view().running, 3);
    }
}
