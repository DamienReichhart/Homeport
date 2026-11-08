//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use std::sync::{Arc, Mutex};

use homeport_clusters::service::ClusterService;
use homeport_core::streams::Streams;
use homeport_engine::{
    containers::exec::ExecSessions, freshness::FreshnessCache, volumes::browse::BrowseSessions,
};
use homeport_git::Repositories;
use homeport_hosts::service::HostService;
use homeport_registries::RegistryStore;
use homeport_settings::SettingsService;
use homeport_stacks::{
    history::DeployHistory,
    linked::{FileCache, LinkedStore},
    store::StackStore,
    synced::SyncedStacks,
};
use homeport_storage::{Db, SecretBox};

pub struct AppState {
    pub db: Arc<Db>,
    /// The app data folder, where `homeport.db` lives.
    pub data_dir: std::path::PathBuf,
    pub secrets: Arc<SecretBox>,
    pub settings: SettingsService,
    pub hosts: HostService,
    pub clusters: ClusterService,
    pub stacks: StackStore,
    pub linked: LinkedStore,
    /// Parsed compose files, for finding Linked Stacks without re-reading them.
    pub linked_files: FileCache,
    /// What each deploy used, for the history and rolling back.
    pub history: Arc<DeployHistory>,
    pub registries: RegistryStore,
    /// Synced Stacks' git repositories and their clones on this machine.
    pub repositories: Arc<Repositories>,
    /// Synced Stacks and their worktrees on this machine.
    pub synced: Arc<SyncedStacks>,
    pub streams: Streams,
    pub exec: ExecSessions,
    /// Volumes being browsed, each in a helper container (ADR 0015).
    pub volume_browser: BrowseSessions,
    /// Registry digests, for telling when a newer image is published.
    pub freshness: FreshnessCache,
    /// The update found by the last check, kept for installing it.
    pub update: Mutex<Option<tauri_plugin_updater::Update>>,
}
