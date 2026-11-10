use std::collections::HashMap;

use super::*;
use crate::store::{HostDraft, HostStore};
use homeport_core::host::HostConnection;
use homeport_storage::SecretBox;

fn setup() -> (Arc<Db>, Arc<EventStore>, String) {
    let db = Arc::new(Db::open_in_memory().unwrap());
    let hosts = HostStore::new(db.clone(), Arc::new(SecretBox::from_key(&[3; 32])));
    let host = hosts
        .create(HostDraft {
            name: "local".into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            engine: crate::EngineKind::Docker,
            secrets: None,
        })
        .unwrap()
        .id;
    (db.clone(), EventStore::new(db), host)
}

fn event(time_nano: i64, action: &str) -> EngineEvent {
    EngineEvent {
        time: time_nano / 1_000_000_000,
        time_nano,
        kind: "container".into(),
        action: action.into(),
        actor_id: "abc".into(),
        attributes: HashMap::from([("name".into(), "web".into())]),
    }
}

#[tokio::test]
async fn events_are_kept_once_in_order() {
    let (_, store, host) = setup();
    let now = now_nano();
    for (t, action) in [(now - 30, "start"), (now - 20, "die"), (now - 30, "start")] {
        store.record(&host, &event(t, action));
    }
    // Two events in the same nanosecond are told apart by what they are.
    store.record(&host, &event(now - 20, "stop"));
    store.flush().await;

    let page = store.page(&host, PAGE, None).unwrap();
    let actions: Vec<_> = page.iter().map(|e| e.action.as_str()).collect();
    assert_eq!(actions, ["start", "die", "stop"]);
    assert_eq!(page[0].attributes["name"], "web");
    assert_eq!(store.last_time_nano(&host).unwrap(), Some(now - 20));
}

#[tokio::test]
async fn pages_go_back_from_a_cursor() {
    let (_, store, host) = setup();
    let now = now_nano();
    for i in 0..5 {
        store.record(&host, &event(now - 100 + i, &format!("a{i}")));
    }
    store.flush().await;
    let latest = store.page(&host, 2, None).unwrap();
    assert_eq!(
        latest.iter().map(|e| &e.action).collect::<Vec<_>>(),
        ["a3", "a4"]
    );
    let oldest = &latest[0];
    let cursor = EventCursor {
        time_nano: oldest.time_nano,
        kind: oldest.kind.clone(),
        action: oldest.action.clone(),
        actor_id: oldest.actor_id.clone(),
    };
    let older = store.page(&host, 10, Some(&cursor)).unwrap();
    assert_eq!(
        older.iter().map(|e| &e.action).collect::<Vec<_>>(),
        ["a0", "a1", "a2"]
    );
}

#[tokio::test]
async fn bursts_are_written_without_a_flush() {
    let (_, store, host) = setup();
    let now = now_nano();
    for i in 0..BATCH as i64 {
        store.record(&host, &event(now - 1_000 + i, "start"));
    }
    for _ in 0..100 {
        if store.page(&host, PAGE, None).unwrap().len() == BATCH {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("a full batch wasn't written at once");
}

#[tokio::test]
async fn retention_and_the_cap_bound_the_history() {
    let (_, store, host) = setup();
    let day = 86_400 * 1_000_000_000_i64;
    let now = now_nano();
    store.record(&host, &event(now - 40 * day, "old"));
    store.record(&host, &event(now - day, "recent"));
    store.flush().await;
    assert_eq!(store.prune(Duration::from_secs(30 * 86_400)).unwrap(), 1);
    let left = store.page(&host, PAGE, None).unwrap();
    assert_eq!(
        left.iter().map(|e| &e.action).collect::<Vec<_>>(),
        ["recent"]
    );

    for i in 0..i64::from(CAP_PER_HOST) + 5 {
        store.record(&host, &event(now - 100_000 + i, "burst"));
    }
    store.flush().await;
    let kept = store.page(&host, CAP_PER_HOST + 10, None).unwrap();
    assert_eq!(kept.len(), CAP_PER_HOST as usize);
    assert_eq!(kept.last().unwrap().action, "burst");
}

#[tokio::test]
async fn removing_a_host_removes_its_events_and_ignores_late_ones() {
    let (db, store, host) = setup();
    store.record(&host, &event(now_nano(), "start"));
    store.flush().await;
    db.with(|c| c.execute("DELETE FROM hosts WHERE id = ?1", [&host]))
        .unwrap();
    assert!(store.page(&host, PAGE, None).unwrap().is_empty());
    // An event still on its way when the Host was removed is dropped.
    store.record(&host, &event(now_nano(), "die"));
    store.flush().await;
    assert!(store.page(&host, PAGE, None).unwrap().is_empty());
}

#[tokio::test]
async fn clearing_forgets_a_hosts_events() {
    let (_, store, host) = setup();
    store.record(&host, &event(now_nano(), "start"));
    store.flush().await;
    store.clear(&host).unwrap();
    assert!(store.page(&host, PAGE, None).unwrap().is_empty());
    assert_eq!(store.last_time_nano(&host).unwrap(), None);
}

#[test]
fn since_keeps_the_nanoseconds() {
    assert_eq!(since(1_700_000_000_000_000_042), "1700000000.000000042");
}
