use std::time::Duration;

use homeport_clusters::{registry::ClusterStatus, ClusterKind};
use homeport_core::error::ErrorCode;

use super::*;
use crate::app::{
    events::Event,
    testing::{app, TestApp},
};

/// The id of a Cluster that was never added.
pub(super) fn unknown() -> String {
    "no-such-cluster".into()
}

/// Whether `result` is the refusal an unknown Cluster or terminal gets.
pub(super) fn gone<T>(result: AppResult<T>) -> bool {
    failed(result, ErrorCode::NotFound)
}

pub(super) fn failed<T>(result: AppResult<T>, code: ErrorCode) -> bool {
    result.is_err_and(|e| e.is(code))
}

/// A kubeconfig with the contexts `kind-dev` and `other`, reaching `server`.
fn kubeconfig(server: &str) -> String {
    format!(
        "clusters:\n- name: dev\n  cluster: {{ server: '{server}' }}\ncontexts:\n- name: kind-dev\n  context: {{ cluster: dev, user: me }}\n- name: other\n  context: {{ cluster: dev, user: me }}\nusers:\n- name: me\n  user: {{ token: t }}\n"
    )
}

/// Adds a Kubernetes Cluster named `name` that nothing answers for.
pub(super) async fn kubernetes(app: &TestApp, name: &str) -> ClusterView {
    let pick = ContextChoice {
        context: "kind-dev".into(),
        name: name.into(),
    };
    let text = kubeconfig("https://127.0.0.1:1");
    let mut added = clusters_import(&app.state, vec![pick], Some(text))
        .await
        .unwrap();
    added.remove(0)
}

/// The next status published for the Cluster `id` that `wanted` accepts.
pub(super) async fn status_event(
    app: &mut TestApp,
    id: &str,
    wanted: impl Fn(&ClusterStatus) -> bool,
) -> ClusterStatus {
    let next = async {
        loop {
            match app.events.recv().await {
                Some(Event::ClusterStatus { cluster_id, status })
                    if cluster_id == id && wanted(&status) =>
                {
                    return (*status).clone();
                }
                Some(_) => {}
                None => panic!("the events ended"),
            }
        }
    };
    tokio::time::timeout(Duration::from_secs(20), next)
        .await
        .expect("no such status was published")
}

#[tokio::test]
async fn a_kubeconfig_context_becomes_a_cluster_that_is_listed_renamed_and_removed() {
    let mut app = app();
    let state = &app.state;
    assert!(clusters_list(state).unwrap().is_empty());
    let text = kubeconfig("https://127.0.0.1:1");
    let offered = clusters_kubeconfig_contexts(state, Some(text.clone())).unwrap();
    assert!(offered.paths.is_empty());
    assert_eq!(offered.contexts.len(), 2);
    assert!(offered.contexts.iter().all(|c| c.added.is_none()));

    let added = kubernetes(&app, " Dev ").await;
    let id = added.cluster.id.clone();
    assert_eq!(added.cluster.name, "Dev");
    assert_eq!(added.cluster.kind, ClusterKind::Kubernetes);
    assert_eq!(
        clusters_get(&app.state, id.clone()).unwrap().cluster,
        added.cluster
    );
    let offered = clusters_kubeconfig_contexts(&app.state, Some(text)).unwrap();
    let dev = offered.contexts.iter().find(|c| c.name == "kind-dev");
    assert_eq!(dev.unwrap().added.as_ref().unwrap().id, id);
    // Nothing listens on its port: the UI hears it can't be reached.
    status_event(&mut app, &id, |s| {
        matches!(s, ClusterStatus::Unreachable { .. })
    })
    .await;

    let state = &app.state;
    let renamed = clusters_update(state, id.clone(), "Staging".into()).unwrap();
    assert_eq!(renamed.cluster.name, "Staging");
    let listed = clusters_list(state).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].cluster.name, "Staging");

    clusters_remove(state, id.clone()).unwrap();
    assert!(clusters_list(state).unwrap().is_empty());
    assert!(gone(clusters_get(state, id)));
}

#[tokio::test]
async fn a_cluster_is_disabled_enabled_reconnected_and_refreshed() {
    let mut app = app();
    let id = kubernetes(&app, "dev").await.cluster.id;

    let off = clusters_set_enabled(&app.state, id.clone(), false)
        .await
        .unwrap();
    assert!(!off.cluster.enabled);
    status_event(&mut app, &id, |s| *s == ClusterStatus::Disabled).await;
    // What runs on it can't be read while it isn't connected.
    assert!(failed(
        on(&app.state, &id).map(drop),
        ErrorCode::ClusterNotConnected
    ));

    let on = clusters_set_enabled(&app.state, id.clone(), true)
        .await
        .unwrap();
    assert!(on.cluster.enabled);
    status_event(&mut app, &id, |s| *s == ClusterStatus::Connecting).await;

    let again = clusters_reconnect(&app.state, id.clone()).await.unwrap();
    assert_eq!(again.cluster.id, id);

    // Without its kubeconfig a pasted one can't be read again.
    assert!(clusters_refresh(&app.state, id.clone(), None)
        .await
        .is_err());
    let moved = kubeconfig("https://127.0.0.1:2");
    let next = clusters_refresh(&app.state, id, Some(moved)).await.unwrap();
    assert_eq!(next.cluster.name, "dev");
    assert_eq!(next.cluster.address(), "https://127.0.0.1:2");
}

#[tokio::test]
async fn importing_refuses_a_wrong_choice_and_adds_nothing() {
    let app = app();
    let state = &app.state;
    let text = kubeconfig("https://127.0.0.1:1");
    assert!(clusters_import(state, vec![], Some(text.clone()))
        .await
        .is_err());
    let missing = ContextChoice {
        context: "gone".into(),
        name: "x".into(),
    };
    assert!(gone(
        clusters_import(state, vec![missing], Some(text)).await
    ));
    assert!(clusters_kubeconfig_contexts(state, Some("not: [a kubeconfig".into())).is_err());
    assert!(clusters_list(state).unwrap().is_empty());
}

#[tokio::test]
async fn an_unknown_cluster_is_refused_as_gone() {
    let app = app();
    let state = &app.state;
    assert!(gone(clusters_get(state, unknown())));
    assert!(gone(clusters_update(state, unknown(), "x".into())));
    assert!(gone(clusters_set_enabled(state, unknown(), false).await));
    assert!(gone(clusters_remove(state, unknown())));
    assert!(gone(clusters_reconnect(state, unknown()).await));
    assert!(gone(clusters_refresh(state, unknown(), None).await));
    assert!(gone(on(state, &unknown()).map(drop)));
}

#[tokio::test]
async fn detecting_local_clusters_needs_no_host() {
    let app = app();
    // No Host is connected, so no kind or k3d container can be missing.
    let local = clusters_detect_local(&app.state).await.unwrap();
    assert!(local.missing.is_empty());
}
