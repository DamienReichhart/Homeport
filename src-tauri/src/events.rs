//! Typed events pushed to the frontend (exported to the bindings as `events`).

use serde::Serialize;
use tauri::AppHandle;
use tauri_specta::Event;

use crate::services::hosts::registry::{EngineEvent, HostStatus, Notifier};

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

/// Forwards what the host registry observes to the webview.
pub struct TauriNotifier(pub AppHandle);

impl Notifier for TauriNotifier {
    fn host_status(&self, host_id: &str, status: &HostStatus) {
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
