//! The desktop shell: the Tauri app, its plugins, and the state and events
//! that connect the backend's crates to the webview.

#[cfg(target_os = "linux")]
mod appimage;
pub mod events;
mod logging;
mod polling;
mod setup;
mod state;

use tauri::Manager;

pub use self::state::AppState;

use crate::commands;

pub fn run() {
    let specta = commands::builder();
    let invoke_handler = specta.invoke_handler();
    #[cfg(debug_assertions)]
    commands::export_bindings(commands::BINDINGS_PATH);

    tauri::Builder::default()
        .plugin(logging::plugin())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(move |app| {
            specta.mount_events(app);
            #[cfg(target_os = "linux")]
            if let Err(e) = appimage::integrate(&app.path().data_dir()?) {
                log::warn!("failed to add the AppImage to the app menu: {e}");
            }
            let state = setup::state(app)?;
            app.manage(state);
            polling::start(app.handle());
            Ok(())
        })
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running Homeport");
}
