//! A Swarm's secrets, configs and overlay networks.

use homeport_core::error::AppResult;
use homeport_engine::{
    networks::{self, NetworkView},
    swarm::objects::{self, ConfigContent, ObjectInput, SwarmObject},
};

use crate::app::AppState;

/// A Swarm's secrets, never their values.
pub async fn swarm_secrets_list(state: &AppState, swarm_id: String) -> AppResult<Vec<SwarmObject>> {
    objects::secrets(&state.hosts.registry.manager(&swarm_id)?).await
}

/// Sends a new secret's value to the Swarm once; Homeport keeps none of it.
pub async fn swarm_secrets_create(
    state: &AppState,
    swarm_id: String,
    input: ObjectInput,
) -> AppResult<String> {
    objects::create_secret(&state.hosts.registry.manager(&swarm_id)?, input).await
}

pub async fn swarm_secrets_remove(state: &AppState, swarm_id: String, id: String) -> AppResult<()> {
    objects::remove_secret(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

pub async fn swarm_configs_list(state: &AppState, swarm_id: String) -> AppResult<Vec<SwarmObject>> {
    objects::configs(&state.hosts.registry.manager(&swarm_id)?).await
}

pub async fn swarm_configs_create(
    state: &AppState,
    swarm_id: String,
    input: ObjectInput,
) -> AppResult<String> {
    objects::create_config(&state.hosts.registry.manager(&swarm_id)?, input).await
}

/// A config's content, as text when it is.
pub async fn swarm_configs_inspect(
    state: &AppState,
    swarm_id: String,
    id: String,
) -> AppResult<ConfigContent> {
    objects::config_content(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

pub async fn swarm_configs_remove(state: &AppState, swarm_id: String, id: String) -> AppResult<()> {
    objects::remove_config(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// The Swarm's overlay networks, which services attach to.
pub async fn swarm_networks_list(
    state: &AppState,
    swarm_id: String,
) -> AppResult<Vec<NetworkView>> {
    let all = networks::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    Ok(all.into_iter().filter(|n| n.driver == "overlay").collect())
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, swarm};
    use crate::app::testing::app;

    fn input() -> ObjectInput {
        ObjectInput {
            name: "token".into(),
            value: objects::ObjectValue::Text {
                text: "value".into(),
            },
            labels: std::collections::HashMap::new(),
        }
    }

    #[tokio::test]
    async fn a_swarm_without_a_connected_manager_is_unreachable() {
        let app = app();
        let s = &app.state;
        let id = || "o1".to_owned();
        let results = [
            ("secrets_list", code(swarm_secrets_list(s, swarm()).await)),
            (
                "secrets_create",
                code(swarm_secrets_create(s, swarm(), input()).await),
            ),
            (
                "secrets_remove",
                code(swarm_secrets_remove(s, swarm(), id()).await),
            ),
            ("configs_list", code(swarm_configs_list(s, swarm()).await)),
            (
                "configs_create",
                code(swarm_configs_create(s, swarm(), input()).await),
            ),
            (
                "configs_inspect",
                code(swarm_configs_inspect(s, swarm(), id()).await),
            ),
            (
                "configs_remove",
                code(swarm_configs_remove(s, swarm(), id()).await),
            ),
            ("networks_list", code(swarm_networks_list(s, swarm()).await)),
        ];
        assert_all(ErrorCode::SwarmUnreachable, &results);
    }
}
