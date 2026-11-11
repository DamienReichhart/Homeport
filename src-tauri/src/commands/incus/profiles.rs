//! Incus profiles.

use tauri::State;

use homeport_incus::profiles::{self, IncusProfile, IncusProfileDetail};

use crate::{app::AppState, error::AppResult};

/// The profiles of `project`, with the Instances using each.
#[tauri::command]
#[specta::specta]
pub async fn incus_profiles(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
) -> AppResult<Vec<IncusProfile>> {
    profiles::list(&state.hosts.registry.incus(&host_id)?, &project).await
}

/// One profile, with its YAML and the ETag to save it with.
#[tauri::command]
#[specta::specta]
pub async fn incus_profile(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<IncusProfileDetail> {
    profiles::get(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}

/// Saves a profile's YAML; refused if it changed since `etag` was read.
#[tauri::command]
#[specta::specta]
pub async fn incus_profile_update(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    yaml: String,
    etag: Option<String>,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    profiles::update(&incus, &project, &name, &yaml, etag.as_deref()).await
}

#[tauri::command]
#[specta::specta]
pub async fn incus_profile_create(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
    description: String,
) -> AppResult<()> {
    let incus = state.hosts.registry.incus(&host_id)?;
    profiles::create(&incus, &project, &name, &description).await
}

/// Deletes an unused profile other than `default`.
#[tauri::command]
#[specta::specta]
pub async fn incus_profile_delete(
    state: State<'_, AppState>,
    host_id: String,
    project: String,
    name: String,
) -> AppResult<()> {
    profiles::delete(&state.hosts.registry.incus(&host_id)?, &project, &name).await
}
