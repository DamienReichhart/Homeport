//! The fake Engine against the real use cases: what Run, Edit and Commit
//! ask of it is what the crate sends, and its answers are read as an
//! Engine's.

use std::sync::Arc;

use homeport_engine::containers::{
    commit::CommitSpec,
    recreate::Loss,
    run::{create_body, PortSpec, RunSpec},
};

use super::Engine;
use crate::{
    api::engine::containers::{
        containers_commit, containers_edit_losses, containers_recreate, containers_run,
    },
    app::{testing, AppState},
};

fn state() -> (Arc<AppState>, impl Sized) {
    let testing::TestApp { state, events, dir } = testing::app();
    (Arc::new(state), (events, dir))
}

fn spec() -> RunSpec {
    RunSpec {
        image: "nginx:1.27".to_owned(),
        name: Some("web".to_owned()),
        env: vec!["MODE=prod".to_owned()],
        ports: vec![PortSpec {
            host_ip: None,
            host_port: Some(8080),
            container_port: 80,
            protocol: "tcp".to_owned(),
        }],
        ..RunSpec::default()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_run_creates_what_the_crate_builds_then_starts_it() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host = engine.host(&state).await;
    let id = containers_run(&state, host, spec(), None).await.unwrap();
    assert_eq!(id, "new-1");
    let created = engine.created();
    assert_eq!(created.len(), 1);
    assert_eq!(created[0].name.as_deref(), Some("web"));
    let built = serde_json::to_value(create_body(&spec(), None).unwrap()).unwrap();
    assert_eq!(created[0].body, built);
    assert_eq!(engine.asked("/containers/new-1/start").len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_refused_create_is_the_use_case_s_failure() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host = engine.host(&state).await;
    engine.refuse_create("invalid reference format");
    let refused = containers_run(&state, host, spec(), None)
        .await
        .unwrap_err();
    assert!(
        format!("{refused:?}").contains("invalid reference format"),
        "{refused:?}"
    );
    assert!(engine.created().is_empty());
}

/// A running container of the `shop` Stack, with an anonymous volume.
pub(crate) fn of_a_stack() -> serde_json::Value {
    let volume = "a".repeat(64);
    serde_json::json!({
        "Id": "c1",
        "Name": "/web",
        "Image": "sha256:1111",
        "State": { "Status": "running", "Running": true },
        "Config": { "Image": "nginx:1.27", "Labels": { "com.docker.compose.project": "shop" } },
        "HostConfig": {},
        "Mounts": [{ "Type": "volume", "Name": volume, "Destination": "/var/cache" }],
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn what_a_recreate_loses_is_read_from_the_container() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host = engine.host(&state).await;
    engine.inspects_as(of_a_stack());
    engine.changed_paths(3);
    let losses = containers_edit_losses(&state, host, "c1".to_owned())
        .await
        .unwrap();
    assert_eq!(
        losses,
        [
            Loss::AnonymousVolume {
                path: "/var/cache".to_owned()
            },
            Loss::FilesystemChanges { count: 3 },
            Loss::Downtime,
            Loss::StackRedeploy {
                stack: "shop".to_owned()
            },
        ]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_recreate_swaps_the_containers_and_removes_the_old_one() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host = engine.host(&state).await;
    let spec = RunSpec {
        based_on: Some("c1".to_owned()),
        ..spec()
    };
    let id = containers_recreate(&state, host, "c1".to_owned(), spec)
        .await
        .unwrap();
    assert_eq!(id, "new-1");
    let created = engine.created();
    assert!(created[0]
        .name
        .as_deref()
        .is_some_and(|name| name.starts_with("web-new-")));
    for step in [
        "POST /containers/c1/stop",
        "POST /containers/c1/rename",
        "POST /containers/new-1/rename",
        "POST /containers/new-1/start",
        "DELETE /containers/c1",
    ] {
        assert_eq!(engine.asked(step).len(), 1, "{step}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_commit_names_the_image_and_pauses_the_container() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host = engine.host(&state).await;
    let spec = CommitSpec {
        reference: "web:backup".to_owned(),
        pause: true,
        changes: vec!["ENV MODE=prod".to_owned()],
        ..CommitSpec::default()
    };
    let id = containers_commit(&state, host, "c1".to_owned(), spec)
        .await
        .unwrap();
    assert_eq!(id, "sha256:2222");
    let commits = engine.commits();
    assert_eq!(commits.len(), 1);
    for part in ["container=c1", "repo=web", "tag=backup", "pause=true"] {
        assert!(commits[0].contains(part), "{part} in {}", commits[0]);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_recreate_that_fails_half_way_puts_the_original_back_as_it_was() {
    let (state, _app) = state();
    let engine = Engine::start();
    let host = engine.host(&state).await;
    // The new container won't start, once the two were swapped.
    engine.refuse("POST /containers/new-1/start", "port is already allocated");
    let spec = RunSpec {
        based_on: Some("c1".to_owned()),
        ..spec()
    };
    let refused = containers_recreate(&state, host, "c1".to_owned(), spec)
        .await
        .unwrap_err();
    assert!(
        refused
            .detail
            .as_deref()
            .is_some_and(|d| d.contains("port is already allocated")),
        "{refused:?}"
    );
    // The new one went; the original has its name back and runs again.
    assert_eq!(engine.asked("DELETE /containers/new-1").len(), 1);
    let renamed = engine.asked("POST /containers/c1/rename");
    assert_eq!(renamed.len(), 2, "aside, then back: {renamed:?}");
    assert!(renamed[1].contains("name=web"), "{renamed:?}");
    assert_eq!(engine.asked("POST /containers/c1/start").len(), 1);
    assert!(
        engine.asked("DELETE /containers/c1").is_empty(),
        "the original stays"
    );
}
