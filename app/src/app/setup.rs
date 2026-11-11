//! Wiring the crates together once the app starts: opens the database and
//! the keychain, then builds the [`AppState`] every use case is given.

use std::{error::Error, path::PathBuf, sync::Arc};

use homeport_clusters::{service::ClusterService, store::ClusterStore};
use homeport_core::error::AppError;
use homeport_git::{Repositories, RepositoryStore};
use homeport_hosts::{
    events::EventStore, registry::HostRegistry, service::HostService, store::HostStore,
};
use homeport_registries::RegistryStore;
use homeport_settings::SettingsService;
use homeport_stacks::{
    linked::{FileCache, LinkedStore},
    store::{history::DeployHistory, StackStore},
    synced::{SyncedStacks, SyncedStore},
    Stacks,
};
use homeport_storage::{Db, Keychain, SecretBox};

use super::{events::Events, AppState};

/// The OS keychain entry holding the key every secret is sealed with.
pub const KEYCHAIN: Keychain = Keychain {
    service: "com.homeport.app",
    account: "secrets-key",
};

/// The folder name the app keeps its files under, as before.
const IDENTIFIER: &str = "com.homeport.app";

/// Why the app's state couldn't be built.
pub enum Failure {
    /// The database at this path couldn't be opened: shown in the window.
    Database(Box<AppError>, PathBuf),
    /// Anything else, which ends the app.
    Other(Box<dyn Error>),
}

impl<E: Into<Box<dyn Error>>> From<E> for Failure {
    fn from(e: E) -> Self {
        Self::Other(e.into())
    }
}

/// Where the app's settings file lives.
fn config_dir() -> Result<PathBuf, Failure> {
    dirs::config_dir()
        .map(|d| d.join(IDENTIFIER))
        .ok_or_else(|| "this computer has no config folder".into())
}

/// Where the app's database and clones live.
fn data_dir() -> Result<PathBuf, Failure> {
    dirs::data_dir()
        .map(|d| d.join(IDENTIFIER))
        .ok_or_else(|| "this computer has no data folder".into())
}

/// Builds the app's state and starts connecting to every Host. Must run
/// inside the Tokio runtime.
pub fn state(events: &Events) -> Result<AppState, Failure> {
    state_in(config_dir()?, data_dir()?, events, |holds_sealed| {
        // Keychain backends may block on D-Bus; keep that off the async runtime.
        std::thread::spawn(move || SecretBox::from_store(&KEYCHAIN, holds_sealed))
            .join()
            .expect("keychain thread panicked")
    })
}

/// [`state`] with its folders and the key its secrets are sealed with given,
/// so tests build one in a temporary folder without the OS keychain.
pub fn state_in(
    config_dir: PathBuf,
    data_dir: PathBuf,
    events: &Events,
    secrets: impl FnOnce(bool) -> SecretBox,
) -> Result<AppState, Failure> {
    std::fs::create_dir_all(&config_dir)?;
    std::fs::create_dir_all(&data_dir)?;
    let database = data_dir.join("homeport.db");
    let db = match Db::open(&database) {
        Ok(db) => Arc::new(db),
        Err(e) => return Err(Failure::Database(Box::new(e), database)),
    };
    let settings = SettingsService::new(db.clone(), &config_dir.join("settings.json"))?;
    // A missing keychain entry makes a new key only when nothing is sealed yet.
    let holds_sealed = db.holds_sealed_data()?;
    let secrets = Arc::new(secrets(holds_sealed));
    let store = Arc::new(HostStore::new(db.clone(), secrets.clone()));
    let host_events = EventStore::new(db.clone());
    let registry = HostRegistry::new(store.clone(), host_events.clone(), Arc::new(events.clone()));
    let start = registry.clone();
    tokio::spawn(async move {
        if let Err(e) = start.start_all() {
            log::error!("failed to start host connections: {e}");
        }
    });
    let cluster_store = Arc::new(ClusterStore::new(db.clone(), secrets.clone()));
    let clusters = ClusterService::new(cluster_store, Arc::new(events.clone()));
    let start = clusters.registry.clone();
    tokio::spawn(async move {
        if let Err(e) = start.start_all() {
            log::error!("failed to start cluster connections: {e}");
        }
    });
    let repositories = Arc::new(Repositories::new(
        RepositoryStore::new(db.clone(), secrets.clone()),
        data_dir.join("repositories"),
    ));
    let synced = Arc::new(SyncedStacks::new(
        SyncedStore::new(db.clone(), secrets.clone()),
        repositories.clone(),
        data_dir.join("synced-stacks"),
    ));
    Ok(AppState {
        stacks: Stacks {
            managed: StackStore::new(db.clone(), secrets.clone()),
            linked: LinkedStore::new(db.clone()),
            linked_files: FileCache::new(),
            history: Arc::new(DeployHistory::new(db.clone(), secrets.clone())),
            synced,
        },
        registries: RegistryStore::new(db.clone(), secrets.clone()),
        repositories,
        secrets,
        settings,
        hosts: HostService {
            store,
            registry,
            events: host_events,
        },
        clusters,
        streams: Default::default(),
        terminals: Default::default(),
        volume_browser: Default::default(),
        freshness: Default::default(),
        events: events.clone(),
        db,
        data_dir,
        config_dir,
    })
}
