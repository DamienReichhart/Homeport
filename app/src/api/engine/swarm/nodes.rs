//! The nodes of a Swarm.

use homeport_core::error::AppResult;
use homeport_engine::swarm::nodes::{self, NodeChange, NodeView};

use super::refresh_members;
use crate::app::AppState;

/// A Swarm's nodes, each with the Host it is, if it's one.
pub async fn swarm_nodes_list(state: &AppState, swarm_id: String) -> AppResult<Vec<SwarmNode>> {
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

pub async fn swarm_nodes_inspect(
    state: &AppState,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    nodes::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Changes a node's availability, role or labels, from the node as read at
/// `version`; refuses to demote the last manager or drain every node.
pub async fn swarm_nodes_update(
    state: &AppState,
    swarm_id: String,
    id: String,
    version: u64,
    change: NodeChange,
) -> AppResult<()> {
    let docker = state.hosts.registry.manager(&swarm_id)?;
    nodes::check(&nodes::list(&docker).await?, &id, &change)?;
    nodes::update(&docker, &id, version, change).await?;
    refresh_members(state).await;
    Ok(())
}

pub async fn swarm_nodes_remove(
    state: &AppState,
    swarm_id: String,
    id: String,
    force: bool,
) -> AppResult<()> {
    nodes::remove(&state.hosts.registry.manager(&swarm_id)?, &id, force).await?;
    refresh_members(state).await;
    Ok(())
}

/// A node of a Swarm, and the Host it is when Homeport connects to it.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwarmNode {
    #[serde(flatten)]
    pub node: NodeView,
    pub host_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, swarm};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_swarm_without_a_connected_manager_is_unreachable() {
        let app = app();
        let s = &app.state;
        let id = || "node1".to_owned();
        let change = NodeChange::default();
        let results = [
            ("list", code(swarm_nodes_list(s, swarm()).await)),
            ("inspect", code(swarm_nodes_inspect(s, swarm(), id()).await)),
            (
                "update",
                code(swarm_nodes_update(s, swarm(), id(), 1, change).await),
            ),
            (
                "remove",
                code(swarm_nodes_remove(s, swarm(), id(), true).await),
            ),
        ];
        assert_all(ErrorCode::SwarmUnreachable, &results);
    }
}
