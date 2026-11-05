//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use std::sync::{Arc, Mutex};

use homeport_core::streams::Streams;
use homeport_engine::{containers::exec::ExecSessions, freshness::FreshnessCache};
use homeport_git::Repositories;
use homeport_hosts::service::HostService;
use homeport_registries::RegistryStore;
use homeport_settings::SettingsService;
use homeport_stacks::{
    history::DeployHistory,
    linked::{FileCache, LinkedStore},
    store::StackStore,
};
use homeport_storage::SecretBox;

pub struct AppState {
    pub secrets: Arc<SecretBox>,
    pub settings: SettingsService,
    pub hosts: HostService,
    pub stacks: StackStore,
    pub linked: LinkedStore,
    /// Parsed compose files, for finding Linked Stacks without re-reading them.
    pub linked_files: FileCache,
    /// What each deploy used, for the history and rolling back.
    pub history: Arc<DeployHistory>,
    pub registries: RegistryStore,
    /// Synced Stacks' git repositories and their clones on this machine.
    pub repositories: Repositories,
    pub streams: Streams,
    pub exec: ExecSessions,
    /// Registry digests, for telling when a newer image is published.
    pub freshness: FreshnessCache,
    /// The update found by the last check, kept for installing it.
    pub update: Mutex<Option<tauri_plugin_updater::Update>>,
}
