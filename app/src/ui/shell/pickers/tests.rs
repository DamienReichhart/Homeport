//! The pickers' rules, as `containerd.test.ts`, `incus.test.ts` and the
//! two `namespace.test.ts` held them.

use std::collections::HashMap;

use homeport_clusters::model::ClusterConnection;
use homeport_containerd::namespaces::ContainerdNamespace;
use homeport_incus::projects::IncusProject;
use homeport_nomad::namespaces::{NomadNamespaceView, NomadNamespaces};

use super::model::{
    cluster_namespace, containerd_namespace, incus_project, listed, nomad_namespace,
    preferred_namespace, preferred_project, Remembered,
};
use crate::test_support::fixtures::cluster_view;

fn ns(name: &str, containers: u32, images: u32) -> ContainerdNamespace {
    ContainerdNamespace {
        name: name.to_owned(),
        labels: HashMap::new(),
        containers,
        images,
    }
}

fn project(name: &str, instances: u32) -> IncusProject {
    IncusProject {
        name: name.to_owned(),
        description: String::new(),
        instances,
        images: 0,
    }
}

fn remembered(id: &str, value: &str) -> Remembered {
    Remembered::from([(id.to_owned(), value.to_owned())])
}

#[test]
fn a_containerd_host_shows_default_unless_only_another_namespace_holds_something() {
    assert_eq!(preferred_namespace(&[]), "default");
    assert_eq!(preferred_namespace(&[ns("default", 0, 0)]), "default");
    assert_eq!(
        preferred_namespace(&[ns("default", 0, 0), ns("k8s.io", 12, 30)]),
        "k8s.io"
    );
    assert_eq!(
        preferred_namespace(&[ns("default", 1, 0), ns("k8s.io", 12, 30)]),
        "default"
    );
    assert_eq!(
        preferred_namespace(&[ns("buildkit", 0, 2), ns("k8s.io", 12, 0)]),
        "default"
    );
}

#[test]
fn the_namespace_remembered_for_the_host_wins_once_picked() {
    let list = [ns("default", 0, 0), ns("k8s.io", 3, 0)];
    let moby = remembered("h1", "moby");
    assert_eq!(
        containerd_namespace("h1", &moby, Some(&list)).as_deref(),
        Some("moby")
    );
    assert_eq!(
        containerd_namespace("h2", &moby, Some(&list)).as_deref(),
        Some("k8s.io")
    );
    assert_eq!(containerd_namespace("h2", &Remembered::new(), None), None);
}

#[test]
fn an_incus_host_shows_default_or_the_restricted_socket_s_own_project() {
    assert_eq!(
        preferred_project(&[project("default", 0), project("lab", 3)]),
        "default"
    );
    assert_eq!(preferred_project(&[]), "default");
    assert_eq!(preferred_project(&[project("user-1000", 2)]), "user-1000");
}

#[test]
fn the_remembered_project_is_kept_while_the_host_has_it() {
    let lab = remembered("h1", "lab");
    let both = [project("default", 0), project("lab", 0)];
    assert_eq!(
        incus_project("h1", &lab, Some(&both)).as_deref(),
        Some("lab")
    );
    assert_eq!(incus_project("h1", &lab, None).as_deref(), Some("lab"));
    // It falls back when it is gone, and waits for the list.
    let gone = remembered("h1", "gone");
    assert_eq!(
        incus_project("h1", &gone, Some(&[project("default", 0)])).as_deref(),
        Some("default")
    );
    assert_eq!(incus_project("h1", &Remembered::new(), None), None);
}

#[test]
fn a_cluster_starts_at_its_context_s_namespace() {
    let cluster = cluster_view("kind-dev");
    assert_eq!(
        cluster_namespace(&cluster, &Remembered::new()).as_deref(),
        Some("default")
    );
    let mut shop = cluster_view("kind-dev");
    if let ClusterConnection::Kubernetes(connection) = &mut shop.cluster.connection {
        connection.namespace = Some("shop".to_owned());
    }
    assert_eq!(
        cluster_namespace(&shop, &Remembered::new()).as_deref(),
        Some("shop")
    );
}

#[test]
fn a_cluster_keeps_the_namespace_remembered_for_it_or_every_namespace() {
    let cluster = cluster_view("kind-dev");
    assert_eq!(
        cluster_namespace(&cluster, &remembered("c1", "kube-system")).as_deref(),
        Some("kube-system")
    );
    assert_eq!(cluster_namespace(&cluster, &remembered("c1", "")), None);
    assert_eq!(
        cluster_namespace(&cluster, &remembered("other", "x")).as_deref(),
        Some("default")
    );
}

#[test]
fn the_jobs_page_asks_nomad_for_the_one_chosen_or_every_one() {
    let name = |name: &str| NomadNamespaceView {
        name: name.to_owned(),
        description: None,
    };
    let listing = NomadNamespaces {
        listable: true,
        namespaces: vec![name("default"), name("shop")],
    };
    let asked = |choice| nomad_namespace(Some(&listing), choice);
    assert_eq!(asked(Some("shop")).as_deref(), Some("shop"));
    assert_eq!(asked(None).as_deref(), Some("*"));
    // A namespace that is gone falls back to default.
    assert_eq!(asked(Some("billing")).as_deref(), Some("default"));
    // The token's default while the namespaces can't be listed.
    let unlistable = NomadNamespaces {
        listable: false,
        namespaces: vec![],
    };
    assert_eq!(nomad_namespace(Some(&unlistable), Some("shop")), None);
    assert_eq!(nomad_namespace(None, None), None);
}

#[test]
fn a_picker_lists_the_chosen_one_even_when_it_is_gone_or_still_loading() {
    let names = || vec!["default".to_owned(), "lab".to_owned()];
    assert_eq!(listed(names(), Some("lab")), ["default", "lab"]);
    assert_eq!(listed(names(), Some("moby")), ["moby", "default", "lab"]);
    assert_eq!(listed(vec![], Some("moby")), ["moby"]);
    assert_eq!(listed(names(), None), ["default", "lab"]);
}
