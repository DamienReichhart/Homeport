//! Live connections to every Host, each supervised by its own task.
//!
//! A supervisor connects, then holds the Engine's event stream open and pings
//! every 15 s. Transient failures retry with backoff (1 s doubling to 60 s);
//! failures that need the user (refused key, untrusted server key, lost
//! credentials) stop until [`HostRegistry::restart`] is called.

mod clients;
mod notifier;
mod status;
mod supervisor;
mod swarms;

use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, Weak,
    },
};

use tokio::task::JoinHandle;

use homeport_containerd::Containerd;
use homeport_core::{error::AppResult, host::HELPER_LABEL};
use homeport_engine::swarm::SwarmMembership;
use homeport_incus::Incus;
use homeport_transport::Connected;

use self::supervisor::supervise;
pub use self::{
    notifier::{EngineEvent, Notifier},
    status::HostStatus,
    supervisor::next_backoff,
    swarms::SwarmView,
};
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
    /// Which supervisor the entry is: what one started before it says is
    /// let go of.
    generation: u64,
}

pub struct HostRegistry {
    store: Arc<HostStore>,
    events: Arc<EventStore>,
    notifier: Arc<dyn Notifier>,
    entries: Mutex<HashMap<HostId, Entry>>,
    /// The last supervisor's number, across every Host.
    generations: AtomicU64,
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
            generations: AtomicU64::new(0),
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
        // The entry is there before the task that fills it starts, and the
        // task is numbered: one that was replaced (restarted again: a
        // Host created while every stored Host is started) may still be
        // running when it is aborted, until its next await, and what it
        // says then is let go of instead of overwriting the new one's.
        let generation = self.generations.fetch_add(1, Ordering::Relaxed) + 1;
        let mut entries = self.lock();
        let entry = entries.entry(id.to_owned()).or_insert_with(|| Entry {
            status: HostStatus::Connecting,
            client: None,
            task: None,
            helpers: HashSet::new(),
            swarm: None,
            generation,
        });
        if let Some(old) = entry.task.take() {
            old.abort();
        }
        entry.client = None;
        entry.generation = generation;
        // The task waits for the lock before it tells its first status.
        entry.task = Some(tokio::spawn(supervise(
            self.me.clone(),
            id.to_owned(),
            generation,
        )));
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

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<HostId, Entry>> {
        self.entries.lock().expect("registry lock poisoned")
    }

    /// What the supervisor `generation` found: let go of when another has
    /// replaced it since.
    fn set_status(&self, id: &str, generation: u64, status: HostStatus, client: Option<Client>) {
        let mut entries = self.lock();
        let Some(entry) = entries.get_mut(id) else {
            return;
        };
        if entry.generation != generation {
            return;
        }
        {
            if let HostStatus::Connected { swarm, .. } = &status {
                entry.swarm.clone_from(swarm);
            }
            entry.status = status.clone();
            entry.client = client;
        }
        drop(entries);
        self.notifier.host_status(id, &status);
    }

    /// The supervisor `generation` found its Host gone from the store.
    fn removed(&self, id: &str, generation: u64) {
        let current = self.lock().get(id).map(|entry| entry.generation);
        if current == Some(generation) {
            self.remove(id);
        }
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
mod tests;
