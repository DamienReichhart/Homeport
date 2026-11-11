//! Swarms, found through their manager Hosts and reached through any
//! connected one (ADR 0016).

pub mod nodes;
pub mod objects;
pub mod services;
pub mod stacks;

use tauri::State;

use homeport_engine::swarm::{
    self,
    membership::{self, JoinInfo, JoinRole},
    SwarmSummary,
};
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

/// Reads every Host's place in a Swarm again, after a change to its nodes.
async fn refresh_members(state: &AppState) {
    for host in state.hosts.store.list().unwrap_or_default() {
        state.hosts.registry.refresh_swarm(&host.id).await;
    }
}

/// Makes `host_id` the first manager of a new Swarm, returning its ID.
#[tauri::command]
#[specta::specta]
pub async fn swarm_init(
    state: State<'_, AppState>,
    host_id: String,
    advertise_addr: Option<String>,
    listen_addr: Option<String>,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let id = membership::init(&docker, advertise_addr.as_deref(), listen_addr.as_deref()).await?;
    state.hosts.registry.refresh_swarm(&host_id).await;
    Ok(id)
}

/// The Swarm a Host joins: one Homeport knows, whose token and manager
/// addresses are fetched through a connected manager, or a pasted token.
#[derive(Clone, serde::Deserialize, specta::Type)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum JoinTarget {
    Known {
        swarm_id: String,
    },
    Token {
        token: String,
        addresses: Vec<String>,
    },
}

impl std::fmt::Debug for JoinTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known { swarm_id } => write!(f, "Known {{ swarm_id: {swarm_id:?} }}"),
            Self::Token { addresses, .. } => write!(f, "Token {{ addresses: {addresses:?} }}"),
        }
    }
}

/// Joins `host_id` to a Swarm as a worker or a manager.
#[tauri::command]
#[specta::specta]
pub async fn swarm_join(
    state: State<'_, AppState>,
    host_id: String,
    target: JoinTarget,
    role: JoinRole,
    advertise_addr: Option<String>,
) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let (token, addresses) = match target {
        JoinTarget::Known { swarm_id } => {
            let info = membership::join_info(&state.hosts.registry.manager(&swarm_id)?).await?;
            (info.token(role).to_owned(), info.manager_addrs)
        }
        JoinTarget::Token { token, addresses } => (token, addresses),
    };
    let joined = membership::join(&docker, &addresses, &token, advertise_addr.as_deref()).await;
    state.hosts.registry.refresh_swarm(&host_id).await;
    joined
}

/// Has `host_id` leave its Swarm; a manager needs `force`.
#[tauri::command]
#[specta::specta]
pub async fn swarm_leave(
    state: State<'_, AppState>,
    host_id: String,
    force: bool,
) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    membership::leave(&docker, force).await?;
    state.hosts.registry.refresh_swarm(&host_id).await;
    Ok(())
}

/// The join tokens and manager addresses of a Swarm, for its Join commands;
/// never stored or logged.
#[tauri::command]
#[specta::specta]
pub async fn swarm_join_info(state: State<'_, AppState>, swarm_id: String) -> AppResult<JoinInfo> {
    membership::join_info(&state.hosts.registry.manager(&swarm_id)?).await
}
