//! Routes and their paths, and the history of a session.

use super::{ApiKind, History, Namespaced, Request, Route};

/// Every route, with `v` for each of its ids and `tab` for its tab.
fn every(v: &str, tab: Option<&str>) -> Vec<Route> {
    let s = || v.to_owned();
    let tab = || tab.map(str::to_owned);
    let object = || Namespaced::new(v, v);
    let api = || ApiKind {
        group: s(),
        version: s(),
        plural: s(),
    };
    vec![
        Route::Overview,
        Route::Board,
        Route::Containers,
        Route::Container {
            id: s(),
            tab: tab(),
        },
        Route::Images,
        Route::Volumes,
        Route::Networks,
        Route::Stacks,
        Route::NewStack,
        Route::Stack { name: s() },
        Route::PodmanPods,
        Route::PodmanPod { id: s() },
        Route::Quadlets,
        Route::Quadlet { name: s() },
        Route::ContainerdContainers,
        Route::ContainerdContainer {
            id: s(),
            tab: tab(),
        },
        Route::ContainerdImages,
        Route::ContainerdSnapshots,
        Route::ContainerdNamespaces,
        Route::IncusInstances,
        Route::IncusInstance {
            name: s(),
            tab: tab(),
        },
        Route::IncusImages,
        Route::IncusProfiles,
        Route::IncusProfile { name: s() },
        Route::IncusStorage { pool: None },
        Route::IncusStorage { pool: Some(s()) },
        Route::IncusNetworks,
        Route::IncusNetwork { name: s() },
        Route::Swarm {
            swarm_id: s(),
            tab: tab(),
        },
        Route::SwarmService {
            swarm_id: s(),
            service_id: s(),
            tab: tab(),
        },
        Route::SwarmStack {
            swarm_id: s(),
            name: s(),
        },
        Route::Workloads { kind: None },
        Route::Workloads { kind: Some(s()) },
        Route::Workload {
            kind: s(),
            object: object(),
            tab: tab(),
        },
        Route::Pods,
        Route::Pod {
            object: object(),
            tab: tab(),
        },
        Route::Services,
        Route::Service {
            object: object(),
            tab: tab(),
        },
        Route::Ingresses,
        Route::Ingress {
            object: object(),
            tab: tab(),
        },
        Route::ConfigMaps,
        Route::ConfigMap {
            object: object(),
            tab: tab(),
        },
        Route::Secrets,
        Route::Secret {
            object: object(),
            tab: tab(),
        },
        Route::Nodes,
        Route::Node {
            name: s(),
            tab: tab(),
        },
        Route::ClusterEvents,
        Route::ApiResources { api: None },
        Route::ApiResources { api: Some(api()) },
        Route::ApiObject {
            api: api(),
            object: object(),
            tab: tab(),
        },
        Route::Rbac { tab: tab() },
        Route::ClusterRole { name: s() },
        Route::Role { object: object() },
        Route::RbacUser { name: s() },
        Route::RbacGroup { name: s() },
        Route::ServiceAccount { object: object() },
        Route::Apply,
        Route::HelmReleases,
        Route::HelmRepositories,
        Route::HelmRelease {
            object: object(),
            tab: tab(),
        },
        Route::PortForwards,
        Route::NomadJobs,
        Route::NomadJob {
            namespace: s(),
            job_id: s(),
        },
        Route::NomadAllocation {
            alloc_id: s(),
            tab: tab(),
        },
        Route::NomadNodes,
        Route::NomadNode { node_id: s() },
        Route::Events,
        Route::Settings,
        Route::Hosts,
        Route::AddHost,
        Route::Host { host_id: s() },
        Route::Clusters,
        Route::AddCluster,
        Route::Cluster { cluster_id: s() },
    ]
}

#[test]
fn every_route_is_read_back_from_its_path_whatever_its_ids_hold() {
    // An empty value (the core API group, no Namespace), what a path gives
    // a meaning to, and names as Kubernetes and users write them.
    let awkward = [
        "abc",
        "",
        "-",
        "a/b",
        "system:serviceaccount:shop?x#y",
        "100% done",
        "%2F",
        "core.example",
        " spaced name ",
        "Ünïcødé-名前",
    ];
    for value in awkward {
        for tab in [None, Some("logs"), Some("a/b?c#d")] {
            for route in every(value, tab) {
                let path = route.path();
                assert!(!path.contains('?') && !path.contains('#'), "{path}");
                assert!(!path.contains("//"), "{path}");
                assert_eq!(Route::parse(&path).as_ref(), Some(&route), "{path}");
            }
        }
    }
}

#[test]
fn an_empty_group_is_core_and_no_namespace_is_a_dash_as_react_wrote_them() {
    let pods = ApiKind {
        group: String::new(),
        version: "v1".to_owned(),
        plural: "nodes".to_owned(),
    };
    let node = Route::ApiObject {
        api: pods,
        object: Namespaced::new("", "worker-1"),
        tab: None,
    };
    assert_eq!(node.path(), "/api-resources/core/v1/nodes/-/worker-1");
    assert_eq!(Route::Overview.path(), "/");
    let logs = Route::Container {
        id: "abc".to_owned(),
        tab: Some("logs".to_owned()),
    };
    assert_eq!(logs.path(), "/containers/abc/logs");
}

#[test]
fn a_fixed_segment_wins_over_a_parameter() {
    assert_eq!(Route::parse("/stacks/new"), Some(Route::NewStack));
    assert_eq!(Route::parse("/hosts/new"), Some(Route::AddHost));
    assert_eq!(
        Route::parse("/helm/repositories"),
        Some(Route::HelmRepositories)
    );
    assert_eq!(
        Route::parse("/rbac/roles"),
        Some(Route::Rbac {
            tab: Some("roles".to_owned())
        })
    );
    assert_eq!(
        Route::parse("/swarms/sw1/services"),
        Some(Route::Swarm {
            swarm_id: "sw1".to_owned(),
            tab: Some("services".to_owned())
        })
    );
}

#[test]
fn a_path_of_no_feature_is_no_route_and_a_query_is_not_part_of_it() {
    assert_eq!(Route::parse("/nowhere"), None);
    assert_eq!(Route::parse("/containers/a/b/c"), None);
    assert_eq!(Route::parse("/images?q=nginx"), Some(Route::Images));
}

#[test]
fn a_name_with_a_slash_keeps_its_segment() {
    let stack = Route::Stack {
        name: "team/blog 100%".to_owned(),
    };
    assert_eq!(stack.path(), "/stacks/team%2Fblog 100%25");
    let user = Route::RbacUser {
        name: "who?#1".to_owned(),
    };
    assert_eq!(user.path(), "/rbac/users/who%3F%231");
    assert_eq!(Route::parse(&stack.path()), Some(stack));
}

#[test]
fn a_navigation_drops_what_was_ahead_and_back_and_forward_move() {
    let mut history = History::default();
    assert_eq!(history.current().route, Route::Overview);
    assert!(!history.can_go_back() && history.back().is_none());
    history.push(Route::Containers.into());
    history.push(Route::Images.into());
    assert_eq!(
        history.back().map(|l| l.route.clone()),
        Some(Route::Containers)
    );
    assert!(history.can_go_forward());
    assert_eq!(
        history.forward().map(|l| l.route.clone()),
        Some(Route::Images)
    );
    assert!(history.forward().is_none());
    history.back();
    history.push(Route::Volumes.into());
    assert!(!history.can_go_forward(), "Images is forgotten");
    assert_eq!(history.count(), 3);
}

#[test]
fn a_redirect_takes_the_place_of_the_page_that_sent_away() {
    let mut history = History::default();
    history.push(Route::Settings.into());
    history.push(Route::Containers.into());
    history.replace(Route::Overview.into());
    assert_eq!(history.current().route, Route::Overview);
    assert_eq!(
        history.back().map(|l| l.route.clone()),
        Some(Route::Settings)
    );
}

#[test]
fn each_navigation_has_its_own_key_and_carries_its_action_and_query() {
    let mut history = History::default();
    let first = history
        .push(Request::to(Route::Containers).starting("run"))
        .clone();
    let second = history
        .push(Request::to(Route::Containers).starting("run"))
        .clone();
    assert_eq!(first.action, Some("run"));
    assert_ne!(first.key, second.key);
    let searched = history.push(Request::to(Route::Images).searching("nginx"));
    assert_eq!(searched.query.as_deref(), Some("nginx"));
}
