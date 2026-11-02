mod commands;
mod error;
mod services;
mod state;

use tauri::Manager;

use crate::{services::settings::SettingsService, state::AppState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta = commands::builder();
    let invoke_handler = specta.invoke_handler();
    #[cfg(debug_assertions)]
    commands::export_bindings(commands::BINDINGS_PATH);

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            specta.mount_events(app);
            let config_dir = app.path().app_config_dir()?;
            let settings = SettingsService::load(config_dir.join("settings.json"))?;
            app.manage(AppState { settings });
            Ok(())
        })
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running Homeport");
}
