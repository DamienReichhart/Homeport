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

use homeport_containerd::Containerd;
use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    host::HELPER_LABEL,
};
use homeport_engine::{
    podman::Podman,
    swarm::{self, SwarmMembership, SwarmRole},
};
use homeport_incus::Incus;
use serde::Serialize;

use crate::messages as msg;
use homeport_transport::{Connected, Dialer};

pub use self::{
    notifier::{EngineEvent, Notifier},
    status::HostStatus,
    supervisor::next_backoff,
};

use self::supervisor::supervise;
use crate::{events::EventStore, store::HostStore, HostId};

/// How a connected Host's Engine is reached, one variant per kind of Engine
/// (`EngineKind`) Homeport connects to.
#[derive(Clone)]
pub(crate) enum Client {
    /// The Docker Engine API through bollard, and the dialer beside it.
    Docker(Connected),
    /// containerd's gRPC API over the Host's dialer (ADR 0020).
    Containerd(Containerd),
    /// Incus's REST API over the Host's dialer (ADR 0021).
    Incus(Incus),
}

struct Entry {
    status: HostStatus,
    client: Option<Client>,
    task: Option<JoinHandle<()>>,
    /// Homeport's own helper containers seen in the events, whose events
    /// (and their volumes' mounts) aren't shown (ADR 0015).
    helpers: HashSet<String>,
    /// The Host's place in a Swarm when it was last connected, so a Swarm
    /// whose managers are all disconnected can still name them. Kept in
    /// memory only: nothing about a Swarm is stored (ADR 0016).
    swarm: Option<SwarmMembership>,
}

/// A Swarm, found through the Hosts that belong to it (ADR 0016).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmView {
    pub id: String,
    /// Its manager Hosts, in the Hosts' order.
    pub managers: Vec<HostId>,
    /// Its worker Hosts, in the Hosts' order.
    pub workers: Vec<HostId>,
    /// Whether one of its managers is connected, so it can be managed.
    pub reachable: bool,
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

    /// Drops every connection and connects to the stored Hosts afresh, as
    /// after a restore replaced them all. Must run inside a Tokio runtime.
    pub fn reload(&self) -> AppResult<()> {
        let old: Vec<Entry> = self.lock().drain().map(|(_, entry)| entry).collect();
        for task in old.into_iter().filter_map(|entry| entry.task) {
            task.abort();
        }
        self.start_all()
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
            swarm: None,
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

    /// The Docker Engine API client of a connected Host;
    /// `EngineUnsupported` for a Host whose Engine doesn't speak it.
    pub fn docker(&self, id: &str) -> AppResult<Docker> {
        self.docker_connection(id).map(|c| c.docker)
    }

    /// What opens streams to a connected Host's Engine socket, for the
    /// compose proxy and for the APIs bollard doesn't speak.
    pub fn dialer(&self, id: &str) -> AppResult<Arc<dyn Dialer>> {
        match self.connected(id)? {
            Client::Docker(connected) => Ok(connected.dialer),
            Client::Containerd(containerd) => Ok(containerd.dialer().clone()),
            Client::Incus(incus) => Ok(incus.dialer().clone()),
        }
    }

    /// containerd's API on a connected containerd Host; `EngineUnsupported`
    /// for any other Engine.
    pub fn containerd(&self, id: &str) -> AppResult<Containerd> {
        match self.connected(id)? {
            Client::Containerd(containerd) => Ok(containerd),
            Client::Docker(_) | Client::Incus(_) => Err(self.not_containerd(id)),
        }
    }

    /// Incus's API on a connected Incus Host; `EngineUnsupported` for any
    /// other Engine.
    pub fn incus(&self, id: &str) -> AppResult<Incus> {
        match self.connected(id)? {
            Client::Incus(incus) => Ok(incus),
            Client::Docker(_) | Client::Containerd(_) => Err(self.not_incus(id)),
        }
    }

    /// Why a Host can't do what only Incus does.
    fn not_incus(&self, id: &str) -> AppError {
        let error = match self.store.get(id) {
            Ok(host) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::NOT_INCUS.with("name", host.name),
            ),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        };
        error.with_hint(msg::NOT_INCUS_HINT)
    }

    /// Why a Host can't do what only containerd does.
    fn not_containerd(&self, id: &str) -> AppError {
        let error = match self.store.get(id) {
            Ok(host) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::NOT_CONTAINERD.with("name", host.name),
            ),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        };
        error.with_hint(msg::NOT_CONTAINERD_HINT)
    }

    /// Podman's own API on a connected Podman Host; `EngineUnsupported`
    /// for any other Engine.
    pub fn podman(&self, id: &str) -> AppResult<Podman> {
        let connected = self.docker_connection(id)?;
        match self.status(id).podman_version() {
            Some(version) => Ok(Podman::new(connected.dialer, version)),
            None => Err(self.not_podman(id)),
        }
    }

    /// Why a Host can't do what only Podman does.
    fn not_podman(&self, id: &str) -> AppError {
        let error = match self.store.get(id) {
            Ok(host) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::NOT_PODMAN.with("name", host.name),
            ),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        };
        error.with_hint(msg::NOT_PODMAN_HINT)
    }

    fn docker_connection(&self, id: &str) -> AppResult<Connected> {
        match self.connected(id)? {
            Client::Docker(connected) => Ok(connected),
            Client::Containerd(_) | Client::Incus(_) => Err(self.unsupported(id)),
        }
    }

    /// Why a Host's Engine can't do what was asked: it is of another kind.
    pub fn unsupported(&self, id: &str) -> AppError {
        match self.store.get(id) {
            Ok(host) => unsupported(&host.name, host.engine.label()),
            Err(_) => AppError::new(
                ErrorCode::EngineUnsupported,
                msg::ENGINE_UNSUPPORTED_UNKNOWN,
            ),
        }
    }

    fn connected(&self, id: &str) -> AppResult<Client> {
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

    fn set_status(&self, id: &str, status: HostStatus, client: Option<Client>) {
        if let Some(entry) = self.lock().get_mut(id) {
            if let HostStatus::Connected { swarm, .. } = &status {
                entry.swarm.clone_from(swarm);
            }
            entry.status = status.clone();
            entry.client = client;
        }
        self.notifier.host_status(id, &status);
    }

    /// Updates a connected Host's place in a Swarm, telling the app when it changed.
    fn set_swarm(&self, id: &str, membership: Option<SwarmMembership>) {
        let changed = {
            let mut entries = self.lock();
            let Some(entry) = entries.get_mut(id) else {
                return;
            };
            match &mut entry.status {
                HostStatus::Connected { swarm, .. } if *swarm != membership => {
                    swarm.clone_from(&membership);
                    entry.swarm = membership;
                    Some(entry.status.clone())
                }
                _ => None,
            }
        };
        if let Some(status) = changed {
            self.notifier.host_status(id, &status);
        }
    }

    /// Reads a Host's place in a Swarm again, after Homeport changed it.
    pub async fn refresh_swarm(&self, id: &str) {
        if let Ok(docker) = self.docker(id) {
            match swarm::membership(&docker).await {
                Ok(membership) => self.set_swarm(id, membership),
                Err(e) => log::debug!("couldn't read the swarm membership of {id}: {e}"),
            }
        }
    }

    /// Every Swarm the Hosts belong to, one per swarm ID however many of
    /// its managers are Hosts. A worker is placed by its node ID, read from
    /// a connected manager's node list, or else by the managers it knows.
    pub async fn swarms(&self) -> Vec<SwarmView> {
        let order: Vec<HostId> = self
            .store
            .list()
            .map(|hosts| hosts.into_iter().map(|h| h.id).collect())
            .unwrap_or_default();
        let members: Vec<(HostId, SwarmMembership, bool)> = {
            let entries = self.lock();
            order
                .iter()
                .filter_map(|id| {
                    let entry = entries.get(id)?;
                    Some((id.clone(), entry.swarm.clone()?, entry.client.is_some()))
                })
                .collect()
        };
        let mut swarms = group_managers(&members);
        for view in &mut swarms {
            let nodes = match self.manager(&view.id) {
                Ok(docker) => swarm::node_ids(&docker).await.unwrap_or_else(|e| {
                    log::debug!("couldn't list the nodes of swarm {}: {e}", view.id);
                    vec![]
                }),
                Err(_) => vec![],
            };
            view.workers = workers_of(view, &members, &nodes);
        }
        swarms
    }

    /// The Host each Swarm node is, by node ID, for the nodes that are Hosts.
    pub fn node_hosts(&self) -> HashMap<String, HostId> {
        self.lock()
            .iter()
            .filter_map(|(id, e)| Some((e.swarm.as_ref()?.node_id.clone(), id.clone())))
            .collect()
    }

    /// The client of a connected manager of the Swarm `swarm_id`.
    pub fn manager(&self, swarm_id: &str) -> AppResult<Docker> {
        let id = self.manager_host(swarm_id)?;
        self.docker(&id)
    }

    /// A connected manager Host of the Swarm `swarm_id`, or why there's none.
    pub fn manager_host(&self, swarm_id: &str) -> AppResult<HostId> {
        let known: Vec<HostId> = {
            let entries = self.lock();
            let managers: Vec<(&HostId, &Entry)> = entries
                .iter()
                .filter(|(_, e)| {
                    e.swarm.as_ref().is_some_and(|m| {
                        m.role == SwarmRole::Manager && m.swarm_id.as_deref() == Some(swarm_id)
                    })
                })
                .collect();
            if let Some((id, _)) = managers.iter().find(|(_, e)| e.client.is_some()) {
                return Ok((*id).clone());
            }
            managers.into_iter().map(|(id, _)| id.clone()).collect()
        };
        let names: Vec<String> = known
            .iter()
            .filter_map(|id| self.store.get(id).ok().map(|h| h.name))
            .collect();
        let error = AppError::new(ErrorCode::SwarmUnreachable, msg::SWARM_UNREACHABLE);
        Err(if names.is_empty() {
            error.with_hint(msg::SWARM_UNREACHABLE_UNKNOWN_HINT)
        } else {
            error.with_hint(msg::SWARM_UNREACHABLE_HINT.with("hosts", names.join(", ")))
        })
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

/// `name`'s Engine, described by `engine`, doesn't offer what was asked.
pub(crate) fn unsupported(name: &str, engine: &str) -> AppError {
    AppError::new(
        ErrorCode::EngineUnsupported,
        msg::ENGINE_UNSUPPORTED
            .with("name", name)
            .with("engine", engine),
    )
    .with_hint(msg::ENGINE_UNSUPPORTED_HINT)
}

/// The Swarms of the manager Hosts among `members`, by swarm ID, in the Hosts' order.
fn group_managers(members: &[(HostId, SwarmMembership, bool)]) -> Vec<SwarmView> {
    let mut swarms: Vec<SwarmView> = Vec::new();
    for (id, membership, connected) in members {
        let Some(swarm_id) = membership
            .swarm_id
            .as_ref()
            .filter(|_| membership.role == SwarmRole::Manager)
        else {
            continue;
        };
        let index = match swarms.iter().position(|s| &s.id == swarm_id) {
            Some(index) => index,
            None => {
                swarms.push(SwarmView {
                    id: swarm_id.clone(),
                    managers: vec![],
                    workers: vec![],
                    reachable: false,
                });
                swarms.len() - 1
            }
        };
        swarms[index].managers.push(id.clone());
        swarms[index].reachable |= connected;
    }
    swarms
}

/// The worker Hosts among `members` that belong to `swarm`: their node is
/// in its `nodes`, or they talk to one of its manager Hosts.
fn workers_of(
    swarm: &SwarmView,
    members: &[(HostId, SwarmMembership, bool)],
    nodes: &[String],
) -> Vec<HostId> {
    let manager_nodes: Vec<&str> = members
        .iter()
        .filter(|(id, _, _)| swarm.managers.contains(id))
        .map(|(_, m, _)| m.node_id.as_str())
        .collect();
    members
        .iter()
        .filter(|(_, m, _)| m.role == SwarmRole::Worker)
        .filter(|(_, m, _)| {
            nodes.contains(&m.node_id)
                || m.managers
                    .iter()
                    .any(|p| manager_nodes.contains(&p.node_id.as_str()))
        })
        .map(|(id, _, _)| id.clone())
        .collect()
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
}
