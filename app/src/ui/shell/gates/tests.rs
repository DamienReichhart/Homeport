//! What each gate decides, for every status a Host or a Cluster can be in.

use std::sync::Arc;

use homeport_clusters::{model::ClusterKind, registry::ClusterStatus};
use homeport_core::{
    error::{AppError, ErrorCode, Message as Text},
    host::EngineKind,
};
use homeport_hosts::{
    registry::{HostStatus, SwarmView},
    service::HostView,
};
use homeport_settings::Settings;

use super::{cluster_down, decide, host_down, lacks, Subject, Verdict};
use crate::{
    data::Resource,
    test_support::{
        fixtures::{cluster_view, connected_host, host_view, nomad_cluster_view, podman_host},
        snapshot::in_language,
    },
    ui::shell::{page::World, registry::Capability, router::Route, selection::Selection},
};

fn refused() -> AppError {
    AppError::new(
        ErrorCode::ConnectionRefused,
        Text {
            key: "errors.test.refused",
            en: "connection refused",
        },
    )
}

fn with_host(view: HostView) -> World {
    let mut world = World::new(Settings::default());
    world.selection = Some(Selection::Host {
        id: view.host.id.clone(),
    });
    world.hosts = Resource::Loaded(Arc::new(vec![view]));
    world.clusters = Resource::Loaded(Arc::new(vec![]));
    world
}

fn with_cluster(view: homeport_clusters::service::ClusterView) -> World {
    let mut world = World::new(Settings::default());
    world.selection = Some(Selection::Cluster {
        id: view.cluster.id.clone(),
    });
    world.hosts = Resource::Loaded(Arc::new(vec![]));
    world.clusters = Resource::Loaded(Arc::new(vec![view]));
    world
}

fn host_of(engine: EngineKind) -> HostView {
    let mut view = connected_host();
    view.host.engine = engine;
    view
}

#[test]
fn a_host_page_waits_for_the_hosts_then_welcomes_when_there_is_none() {
    let mut world = World::new(Settings::default());
    assert_eq!(decide(&Route::Containers, &world), Verdict::Loading);
    world.hosts = Resource::Loaded(Arc::new(vec![]));
    assert_eq!(decide(&Route::Containers, &world), Verdict::Welcome);
    assert_eq!(decide(&Route::Overview, &world), Verdict::Welcome);
    // What is about the app shows whatever is selected.
    assert_eq!(
        decide(&Route::Settings, &world),
        Verdict::Open(Subject::App)
    );
    assert_eq!(decide(&Route::Board, &world), Verdict::Open(Subject::App));
}

#[test]
fn a_connected_host_opens_its_pages_and_a_connecting_one_says_so() {
    let world = with_host(connected_host());
    let open = Verdict::Open(Subject::Host {
        host_id: "h1".to_owned(),
    });
    assert_eq!(decide(&Route::Containers, &world), open);
    assert_eq!(decide(&Route::Overview, &world), open);
    let world = with_host(host_view("lab", HostStatus::Connecting));
    assert_eq!(decide(&Route::Images, &world), Verdict::Connecting("lab"));
}

#[test]
fn every_status_that_isn_t_connected_says_why_and_offers_what_react_did() {
    let _english = in_language("en");
    let down = |status| {
        let view = host_view("prod", status);
        let world = with_host(host_view("prod", view.status.clone()));
        assert!(matches!(
            decide(&Route::Containers, &world),
            Verdict::HostDown(_)
        ));
        let down = host_down(&view);
        (
            down.title,
            down.error.cloned(),
            down.retrying_in,
            down.detail,
            down.retry_now,
        )
    };
    let error = Some(refused());
    assert_eq!(
        down(HostStatus::Disabled),
        ("prod: Disabled".to_owned(), None, None, None, false)
    );
    assert_eq!(
        down(HostStatus::Unreachable {
            error: refused(),
            retry_in_secs: 8
        }),
        (
            "prod: Unreachable".to_owned(),
            error.clone(),
            Some(8),
            None,
            false
        )
    );
    assert_eq!(
        down(HostStatus::Unauthorized { error: refused() }),
        (
            "prod: Access refused".to_owned(),
            error.clone(),
            None,
            None,
            false
        )
    );
    let untrusted = down(HostStatus::HostKeyUntrusted {
        key: "ssh-ed25519 AAAA".to_owned(),
        fingerprint: "SHA256:abc".to_owned(),
        bastion: None,
    });
    assert_eq!(untrusted.0, "prod: Server key not trusted");
    assert!(untrusted.3.unwrap().contains("SHA256:abc"));
    assert!(untrusted.4, "no cause to retry from: a Retry now button");
    let mismatch = down(HostStatus::HostKeyMismatch {
        key: String::new(),
        fingerprint: "SHA256:new".to_owned(),
        expected_fingerprint: "SHA256:old".to_owned(),
        bastion: None,
    });
    assert_eq!(
        mismatch.3.as_deref(),
        Some("Expected SHA256:old, got SHA256:new.")
    );
    assert!(mismatch.4);
    let lost = down(HostStatus::CredentialsLost);
    assert_eq!(lost.0, "prod: Credentials lost");
    assert!(lost.3.is_some() && lost.4);
    assert_eq!(
        down(HostStatus::KeychainUnavailable { error: refused() }).1,
        error
    );
    assert_eq!(
        down(HostStatus::Invalid { error: refused() }).0,
        "prod: Misconfigured"
    );
}

#[test]
fn a_page_says_when_the_host_s_engine_doesn_t_offer_what_it_requires() {
    let docker = with_host(connected_host());
    assert!(matches!(
        decide(&Route::PodmanPods, &docker),
        Verdict::Unsupported {
            requires: Capability::Podman,
            ..
        }
    ));
    assert!(matches!(
        decide(&Route::IncusInstances, &docker),
        Verdict::Unsupported {
            requires: Capability::Incus,
            ..
        }
    ));
    let incus = with_host(host_of(EngineKind::Incus));
    assert!(matches!(
        decide(&Route::Containers, &incus),
        Verdict::Unsupported {
            requires: Capability::Docker,
            ..
        }
    ));
    assert!(matches!(
        decide(&Route::PodmanPods, &with_host(podman_host())),
        Verdict::Open(_)
    ));
    // Podman is known only once connected: until then the page connects.
    let connecting = host_view("lab", HostStatus::Connecting);
    assert_eq!(lacks(Some(&connecting), Some(Capability::Podman)), None);
    assert_eq!(
        decide(&Route::PodmanPods, &with_host(connecting)),
        Verdict::Connecting("lab")
    );
    assert_eq!(lacks(None, Some(Capability::Docker)), None);
}

#[test]
fn a_containerd_page_waits_for_the_namespace_its_pages_show() {
    let mut world = with_host(host_of(EngineKind::Containerd));
    assert_eq!(decide(&Route::ContainerdImages, &world), Verdict::Loading);
    world.containerd = Resource::Loaded(Arc::new(vec![]));
    assert_eq!(
        decide(&Route::ContainerdImages, &world),
        Verdict::Open(Subject::Containerd {
            host_id: "h1".to_owned(),
            namespace: "default".to_owned(),
        })
    );
    // The page of every namespace needs none.
    world.containerd = Resource::Idle;
    assert!(matches!(
        decide(&Route::ContainerdNamespaces, &world),
        Verdict::Open(Subject::Host { .. })
    ));
    // A list that can't be read shows `default`; a remembered one wins.
    world.containerd = Resource::Failed(refused(), None);
    assert!(matches!(
        decide(&Route::ContainerdImages, &world),
        Verdict::Open(_)
    ));
    world
        .settings
        .containerd_namespaces
        .insert("h1".to_owned(), "k8s.io".to_owned());
    world.containerd = Resource::Loading;
    assert_eq!(
        decide(&Route::ContainerdSnapshots, &world),
        Verdict::Open(Subject::Containerd {
            host_id: "h1".to_owned(),
            namespace: "k8s.io".to_owned(),
        })
    );
}

#[test]
fn an_incus_page_waits_for_the_project_its_pages_show() {
    let mut world = with_host(host_of(EngineKind::Incus));
    assert_eq!(decide(&Route::IncusInstances, &world), Verdict::Loading);
    world.incus = Resource::Loaded(Arc::new(vec![]));
    assert_eq!(
        decide(&Route::IncusStorage { pool: None }, &world),
        Verdict::Open(Subject::Incus {
            host_id: "h1".to_owned(),
            project: "default".to_owned(),
        })
    );
}

#[test]
fn a_cluster_page_opens_on_a_connected_cluster_with_its_namespace() {
    let world = with_cluster(cluster_view("kind-dev"));
    let open = Verdict::Open(Subject::Cluster {
        cluster_id: "c1".to_owned(),
        namespace: Some("default".to_owned()),
    });
    assert_eq!(decide(&Route::Pods, &world), open);
    assert_eq!(
        decide(&Route::Overview, &world),
        open,
        "the Cluster's overview"
    );
    assert_eq!(
        decide(&Route::PortForwards, &world),
        Verdict::Open(Subject::App)
    );
    let mut none = World::new(Settings::default());
    assert_eq!(decide(&Route::Pods, &none), Verdict::Loading);
    none.clusters = Resource::Loaded(Arc::new(vec![]));
    assert_eq!(decide(&Route::Pods, &none), Verdict::NoCluster);
    none.clusters = Resource::Failed(refused(), None);
    assert_eq!(
        decide(&Route::Pods, &none),
        Verdict::ClustersFailed(&refused())
    );
}

#[test]
fn a_cluster_that_isn_t_connected_says_why_and_a_wrong_kind_says_so() {
    let _english = in_language("en");
    let mut view = cluster_view("kind-dev");
    view.status = ClusterStatus::Connecting;
    assert_eq!(
        decide(&Route::Pods, &with_cluster(view)),
        Verdict::Connecting("kind-dev")
    );
    let down = |status: ClusterStatus| {
        let mut view = cluster_view("kind-dev");
        view.status = status;
        let world = with_cluster(cluster_view("kind-dev"));
        drop(world);
        let down = cluster_down(&view);
        (
            down.title,
            down.error.is_some(),
            down.retrying_in,
            down.detail.is_some(),
            down.retry_now,
        )
    };
    assert_eq!(
        down(ClusterStatus::Disabled),
        ("kind-dev: Disabled".to_owned(), false, None, false, false)
    );
    assert_eq!(
        down(ClusterStatus::Unreachable {
            error: refused(),
            retry_in_secs: 4
        }),
        (
            "kind-dev: Unreachable".to_owned(),
            true,
            Some(4),
            false,
            false
        )
    );
    assert_eq!(
        down(ClusterStatus::Unauthorized { error: refused() }),
        (
            "kind-dev: Access refused".to_owned(),
            true,
            None,
            false,
            false
        )
    );
    assert_eq!(
        down(ClusterStatus::CredentialsLost),
        (
            "kind-dev: Credentials lost".to_owned(),
            false,
            None,
            true,
            true
        )
    );
    assert!(down(ClusterStatus::KeychainUnavailable { error: refused() }).1);
    assert!(down(ClusterStatus::Invalid { error: refused() }).1);
    let mut unreachable = cluster_view("kind-dev");
    unreachable.status = ClusterStatus::Unreachable {
        error: refused(),
        retry_in_secs: 4,
    };
    assert!(matches!(
        decide(&Route::Nodes, &with_cluster(unreachable)),
        Verdict::ClusterDown(_)
    ));
    // A Nomad Cluster can't show a Kubernetes page, nor the reverse.
    let nomad = with_cluster(nomad_cluster_view("lab"));
    assert!(matches!(
        decide(&Route::Pods, &nomad),
        Verdict::WrongKind {
            wanted: ClusterKind::Kubernetes,
            ..
        }
    ));
    assert!(matches!(
        decide(&Route::NomadJobs, &nomad),
        Verdict::Open(_)
    ));
    assert!(matches!(
        decide(&Route::NomadJobs, &with_cluster(cluster_view("kind-dev"))),
        Verdict::WrongKind {
            wanted: ClusterKind::Nomad,
            ..
        }
    ));
}

#[test]
fn a_swarm_page_opens_once_a_manager_answers() {
    let route = Route::Swarm {
        swarm_id: "sw1".to_owned(),
        tab: None,
    };
    let mut world = with_host(connected_host());
    assert_eq!(decide(&route, &world), Verdict::Loading);
    world.swarms = Resource::Loaded(Arc::new(vec![]));
    assert_eq!(decide(&route, &world), Verdict::SwarmGone);
    world.swarms = Resource::Loaded(Arc::new(vec![SwarmView {
        id: "sw1".to_owned(),
        managers: vec!["h1".to_owned()],
        workers: vec![],
        reachable: true,
    }]));
    assert_eq!(
        decide(&route, &world),
        Verdict::Open(Subject::Swarm {
            swarm_id: "sw1".to_owned()
        })
    );
    world.swarm = Resource::Failed(refused(), None);
    assert_eq!(decide(&route, &world), Verdict::SwarmFailed(&refused()));
}
