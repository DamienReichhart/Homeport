//! Live connections to every Host, each supervised by its own task.
//!
//! A supervisor connects, then holds the Engine's event stream open and pings
//! every 15 s. Transient failures retry with backoff (1 s doubling to 60 s);
//! failures that need the user (refused key, untrusted server key, lost
//! credentials) stop until [`HostRegistry::restart`] is called.
mod notifier;
mod status;
mod supervisor;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, Weak},
};

use bollard::Docker;
use tokio::task::JoinHandle;

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    host::HELPER_LABEL,
};

use crate::messages as msg;
use homeport_transport::{Connected, Dialer};

pub use self::{
    notifier::{EngineEvent, Notifier},
    status::HostStatus,
    supervisor::next_backoff,
};

use self::supervisor::supervise;
use crate::{events::EventStore, store::HostStore, HostId};

struct Entry {
    status: HostStatus,
    client: Option<Connected>,
    task: Option<JoinHandle<()>>,
    /// Homeport's own helper containers seen in the events, whose events
    /// (and their volumes' mounts) aren't shown (ADR 0015).
    helpers: HashSet<String>,
}

pub struct HostRegistry {
    store: Arc<HostStore>,
    events: Arc<EventStore>,
    notifier: Arc<dyn Notifier>,
    entries: Mutex<HashMap<HostId, Entry>>,
    me: Weak<HostRegistry>,
}

impl HostRegistry {
    pub fn new(
        store: Arc<HostStore>,
        events: Arc<EventStore>,
        notifier: Arc<dyn Notifier>,
    ) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            store,
            events,
            notifier,
            entries: Mutex::new(HashMap::new()),
            me: me.clone(),
        })
    }

    /// Starts supervising every stored Host. Must run inside a Tokio runtime.
    pub fn start_all(&self) -> AppResult<()> {
        for host in self.store.list()? {
            self.restart(&host.id);
        }
        Ok(())
    }

    /// (Re)starts a Host's supervisor, e.g. after it was edited or trusted.
    pub fn restart(&self, id: &str) {
        let task = tokio::spawn(supervise(self.me.clone(), id.to_owned()));
        let mut entries = self.lock();
        let entry = entries.entry(id.to_owned()).or_insert_with(|| Entry {
            status: HostStatus::Connecting,
            client: None,
            task: None,
            helpers: HashSet::new(),
        });
        if let Some(old) = entry.task.replace(task) {
            old.abort();
        }
        entry.client = None;
    }

    pub fn remove(&self, id: &str) {
        if let Some(entry) = self.lock().remove(id) {
            if let Some(task) = entry.task {
                task.abort();
            }
        }
    }

    pub fn status(&self, id: &str) -> HostStatus {
        self.lock()
            .get(id)
            .map_or(HostStatus::Connecting, |e| e.status.clone())
    }

    /// The Engine client of a connected Host.
    pub fn docker(&self, id: &str) -> AppResult<Docker> {
        self.connected(id).map(|c| c.docker)
    }

    /// The dialer of a connected TLS or SSH Host (`None` for local and TCP).
    #[allow(dead_code)] // used by the compose proxy
    pub fn dialer(&self, id: &str) -> AppResult<Option<Arc<dyn Dialer>>> {
        self.connected(id).map(|c| c.dialer)
    }

    fn connected(&self, id: &str) -> AppResult<Connected> {
        let status = {
            let entries = self.lock();
            match entries.get(id) {
                Some(Entry {
                    client: Some(client),
                    ..
                }) => return Ok(client.clone()),
                Some(entry) => entry.status.clone(),
                None => HostStatus::Connecting,
            }
        };
        Err(self.not_connected(id, &status))
    }

    /// Why a command can't reach the Host right now, carrying the cause from its status.
    fn not_connected(&self, id: &str, status: &HostStatus) -> AppError {
        let message = match self.store.get(id) {
            Ok(host) => msg::NOT_CONNECTED.with("name", host.name),
            Err(_) => msg::NOT_CONNECTED_UNKNOWN.into(),
        };
        let error = AppError::new(ErrorCode::HostNotConnected, message);
        if let Some(cause) = status.error() {
            return AppError {
                hint: cause.hint.clone(),
                detail: Some(match &cause.detail {
                    Some(detail) => format!("{}\n{detail}", cause.message),
                    None => cause.message.clone(),
                }),
                ..error
            };
        }
        match status {
            HostStatus::Disabled => error.with_hint(msg::NOT_CONNECTED_DISABLED_HINT),
            HostStatus::Connecting => error.with_hint(msg::NOT_CONNECTED_CONNECTING_HINT),
            _ => error.with_hint(msg::NOT_CONNECTED_HINT),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<HostId, Entry>> {
        self.entries.lock().expect("registry lock poisoned")
    }

    fn set_status(&self, id: &str, status: HostStatus, client: Option<Connected>) {
        if let Some(entry) = self.lock().get_mut(id) {
            entry.status = status.clone();
            entry.client = client;
        }
        self.notifier.host_status(id, &status);
    }

    fn record_event(&self, id: &str, event: EngineEvent) {
        if let Some(entry) = self.lock().get_mut(id) {
            if is_helpers(&mut entry.helpers, &event) {
                return;
            }
        }
        self.events.record(id, &event);
        self.notifier.engine_event(id, &event);
    }
}

/// Whether `event` is about a helper container, or a volume mounted in one;
/// remembers the helpers it sees in `helpers`.
fn is_helpers(helpers: &mut HashSet<String>, event: &EngineEvent) -> bool {
    match event.kind.as_str() {
        "container" if event.attributes.contains_key(HELPER_LABEL) => {
            if event.action == "destroy" {
                helpers.remove(&event.actor_id);
            } else {
                helpers.insert(event.actor_id.clone());
            }
            true
        }
        "volume" => event
            .attributes
            .get("container")
            .is_some_and(|c| helpers.contains(c)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use homeport_storage::{Db, SecretBox};

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

    #[test]
    fn helper_events_are_left_out() {
        let event =
            |kind: &str, action: &str, actor: &str, attributes: &[(&str, &str)]| EngineEvent {
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
    #[ignore = "needs a local Docker Engine"]
    async fn reachable_hosts_connect_and_hand_out_a_client() {
        let (reg, _, id) = setup("/var/run/docker.sock");
        reg.restart(&id);
        wait_for(&reg, &id, |s| matches!(s, HostStatus::Connected { .. })).await;
        reg.docker(&id).unwrap().ping().await.unwrap();
    }
}
