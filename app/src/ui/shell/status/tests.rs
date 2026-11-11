//! The status models, against the TypeScript tests they were ported from
//! (`status.test.ts`, `engine.test.ts`, the Cluster's `status.test.ts`,
//! `naming.test.ts`).

use homeport_clusters::{model::ClusterKind, registry::ClusterStatus};
use homeport_core::{
    error::{AppError, ErrorCode},
    host::EngineKind,
};
use homeport_hosts::registry::{HostStatus, SwarmView};

use super::{cluster, engine, host, swarm};
use crate::{
    test_support::{
        fixtures::{connected, connected_host, host_view, podman_host},
        snapshot::in_language,
    },
    ui::{shell::registry::Capability, theme::tokens::Tone},
};

fn refused() -> AppError {
    AppError::new(
        ErrorCode::ConnectionRefused,
        homeport_core::error::Message {
            key: "errors.test.refused",
            en: "connection refused",
        },
    )
}

#[test]
fn a_connected_host_is_summarised_with_what_it_runs() {
    let _english = in_language("en");
    let summary = host::describe(&connected());
    assert_eq!(summary.label, "Connected");
    assert_eq!(summary.tone, Tone::Success);
    assert_eq!(summary.detail.as_deref(), Some("Engine 29.1"));
    let podman = host::describe(&podman_host().status);
    assert_eq!(podman.detail.as_deref(), Some("Podman 5.8.1"));
    assert_eq!(summary.title(), "Connected: Engine 29.1");
}

#[test]
fn an_unreachable_host_says_why_and_when_it_retries() {
    let _english = in_language("en");
    let status = HostStatus::Unreachable {
        error: refused(),
        retry_in_secs: 8,
    };
    let summary = host::describe(&status);
    assert_eq!(summary.tone, Tone::Danger);
    assert_eq!(
        summary.detail.as_deref(),
        Some("connection refused — retrying in 8s")
    );
    assert_eq!(host::error(&status), Some(&refused()));
    assert_eq!(host::error(&HostStatus::Connecting), None);
}

#[test]
fn an_untrusted_server_key_asks_to_be_checked() {
    let _english = in_language("en");
    let summary = host::describe(&HostStatus::HostKeyUntrusted {
        key: "ssh-ed25519 AAAA".to_owned(),
        fingerprint: "SHA256:abc".to_owned(),
        bastion: None,
    });
    assert_eq!(summary.tone, Tone::Warning);
    assert!(summary.detail.unwrap().contains("SHA256:abc"));
}

#[test]
fn a_docker_engine_has_the_docker_pages_and_podman_both() {
    let docker = connected_host();
    assert!(engine::supports(Some(&docker), Capability::Docker));
    assert!(!engine::supports(Some(&docker), Capability::Podman));
    let podman = podman_host();
    assert!(engine::supports(Some(&podman), Capability::Docker));
    assert!(engine::supports(Some(&podman), Capability::Podman));
    // Podman is known only once connected.
    let connecting = host_view("lab", HostStatus::Connecting);
    assert!(engine::supports(Some(&connecting), Capability::Docker));
    assert!(!engine::supports(Some(&connecting), Capability::Podman));
    assert!(!engine::capability_known(&connecting, Capability::Podman));
    assert!(engine::capability_known(&connecting, Capability::Incus));
}

#[test]
fn another_kind_of_engine_has_its_own_pages_only() {
    let mut incus = connected_host();
    incus.host.engine = EngineKind::Incus;
    assert!(!engine::supports(Some(&incus), Capability::Docker));
    assert!(!engine::supports(Some(&incus), Capability::Podman));
    assert!(!engine::supports(Some(&incus), Capability::Containerd));
    assert!(engine::supports(Some(&incus), Capability::Incus));
    assert!(!engine::supports(None, Capability::Docker));
    let mut containerd = connected_host();
    containerd.host.engine = EngineKind::Containerd;
    assert!(engine::supports(Some(&containerd), Capability::Containerd));
    assert!(!engine::supports(Some(&containerd), Capability::Docker));
    assert!(!engine::supports(
        Some(&connected_host()),
        Capability::Containerd
    ));
}

#[test]
fn versions_compare_their_numbers_in_order() {
    assert!(engine::version_at_least("5.8.0", "5.8"));
    assert!(engine::version_at_least("5.10.1", "5.8"));
    assert!(engine::version_at_least("6.0.0-dev", "5.8"));
    assert!(!engine::version_at_least("5.7.2", "5.8"));
    assert!(!engine::version_at_least("4.9", "5.7"));
    assert!(!engine::version_at_least("", "5.7"));
    assert!(engine::podman_at_least(Some(&podman_host()), "5.8"));
    assert!(!engine::podman_at_least(Some(&podman_host()), "5.9"));
    assert!(!engine::podman_at_least(Some(&connected_host()), "1.0"));
}

#[test]
fn a_cluster_names_what_it_runs_once_connected() {
    let _english = in_language("en");
    let connected = |version: &str, region: Option<&str>| ClusterStatus::Connected {
        server_version: version.to_owned(),
        platform: None,
        region: region.map(str::to_owned),
    };
    let kubernetes = cluster::describe(&connected("v1.33.1", None), ClusterKind::Kubernetes);
    assert_eq!(kubernetes.label, "Connected");
    assert_eq!(kubernetes.detail.as_deref(), Some("Kubernetes v1.33.1"));
    let nomad = |version| cluster::describe(&connected(version, Some("eu")), ClusterKind::Nomad);
    assert_eq!(
        nomad("1.10.3").detail.as_deref(),
        Some("Nomad 1.10.3, region eu")
    );
    assert_eq!(nomad("").detail.as_deref(), Some("Nomad, region eu"));
}

#[test]
fn a_cluster_says_when_it_retries_and_what_needs_the_user() {
    let _english = in_language("en");
    let unreachable = ClusterStatus::Unreachable {
        error: refused(),
        retry_in_secs: 4,
    };
    assert_eq!(
        cluster::describe(&unreachable, ClusterKind::Kubernetes)
            .detail
            .as_deref(),
        Some("connection refused — retrying in 4s")
    );
    let unauthorized = ClusterStatus::Unauthorized { error: refused() };
    assert_eq!(
        cluster::describe(&unauthorized, ClusterKind::Kubernetes).tone,
        Tone::Danger
    );
    assert_eq!(unauthorized.error(), Some(&refused()));
    assert_eq!(ClusterStatus::Connecting.error(), None);
}

#[test]
fn a_swarm_is_named_after_its_first_manager_host() {
    let _english = in_language("en");
    let swarm = SwarmView {
        id: "swarm123456789abc".to_owned(),
        managers: vec!["gone".to_owned(), "h1".to_owned()],
        workers: vec![],
        reachable: true,
    };
    assert_eq!(swarm::label(&swarm, &[connected_host()]), "Swarm of local");
    assert_eq!(swarm::label(&swarm, &[]), "Swarm swarm1234567");
}
