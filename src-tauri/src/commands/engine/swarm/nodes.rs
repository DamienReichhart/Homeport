//! The nodes of a Swarm.

use tauri::State;

use homeport_engine::swarm::nodes::{self, NodeChange, NodeView};

use super::refresh_members;
use crate::{app::AppState, error::AppResult};

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
