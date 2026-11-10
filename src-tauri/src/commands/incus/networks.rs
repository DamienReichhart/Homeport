//! Incus networks.

use tauri::State;

use homeport_incus::networks::{self, BridgeRequest, IncusNetwork, IncusNetworkState};

use crate::{app::AppState, error::AppResult};

/// The networks `project` sees, managed ones first.
#[tauri::command]
#[specta::specta]
pub async fn incus_networks(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusNetwork>> {
    networks::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// A network's addresses on the host and its DHCP leases.
#[tauri::command]
#[specta::specta]
pub async fn incus_network_state(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusNetworkState> {
    networks::state(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_bridge_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    request: BridgeRequest,
) -> AppResult<()> {
    networks::create_bridge(&state.hosts.registry.incus(&host_id)?, &project, &request).await
}

/// Deletes a managed network nothing uses.
#[tauri::command]
#[specta::specta]
pub async fn incus_network_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    networks::delete(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}
