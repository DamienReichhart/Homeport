//! Networks on a Host.

use homeport_core::error::AppResult;
use homeport_engine::{
    images::PruneReport,
    networks::{self, NetworkInput, NetworkView},
};

use crate::app::AppState;

pub async fn networks_list(state: &AppState, host_id: String) -> AppResult<Vec<NetworkView>> {
    networks::list(&state.hosts.registry.docker(&host_id)?).await
}

pub async fn networks_inspect(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    networks::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

pub async fn networks_create(
    state: &AppState,
    host_id: String,
    input: NetworkInput,
) -> AppResult<String> {
    networks::create(&state.hosts.registry.docker(&host_id)?, input).await
}

pub async fn networks_remove(state: &AppState, host_id: String, id: String) -> AppResult<()> {
    networks::remove(&state.hosts.registry.docker(&host_id)?, &id).await
}

pub async fn networks_connect(
    state: &AppState,
    host_id: String,
    network: String,
    container: String,
) -> AppResult<()> {
    networks::connect(
        &state.hosts.registry.docker(&host_id)?,
        &network,
        &container,
    )
    .await
}

pub async fn networks_disconnect(
    state: &AppState,
    host_id: String,
    network: String,
    container: String,
    force: bool,
) -> AppResult<()> {
    networks::disconnect(
        &state.hosts.registry.docker(&host_id)?,
        &network,
        &container,
        force,
    )
    .await
}

pub async fn networks_prune(state: &AppState, host_id: String) -> AppResult<PruneReport> {
    networks::prune(&state.hosts.registry.docker(&host_id)?).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, host};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_host_that_isnt_connected_is_refused() {
        let app = app();
        let s = &app.state;
        let net = || "net".to_owned();
        let c = || "c1".to_owned();
        let input = NetworkInput {
            name: net(),
            driver: None,
            subnet: None,
            gateway: None,
            internal: false,
            attachable: true,
        };
        let results = [
            ("list", code(networks_list(s, host()).await)),
            ("inspect", code(networks_inspect(s, host(), net()).await)),
            ("create", code(networks_create(s, host(), input).await)),
            ("remove", code(networks_remove(s, host(), net()).await)),
            (
                "connect",
                code(networks_connect(s, host(), net(), c()).await),
            ),
            (
                "disconnect",
                code(networks_disconnect(s, host(), net(), c(), true).await),
            ),
            ("prune", code(networks_prune(s, host()).await)),
        ];
        assert_all(ErrorCode::HostNotConnected, &results);
    }
}
