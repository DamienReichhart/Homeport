//! The keys keep the parts the React app gave them, and its scopes.

use homeport_kubernetes::resources::{ApiType, ResourceKind};
use homeport_stacks::StackOwner;

use super::{app, cluster, host, host::IncusResource, nomad, stacks, swarm};
use crate::data::Key;

fn parts(key: &Key) -> Vec<&str> {
    key.parts().iter().map(String::as_str).collect()
}

#[test]
fn everything_of_a_host_is_in_its_scope() {
    let scope = host::scope("h1");
    let keys = [
        host::containers("h1"),
        host::images("h1"),
        host::freshness("h1"),
        host::volumes("h1"),
        host::volume_sizes("h1"),
        host::networks("h1"),
        host::stacks("h1"),
        host::system("h1"),
        host::system_info("h1"),
        host::disk_usage("h1"),
        host::events("h1"),
        host::pods("h1"),
        host::quadlets("h1"),
        host::containerd_namespaces("h1"),
        host::containerd_containers("h1", Some("default")),
        host::containerd_images("h1", None),
        host::containerd_snapshots("h1", Some("k8s.io")),
        host::incus_projects("h1"),
        host::incus("h1", IncusResource::Instances, Some("default")),
        host::volume_helper_image("h1"),
        host::volume_browse("h1", "s1"),
        stacks::history(&StackOwner::Host("h1".to_owned()), "web"),
    ];
    for key in &keys {
        assert!(key.is_in(&scope), "{key:?}");
        assert!(!key.is_in(&host::scope("h2")), "{key:?}");
    }
}

#[test]
fn host_keys_keep_their_parts() {
    assert_eq!(parts(&host::list()), ["hosts"]);
    assert_eq!(parts(&host::local_engines()), ["local-engines"]);
    assert_eq!(parts(&host::containers("h1")), ["host", "h1", "containers"]);
    assert_eq!(
        parts(&host::freshness("h1")),
        ["host", "h1", "images", "freshness"]
    );
    assert_eq!(
        parts(&host::disk_usage("h1")),
        ["host", "h1", "system", "df"]
    );
    assert_eq!(
        parts(&host::incus("h1", IncusResource::Storage, Some("p"))),
        ["host", "h1", "incus", "storage", "p"]
    );
    assert_eq!(parts(&host::ssh_setup("h1")), ["hosts", "h1", "ssh-setup"]);
    assert_eq!(
        parts(&host::import_preview("sshConfig")),
        ["hosts", "import", "sshConfig"]
    );
}

#[test]
fn every_namespace_or_project_holds_each_one() {
    let all = host::containerd_containers("h1", None);
    assert!(host::containerd_containers("h1", Some("default")).is_in(&all));
    let all = host::incus("h1", IncusResource::Images, None);
    assert!(host::incus("h1", IncusResource::Images, Some("default")).is_in(&all));
    assert!(!host::incus("h1", IncusResource::Profiles, Some("default")).is_in(&all));
    // Freshness is refreshed with the images.
    assert!(host::freshness("h1").is_in(&host::images("h1")));
    // Sizes are not refreshed with the volumes.
    assert!(!host::volume_sizes("h1").is_in(&host::volumes("h1")));
}

#[test]
fn everything_of_a_cluster_is_in_its_scope() {
    let scope = cluster::scope("c1");
    let api = ApiType {
        group: "apps".to_owned(),
        version: "v1".to_owned(),
        kind: "Deployment".to_owned(),
        plural: "deployments".to_owned(),
        namespaced: true,
    };
    let keys = [
        cluster::kind("c1", ResourceKind::Pod),
        cluster::objects("c1", ResourceKind::Pod, None),
        cluster::object("c1", ResourceKind::Pod, Some("default"), "web"),
        cluster::object_events("c1", ResourceKind::Pod, Some("default"), "web"),
        cluster::api("c1", &api),
        cluster::api_types("c1", None),
        cluster::yaml("c1", ResourceKind::Pod, Some("default"), "web"),
        cluster::summary("c1", None),
        cluster::rbac("c1"),
        cluster::helm("c1"),
        cluster::helm_release("c1", "default", "web"),
    ];
    for key in &keys {
        assert!(key.is_in(&scope), "{key:?}");
    }
    assert_eq!(
        parts(&cluster::api("c1", &api)),
        ["cluster", "c1", "api", "apps", "v1", "deployments"]
    );
    assert_eq!(parts(&cluster::list()), ["clusters"]);
    assert_eq!(parts(&cluster::kubeconfig()), ["clusters", "kubeconfig"]);
    assert_eq!(parts(&cluster::local_clusters()), ["local-clusters"]);
    assert_eq!(parts(&cluster::helm_repositories()), ["helm-repositories"]);
    assert_eq!(parts(&cluster::port_forwards()), ["port-forwards"]);
}

#[test]
fn a_kind_is_named_as_it_is_serialised_and_holds_its_objects() {
    let kind = cluster::kind("c1", ResourceKind::StatefulSet);
    assert_eq!(parts(&kind), ["cluster", "c1", "statefulSet"]);
    assert!(cluster::objects("c1", ResourceKind::StatefulSet, Some("default")).is_in(&kind));
    assert!(cluster::yaml("c1", ResourceKind::StatefulSet, None, "db").is_in(&kind));
    // An object's Events are refreshed by the Events' watch, not its kind's.
    let events = cluster::object_events("c1", ResourceKind::Pod, Some("default"), "web");
    assert!(events.is_in(&cluster::kind("c1", ResourceKind::Event)));
    assert_eq!(
        parts(&events),
        ["cluster", "c1", "event", "of", "pod", "default", "web"]
    );
}

#[test]
fn nomad_and_swarm_keys_are_in_their_scopes() {
    let scope = nomad::scope("c1");
    for key in [
        nomad::jobs("c1", None),
        nomad::job("c1", "default", "web"),
        nomad::allocation("c1", "a1"),
        nomad::nodes("c1"),
        nomad::node("c1", "n1"),
        nomad::namespaces("c1"),
    ] {
        assert!(key.is_in(&scope), "{key:?}");
    }
    assert_eq!(parts(&nomad::jobs("c1", None)), ["nomad", "c1", "jobs", ""]);

    let scope = swarm::scope("s1");
    for key in [
        swarm::summary("s1"),
        swarm::nodes("s1"),
        swarm::services("s1"),
        swarm::tasks("s1"),
        swarm::secrets("s1"),
        swarm::configs("s1"),
        swarm::stacks("s1"),
        swarm::networks("s1"),
        swarm::join_info("s1"),
    ] {
        assert!(key.is_in(&scope), "{key:?}");
    }
    assert_eq!(parts(&swarm::list()), ["swarms"]);
}

#[test]
fn a_stack_is_filed_under_its_owner() {
    let host = StackOwner::Host("h1".to_owned());
    let swarm_owner = StackOwner::Swarm("s1".to_owned());
    assert_eq!(stacks::of(&host), host::stacks("h1"));
    assert_eq!(stacks::of(&swarm_owner), swarm::stacks("s1"));
    assert_eq!(stacks::scope(&host), host::scope("h1"));
    assert_eq!(stacks::scope(&swarm_owner), swarm::scope("s1"));
    assert_eq!(
        parts(&stacks::history(&swarm_owner, "web")),
        ["swarm", "s1", "stack-history", "web"]
    );
    assert_eq!(parts(&stacks::managed("m1")), ["managed-stack", "m1"]);
    assert_eq!(parts(&stacks::linked("l1")), ["linked-stack", "l1"]);
    assert_eq!(parts(&stacks::synced("y1")), ["synced-stack", "y1"]);
    assert_eq!(parts(&stacks::deploy("d1")), ["stack-deploy", "d1"]);
    assert!(stacks::registry_list().is_in(&stacks::registries()));
    assert!(stacks::hub_tags("nginx", "1.").is_in(&stacks::registries()));
    assert!(stacks::repository_refs("r1").is_in(&stacks::repositories()));
}

#[test]
fn the_apps_own_keys_keep_their_names() {
    assert_eq!(parts(&app::settings()), ["settings"]);
    assert_eq!(parts(&app::info()), ["app-info"]);
    assert_eq!(parts(&app::capabilities()), ["capabilities"]);
    assert_eq!(parts(&app::updates()), ["updates"]);
    assert_eq!(parts(&app::start_at_login()), ["start-at-login"]);
    assert_eq!(parts(&app::licenses()), ["licenses"]);
    assert_eq!(parts(&app::diagnostics(true)), ["diagnostics", "true"]);
}
