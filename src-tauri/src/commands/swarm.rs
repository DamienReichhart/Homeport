//! Swarms, found through their manager Hosts and reached through any
//! connected one (ADR 0016).

use tauri::State;

use homeport_engine::swarm::{self, SwarmSummary};
use homeport_hosts::registry::SwarmView;

use crate::{app::AppState, error::AppResult};

/// Every Swarm the Hosts belong to, once each.
#[tauri::command]
#[specta::specta]
pub async fn swarms_list(state: State<'_, AppState>) -> AppResult<Vec<SwarmView>> {
    Ok(state.hosts.registry.swarms().await)
}

/// What a connected manager says about the Swarm `swarm_id`.
#[tauri::command]
#[specta::specta]
pub async fn swarm_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<SwarmSummary> {
    swarm::summary(&state.hosts.registry.manager(&swarm_id)?).await
}
