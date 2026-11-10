//! Port forwards from this computer to a Cluster's Pods and Services.
//! They listen on loopback only and end when Homeport quits.

use tauri::State;

use homeport_kubernetes::forward::{self, ForwardId, ForwardTarget, PortForward};

use super::on;
use crate::{app::AppState, error::AppResult};

/// Forwards `local_port` (a free one without) to `remote_port` of a Pod or Service.
#[tauri::command]
#[specta::specta]
pub async fn clusters_forward_start(
    state: State<'_, AppState>,
    id: String,
    target: ForwardTarget,
    remote_port: u16,
    local_port: Option<u16>,
) -> AppResult<PortForward> {
    let on = on(&state, &id)?;
    state
        .clusters
        .forwards
        .start(on, target, remote_port, local_port)
        .await
}

/// Every active forward, across Clusters.
#[tauri::command]
#[specta::specta]
pub fn clusters_forward_list(state: State<'_, AppState>) -> Vec<PortForward> {
    state.clusters.forwards.list()
}

#[tauri::command]
#[specta::specta]
pub fn clusters_forward_stop(state: State<'_, AppState>, forward: ForwardId) {
    state.clusters.forwards.stop(forward);
}

/// Whether `port` is free on this computer, to propose it as the local port.
#[tauri::command]
#[specta::specta]
pub async fn clusters_forward_port_free(port: u16) -> bool {
    forward::is_free(port).await
}
