//! A Swarm's nodes: listing them, setting their availability, role and
//! labels, and removing them, through a manager.

use std::collections::HashMap;

use bollard::{
    models::{Node, NodeSpec, NodeSpecAvailabilityEnum, NodeSpecRoleEnum, NodeState, Reachability},
    query_parameters::{DeleteNodeOptionsBuilder, ListNodesOptions, UpdateNodeOptionsBuilder},
    Docker,
};
use serde::{Deserialize, Serialize};

use homeport_core::error::{engine::from_bollard, AppError, AppResult, ErrorCode};

use super::SwarmRole;
use crate::messages as msg;

/// Whether the scheduler may place tasks on a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Availability {
    /// Runs new tasks.
    Active,
    /// Keeps its tasks, gets no new ones.
    Pause,
    /// Its tasks move to other nodes.
    Drain,
}

/// How a manager node takes part in the Swarm's Raft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ManagerStatus {
    Leader,
    Reachable,
    Unreachable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NodeView {
    pub id: String,
    /// The node's `Version.Index`, sent back with an update.
    pub version: u64,
    pub hostname: String,
    pub role: SwarmRole,
    pub availability: Availability,
    /// `ready`, `down`, `unknown` or `disconnected`.
    pub state: String,
    /// Why the node is in its state, in the Engine's words.
    pub message: Option<String>,
    /// `None` for workers.
    pub manager_status: Option<ManagerStatus>,
    pub engine_version: Option<String>,
    pub address: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub labels: HashMap<String, String>,
    pub created: Option<String>,
}

impl NodeView {
    pub fn is_ready(&self) -> bool {
        self.state == "ready"
    }

    /// A manager that takes part in the Swarm's Raft right now.
    pub fn is_reachable_manager(&self) -> bool {
        matches!(
            self.manager_status,
            Some(ManagerStatus::Leader | ManagerStatus::Reachable)
        )
    }
}

/// What to change on a node; `None` leaves that part as it is.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NodeChange {
    pub availability: Option<Availability>,
    /// Promote to manager or demote to worker.
    pub role: Option<NodeRole>,
    /// Replaces every label.
    pub labels: Option<HashMap<String, String>>,
}

/// [`SwarmRole`] as an input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum NodeRole {
    Manager,
    Worker,
}

pub fn view(node: Node) -> NodeView {
    let spec = node.spec.unwrap_or_default();
    let description = node.description.unwrap_or_default();
    let status = node.status.unwrap_or_default();
    let platform = description.platform.unwrap_or_default();
    let non_empty = |s: Option<String>| s.filter(|s| !s.is_empty());
    NodeView {
        id: node.id.unwrap_or_default(),
        version: node.version.and_then(|v| v.index).unwrap_or(0),
        hostname: description.hostname.unwrap_or_default(),
        role: match spec.role {
            Some(NodeSpecRoleEnum::MANAGER) => SwarmRole::Manager,
            _ => SwarmRole::Worker,
        },
        availability: match spec.availability {
            Some(NodeSpecAvailabilityEnum::PAUSE) => Availability::Pause,
            Some(NodeSpecAvailabilityEnum::DRAIN) => Availability::Drain,
            _ => Availability::Active,
        },
        state: status
            .state
            .map_or_else(|| NodeState::UNKNOWN.to_string(), |s| s.to_string()),
        message: non_empty(status.message),
        manager_status: node.manager_status.map(|m| {
            if m.leader == Some(true) {
                ManagerStatus::Leader
            } else if m.reachability == Some(Reachability::REACHABLE) {
                ManagerStatus::Reachable
            } else {
                ManagerStatus::Unreachable
            }
        }),
        engine_version: non_empty(description.engine.and_then(|e| e.engine_version)),
        address: non_empty(status.addr),
        os: non_empty(platform.os),
        arch: non_empty(platform.architecture),
        labels: spec.labels.unwrap_or_default(),
        created: node.created_at.map(|d| d.to_string()),
    }
}

pub async fn list(docker: &Docker) -> AppResult<Vec<NodeView>> {
    let mut nodes: Vec<NodeView> = docker
        .list_nodes(None::<ListNodesOptions>)
        .await?
        .into_iter()
        .map(view)
        .collect();
    nodes.sort_by(|a, b| {
        (a.role != SwarmRole::Manager, &a.hostname)
            .cmp(&(b.role != SwarmRole::Manager, &b.hostname))
    });
    Ok(nodes)
}

pub async fn inspect(docker: &Docker, id: &str) -> AppResult<serde_json::Value> {
    Ok(serde_json::to_value(docker.inspect_node(id).await?)?)
}

/// Refuses a change that would leave the Swarm without a reachable manager,
/// or without a node to run its tasks, before anything is sent.
pub fn check(nodes: &[NodeView], id: &str, change: &NodeChange) -> AppResult<()> {
    let node = nodes
        .iter()
        .find(|n| n.id == id)
        .ok_or_else(|| AppError::not_found(msg::NODE_GONE))?;
    let others = || nodes.iter().filter(move |n| n.id != id);
    if change.role == Some(NodeRole::Worker)
        && node.role == SwarmRole::Manager
        && !others().any(NodeView::is_reachable_manager)
    {
        return Err(
            AppError::validation(msg::NODE_LAST_MANAGER).with_hint(msg::NODE_LAST_MANAGER_HINT)
        );
    }
    if change.availability == Some(Availability::Drain)
        && node.availability != Availability::Drain
        && !others().any(|n| n.is_ready() && n.availability == Availability::Active)
    {
        return Err(AppError::validation(msg::NODE_LAST_ACTIVE));
    }
    if let Some(labels) = &change.labels {
        if labels.keys().any(|k| k.trim().is_empty()) {
            return Err(AppError::validation(msg::NODE_LABEL_KEY));
        }
    }
    Ok(())
}

/// Applies `change` to the node `id`, read at `version`: a node updated
/// since fails with a conflict rather than being overwritten.
pub async fn update(docker: &Docker, id: &str, version: u64, change: NodeChange) -> AppResult<()> {
    let current = docker.inspect_node(id).await?;
    let mut spec: NodeSpec = current.spec.unwrap_or_default();
    if let Some(availability) = change.availability {
        spec.availability = Some(match availability {
            Availability::Active => NodeSpecAvailabilityEnum::ACTIVE,
            Availability::Pause => NodeSpecAvailabilityEnum::PAUSE,
            Availability::Drain => NodeSpecAvailabilityEnum::DRAIN,
        });
    }
    if let Some(role) = change.role {
        spec.role = Some(match role {
            NodeRole::Manager => NodeSpecRoleEnum::MANAGER,
            NodeRole::Worker => NodeSpecRoleEnum::WORKER,
        });
    }
    if let Some(labels) = change.labels {
        spec.labels = Some(labels);
    }
    let options = UpdateNodeOptionsBuilder::default()
        .version(i64::try_from(version).unwrap_or(i64::MAX))
        .build();
    docker
        .update_node(id, spec, options)
        .await
        .map_err(|e| from_bollard(&e))
}

/// Removes the node `id` from the Swarm; `force` removes one that's still up.
pub async fn remove(docker: &Docker, id: &str, force: bool) -> AppResult<()> {
    docker
        .delete_node(
            id,
            Some(DeleteNodeOptionsBuilder::default().force(force).build()),
        )
        .await
        .map_err(|e| removal_error(&e))
}

fn removal_error(e: &bollard::errors::Error) -> AppError {
    let text = e.to_string().to_lowercase();
    if text.contains("must be demoted") {
        return AppError::new(ErrorCode::Conflict, msg::NODE_REMOVE_MANAGER).with_cause(e);
    }
    if text.contains("is not down") {
        return AppError::new(ErrorCode::Conflict, msg::NODE_REMOVE_READY)
            .with_hint(msg::NODE_REMOVE_READY_HINT)
            .with_cause(e);
    }
    from_bollard(e)
}

#[cfg(test)]
mod tests;
