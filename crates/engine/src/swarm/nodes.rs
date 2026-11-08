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
mod tests {
    use bollard::models::{ManagerStatus as Bm, NodeDescription, NodeStatus, ObjectVersion};

    use super::*;

    fn node(id: &str, role: SwarmRole, availability: Availability, state: &str) -> NodeView {
        NodeView {
            id: id.into(),
            version: 1,
            hostname: id.into(),
            role,
            availability,
            state: state.into(),
            message: None,
            manager_status: (role == SwarmRole::Manager).then_some(ManagerStatus::Reachable),
            engine_version: None,
            address: None,
            os: None,
            arch: None,
            labels: HashMap::new(),
            created: None,
        }
    }

    #[test]
    fn maps_a_leader_node() {
        let v = view(Node {
            id: Some("n1".into()),
            version: Some(ObjectVersion { index: Some(42) }),
            spec: Some(NodeSpec {
                role: Some(NodeSpecRoleEnum::MANAGER),
                availability: Some(NodeSpecAvailabilityEnum::DRAIN),
                labels: Some([("zone".to_string(), "a".to_string())].into()),
                ..Default::default()
            }),
            description: Some(NodeDescription {
                hostname: Some("alpha".into()),
                ..Default::default()
            }),
            status: Some(NodeStatus {
                state: Some(NodeState::READY),
                addr: Some("10.0.0.1".into()),
                ..Default::default()
            }),
            manager_status: Some(Bm {
                leader: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        });
        assert_eq!(v.version, 42);
        assert_eq!(v.role, SwarmRole::Manager);
        assert_eq!(v.availability, Availability::Drain);
        assert_eq!(v.state, "ready");
        assert_eq!(v.manager_status, Some(ManagerStatus::Leader));
        assert_eq!(v.labels["zone"], "a");
        let worker = view(Node::default());
        assert_eq!(worker.role, SwarmRole::Worker);
        assert_eq!(worker.manager_status, None);
        assert_eq!(worker.state, "unknown");
    }

    #[test]
    fn the_last_reachable_manager_cant_be_demoted() {
        use SwarmRole::{Manager, Worker};
        let demote = NodeChange {
            role: Some(NodeRole::Worker),
            ..Default::default()
        };
        let mut nodes = vec![
            node("m1", Manager, Availability::Active, "ready"),
            node("w1", Worker, Availability::Active, "ready"),
        ];
        assert!(matches!(check(&nodes, "m1", &demote), Err(e) if e.is(ErrorCode::Validation)));
        nodes.push(node("m2", Manager, Availability::Active, "ready"));
        assert!(check(&nodes, "m1", &demote).is_ok());
        nodes[2].manager_status = Some(ManagerStatus::Unreachable);
        assert!(check(&nodes, "m1", &demote).is_err());
    }

    #[test]
    fn draining_every_node_is_refused() {
        use SwarmRole::{Manager, Worker};
        let drain = NodeChange {
            availability: Some(Availability::Drain),
            ..Default::default()
        };
        let nodes = vec![
            node("m1", Manager, Availability::Active, "ready"),
            node("w1", Worker, Availability::Drain, "ready"),
            node("w2", Worker, Availability::Active, "down"),
        ];
        assert!(check(&nodes, "m1", &drain).is_err());
        assert!(check(&nodes, "w1", &drain).is_ok(), "already drained");
        assert!(check(&nodes, "w2", &drain).is_ok());
        assert!(check(&nodes, "gone", &drain).is_err());
    }

    #[test]
    fn labels_need_a_key() {
        let nodes = vec![node(
            "m1",
            SwarmRole::Manager,
            Availability::Active,
            "ready",
        )];
        let change = NodeChange {
            labels: Some([(" ".to_string(), "x".to_string())].into()),
            ..Default::default()
        };
        assert!(check(&nodes, "m1", &change).is_err());
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn the_single_node_drains_and_comes_back() {
        let docker = crate::testing::docker();
        let _swarm = crate::testing::swarm(&docker).await;
        let node = list(&docker).await.unwrap().remove(0);
        // Pausing the only node is allowed; draining it wouldn't be.
        let drain = NodeChange {
            availability: Some(Availability::Pause),
            labels: Some([("homeport.test".to_string(), "1".to_string())].into()),
            ..Default::default()
        };
        check(std::slice::from_ref(&node), &node.id, &drain).unwrap();
        update(&docker, &node.id, node.version, drain)
            .await
            .unwrap();
        let paused = list(&docker).await.unwrap().remove(0);
        assert_eq!(paused.availability, Availability::Pause);
        assert_eq!(paused.labels["homeport.test"], "1");
        // A stale version is a conflict.
        let stale = update(&docker, &node.id, node.version, NodeChange::default()).await;
        assert!(
            matches!(&stale, Err(e) if e.is(ErrorCode::Conflict)),
            "{stale:?}"
        );
        let back = NodeChange {
            availability: Some(Availability::Active),
            labels: Some(HashMap::new()),
            ..Default::default()
        };
        update(&docker, &node.id, paused.version, back)
            .await
            .unwrap();
        assert_eq!(
            list(&docker).await.unwrap()[0].availability,
            Availability::Active
        );
    }
}
