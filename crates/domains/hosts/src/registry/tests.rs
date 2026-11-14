use std::time::Duration;

use bollard::Docker;

use homeport_core::error::ErrorCode;
use homeport_engine::swarm::SwarmRole;
use homeport_storage::{Db, SecretBox};

use super::swarms::{group_managers, workers_of};
use super::*;
use crate::{store::HostDraft, HostConnection};

#[derive(Default)]
struct Recorder(Mutex<Vec<HostStatus>>);

impl Notifier for Recorder {
    fn host_status(&self, _: &str, status: &HostStatus) {
        self.0.lock().unwrap().push(status.clone());
    }
    fn engine_event(&self, _: &str, _: &EngineEvent) {}
}

fn setup(socket_path: &str) -> (Arc<HostRegistry>, Arc<Recorder>, HostId) {
    setup_engine(socket_path, crate::EngineKind::Docker)
}

fn setup_engine(
    socket_path: &str,
    engine: crate::EngineKind,
) -> (Arc<HostRegistry>, Arc<Recorder>, HostId) {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let store = Arc::new(HostStore::new(
        db.clone(),
        Arc::new(SecretBox::from_key(&[3; 32])),
    ));
    let id = store
        .create(HostDraft {
            name: "local".into(),
            connection: HostConnection::Local {
                socket_path: socket_path.into(),
            },
            engine,
            secrets: None,
        })
        .unwrap()
        .id;
    let recorder = Arc::new(Recorder::default());
    let events = EventStore::new(db);
    (
        HostRegistry::new(store, events, recorder.clone()),
        recorder,
        id,
    )
}

async fn wait_for(reg: &HostRegistry, id: &str, pred: impl Fn(&HostStatus) -> bool) {
    for _ in 0..200 {
        if pred(&reg.status(id)) {
            return;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("status never matched, last: {:?}", reg.status(id));
}

fn member(swarm: Option<&str>, node: &str, role: SwarmRole, knows: &[&str]) -> SwarmMembership {
    SwarmMembership {
        swarm_id: swarm.map(str::to_owned),
        node_id: node.into(),
        role,
        state: homeport_engine::swarm::SwarmNodeState::Active,
        node_addr: None,
        managers: knows
            .iter()
            .map(|n| homeport_engine::swarm::SwarmPeer {
                node_id: (*n).into(),
                addr: format!("{n}:2377"),
            })
            .collect(),
        error: None,
    }
}

#[test]
fn two_managers_of_one_swarm_make_one_swarm() {
    use SwarmRole::{Manager, Worker};
    let members = vec![
        (
            "a".to_owned(),
            member(Some("s1"), "n1", Manager, &[]),
            false,
        ),
        ("b".to_owned(), member(Some("s1"), "n2", Manager, &[]), true),
        (
            "c".to_owned(),
            member(Some("s2"), "n9", Manager, &[]),
            false,
        ),
        ("w".to_owned(), member(None, "n3", Worker, &["n1"]), true),
        ("x".to_owned(), member(None, "n4", Worker, &["n7"]), true),
    ];
    let swarms = group_managers(&members);
    assert_eq!(swarms.len(), 2);
    assert_eq!(swarms[0].id, "s1");
    assert_eq!(swarms[0].managers, ["a", "b"]);
    assert!(swarms[0].reachable);
    assert!(!swarms[1].reachable);
    // By the managers it knows, then by the Swarm's node list.
    assert_eq!(workers_of(&swarms[0], &members, &[]), ["w"]);
    assert_eq!(
        workers_of(&swarms[0], &members, &["n4".to_owned()]),
        ["w", "x"]
    );
    assert!(workers_of(&swarms[1], &members, &[]).is_empty());
}

#[test]
fn helper_events_are_left_out() {
    let event = |kind: &str, action: &str, actor: &str, attributes: &[(&str, &str)]| EngineEvent {
        time: 0,
        time_nano: 0,
        kind: kind.into(),
        action: action.into(),
        actor_id: actor.into(),
        attributes: attributes
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    };
    let mut helpers = HashSet::new();
    let label = [(HELPER_LABEL, "true")];
    assert!(is_helpers(
        &mut helpers,
        &event("container", "create", "h1", &label)
    ));
    assert!(is_helpers(
        &mut helpers,
        &event("volume", "mount", "db", &[("container", "h1")])
    ));
    assert!(!is_helpers(
        &mut helpers,
        &event("volume", "mount", "db", &[("container", "web")])
    ));
    assert!(!is_helpers(
        &mut helpers,
        &event("container", "start", "web", &[])
    ));
    assert!(is_helpers(
        &mut helpers,
        &event("container", "destroy", "h1", &label)
    ));
    assert!(helpers.is_empty());
}

#[tokio::test]
async fn unreachable_hosts_report_a_retry() {
    let (reg, recorder, id) = setup("/nonexistent/docker.sock");
    reg.restart(&id);
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Unreachable { .. })).await;
    assert_eq!(recorder.0.lock().unwrap()[0], HostStatus::Connecting);
    assert!(matches!(
        reg.docker(&id),
        Err(e) if e.is(ErrorCode::HostNotConnected)
    ));
}

#[tokio::test]
async fn disabled_hosts_stay_disconnected() {
    let (reg, _, id) = setup("/var/run/docker.sock");
    reg.store.set_enabled(&id, false).unwrap();
    reg.restart(&id);
    wait_for(&reg, &id, |s| *s == HostStatus::Disabled).await;
}

/// Creates a network named `name` and returns the function removing it.
async fn network(docker: &Docker, name: &str) {
    docker
        .create_network(bollard::models::NetworkCreateRequest {
            name: name.to_owned(),
            ..Default::default()
        })
        .await
        .unwrap();
}

fn created(events: &[EngineEvent], name: &str) -> usize {
    events
        .iter()
        .filter(|e| e.kind == "network" && e.action == "create")
        .filter(|e| e.attributes.get("name").map(String::as_str) == Some(name))
        .count()
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn events_outlive_a_restart_and_the_gap_is_filled() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("homeport.db");
    let docker = Docker::connect_with_local_defaults().unwrap();
    let suffix = &uuid::Uuid::new_v4().simple().to_string()[..8];
    let (seen, missed) = (
        format!("homeport-test-seen-{suffix}"),
        format!("homeport-test-missed-{suffix}"),
    );
    let start = |db: Arc<Db>| {
        let store = Arc::new(HostStore::new(
            db.clone(),
            Arc::new(SecretBox::from_key(&[3; 32])),
        ));
        let id = match store.list().unwrap().into_iter().next() {
            Some(host) => host.id,
            None => {
                store
                    .create(HostDraft {
                        name: "local".into(),
                        connection: HostConnection::Local {
                            socket_path: "/var/run/docker.sock".into(),
                        },
                        engine: crate::EngineKind::Docker,
                        secrets: None,
                    })
                    .unwrap()
                    .id
            }
        };
        let events = EventStore::new(db);
        let reg = HostRegistry::new(store, events.clone(), Arc::new(Recorder::default()));
        reg.restart(&id);
        (reg, events, id)
    };
    let until = |events: Arc<EventStore>, id: String, name: String| async move {
        for _ in 0..200 {
            events.flush().await;
            let page = events.page(&id, crate::events::PAGE, None).unwrap();
            if created(&page, &name) > 0 {
                return page;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        panic!("{name} never showed up");
    };

    // Homeport runs and sees a network created.
    let (reg, events, id) = start(Arc::new(Db::open(&path).unwrap()));
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Connected { .. })).await;
    network(&docker, &seen).await;
    until(events.clone(), id.clone(), seen.clone()).await;
    reg.remove(&id);
    drop((reg, events));

    // Then quits; something happens meanwhile.
    network(&docker, &missed).await;

    // On the next start, both are listed once, in order.
    let (reg, events, id) = start(Arc::new(Db::open(&path).unwrap()));
    let page = until(events.clone(), id.clone(), missed.clone()).await;
    reg.remove(&id);
    for name in [&seen, &missed] {
        docker.remove_network(name).await.ok();
    }
    assert_eq!(created(&page, &seen), 1, "{page:?}");
    assert_eq!(created(&page, &missed), 1, "{page:?}");
    let times: Vec<i64> = page.iter().map(|e| e.time_nano).collect();
    assert!(times.windows(2).all(|w| w[0] <= w[1]));
}

#[tokio::test]
async fn a_stopped_incus_is_explained_as_incus() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("unix.socket");
    let (reg, _, id) = setup_engine(socket.to_str().unwrap(), crate::EngineKind::Incus);
    reg.restart(&id);
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Unreachable { .. })).await;
    let error = reg.status(&id).error().cloned().unwrap();
    assert_eq!(error.code, ErrorCode::EngineNotRunning, "{error:?}");
    assert!(error.message.contains("Incus"), "{error:?}");
    assert!(reg.incus(&id).is_err());
    assert!(reg.docker(&id).is_err());
}

#[tokio::test]
async fn pages_of_another_engine_kind_say_what_the_host_runs() {
    let (reg, _, id) = setup_engine("/nonexistent/unix.socket", crate::EngineKind::Incus);
    let unsupported = reg.unsupported(&id);
    assert!(unsupported.is(ErrorCode::EngineUnsupported));
    assert!(
        unsupported.message.contains("local runs Incus"),
        "{unsupported:?}"
    );
    assert!(unsupported.hint.is_some());
}

#[tokio::test]
#[ignore = "needs a local Docker Engine"]
async fn reachable_hosts_connect_and_hand_out_a_client() {
    let (reg, _, id) = setup("/var/run/docker.sock");
    reg.restart(&id);
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Connected { .. })).await;
    reg.docker(&id).unwrap().ping().await.unwrap();
    assert_eq!(reg.status(&id).podman_version(), None);
    let err = reg.podman(&id).err().unwrap();
    assert!(err.is(ErrorCode::EngineUnsupported), "{err:?}");
    assert!(err.message.contains("doesn't run Podman"), "{err:?}");
}

#[tokio::test]
async fn a_stopped_containerd_is_explained_as_containerd() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("containerd.sock");
    let (reg, _, id) = setup_engine(socket.to_str().unwrap(), crate::EngineKind::Containerd);
    reg.restart(&id);
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Unreachable { .. })).await;
    let error = reg.status(&id).error().cloned().unwrap();
    assert_eq!(error.code, ErrorCode::EngineNotRunning, "{error:?}");
    assert!(error.message.contains("containerd"), "{error:?}");
    assert!(reg.containerd(&id).is_err());
}

#[tokio::test]
#[ignore = "needs a local containerd"]
async fn containerd_live_hosts_connect_and_hand_out_their_api() {
    let socket = std::env::var("HOMEPORT_CONTAINERD_SOCKET")
        .unwrap_or_else(|_| "/run/containerd/containerd.sock".into());
    let (reg, _, id) = setup_engine(&socket, crate::EngineKind::Containerd);
    reg.restart(&id);
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Connected { .. })).await;
    let status = reg.status(&id);
    let version = status
        .containerd_version()
        .expect("containerd says its version");
    assert!(
        version.chars().next().unwrap().is_ascii_digit(),
        "{version}"
    );
    let client = reg.containerd(&id).unwrap();
    assert_eq!(client.version(), version);
    homeport_containerd::namespaces::list(&client)
        .await
        .unwrap();
    let err = reg.docker(&id).err().unwrap();
    assert!(err.is(ErrorCode::EngineUnsupported), "{err:?}");
    assert!(reg.dialer(&id).is_ok());

    // Its events are kept, as a Docker Engine's are.
    let name = format!(
        "homeport-test-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..8]
    );
    tokio::time::sleep(Duration::from_millis(200)).await;
    homeport_containerd::namespaces::create(&client, &name)
        .await
        .unwrap();
    let mut seen = false;
    for _ in 0..40 {
        tokio::time::sleep(Duration::from_millis(50)).await;
        reg.events.flush().await;
        let page = reg.events.page(&id, crate::events::PAGE, None).unwrap();
        if page
            .iter()
            .any(|e| e.kind == "namespace" && e.actor_id == name)
        {
            seen = true;
            break;
        }
    }
    homeport_containerd::namespaces::remove(&client, &name)
        .await
        .unwrap();
    assert!(seen, "the namespace's event was kept");
}

#[tokio::test]
#[ignore = "needs a local Podman Engine"]
async fn podman_hosts_say_so_and_hand_out_its_api() {
    let socket = std::env::var("HOMEPORT_PODMAN_SOCKET")
        .unwrap_or_else(|_| "/run/podman/podman.sock".into());
    if !std::path::Path::new(&socket).exists() {
        eprintln!("no local Podman Engine at {socket}; skipping");
        return;
    }
    let (reg, _, id) = setup(&socket);
    reg.restart(&id);
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Connected { .. })).await;
    let version = reg.status(&id).podman_version().map(str::to_owned);
    assert!(version.is_some(), "{:?}", reg.status(&id));
    let podman = reg.podman(&id).unwrap();
    assert_eq!(Some(podman.version()), version.as_deref());
    homeport_engine::podman::pods::list(&podman).await.unwrap();

    // Podman's own kinds of event, such as a pod's, keep the Host connected.
    use homeport_engine::podman::pods;
    let name = format!(
        "homeport-test-events-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..8]
    );
    let spec = pods::PodSpec {
        name: name.clone(),
        ..Default::default()
    };
    let pod = pods::create(&podman, &spec).await.unwrap();
    let mut seen = false;
    for _ in 0..40 {
        tokio::time::sleep(Duration::from_millis(50)).await;
        reg.events.flush().await;
        let page = reg.events.page(&id, crate::events::PAGE, None).unwrap();
        if page.iter().any(|e| e.kind == "pod" && e.actor_id == pod) {
            seen = true;
            break;
        }
    }
    let still = reg.status(&id);
    pods::remove(&podman, &pod, true).await.unwrap();
    assert!(seen, "the pod's event was kept");
    assert!(matches!(still, HostStatus::Connected { .. }), "{still:?}");
}

#[tokio::test]
#[ignore = "needs a local Incus"]
async fn incus_live_hosts_connect_and_hand_out_their_api() {
    let socket = std::env::var("HOMEPORT_INCUS_SOCKET")
        .unwrap_or_else(|_| "/var/lib/incus/unix.socket".into());
    let (reg, _, id) = setup_engine(&socket, crate::EngineKind::Incus);
    reg.restart(&id);
    wait_for(&reg, &id, |s| matches!(s, HostStatus::Connected { .. })).await;
    assert!(reg
        .status(&id)
        .incus_version()
        .is_some_and(|v| !v.is_empty()));
    let client = reg.incus(&id).unwrap();
    let projects = homeport_incus::projects::list(&client).await.unwrap();
    assert!(projects.iter().any(|p| p.name == "default"), "{projects:?}");
    assert!(reg.docker(&id).is_err());
    assert!(reg.containerd(&id).is_err());
}

/// A supervisor that was replaced (its Host restarted again, as when a
/// Host is created while every stored one is being started) can still be
/// running until its next await. What it says then must not overwrite
/// what the supervisor that replaced it found: under load, that left a
/// connected Host "connecting" for good.
#[tokio::test]
async fn a_replaced_supervisor_s_late_word_is_let_go_of() {
    let (reg, _recorder, id) = setup("/nonexistent/homeport-test.sock");
    reg.restart(&id);
    let replaced = reg.lock()[&id].generation;
    reg.restart(&id);
    let current = reg.lock()[&id].generation;
    assert_ne!(replaced, current);
    let disabled = HostStatus::Disabled;
    reg.set_status(&id, current, disabled.clone(), None);
    // The replaced one wakes up late and says it is connecting.
    reg.set_status(&id, replaced, HostStatus::Connecting, None);
    assert_eq!(reg.status(&id), disabled);
    // Nor does it take the entry away when it can't read the Host.
    reg.removed(&id, replaced);
    assert_eq!(reg.status(&id), disabled);
    reg.removed(&id, current);
    assert!(!reg.lock().contains_key(&id));
}

/// Notes, for each status told, whether the registry held its entries
/// then.
#[derive(Default)]
struct Held {
    registry: Mutex<Weak<HostRegistry>>,
    told: Mutex<Vec<(HostStatus, bool)>>,
}

impl Notifier for Held {
    fn host_status(&self, _: &str, status: &HostStatus) {
        let registry = self.registry.lock().unwrap().upgrade();
        let held = registry.is_some_and(|reg| reg.entries.try_lock().is_err());
        self.told.lock().unwrap().push((status.clone(), held));
    }
    fn engine_event(&self, _: &str, _: &EngineEvent) {}
}

/// A Host's entry, as its supervisor `generation` left it.
fn entry(status: HostStatus, generation: u64) -> Entry {
    Entry {
        status,
        client: None,
        task: None,
        helpers: HashSet::new(),
        swarm: None,
        generation,
    }
}

fn connected() -> HostStatus {
    HostStatus::Connected {
        engine_version: "27.0".into(),
        api_version: "1.46".into(),
        os: "linux".into(),
        arch: "x86_64".into(),
        podman_version: None,
        containerd_version: None,
        incus_version: None,
        swarm: None,
    }
}

/// What is told while the entry is held is told in the order the entry
/// took it: a restart racing a supervisor waits for the lock, and its
/// supervisor's first word comes after.
#[test]
fn a_status_is_told_while_the_entry_is_held() {
    let (plain, _, id) = setup("/nonexistent/homeport-test.sock");
    let held = Arc::new(Held::default());
    let reg = HostRegistry::new(plain.store.clone(), plain.events.clone(), held.clone());
    *held.registry.lock().unwrap() = Arc::downgrade(&reg);
    reg.lock()
        .insert(id.clone(), entry(HostStatus::Connecting, 1));
    reg.set_status(&id, 1, connected(), None);
    reg.set_swarm(
        &id,
        1,
        Some(member(Some("s1"), "n1", SwarmRole::Manager, &[])),
    );
    let told = held.told.lock().unwrap().clone();
    assert_eq!(told.len(), 2, "{told:?}");
    assert!(told.iter().all(|(_, held)| *held), "{told:?}");
}

/// Asks the registry for the status it is told: a notifier must not.
#[derive(Default)]
struct CallsBack(Mutex<Weak<HostRegistry>>);

impl Notifier for CallsBack {
    fn host_status(&self, id: &str, _: &HostStatus) {
        if let Some(reg) = self.0.lock().unwrap().upgrade() {
            let _ = reg.status(id);
        }
    }
    fn engine_event(&self, _: &str, _: &EngineEvent) {}
}

/// It would wait on the lock it is told under for ever; a build with debug
/// assertions says why instead.
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "called the Hosts registry back")]
fn a_notifier_that_calls_the_registry_back_is_caught() {
    let (plain, _, id) = setup("/nonexistent/homeport-test.sock");
    let calls_back = Arc::new(CallsBack::default());
    let reg = HostRegistry::new(
        plain.store.clone(),
        plain.events.clone(),
        calls_back.clone(),
    );
    *calls_back.0.lock().unwrap() = Arc::downgrade(&reg);
    reg.lock()
        .insert(id.clone(), entry(HostStatus::Connecting, 1));
    reg.set_status(&id, 1, HostStatus::Disabled, None);
}

/// A replaced supervisor reads the Swarm through its old client, and says
/// so late: let go of, as its status is.
#[test]
fn a_replaced_supervisor_s_swarm_is_let_go_of() {
    let (reg, recorder, id) = setup("/nonexistent/homeport-test.sock");
    reg.lock().insert(id.clone(), entry(connected(), 2));
    let left = member(Some("old"), "n1", SwarmRole::Manager, &[]);
    reg.set_swarm(&id, 1, Some(left));
    assert_eq!(reg.lock()[&id].swarm, None);
    assert!(recorder.0.lock().unwrap().is_empty(), "nothing told");
    let joined = member(Some("s1"), "n1", SwarmRole::Manager, &[]);
    reg.set_swarm(&id, 2, Some(joined.clone()));
    assert_eq!(reg.lock()[&id].swarm, Some(joined));
}
