//! Wiring the crates together once the app starts: opens the database and
//! the keychain, then builds the [`AppState`] every command is given.

use std::{error::Error, sync::Arc};

use tauri::{App, Manager};

use homeport_clusters::{service::ClusterService, store::ClusterStore};
use homeport_git::{Repositories, RepositoryStore};
use homeport_hosts::{
    events::EventStore, registry::HostRegistry, service::HostService, store::HostStore,
};
use homeport_registries::RegistryStore;
use homeport_settings::SettingsService;
use homeport_stacks::{
    history::DeployHistory,
    linked::{FileCache, LinkedStore},
    store::StackStore,
    synced::{SyncedStacks, SyncedStore},
};
use homeport_storage::{Db, Keychain, SecretBox};

use super::{
    events::{TauriClusterNotifier, TauriNotifier},
    AppState,
};

/// Builds the app's state and starts connecting to every Host.
pub fn state(app: &App) -> Result<AppState, Box<dyn Error>> {
    let config_dir = app.path().app_config_dir()?;
    let data_dir = app.path().app_data_dir()?;
    let db = Arc::new(Db::open(&data_dir.join("homeport.db"))?);
    let settings = SettingsService::new(db.clone(), &config_dir.join("settings.json"))?;
    // Keychain backends may block on D-Bus; keep that off the async runtime.
    let secrets = std::thread::spawn(|| {
        SecretBox::from_store(&Keychain {
            service: "com.homeport.app",
            account: "secrets-key",
        })
    })
    .join()
    .expect("keychain thread panicked");
    let secrets = Arc::new(secrets);
    let store = Arc::new(HostStore::new(db.clone(), secrets.clone()));
    let events = EventStore::new(db.clone());
    let registry = HostRegistry::new(
        store.clone(),
        events.clone(),
        Arc::new(TauriNotifier::new(app.handle().clone())),
    );
    let start = registry.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = start.start_all() {
            log::error!("failed to start host connections: {e}");
        }
    });
    let cluster_store = Arc::new(ClusterStore::new(db.clone(), secrets.clone()));
    let clusters = ClusterService::new(
        cluster_store,
        Arc::new(TauriClusterNotifier::new(app.handle().clone())),
    );
    let start = clusters.registry.clone();
    tauri::async_runtime::spawn(async move {
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
        stacks: StackStore::new(db.clone(), secrets.clone()),
        linked: LinkedStore::new(db.clone()),
        linked_files: FileCache::new(),
        history: Arc::new(DeployHistory::new(db.clone(), secrets.clone())),
        registries: RegistryStore::new(db.clone(), secrets.clone()),
        repositories,
        synced,
        secrets,
        settings,
        hosts: HostService {
            store,
            registry,
            events,
        },
        clusters,
        streams: Default::default(),
        exec: Default::default(),
        containerd_exec: Default::default(),
        pod_terminals: Default::default(),
        volume_browser: Default::default(),
        freshness: Default::default(),
        update: Default::default(),
        db,
        data_dir,
    })
}
