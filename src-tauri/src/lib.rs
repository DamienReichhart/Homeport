mod commands;
mod error;
mod events;
mod services;
mod state;

use tauri::Manager;

use std::sync::Arc;

use crate::{
    events::TauriNotifier,
    services::{
        crypto::{Keychain, SecretBox},
        db::Db,
        hosts::{registry::HostRegistry, service::HostService, store::HostStore},
        settings::SettingsService,
        stacks::store::StackStore,
    },
    state::AppState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta = commands::builder();
    let invoke_handler = specta.invoke_handler();
    #[cfg(debug_assertions)]
    commands::export_bindings(commands::BINDINGS_PATH);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                // bollard and hyper log every request at debug/trace level.
                .level_for("bollard", log::LevelFilter::Warn)
                .level_for("hyper_util", log::LevelFilter::Warn)
                .level_for("russh", log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            specta.mount_events(app);
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
            let registry =
                HostRegistry::new(store.clone(), Arc::new(TauriNotifier(app.handle().clone())));
            let start = registry.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = start.start_all() {
                    log::error!("failed to start host connections: {e}");
                }
            });
            app.manage(AppState {
                stacks: StackStore::new(db.clone(), secrets.clone()),
                secrets,
                settings,
                hosts: HostService { store, registry },
                streams: Default::default(),
                exec: Default::default(),
            });
            Ok(())
        })
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running Homeport");
}
