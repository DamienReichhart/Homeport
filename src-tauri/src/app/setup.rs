//! Wiring the crates together once the app starts: opens the database and
//! the keychain, then builds the [`AppState`] every command is given.

use std::{error::Error, sync::Arc};

use tauri::{App, Manager};

use homeport_hosts::{registry::HostRegistry, service::HostService, store::HostStore};
use homeport_registries::RegistryStore;
use homeport_settings::SettingsService;
use homeport_stacks::{
    linked::{FileCache, LinkedStore},
    store::StackStore,
};
use homeport_storage::{Db, Keychain, SecretBox};

use super::{events::TauriNotifier, AppState};

/// Builds the app's state and starts connecting to every Host.
pub fn state(app: &App) -> Result<AppState, Box<dyn Error>> {
    let config_dir = app.path().app_config_dir()?;
    let db = Arc::new(Db::open(&app.path().app_data_dir()?.join("homeport.db"))?);
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
    let registry = HostRegistry::new(store.clone(), Arc::new(TauriNotifier(app.handle().clone())));
    let start = registry.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = start.start_all() {
            log::error!("failed to start host connections: {e}");
        }
    });
    Ok(AppState {
        stacks: StackStore::new(db.clone(), secrets.clone()),
        linked: LinkedStore::new(db.clone()),
        linked_files: FileCache::new(),
        registries: RegistryStore::new(db.clone(), secrets.clone()),
        secrets,
        settings,
        hosts: HostService { store, registry },
        streams: Default::default(),
        exec: Default::default(),
        freshness: Default::default(),
        update: Default::default(),
    })
}
