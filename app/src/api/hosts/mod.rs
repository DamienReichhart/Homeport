//! Hosts: adding, editing and connecting to them.

pub mod registries;

use std::path::PathBuf;

use homeport_core::error::AppResult;
use homeport_hosts::{
    detect::{self, DetectedEngine},
    events::{self, EventCursor},
    import::{ImportChoice, ImportDraft, ImportOutcome},
    registry::EngineEvent,
    service::{BundleChoice, BundleHost, ExportOutcome, HostInput, HostView, SshSetup},
};

use crate::app::{events::Event, AppState};

pub fn hosts_list(state: &AppState) -> AppResult<Vec<HostView>> {
    state.hosts.list()
}

pub async fn hosts_detect_local() -> Vec<DetectedEngine> {
    detect::detect().await
}

pub async fn hosts_create(state: &AppState, input: HostInput) -> AppResult<HostView> {
    state.hosts.create(input)
}

pub async fn hosts_update(state: &AppState, id: String, input: HostInput) -> AppResult<HostView> {
    let view = state.hosts.update(&id, input)?;
    // A new name reaches the tray only when it's told.
    state.events.send(Event::HostsChanged);
    Ok(view)
}

pub fn hosts_delete(state: &AppState, id: String) -> AppResult<()> {
    state.hosts.delete(&id)?;
    state.events.send(Event::HostsChanged);
    Ok(())
}

pub async fn hosts_set_enabled(state: &AppState, id: String, enabled: bool) -> AppResult<HostView> {
    state.hosts.set_enabled(&id, enabled)
}

pub async fn hosts_trust_key(
    state: &AppState,
    id: String,
    key: String,
    bastion: Option<String>,
) -> AppResult<HostView> {
    state.hosts.trust_host_key(&id, &key, bastion.as_deref())
}

pub async fn hosts_reconnect(state: &AppState, id: String) -> AppResult<HostView> {
    state.hosts.reconnect(&id)
}

pub async fn hosts_regenerate_key(state: &AppState, id: String) -> AppResult<HostView> {
    state.hosts.regenerate_ssh_key(&id)
}

pub fn hosts_ssh_setup(state: &AppState, id: String) -> AppResult<SshSetup> {
    state.hosts.ssh_setup(&id)
}

/// Up to `limit` (1,000 by default) of a Host's kept Engine events, the
/// latest, or the latest before `before`; oldest first.
pub async fn hosts_events(
    state: &AppState,
    id: String,
    limit: Option<u32>,
    before: Option<EventCursor>,
) -> AppResult<Vec<EngineEvent>> {
    // Events from the last second may still be waiting to be written.
    state.hosts.events.flush().await;
    let limit = limit.unwrap_or(events::PAGE).clamp(1, events::CAP_PER_HOST);
    state.hosts.events.page(&id, limit, before.as_ref())
}

/// Forgets a Host's kept Engine events.
pub async fn hosts_events_clear(state: &AppState, id: String) -> AppResult<()> {
    state.hosts.events.flush().await;
    state.hosts.events.clear(&id)
}

/// The servers `~/.ssh/config` defines, as drafts of Hosts.
pub async fn hosts_ssh_config_preview(state: &AppState) -> AppResult<Vec<ImportDraft>> {
    state.hosts.preview_ssh_config()
}

pub async fn hosts_ssh_config_import(
    state: &AppState,
    choices: Vec<ImportChoice>,
) -> AppResult<ImportOutcome> {
    state.hosts.import_ssh_config(choices)
}

/// The Docker CLI's contexts, as drafts of Hosts.
pub async fn hosts_docker_contexts_preview(state: &AppState) -> AppResult<Vec<ImportDraft>> {
    state.hosts.preview_docker_contexts()
}

pub async fn hosts_docker_contexts_import(
    state: &AppState,
    choices: Vec<ImportChoice>,
) -> AppResult<ImportOutcome> {
    state.hosts.import_docker_contexts(choices)
}

/// Writes the Hosts `ids` to a file protected by `passphrase` (ADR 0013).
pub async fn hosts_export(
    state: &AppState,
    ids: Vec<String>,
    path: PathBuf,
    passphrase: String,
) -> AppResult<ExportOutcome> {
    state.hosts.export_hosts(&ids, &path, &passphrase)
}

/// The Hosts of an export file.
pub async fn hosts_bundle_preview(
    state: &AppState,
    path: PathBuf,
    passphrase: String,
) -> AppResult<Vec<BundleHost>> {
    state.hosts.preview_bundle(&path, &passphrase)
}

pub async fn hosts_bundle_import(
    state: &AppState,
    path: PathBuf,
    passphrase: String,
    choices: Vec<BundleChoice>,
) -> AppResult<ImportOutcome> {
    state.hosts.import_bundle(&path, &passphrase, choices)
}

#[cfg(test)]
mod tests;
