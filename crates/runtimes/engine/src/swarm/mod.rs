//! Swarm mode, read and driven through the Engine API of a Host that belongs
//! to the Swarm (ADR 0016). Every function here takes the client of one
//! node: [`membership`] works on any node; the others need a manager, which
//! `homeport_hosts::registry::HostRegistry::manager` hands out.

pub mod membership;
pub mod nodes;
pub mod objects;
pub mod services;
pub mod spec;
pub mod tasks;

use bollard::{
    models::{LocalNodeState, SystemInfo},
    query_parameters::ListServicesOptions,
    Docker,
};
use serde::Serialize;

use homeport_core::error::AppResult;

/// Whether a node manages the Swarm or only runs its tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SwarmRole {
    Manager,
    Worker,
}

/// Where a node stands in its Swarm, as its Engine reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SwarmNodeState {
    Active,
    /// Joining, or a manager waiting for the others.
    Pending,
    /// Encrypted at rest and waiting for its unlock key.
    Locked,
    Error,
}

/// A Host's place in a Swarm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmMembership {
    /// The Swarm's ID. Only managers know it; a worker's Swarm is found
    /// through the managers it talks to.
    pub swarm_id: Option<String>,
    /// This node's ID in the Swarm.
    pub node_id: String,
    pub role: SwarmRole,
    pub state: SwarmNodeState,
    /// The address other nodes reach this one at.
    pub node_addr: Option<String>,
    /// The node IDs and addresses of the managers this node knows.
    pub managers: Vec<SwarmPeer>,
    /// Why the node is in error, in the Engine's words.
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmPeer {
    pub node_id: String,
    pub addr: String,
}

/// What a manager says about its Swarm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmSummary {
    pub id: String,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub nodes: u32,
    pub managers: u32,
    pub services: u32,
}

/// The Host's place in a Swarm, `None` when it isn't in one.
pub async fn membership(docker: &Docker) -> AppResult<Option<SwarmMembership>> {
    Ok(membership_from(&docker.info().await?))
}

/// [`membership`] from the Engine's `/info`.
pub fn membership_from(info: &SystemInfo) -> Option<SwarmMembership> {
    let swarm = info.swarm.as_ref()?;
    let state = match swarm.local_node_state.unwrap_or_default() {
        LocalNodeState::EMPTY | LocalNodeState::INACTIVE => return None,
        LocalNodeState::PENDING => SwarmNodeState::Pending,
        LocalNodeState::ACTIVE => SwarmNodeState::Active,
        LocalNodeState::ERROR => SwarmNodeState::Error,
        LocalNodeState::LOCKED => SwarmNodeState::Locked,
    };
    let manager = swarm.control_available.unwrap_or(false);
    let non_empty = |s: &Option<String>| s.clone().filter(|s| !s.is_empty());
    Some(SwarmMembership {
        swarm_id: swarm.cluster.as_ref().and_then(|c| non_empty(&c.id)),
        node_id: swarm.node_id.clone().unwrap_or_default(),
        role: if manager {
            SwarmRole::Manager
        } else {
            SwarmRole::Worker
        },
        state,
        node_addr: non_empty(&swarm.node_addr),
        managers: swarm
            .remote_managers
            .iter()
            .flatten()
            .filter_map(|peer| {
                Some(SwarmPeer {
                    node_id: peer.node_id.clone()?,
                    addr: peer.addr.clone().unwrap_or_default(),
                })
            })
            .collect(),
        error: non_empty(&swarm.error),
    })
}

/// The Swarm a manager belongs to, with its node and service counts.
pub async fn summary(docker: &Docker) -> AppResult<SwarmSummary> {
    let info = docker.info().await?;
    let swarm = info.swarm.unwrap_or_default();
    let cluster = swarm.cluster.unwrap_or_default();
    let services = docker
        .list_services(None::<ListServicesOptions>)
        .await?
        .len();
    let count = |n: Option<i64>| u32::try_from(n.unwrap_or(0)).unwrap_or(0);
    Ok(SwarmSummary {
        id: cluster.id.unwrap_or_default(),
        created: cluster.created_at.map(|d| d.to_string()),
        updated: cluster.updated_at.map(|d| d.to_string()),
        nodes: count(swarm.nodes),
        managers: count(swarm.managers),
        services: u32::try_from(services).unwrap_or(u32::MAX),
    })
}

/// The IDs of every node of the Swarm, read through a manager.
pub async fn node_ids(docker: &Docker) -> AppResult<Vec<String>> {
    Ok(docker
        .list_nodes(None::<bollard::query_parameters::ListNodesOptions>)
        .await?
        .into_iter()
        .filter_map(|n| n.id)
        .collect())
}

#[cfg(test)]
mod tests;
