//! A Swarm's secrets, configs and overlay networks.

use tauri::State;

use homeport_engine::{
    networks::{self, NetworkView},
    swarm::objects::{self, ConfigContent, ObjectInput, SwarmObject},
};

use crate::{app::AppState, error::AppResult};

/// A Swarm's secrets, never their values.
#[tauri::command]
#[specta::specta]
pub async fn swarm_secrets_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<SwarmObject>> {
    objects::secrets(&state.hosts.registry.manager(&swarm_id)?).await
}

/// Sends a new secret's value to the Swarm once; Homeport keeps none of it.
#[tauri::command]
#[specta::specta]
pub async fn swarm_secrets_create(
    state: State<'_, AppState>,
    swarm_id: String,
    input: ObjectInput,
) -> AppResult<String> {
    objects::create_secret(&state.hosts.registry.manager(&swarm_id)?, input).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_secrets_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    objects::remove_secret(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<SwarmObject>> {
    objects::configs(&state.hosts.registry.manager(&swarm_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_create(
    state: State<'_, AppState>,
    swarm_id: String,
    input: ObjectInput,
) -> AppResult<String> {
    objects::create_config(&state.hosts.registry.manager(&swarm_id)?, input).await
}

/// A config's content, as text when it is.
#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_inspect(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<ConfigContent> {
    objects::config_content(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn swarm_configs_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    objects::remove_config(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// The Swarm's overlay networks, which services attach to.
#[tauri::command]
#[specta::specta]
pub async fn swarm_networks_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<NetworkView>> {
    let all = networks::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    Ok(all.into_iter().filter(|n| n.driver == "overlay").collect())
}
