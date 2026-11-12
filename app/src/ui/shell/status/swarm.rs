//! A Swarm's name (`src/core/swarm/model/naming.ts`).

use homeport_hosts::{registry::SwarmView, service::HostView};

use crate::t;

/// The first characters of a swarm ID, as Docker shows IDs.
pub fn short_id(id: &str) -> &str {
    id.char_indices().nth(12).map_or(id, |(at, _)| &id[..at])
}

/// A Swarm's name. Swarms have none of their own (`Spec.Name` is always
/// `default`), so it's named after its first manager Host.
pub fn label(swarm: &SwarmView, hosts: &[HostView]) -> String {
    let first = swarm
        .managers
        .iter()
        .find_map(|id| hosts.iter().find(|view| &view.host.id == id))
        .map(|view| view.host.name.as_str());
    match first {
        Some(host) => t!("core.swarm.label", host = host),
        None => t!("core.swarm.labelUnknown", id = short_id(&swarm.id)),
    }
}

/// The Swarm a Host belongs to among `swarms`, as a manager or a worker
/// (`swarmOfHost`).
pub fn of_host<'a>(swarms: &'a [SwarmView], host_id: &str) -> Option<&'a SwarmView> {
    swarms.iter().find(|swarm| {
        let member = |id: &String| id == host_id;
        swarm.managers.iter().any(member) || swarm.workers.iter().any(member)
    })
}
