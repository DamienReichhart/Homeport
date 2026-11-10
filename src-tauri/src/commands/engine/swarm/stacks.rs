//! The stacks deployed on a Swarm with `docker stack deploy` (ADR 0017).

use tauri::State;

use homeport_engine::swarm::services::{self};
use homeport_stacks::{
    linked::{files as linked_files, folder, LinkedStack},
    swarm::{self as stacks_swarm, RemoveReport, SwarmStackParts},
    Stack, StackOwner,
};

use crate::{app::AppState, error::AppResult};

/// The Stacks deployed to a Swarm, found by their namespace label.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_list(
    state: State<'_, AppState>,
    swarm_id: String,
) -> AppResult<Vec<Stack>> {
    let services = services::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    stacks_swarm::list(
        &swarm_id,
        &services,
        &state.stacks.managed,
        &state.stacks.linked,
        &state.stacks.synced.store,
        &state.stacks.linked_files,
    )
}

/// The networks, secrets and configs of a swarm stack.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_parts(
    state: State<'_, AppState>,
    swarm_id: String,
    name: String,
) -> AppResult<SwarmStackParts> {
    stacks_swarm::parts(&state.hosts.registry.manager(&swarm_id)?, &name).await
}

/// Removes a swarm stack as `docker stack rm` does, through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_remove(
    state: State<'_, AppState>,
    swarm_id: String,
    name: String,
) -> AppResult<RemoveReport> {
    stacks_swarm::remove(&state.hosts.registry.manager(&swarm_id)?, &name).await
}

/// Links the External swarm stack `name` to a folder on this computer,
/// once its compose files are checked to declare the services it runs.
#[tauri::command]
#[specta::specta]
pub async fn swarm_stacks_link(
    state: State<'_, AppState>,
    swarm_id: String,
    name: String,
    working_dir: String,
) -> AppResult<LinkedStack> {
    let services = services::list(&state.hosts.registry.manager(&swarm_id)?).await?;
    let found = folder::inspect(std::path::Path::new(&working_dir))?;
    let texts: Vec<&str> = found.files.iter().map(|f| f.content.as_str()).collect();
    stacks_swarm::check_link(&name, &services, &texts)?;
    let config_files: Vec<String> = found.files.iter().map(|f| f.path.clone()).collect();
    let fingerprint = linked_files::files_fingerprint(&config_files);
    state.stacks.linked.create(
        StackOwner::Swarm(swarm_id),
        &name,
        &found.working_dir,
        &config_files,
        &fingerprint,
    )
}
