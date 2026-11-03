//! Live connections to every Host, each supervised by its own task.
//!
//! A supervisor connects, then holds the Engine's event stream open and pings
//! every 15 s. Transient failures retry with backoff (1 s doubling to 60 s);
//! failures that need the user (refused key, untrusted server key, lost
//! credentials) stop until [`HostRegistry::restart`] is called.

use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, Weak},
    time::Duration,
};

use bollard::{query_parameters::EventsOptions, Docker};
use futures_util::StreamExt;
use serde::Serialize;
use tokio::task::JoinHandle;

use super::{store::HostStore, HostId};
use crate::{
    error::{AppError, AppResult, ErrorCode},
    services::transport::{self, ConnectError, Connected, Dialer},
};

const PING_INTERVAL: Duration = Duration::from_secs(15);
const MAX_BACKOFF_SECS: u32 = 60;
/// How many Engine events each Host keeps for the Events page.
const EVENT_HISTORY: usize = 500;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(
    tag = "state",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum HostStatus {
    Disabled,
    Connecting,
    Connected {
        engine_version: String,
        api_version: String,
        os: String,
        arch: String,
    },
    Unreachable {
        error: AppError,
        retry_in_secs: u32,
    },
    Unauthorized {
        error: AppError,
    },
    HostKeyUntrusted {
        key: String,
        fingerprint: String,
    },
    HostKeyMismatch {
        key: String,
        fingerprint: String,
        expected_fingerprint: String,
    },
    CredentialsLost,
    KeychainUnavailable {
        error: AppError,
    },
    Invalid {
        error: AppError,
    },
}

impl From<ConnectError> for HostStatus {
    fn from(e: ConnectError) -> Self {
        match e {
            ConnectError::Unreachable(error) => Self::Unreachable {
                error,
                retry_in_secs: 0,
            },
            ConnectError::Unauthorized(error) => Self::Unauthorized { error },
            ConnectError::HostKeyUntrusted { key, fingerprint } => {
                Self::HostKeyUntrusted { key, fingerprint }
            }
            ConnectError::HostKeyMismatch {
                key,
                fingerprint,
                expected_fingerprint,
            } => Self::HostKeyMismatch {
                key,
                fingerprint,
                expected_fingerprint,
            },
            ConnectError::Invalid(error) => Self::Invalid { error },
        }
    }
}

/// One entry of a Host's Engine event stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EngineEvent {
    /// Unix time in seconds.
    pub time: i64,
    /// `container`, `image`, `volume`, `network`, …
    pub kind: String,
    pub action: String,
    pub actor_id: String,
    pub attributes: HashMap<String, String>,
}

/// Receives what the registry observes; the app forwards it to the frontend.
pub trait Notifier: Send + Sync + 'static {
    fn host_status(&self, host_id: &str, status: &HostStatus);
    fn engine_event(&self, host_id: &str, event: &EngineEvent);
}

struct Entry {
    status: HostStatus,
    client: Option<Connected>,
    task: Option<JoinHandle<()>>,
    events: VecDeque<EngineEvent>,
}

pub struct HostRegistry {
    store: Arc<HostStore>,
    notifier: Arc<dyn Notifier>,
    entries: Mutex<HashMap<HostId, Entry>>,
    me: Weak<HostRegistry>,
}

impl HostRegistry {
    pub fn new(store: Arc<HostStore>, notifier: Arc<dyn Notifier>) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            store,
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
            events: VecDeque::new(),
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

    pub fn events(&self, id: &str) -> Vec<EngineEvent> {
        self.lock()
            .get(id)
            .map(|e| e.events.iter().cloned().collect())
            .unwrap_or_default()
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
        let name = self
            .store
            .get(id)
            .map_or_else(|_| "This host".to_owned(), |h| h.name);
        let error = AppError::new(
            ErrorCode::HostNotConnected,
            format!("{name} isn't connected."),
        );
        match status {
            HostStatus::Unreachable { error: cause, .. }
            | HostStatus::Unauthorized { error: cause }
            | HostStatus::KeychainUnavailable { error: cause }
            | HostStatus::Invalid { error: cause } => AppError {
                hint: cause.hint.clone(),
                detail: Some(match &cause.detail {
                    Some(detail) => format!("{}\n{detail}", cause.message),
                    None => cause.message.clone(),
                }),
                ..error
            },
            HostStatus::Disabled => error.with_hint("Enable the host on its page to use it."),
            HostStatus::Connecting => {
                error.with_hint("Homeport is still connecting; try again in a moment.")
            }
            _ => error.with_hint("Open the host's page to see why and fix it."),
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
            if entry.events.len() == EVENT_HISTORY {
                entry.events.pop_front();
            }
            entry.events.push_back(event.clone());
        }
        self.notifier.engine_event(id, &event);
    }
}

/// The next wait after `current` seconds: doubling from 1 up to 60.
pub fn next_backoff(current: u32) -> u32 {
    (current.max(1) * 2).min(MAX_BACKOFF_SECS)
}

async fn supervise(registry: Weak<HostRegistry>, id: HostId) {
    let mut backoff = 1;
    loop {
        let Some(reg) = registry.upgrade() else {
            return;
        };
        let host = match reg.store.get(&id) {
            Ok(host) => host,
            Err(_) => return reg.remove(&id),
        };
        if !host.enabled {
            return reg.set_status(&id, HostStatus::Disabled, None);
        }
        reg.set_status(&id, HostStatus::Connecting, None);

        let secrets = if host.connection.needs_secrets() {
            match reg.store.secrets(&id) {
                Ok(secrets) => secrets,
                Err(e) if e.is(ErrorCode::CredentialsLost) => {
                    return reg.set_status(&id, HostStatus::CredentialsLost, None)
                }
                Err(error) if error.is(ErrorCode::KeychainUnavailable) => {
                    return reg.set_status(&id, HostStatus::KeychainUnavailable { error }, None)
                }
                Err(error) => return reg.set_status(&id, HostStatus::Invalid { error }, None),
            }
        } else {
            None
        };

        let error =
            match transport::connect(&host.connection, secrets, host.host_key.as_deref()).await {
                Ok(connected) => match describe(&connected.docker).await {
                    Ok(status) => {
                        backoff = 1;
                        let docker = connected.docker.clone();
                        reg.set_status(&id, status, Some(connected));
                        drop(reg);
                        watch(&registry, &id, docker).await;
                        AppError::new(
                            ErrorCode::ConnectionLost,
                            "Homeport lost the connection to the Docker Engine.",
                        )
                    }
                    Err(error) => error,
                },
                Err(ConnectError::Unreachable(error)) => error,
                Err(other) => return reg.set_status(&id, other.into(), None),
            };

        let Some(reg) = registry.upgrade() else {
            return;
        };
        reg.set_status(
            &id,
            HostStatus::Unreachable {
                error,
                retry_in_secs: backoff,
            },
            None,
        );
        drop(reg);
        tokio::time::sleep(Duration::from_secs(backoff.into())).await;
        backoff = next_backoff(backoff);
    }
}

async fn describe(docker: &Docker) -> Result<HostStatus, AppError> {
    let v = docker.version().await?;
    Ok(HostStatus::Connected {
        engine_version: v.version.unwrap_or_default(),
        api_version: v.api_version.unwrap_or_default(),
        os: v.os.unwrap_or_default(),
        arch: v.arch.unwrap_or_default(),
    })
}

/// Forwards Engine events until the stream breaks or a ping fails.
async fn watch(registry: &Weak<HostRegistry>, id: &str, docker: Docker) {
    let events = async {
        let mut stream = docker.events(None::<EventsOptions>);
        while let Some(Ok(msg)) = stream.next().await {
            let Some(reg) = registry.upgrade() else {
                return;
            };
            let actor = msg.actor.unwrap_or_default();
            reg.record_event(
                id,
                EngineEvent {
                    time: msg.time.unwrap_or_default(),
                    kind: msg.typ.map(|t| t.to_string()).unwrap_or_default(),
                    action: msg.action.unwrap_or_default(),
                    actor_id: actor.id.unwrap_or_default(),
                    attributes: actor.attributes.unwrap_or_default(),
                },
            );
        }
    };
    let pings = async {
        loop {
            tokio::time::sleep(PING_INTERVAL).await;
            match tokio::time::timeout(transport::CONNECT_TIMEOUT, docker.ping()).await {
                Ok(Ok(_)) => {}
                _ => return,
            }
        }
    };
    tokio::select! {
        _ = events => {}
        _ = pings => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{
        crypto::SecretBox,
        db::Db,
        hosts::{store::HostDraft, HostConnection},
    };

    #[derive(Default)]
    struct Recorder(Mutex<Vec<HostStatus>>);

    impl Notifier for Recorder {
        fn host_status(&self, _: &str, status: &HostStatus) {
            self.0.lock().unwrap().push(status.clone());
        }
        fn engine_event(&self, _: &str, _: &EngineEvent) {}
    }

    fn setup(socket_path: &str) -> (Arc<HostRegistry>, Arc<Recorder>, HostId) {
        let store = Arc::new(HostStore::new(
            Arc::new(Db::open_in_memory().unwrap()),
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
        (HostRegistry::new(store, recorder.clone()), recorder, id)
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
    fn backoff_doubles_up_to_a_minute() {
        let waits: Vec<_> = std::iter::successors(Some(1), |&b| Some(next_backoff(b)))
            .take(8)
            .collect();
        assert_eq!(waits, [1, 2, 4, 8, 16, 32, 60, 60]);
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

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn reachable_hosts_connect_and_hand_out_a_client() {
        let (reg, _, id) = setup("/var/run/docker.sock");
        reg.restart(&id);
        wait_for(&reg, &id, |s| matches!(s, HostStatus::Connected { .. })).await;
        reg.docker(&id).unwrap().ping().await.unwrap();
    }
}
