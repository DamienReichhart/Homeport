//! Keeping the Engine event history within the retention the user set: the
//! store lives in `homeport-hosts`; this prunes it when Homeport starts,
//! every hour, and when the setting changes.

use std::time::Duration;

use tauri::{AppHandle, Manager};

use super::AppState;

const PRUNE_EVERY: Duration = Duration::from_secs(3600);

/// Prunes the history now and every hour while Homeport runs.
pub fn start(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            prune(&app);
            tokio::time::sleep(PRUNE_EVERY).await;
        }
    });
}

/// Removes the events older than the retention setting, and each Host's
/// beyond the cap.
pub fn prune(app: &AppHandle) {
    let Some(state) = app.try_state::<AppState>() else {
        return;
    };
    let keep = match state.settings.get() {
        Ok(settings) => settings.keep_events(),
        Err(e) => return log::warn!("couldn't read how long to keep events: {e}"),
    };
    match state.hosts.events.prune(keep) {
        Ok(0) => {}
        Ok(n) => log::info!("removed {n} engine events past their retention"),
        Err(e) => log::warn!("couldn't prune the engine events: {e}"),
    }
}
