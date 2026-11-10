//! Typed events pushed to the frontend (exported to the bindings as `events`).

use serde::Serialize;
use tauri::{AppHandle, Manager};
use tauri_specta::Event;

use homeport_clusters::registry::{ClusterNotifier, ClusterStatus};
use homeport_core::error::AppError;
use homeport_engine::volumes::helper;
use homeport_hosts::registry::{EngineEvent, HostStatus, Notifier};
use homeport_kubernetes::forward::ForwardEvent;
use homeport_stacks::StackOwner;

use super::{notifications::Notifications, AppState};

#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct HostStatusChanged {
    pub host_id: String,
    pub status: HostStatus,
}

#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct ClusterStatusChanged {
    pub cluster_id: String,
    pub status: ClusterStatus,
}

/// A port forward failed, or ended because its target is gone.
#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct PortForwardChanged {
    pub event: ForwardEvent,
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
    /// The Host or Swarm it's deployed to.
    pub owner: StackOwner,
    pub stack_id: String,
    pub name: String,
}

/// A Synced Stack's automatic deploy ended; `error` says why it failed.
#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct SyncedAutoDeployed {
    /// The Host or Swarm it's deployed to.
    pub owner: StackOwner,
    pub stack_id: String,
    pub name: String,
    pub commit: String,
    pub error: Option<AppError>,
}

/// A Host was picked in the tray's menu: the window shows its Overview.
#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct TrayHostChosen {
    pub host_id: String,
}

/// **Add host…** was picked in the tray's menu.
#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
pub struct TrayAddHostChosen;

/// Forwards what the host registry observes to the webview, the tray and
/// the OS's notifications.
pub struct TauriNotifier {
    app: AppHandle,
    notifications: Notifications,
}

impl TauriNotifier {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            notifications: Notifications::default(),
        }
    }
}

impl Notifier for TauriNotifier {
    fn host_status(&self, host_id: &str, status: &HostStatus) {
        if matches!(status, HostStatus::Connected { .. }) {
            remove_leftover_helpers(&self.app, host_id);
        }
        let event = HostStatusChanged {
            host_id: host_id.to_owned(),
            status: status.clone(),
        };
        if let Err(e) = event.emit(&self.app) {
            log::warn!("failed to emit host status: {e}");
        }
        super::tray::refresh(&self.app);
        self.notifications.host_status(&self.app, host_id, status);
    }

    fn engine_event(&self, host_id: &str, event: &EngineEvent) {
        self.notifications.engine_event(&self.app, host_id, event);
        let event = EngineEventReceived {
            host_id: host_id.to_owned(),
            event: event.clone(),
        };
        if let Err(e) = event.emit(&self.app) {
            log::warn!("failed to emit engine event: {e}");
        }
    }
}

/// Forwards each Cluster's status changes to the webview.
pub struct TauriClusterNotifier {
    app: AppHandle,
}

impl TauriClusterNotifier {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl ClusterNotifier for TauriClusterNotifier {
    fn cluster_status(&self, cluster_id: &str, status: &ClusterStatus) {
        let event = ClusterStatusChanged {
            cluster_id: cluster_id.to_owned(),
            status: status.clone(),
        };
        if let Err(e) = event.emit(&self.app) {
            log::warn!("failed to emit cluster status: {e}");
        }
    }

    fn port_forward(&self, event: &ForwardEvent) {
        let event = PortForwardChanged {
            event: event.clone(),
        };
        if let Err(e) = event.emit(&self.app) {
            log::warn!("failed to emit a port forward's change: {e}");
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
