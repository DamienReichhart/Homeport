use tauri::State;

use crate::{
    error::AppResult,
    services::hosts::{
        detect::{self, DetectedEngine},
        registry::EngineEvent,
        service::{HostInput, HostView, SshSetup},
    },
    state::AppState,
};

#[tauri::command]
#[specta::specta]
pub fn hosts_list(state: State<'_, AppState>) -> AppResult<Vec<HostView>> {
    state.hosts.list()
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_detect_local() -> Vec<DetectedEngine> {
    detect::detect().await
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_create(state: State<'_, AppState>, input: HostInput) -> AppResult<HostView> {
    state.hosts.create(input)
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_update(
    state: State<'_, AppState>,
    id: String,
    input: HostInput,
) -> AppResult<HostView> {
    state.hosts.update(&id, input)
}

#[tauri::command]
#[specta::specta]
pub fn hosts_delete(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.hosts.delete(&id)
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_set_enabled(
    state: State<'_, AppState>,
    id: String,
    enabled: bool,
) -> AppResult<HostView> {
    state.hosts.set_enabled(&id, enabled)
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_trust_key(
    state: State<'_, AppState>,
    id: String,
    key: String,
) -> AppResult<HostView> {
    state.hosts.trust_host_key(&id, &key)
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_reconnect(state: State<'_, AppState>, id: String) -> AppResult<HostView> {
    state.hosts.reconnect(&id)
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_regenerate_key(state: State<'_, AppState>, id: String) -> AppResult<HostView> {
    state.hosts.regenerate_ssh_key(&id)
}

#[tauri::command]
#[specta::specta]
pub fn hosts_ssh_setup(state: State<'_, AppState>, id: String) -> AppResult<SshSetup> {
    state.hosts.ssh_setup(&id)
}

#[tauri::command]
#[specta::specta]
pub fn hosts_events(state: State<'_, AppState>, id: String) -> Vec<EngineEvent> {
    state.hosts.registry.events(&id)
}
