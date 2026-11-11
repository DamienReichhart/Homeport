//! Keys of the Swarms and of what each one holds, whichever manager Host
//! answers (`swarmKeys` in React).

use crate::data::Key;

/// The Swarms found through the connected manager Hosts.
pub fn list() -> Key {
    Key::new(["swarms"])
}

/// Everything read from one Swarm.
pub fn scope(swarm_id: &str) -> Key {
    Key::new(["swarm", swarm_id])
}

pub fn summary(swarm_id: &str) -> Key {
    scope(swarm_id).with("summary")
}

pub fn nodes(swarm_id: &str) -> Key {
    scope(swarm_id).with("nodes")
}

pub fn services(swarm_id: &str) -> Key {
    scope(swarm_id).with("services")
}

pub fn tasks(swarm_id: &str) -> Key {
    scope(swarm_id).with("tasks")
}

pub fn secrets(swarm_id: &str) -> Key {
    scope(swarm_id).with("secrets")
}

pub fn configs(swarm_id: &str) -> Key {
    scope(swarm_id).with("configs")
}

pub fn stacks(swarm_id: &str) -> Key {
    scope(swarm_id).with("stacks")
}

/// The networks a service of the Swarm may join.
pub fn networks(swarm_id: &str) -> Key {
    scope(swarm_id).with("networks")
}

/// The tokens and address another Host joins the Swarm with.
pub fn join_info(swarm_id: &str) -> Key {
    scope(swarm_id).with("join-info")
}
