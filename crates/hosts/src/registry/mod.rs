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
mod tests;
