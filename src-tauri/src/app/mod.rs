//! The desktop shell: the Tauri app, its plugins, and the state and events
//! that connect the backend's crates to the webview.

#[cfg(target_os = "linux")]
mod appimage;
pub mod events;
pub mod history;
mod logging;
mod polling;
mod setup;
mod state;
pub mod window;

use tauri::Manager;

pub use self::state::AppState;

use crate::commands;

pub fn run() {
    let specta = commands::builder();
    let invoke_handler = specta.invoke_handler();
    #[cfg(debug_assertions)]
    commands::export_bindings(commands::BINDINGS_PATH);

    tauri::Builder::default()
        // First, so a second launch hands over before anything else starts.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            window::show(app)
        }))
        .plugin(logging::plugin())
        .plugin(window::state_plugin())
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
            history::start(app.handle());
            // The window starts hidden (`tauri.conf.json`), so it shows once,
            // where it was left.
            if let Some(main) = app.get_webview_window(window::MAIN) {
                window::keep_on_screen(&main)?;
                window::show(app.handle());
            }
            Ok(())
        })
        .invoke_handler(invoke_handler)
        .build(tauri::generate_context!())
        .expect("error while building Homeport")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event {
                // A volume being browsed keeps a helper container on its Host.
                if let Some(state) = app.try_state::<AppState>() {
                    tauri::async_runtime::block_on(async {
                        state.volume_browser.end_all().await;
                        // The last second of Engine events may still be waiting.
                        state.hosts.events.flush().await;
                    });
                }
            }
        });
}
