//! Incus networks.

use homeport_core::error::AppResult;
use homeport_incus::networks::{self, BridgeRequest, IncusNetwork, IncusNetworkState};

use crate::app::AppState;

/// The networks `project` sees, managed ones first.
pub async fn incus_networks(
    state: &AppState,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusNetwork>> {
    networks::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// A network's addresses on the host and its DHCP leases.
pub async fn incus_network_state(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusNetworkState> {
    networks::state(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

pub async fn incus_bridge_create(
    state: &AppState,
    host_id: String,
    project: String,
    request: BridgeRequest,
) -> AppResult<()> {
    networks::create_bridge(&state.hosts.registry.incus(&host_id)?, &project, &request).await
}

/// Deletes a managed network nothing uses.
pub async fn incus_network_delete(
    state: &AppState,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    networks::delete(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::incus::tests::{not_connected, NO_HOST},
        app::testing,
    };

    fn host() -> String {
        NO_HOST.to_owned()
    }

    fn project() -> String {
        "default".to_owned()
    }

    #[tokio::test]
    async fn networks_need_a_connected_host() {
        let app = testing::app();
        let state = &app.state;
        let name = || "incusbr1".to_owned();
        not_connected(incus_networks(state, host(), project()).await);
        not_connected(incus_network_state(state, host(), project(), name()).await);
        let request = BridgeRequest {
            name: name(),
            ipv4: "auto".into(),
            ipv6: "none".into(),
            nat: true,
            description: String::new(),
        };
        not_connected(incus_bridge_create(state, host(), project(), request).await);
        not_connected(incus_network_delete(state, host(), project(), name()).await);
    }
}
