//! Swarms, found through their manager Hosts and reached through any
//! connected one (ADR 0016).

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::{
    containers::logs::{LogEvent, LogOptions},
    swarm::{
        self,
        nodes::{self, NodeChange, NodeView},
        services::{self, ServiceView},
        SwarmSummary,
    },
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

/// A Swarm's nodes, each with the Host it is, if it's one.
#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<SwarmNode>> {
    let nodes = nodes::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    let hosts = state.hosts.registry.node_hosts();
    Ok(nodes
        .into_iter()
        .map(|node| SwarmNode {
            host_id: hosts.get(&node.id).cloned(),
            node,
        })
        .collect())
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    nodes::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Changes a node's availability, role or labels, from the node as read at
/// `version`; refuses to demote the last manager or drain every node.
#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_update(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    version: u64,
    change: NodeChange,
) -> AppResult<()> {
    let docker = state.hosts.registry.manager(&swarm_id)?;
    nodes::check(&nodes::list(&docker).await?, &id, &change)?;
    nodes::update(&docker, &id, version, change).await?;
    refresh_members(&state).await;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_nodes_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    force: bool,
) -> AppResult<()> {
    nodes::remove(&state.hosts.registry.manager(&swarm_id)?, &id, force).await?;
    refresh_members(&state).await;
    Ok(())
}

/// A node of a Swarm, and the Host it is when Homeport connects to it.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmNode {
    #[serde(flatten)]
    pub node: NodeView,
    pub host_id: Option<String>,
}

/// Reads every Host's place in a Swarm again, after a change to its nodes.
async fn refresh_members(state: &AppState) {
    for host in state.hosts.store.list().unwrap_or_default() {
        state.hosts.registry.refresh_swarm(&host.id).await;
    }
}

/// Every service of a Swarm, with its running and desired tasks.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<ServiceView>> {
    services::list(&state.hosts.registry.manager(&swarm_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_services_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    services::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Sets a replicated service's replicas, from the service as read at `version`.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_scale(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    version: u64,
    replicas: u64,
) -> AppResult<()> {
    services::scale(
        &state.hosts.registry.manager(&swarm_id)?,
        &id,
        version,
        replicas,
    )
    .await
}

/// Puts back a service's previous spec.
#[tauri::command]
#[specta::specta]
pub async fn swarm_services_rollback(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::rollback(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_services_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::remove(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Streams the logs of every task of a service, each line labelled with its
/// task and node.
#[tauri::command]
#[specta::specta]
pub async fn swarm_service_logs_subscribe(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
    options: LogOptions,
    on_event: Channel<LogEvent>,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.manager(&swarm_id)?;
    Ok(state
        .streams
        .spawn(services::stream_logs(docker, id, options, move |e| {
            on_event.send(e).is_ok()
        })))
}
