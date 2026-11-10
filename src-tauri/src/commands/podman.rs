//! What only Podman offers, through its own API: pods (containers run into
//! one go through `containers_run`) and Quadlet units, whose services start
//! and stop with `systemctl` where the Host runs commands (ADR 0019). A
//! Host that isn't Podman answers `EngineUnsupported`.

use tauri::State;

use homeport_engine::{
    containers,
    podman::{
        pods::{self, Pod, PodAction, PodDetail, PodSpec},
        quadlets::{self, QuadletInstall, QuadletUnits, UnitAction, UnitScope},
    },
};

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn pods_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Pod>> {
    pods::list(&state.hosts.registry.podman(&host_id)?).await
}

#[tauri::command]
#[specta::specta]
pub async fn pods_inspect(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
) -> AppResult<PodDetail> {
    pods::inspect(&state.hosts.registry.podman(&host_id)?, &id).await
}

#[tauri::command]
#[specta::specta]
pub async fn pods_act(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    action: PodAction,
) -> AppResult<()> {
    pods::act(&state.hosts.registry.podman(&host_id)?, &id, action).await
}

/// Removes a pod and its containers; returns how many containers went with it.
#[tauri::command]
#[specta::specta]
pub async fn pods_remove(
    state: State<'_, AppState>,
    host_id: String,
    id: String,
    force: bool,
) -> AppResult<u32> {
    pods::remove(&state.hosts.registry.podman(&host_id)?, &id, force).await
}

/// Creates a pod; returns its id.
#[tauri::command]
#[specta::specta]
pub async fn pods_create(
    state: State<'_, AppState>,
    host_id: String,
    spec: PodSpec,
) -> AppResult<String> {
    pods::create(&state.hosts.registry.podman(&host_id)?, &spec).await
}

/// The Host's Quadlet units, with the containers each created.
#[tauri::command]
#[specta::specta]
pub async fn quadlets_list(state: State<'_, AppState>, host_id: String) -> AppResult<QuadletUnits> {
    let podman = state.hosts.registry.podman(&host_id)?;
    let containers = containers::list(&state.hosts.registry.docker(&host_id)?).await?;
    quadlets::list(&podman, &containers).await
}

/// A unit's file, as Podman reads it on the Host (Podman 5.8 and later).
#[tauri::command]
#[specta::specta]
pub async fn quadlets_file(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<String> {
    quadlets::file(&state.hosts.registry.podman(&host_id)?, &name).await
}

/// Installs a unit with its extra files; `replace` edits an installed one.
/// `expected` is the unit file as the edit opened it: a file changed on the
/// Host since is refused, unless `None` overwrites it. Returns where the
/// files went on the Host.
#[tauri::command]
#[specta::specta]
pub async fn quadlets_install(
    state: State<'_, AppState>,
    host_id: String,
    install: QuadletInstall,
    replace: bool,
    expected: Option<String>,
) -> AppResult<Vec<String>> {
    quadlets::install(
        &state.hosts.registry.podman(&host_id)?,
        &install,
        replace,
        expected.as_deref(),
    )
    .await
}

/// Removes a unit; `force` stops its service first.
#[tauri::command]
#[specta::specta]
pub async fn quadlets_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    force: bool,
) -> AppResult<()> {
    quadlets::remove(&state.hosts.registry.podman(&host_id)?, &name, force).await
}

/// Starts, stops or restarts a unit's service with `systemctl`.
#[tauri::command]
#[specta::specta]
pub async fn quadlets_act(
    state: State<'_, AppState>,
    host_id: String,
    unit: String,
    scope: UnitScope,
    action: UnitAction,
) -> AppResult<()> {
    quadlets::act(
        &state.hosts.registry.podman(&host_id)?,
        &unit,
        scope,
        action,
    )
    .await
}
