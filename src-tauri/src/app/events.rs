//! Typed events pushed to the frontend (exported to the bindings as `events`).

use serde::Serialize;
use tauri::{AppHandle, Manager};
use tauri_specta::Event;

use homeport_core::error::AppError;
use homeport_engine::volumes::helper;
use homeport_hosts::registry::{EngineEvent, HostStatus, Notifier};

use super::AppState;

#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct HostStatusChanged {
    pub host_id: String,
    pub status: HostStatus,
}

#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct EngineEventReceived {
    pub host_id: String,
    pub event: EngineEvent,
}

/// A Synced Stack's poll found a new commit that changes its compose file's folder.
#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct SyncedUpdateAvailable {
    pub host_id: String,
    pub stack_id: String,
    pub name: String,
}

/// A Synced Stack's automatic deploy ended; `error` says why it failed.
#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct SyncedAutoDeployed {
    pub host_id: String,
    pub stack_id: String,
    pub name: String,
    pub commit: String,
    pub error: Option<AppError>,
}

/// Forwards what the host registry observes to the webview.
pub struct TauriNotifier(pub AppHandle);

impl Notifier for TauriNotifier {
    fn host_status(&self, host_id: &str, status: &HostStatus) {
        if matches!(status, HostStatus::Connected { .. }) {
            remove_leftover_helpers(&self.0, host_id);
        }
        let event = HostStatusChanged {
            host_id: host_id.to_owned(),
            status: status.clone(),
        };
        if let Err(e) = event.emit(&self.0) {
            log::warn!("failed to emit host status: {e}");
        }
    }

    fn engine_event(&self, host_id: &str, event: &EngineEvent) {
        let event = EngineEventReceived {
            host_id: host_id.to_owned(),
            event: event.clone(),
        };
        if let Err(e) = event.emit(&self.0) {
            log::warn!("failed to emit engine event: {e}");
        }
    }
}

/// Removes the helper containers this computer left on a Host it just
/// connected to (Homeport quit or crashed while a volume was open), keeping
/// those of the sessions still open (ADR 0015).
fn remove_leftover_helpers(app: &AppHandle, host_id: &str) {
    let app = app.clone();
    let host_id = host_id.to_owned();
    tauri::async_runtime::spawn(async move {
        let Some(state) = app.try_state::<AppState>() else {
            return;
        };
        let Ok(docker) = state.hosts.registry.docker(&host_id) else {
            return;
        };
        let keep = state.volume_browser.helper_ids();
        match helper::remove_leftovers(&docker, &keep).await {
            Ok(0) => {}
            Ok(n) => log::info!("removed {n} leftover helper containers from {host_id}"),
            Err(e) => log::warn!("couldn't look for leftover helper containers: {e}"),
        }
    });
}
