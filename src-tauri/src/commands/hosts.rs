//! Hosts: adding, editing and connecting to them.

use std::path::PathBuf;

use tauri::State;

use homeport_hosts::{
    detect::{self, DetectedEngine},
    import::{ImportChoice, ImportDraft, ImportOutcome},
    registry::EngineEvent,
    service::{BundleChoice, BundleHost, ExportOutcome, HostInput, HostView, SshSetup},
};

use crate::{app::AppState, error::AppResult};

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
    bastion: Option<String>,
) -> AppResult<HostView> {
    state.hosts.trust_host_key(&id, &key, bastion.as_deref())
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

/// The servers `~/.ssh/config` defines, as drafts of Hosts.
#[tauri::command]
#[specta::specta]
pub async fn hosts_ssh_config_preview(state: State<'_, AppState>) -> AppResult<Vec<ImportDraft>> {
    state.hosts.preview_ssh_config()
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_ssh_config_import(
    state: State<'_, AppState>,
    choices: Vec<ImportChoice>,
) -> AppResult<ImportOutcome> {
    state.hosts.import_ssh_config(choices)
}

/// The Docker CLI's contexts, as drafts of Hosts.
#[tauri::command]
#[specta::specta]
pub async fn hosts_docker_contexts_preview(
    state: State<'_, AppState>,
) -> AppResult<Vec<ImportDraft>> {
    state.hosts.preview_docker_contexts()
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_docker_contexts_import(
    state: State<'_, AppState>,
    choices: Vec<ImportChoice>,
) -> AppResult<ImportOutcome> {
    state.hosts.import_docker_contexts(choices)
}

/// Writes the Hosts `ids` to a file protected by `passphrase` (ADR 0013).
#[tauri::command]
#[specta::specta]
pub async fn hosts_export(
    state: State<'_, AppState>,
    ids: Vec<String>,
    path: PathBuf,
    passphrase: String,
) -> AppResult<ExportOutcome> {
    state.hosts.export_hosts(&ids, &path, &passphrase)
}

/// The Hosts of an export file.
#[tauri::command]
#[specta::specta]
pub async fn hosts_bundle_preview(
    state: State<'_, AppState>,
    path: PathBuf,
    passphrase: String,
) -> AppResult<Vec<BundleHost>> {
    state.hosts.preview_bundle(&path, &passphrase)
}

#[tauri::command]
#[specta::specta]
pub async fn hosts_bundle_import(
    state: State<'_, AppState>,
    path: PathBuf,
    passphrase: String,
    choices: Vec<BundleChoice>,
) -> AppResult<ImportOutcome> {
    state.hosts.import_bundle(&path, &passphrase, choices)
}
