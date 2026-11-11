//! Routes and their paths, and the history of a session.

use super::{ApiKind, History, Namespaced, Request, Route};

fn samples() -> Vec<Route> {
    let s = str::to_owned;
    let tab = || Some(s("logs"));
    let object = || Namespaced::new("shop", "web");
    let api = || ApiKind {
        group: s("apps"),
        version: s("v1"),
        plural: s("deployments"),
    };
    vec![
        Route::Overview,
        Route::Container {
            id: s("abc"),
            tab: None,
        },
        Route::Container {
            id: s("abc"),
            tab: tab(),
        },
        Route::NewStack,
        Route::Stack { name: s("blog") },
        Route::ContainerdContainer {
            id: s("abc"),
            tab: tab(),
        },
        Route::IncusStorage { pool: None },
        Route::IncusStorage {
            pool: Some(s("default")),
        },
        Route::Swarm {
            swarm_id: s("sw1"),
            tab: None,
        },
        Route::Swarm {
            swarm_id: s("sw1"),
            tab: Some(s("services")),
        },
        Route::SwarmService {
            swarm_id: s("sw1"),
            service_id: s("svc"),
            tab: tab(),
        },
        Route::SwarmStack {
            swarm_id: s("sw1"),
            name: s("blog"),
        },
        Route::Workloads {
            kind: Some(s("deployments")),
        },
        Route::Workload {
            kind: s("deployments"),
            object: object(),
            tab: tab(),
        },
        Route::Pod {
            object: object(),
            tab: None,
        },
        Route::Secret {
            object: object(),
            tab: tab(),
        },
        Route::ApiResources { api: Some(api()) },
        Route::ApiObject {
            api: api(),
            object: object(),
            tab: None,
        },
        Route::Rbac {
            tab: Some(s("bindings")),
        },
        Route::ClusterRole { name: s("admin") },
        Route::Role { object: object() },
        Route::ServiceAccount { object: object() },
        Route::HelmRepositories,
        Route::HelmRelease {
            object: object(),
            tab: tab(),
        },
        Route::NomadJob {
            namespace: s("default"),
            job_id: s("web"),
        },
        Route::NomadAllocation {
            alloc_id: s("a1"),
            tab: None,
        },
        Route::AddHost,
        Route::Host { host_id: s("h1") },
        Route::AddCluster,
        Route::Cluster {
            cluster_id: s("c1"),
        },
    ]
}

#[test]
fn a_route_is_read_back_from_its_path() {
    for route in samples() {
        assert_eq!(
            Route::parse(&route.path()).as_ref(),
            Some(&route),
            "{}",
            route.path()
        );
    }
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
