//! The desktop shell: the Tauri app, its plugins, and the state and events
//! that connect the backend's crates to the webview.

#[cfg(target_os = "linux")]
mod appimage;
pub mod autostart;
pub mod events;
pub mod history;
mod logging;
mod notifications;
mod polling;
mod setup;
pub mod startup;
mod state;
pub mod tray;
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
        .plugin(autostart::plugin())
        .plugin(tauri_plugin_notification::init())
        .setup(move |app| {
            specta.mount_events(app);
            #[cfg(target_os = "linux")]
            {
                if let Err(e) = appimage::integrate(&app.path().data_dir()?) {
                    log::warn!("failed to add the AppImage to the app menu: {e}");
                }
                autostart::follow_appimage(app.handle());
            }
            let state = match setup::state(app) {
                Ok(state) => state,
                Err(setup::Failure::Database(error, database)) => {
                    // Nothing that needs the database starts; the window
                    // shows why, and where the file is.
                    let error = error.logged("failed to open the database");
                    app.manage(startup::Startup::failed(error, database));
                    window::show(app.handle());
                    return Ok(());
                }
                Err(setup::Failure::Other(e)) => return Err(e),
            };
            app.manage(startup::Startup::default());
            let start_hidden = state.settings.get().is_ok_and(|s| s.start_hidden);
            app.manage(state);
            tray::create(app.handle());
            polling::start(app.handle());
            history::start(app.handle());
            // The window starts hidden (`tauri.conf.json`), so it shows once,
            // where it was left, unless started at login to stay in the tray.
            if let Some(main) = app.get_webview_window(window::MAIN) {
                window::keep_on_screen(&main)?;
                let tray = tray::is_available(app.handle());
                if !autostart::starts_hidden(std::env::args(), start_hidden, tray) {
                    window::show(app.handle());
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();
                if window.label() == window::MAIN && closes_to_tray(app) {
                    api.prevent_close();
                    window::hide(app);
                }
            }
        })
        .invoke_handler(invoke_handler)
        .build(tauri::generate_context!())
        .expect("error while building Homeport")
        .run(|app, event| match event {
            // The Dock icon was clicked while the window was hidden.
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Reopen { .. } => window::show(app),
            tauri::RunEvent::Exit => {
                // A volume being browsed keeps a helper container on its Host.
                if let Some(state) = app.try_state::<AppState>() {
                    tauri::async_runtime::block_on(async {
                        state.volume_browser.end_all().await;
                        // The last second of Engine events may still be waiting.
                        state.hosts.events.flush().await;
                    });
                }
            }
            _ => {}
        });
}

/// Whether closing the window hides it in the tray: the user asked for it,
/// and there is a tray to bring it back from.
fn closes_to_tray(app: &tauri::AppHandle) -> bool {
    let Some(state) = app.try_state::<AppState>() else {
        return false;
    };
    tray::is_available(app) && state.settings.get().is_ok_and(|s| s.close_to_tray)
}
