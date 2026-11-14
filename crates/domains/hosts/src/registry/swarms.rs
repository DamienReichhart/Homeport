//! The Swarms the connected Hosts belong to, found through their manager Hosts
//! (ADR 0016).

use std::collections::HashMap;

use bollard::Docker;
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_engine::swarm::{self, SwarmMembership, SwarmRole};

use super::{Entry, HostRegistry, HostStatus};
use crate::messages as msg;
use crate::HostId;

impl HostRegistry {
    /// Updates a connected Host's place in a Swarm, telling the app when it changed.
    pub(super) fn set_swarm(&self, id: &str, membership: Option<SwarmMembership>) {
        let mut entries = self.lock();
        let Some(entry) = entries.get_mut(id) else {
            return;
        };
        if let HostStatus::Connected { swarm, .. } = &mut entry.status {
            if *swarm != membership {
                swarm.clone_from(&membership);
                entry.swarm = membership;
                // Told while the entry is held, as every status is.
                self.tell(id, &entry.status);
            }
        }
    }

    /// Reads a Host's place in a Swarm again, after Homeport changed it.
    pub async fn refresh_swarm(&self, id: &str) {
        if let Ok(docker) = self.docker(id) {
            match swarm::membership(&docker).await {
                Ok(membership) => self.set_swarm(id, membership),
                Err(e) => log::debug!("couldn't read the swarm membership of {id}: {e}"),
            }
        }
    }

    /// Every Swarm the Hosts belong to, one per swarm ID however many of
    /// its managers are Hosts. A worker is placed by its node ID, read from
    /// a connected manager's node list, or else by the managers it knows.
    pub async fn swarms(&self) -> Vec<SwarmView> {
        let order: Vec<HostId> = self
            .store
            .list()
            .map(|hosts| hosts.into_iter().map(|h| h.id).collect())
            .unwrap_or_default();
        let members: Vec<(HostId, SwarmMembership, bool)> = {
            let entries = self.lock();
            order
                .iter()
                .filter_map(|id| {
                    let entry = entries.get(id)?;
                    Some((id.clone(), entry.swarm.clone()?, entry.client.is_some()))
                })
                .collect()
        };
        let mut swarms = group_managers(&members);
        for view in &mut swarms {
            let nodes = match self.manager(&view.id) {
                Ok(docker) => swarm::node_ids(&docker).await.unwrap_or_else(|e| {
                    log::debug!("couldn't list the nodes of swarm {}: {e}", view.id);
                    vec![]
                }),
                Err(_) => vec![],
            };
            view.workers = workers_of(view, &members, &nodes);
        }
        swarms
    }

    /// The Host each Swarm node is, by node ID, for the nodes that are Hosts.
    pub fn node_hosts(&self) -> HashMap<String, HostId> {
        self.lock()
            .iter()
            .filter_map(|(id, e)| Some((e.swarm.as_ref()?.node_id.clone(), id.clone())))
            .collect()
    }

    /// The client of a connected manager of the Swarm `swarm_id`.
    pub fn manager(&self, swarm_id: &str) -> AppResult<Docker> {
        let id = self.manager_host(swarm_id)?;
        self.docker(&id)
    }

    /// A connected manager Host of the Swarm `swarm_id`, or why there's none.
    pub fn manager_host(&self, swarm_id: &str) -> AppResult<HostId> {
        let known: Vec<HostId> = {
            let entries = self.lock();
            let managers: Vec<(&HostId, &Entry)> = entries
                .iter()
                .filter(|(_, e)| {
                    e.swarm.as_ref().is_some_and(|m| {
                        m.role == SwarmRole::Manager && m.swarm_id.as_deref() == Some(swarm_id)
                    })
                })
                .collect();
            if let Some((id, _)) = managers.iter().find(|(_, e)| e.client.is_some()) {
                return Ok((*id).clone());
            }
            managers.into_iter().map(|(id, _)| id.clone()).collect()
        };
        let names: Vec<String> = known
            .iter()
            .filter_map(|id| self.store.get(id).ok().map(|h| h.name))
            .collect();
        let error = AppError::new(ErrorCode::SwarmUnreachable, msg::SWARM_UNREACHABLE);
        Err(if names.is_empty() {
            error.with_hint(msg::SWARM_UNREACHABLE_UNKNOWN_HINT)
        } else {
            error.with_hint(msg::SWARM_UNREACHABLE_HINT.with("hosts", names.join(", ")))
        })
    }
}

/// A Swarm, found through the Hosts that belong to it (ADR 0016).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SwarmView {
    pub id: String,
    /// Its manager Hosts, in the Hosts' order.
    pub managers: Vec<HostId>,
    /// Its worker Hosts, in the Hosts' order.
    pub workers: Vec<HostId>,
    /// Whether one of its managers is connected, so it can be managed.
    pub reachable: bool,
}

/// The Swarms of the manager Hosts among `members`, by swarm ID, in the Hosts' order.
pub(super) fn group_managers(members: &[(HostId, SwarmMembership, bool)]) -> Vec<SwarmView> {
    let mut swarms: Vec<SwarmView> = Vec::new();
    for (id, membership, connected) in members {
        let Some(swarm_id) = membership
            .swarm_id
            .as_ref()
            .filter(|_| membership.role == SwarmRole::Manager)
        else {
            continue;
        };
        let index = match swarms.iter().position(|s| &s.id == swarm_id) {
            Some(index) => index,
            None => {
                swarms.push(SwarmView {
                    id: swarm_id.clone(),
                    managers: vec![],
                    workers: vec![],
                    reachable: false,
                });
                swarms.len() - 1
            }
        };
        swarms[index].managers.push(id.clone());
        swarms[index].reachable |= connected;
    }
    swarms
}

/// The worker Hosts among `members` that belong to `swarm`: their node is
/// in its `nodes`, or they talk to one of its manager Hosts.
pub(super) fn workers_of(
    swarm: &SwarmView,
    members: &[(HostId, SwarmMembership, bool)],
    nodes: &[String],
) -> Vec<HostId> {
    let manager_nodes: Vec<&str> = members
        .iter()
        .filter(|(id, _, _)| swarm.managers.contains(id))
        .map(|(_, m, _)| m.node_id.as_str())
        .collect();
    members
        .iter()
        .filter(|(_, m, _)| m.role == SwarmRole::Worker)
        .filter(|(_, m, _)| {
            nodes.contains(&m.node_id)
                || m.managers
                    .iter()
                    .any(|p| manager_nodes.contains(&p.node_id.as_str()))
        })
        .map(|(id, _, _)| id.clone())
        .collect()
}
