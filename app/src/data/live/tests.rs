//! What each of the backend's events does to the cache.

use std::{collections::HashMap, sync::Arc};

use homeport_core::error::ErrorCode;
use homeport_hosts::{
    registry::{EngineEvent, HostStatus},
    service::HostView,
};
use homeport_kubernetes::streams::forward::ForwardEvent;
use homeport_stacks::StackOwner;

use super::{append_event, stale_keys, stale_swarm_keys};
use crate::{
    app::events::Event,
    data::{
        keys::{cluster, host, host::IncusResource, stacks, swarm},
        tests::{down, drive, names, state, Source},
        Data, Key, Resource,
    },
    test_support::fixtures::{connected, host_view},
};

fn engine_event(kind: &str, action: &str, time_nano: i64) -> EngineEvent {
    EngineEvent {
        time: time_nano / 1_000_000_000,
        time_nano,
        kind: kind.to_owned(),
        action: action.to_owned(),
        actor_id: "c1".to_owned(),
        attributes: HashMap::new(),
    }
}

fn status(host_id: &str, status: HostStatus) -> Event {
    Event::HostStatus {
        host_id: host_id.to_owned(),
        status: Arc::new(status),
    }
}

fn engine(host_id: &str, kind: &str, time_nano: i64) -> Event {
    Event::Engine {
        host_id: host_id.to_owned(),
        event: Arc::new(engine_event(kind, "start", time_nano)),
    }
}

#[test]
fn a_command_run_in_a_container_makes_nothing_out_of_date() {
    // As the Engine names them: `exec_start: sh -c find …`.
    for action in [
        "exec_create: find / -mindepth 1",
        "exec_start: find / -mindepth 1",
        "exec_die",
        "exec_detach",
    ] {
        assert!(stale_keys("h1", "container", action).is_empty(), "{action}");
    }
    // A healthcheck's result is a status the lists show.
    assert!(
        stale_keys("h1", "container", "health_status: unhealthy").contains(&host::containers("h1"))
    );
}

#[test]
fn a_container_event_makes_what_containers_change_out_of_date() {
    let stale = stale_keys("h1", "container", "start");
    for key in [
        host::containers("h1"),
        host::stacks("h1"),
        host::system("h1"),
        host::volumes("h1"),
        host::networks("h1"),
        host::pods("h1"),
        host::quadlets("h1"),
        host::containerd_containers("h1", None),
    ] {
        assert!(stale.contains(&key), "{key:?}");
    }
    // Images and volume sizes are not read again for a container.
    assert!(!stale.contains(&host::images("h1")));
    assert!(!stale.contains(&host::volume_sizes("h1")));
}

#[test]
fn each_kind_of_event_names_its_lists() {
    assert_eq!(
        stale_keys("h1", "volume", "start"),
        [
            host::volumes("h1"),
            host::volume_sizes("h1"),
            host::system("h1")
        ]
    );
    assert_eq!(
        stale_keys("h1", "network", "start"),
        [
            host::networks("h1"),
            host::incus("h1", IncusResource::Networks, None)
        ]
    );
    assert!(stale_keys("h1", "image", "start").contains(&host::images("h1")));
    assert_eq!(stale_keys("h1", "pod", "start"), [host::pods("h1")]);
    assert_eq!(
        stale_keys("h1", "task", "start"),
        [host::containerd_containers("h1", None)]
    );
    assert_eq!(
        stale_keys("h1", "storage-volume", "start"),
        [host::incus("h1", IncusResource::Storage, None)]
    );
    assert_eq!(stale_keys("h1", "instance", "start").len(), 5);
    assert_eq!(
        stale_keys("h1", "project", "start"),
        [host::incus_projects("h1")]
    );
    assert!(stale_keys("h1", "daemon", "start").is_empty());
}

#[test]
fn a_managers_events_name_its_swarms_lists() {
    assert_eq!(
        stale_swarm_keys("s1", "service"),
        [
            swarm::summary("s1"),
            swarm::services("s1"),
            swarm::tasks("s1"),
            swarm::stacks("s1")
        ]
    );
    assert!(stale_swarm_keys("s1", "node").contains(&swarm::list()));
    assert_eq!(
        stale_swarm_keys("s1", "container"),
        [swarm::tasks("s1"), swarm::services("s1")]
    );
    assert_eq!(
        stale_swarm_keys("s1", "secret"),
        [swarm::secrets("s1"), swarm::stacks("s1")]
    );
    assert_eq!(
        stale_swarm_keys("s1", "config"),
        [swarm::configs("s1"), swarm::stacks("s1")]
    );
    assert!(stale_swarm_keys("s1", "image").is_empty());
}

#[test]
fn an_event_is_appended_once_and_the_page_stays_a_page() {
    let first = engine_event("container", "start", 1);
    let list = append_event(&[], &first).unwrap();
    assert_eq!(list, std::slice::from_ref(&first));
    // A reconnection asks the Engine again from the last kept event.
    assert_eq!(append_event(&list, &first), None);
    // Same instant, another action: another event.
    let died = engine_event("container", "die", 1);
    assert_eq!(append_event(&list, &died).unwrap().len(), 2);

    let full: Vec<EngineEvent> = (0..1000)
        .map(|n| engine_event("container", "start", n))
        .collect();
    let latest = engine_event("container", "start", 1000);
    let page = append_event(&full, &latest).unwrap();
    assert_eq!(page.len(), 1000);
    assert_eq!(page[0].time_nano, 1);
    assert_eq!(page[999], latest);
}

#[tokio::test(start_paused = true)]
async fn a_command_run_in_a_container_reads_nothing_again() {
    let (state, _app) = state();
    let mut data = Data::default();
    let exec = Event::Engine {
        host_id: "h1".to_owned(),
        event: Arc::new(engine_event("container", "exec_die", 1)),
    };
    // Nothing waits to be read again: no flush is planned.
    assert!(crate::test_support::tasks::is_none(
        data.on_event(&state, &exec)
    ));
    let start = engine("h1", "container", 2);
    assert!(!crate::test_support::tasks::is_none(
        data.on_event(&state, &start)
    ));
}

#[tokio::test(start_paused = true)]
async fn a_host_going_down_fails_its_resources_and_coming_back_refreshes_them() {
    let (state, _app) = state();
    let mut data = Data::default();
    let (containers, other) = (Source::default(), Source::default());
    let key = host::containers("h1");
    let task = iced::Task::batch([
        data.watch(&state, &containers.query(key.clone())),
        data.watch(&state, &other.query(host::containers("h2"))),
    ]);
    drive(&mut data, &state, task).await;

    let unreachable = HostStatus::Unreachable {
        error: down(),
        retry_in_secs: 5,
    };
    let task = data.on_event(&state, &status("h1", unreachable));
    drive(&mut data, &state, task).await;
    let Resource::Failed(error, kept) = names(&data, &key) else {
        panic!("h1's containers should be failed");
    };
    assert!(error.is(ErrorCode::HostNotConnected));
    assert_eq!(kept, Some(vec!["web-1".to_owned()]));
    // Nothing was read for it, and h2 is as it was.
    assert_eq!(containers.calls(), 1);
    assert_eq!(
        names(&data, &host::containers("h2")),
        Resource::Loaded(vec!["web-1".to_owned()])
    );

    let task = data.on_event(&state, &status("h1", connected()));
    assert_eq!(
        names(&data, &key),
        Resource::Refreshing(vec!["web-1".to_owned()])
    );
    drive(&mut data, &state, task).await;
    assert_eq!(
        names(&data, &key),
        Resource::Loaded(vec!["web-2".to_owned()])
    );
    assert_eq!(other.calls(), 1);
}

#[tokio::test(start_paused = true)]
async fn a_status_is_put_in_the_cached_list_of_hosts() {
    let (state, _app) = state();
    let mut data = Data::default();
    data.set(
        &host::list(),
        vec![
            host_view("prod", HostStatus::Connecting),
            host_view("staging", HostStatus::Disabled),
        ],
    );
    let task = data.on_event(&state, &status("prod-id", connected()));
    drive(&mut data, &state, task).await;
    let hosts = data.read::<Vec<HostView>>(&host::list());
    let hosts = hosts.data().unwrap();
    assert_eq!(hosts[0].status, connected());
    assert_eq!(hosts[1].status, HostStatus::Disabled);
}

#[tokio::test(start_paused = true)]
async fn an_unknown_host_or_a_change_of_hosts_reads_the_list_again() {
    let (state, _app) = state();
    let mut data = Data::default();
    let (hosts, swarms) = (Source::default(), Source::default());
    let task = iced::Task::batch([
        data.watch(&state, &hosts.query(host::list())),
        data.watch(&state, &swarms.query(swarm::list())),
    ]);
    drive(&mut data, &state, task).await;

    // A Host that was just added is only told through its status.
    let task = data.on_event(&state, &status("new", HostStatus::Connecting));
    drive(&mut data, &state, task).await;
    assert_eq!((hosts.calls(), swarms.calls()), (2, 2));

    let task = data.on_event(&state, &Event::HostsChanged);
    drive(&mut data, &state, task).await;
    assert_eq!((hosts.calls(), swarms.calls()), (3, 3));
}

#[tokio::test(start_paused = true)]
async fn a_burst_of_engine_events_reads_each_list_once_and_keeps_the_events() {
    let (state, _app) = state();
    let mut data = Data::default();
    let (containers, images) = (Source::default(), Source::default());
    let task = iced::Task::batch([
        data.watch(&state, &containers.query(host::containers("h1"))),
        data.watch(&state, &images.query(host::images("h1"))),
    ]);
    drive(&mut data, &state, task).await;
    data.set(&host::events("h1"), Vec::<EngineEvent>::new());

    let tasks: Vec<_> = (0..30)
        .map(|n| data.on_event(&state, &engine("h1", "container", n)))
        .collect();
    drive(&mut data, &state, iced::Task::batch(tasks)).await;
    assert_eq!((containers.calls(), images.calls()), (2, 1));
    let events = data.read::<Vec<EngineEvent>>(&host::events("h1"));
    assert_eq!(events.data().unwrap().len(), 30);
}

#[tokio::test(start_paused = true)]
async fn forwards_and_synced_stacks_refresh_what_they_change() {
    let (state, _app) = state();
    let mut data = Data::default();
    let sources: Vec<(Key, Source)> = [
        cluster::port_forwards().with("list"),
        stacks::synced("y1"),
        host::stacks("h1"),
        host::containers("h1"),
    ]
    .into_iter()
    .map(|key| (key, Source::default()))
    .collect();
    let task = iced::Task::batch(
        sources
            .iter()
            .map(|(key, source)| data.watch(&state, &source.query(key.clone()))),
    );
    drive(&mut data, &state, task).await;
    let calls = |sources: &[(Key, Source)]| -> Vec<usize> {
        sources.iter().map(|(_, source)| source.calls()).collect()
    };

    let ended = ForwardEvent {
        id: 1,
        cluster_id: "c1".to_owned(),
        error: down(),
        ended: true,
    };
    let task = data.on_event(&state, &Event::PortForward(Arc::new(ended)));
    drive(&mut data, &state, task).await;
    assert_eq!(calls(&sources), [2, 1, 1, 1]);

    let owner = StackOwner::Host("h1".to_owned());
    let found = Event::SyncedUpdateAvailable {
        owner: owner.clone(),
        stack_id: "y1".to_owned(),
        name: "web".to_owned(),
    };
    let task = data.on_event(&state, &found);
    drive(&mut data, &state, task).await;
    assert_eq!(calls(&sources), [2, 2, 2, 1]);

    // A deploy changes everything its Host has.
    let deployed = Event::SyncedAutoDeployed {
        owner,
        stack_id: "y1".to_owned(),
        name: "web".to_owned(),
        commit: "0123456789".to_owned(),
        error: None,
    };
    let task = data.on_event(&state, &deployed);
    drive(&mut data, &state, task).await;
    assert_eq!(calls(&sources), [2, 3, 3, 2]);
}
