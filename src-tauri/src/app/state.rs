//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use std::sync::{Arc, Mutex};

use homeport_clusters::service::ClusterService;
use homeport_containerd::exec::ContainerdExecSessions;
use homeport_core::streams::Streams;
use homeport_engine::{
    containers::exec::ExecSessions, freshness::FreshnessCache, volumes::browse::BrowseSessions,
};
use homeport_git::Repositories;
use homeport_hosts::service::HostService;
use homeport_incus::instances::terminal::IncusTerminals;
use homeport_kubernetes::terminal::PodTerminals;
use homeport_nomad::exec::NomadTerminals;
use homeport_registries::RegistryStore;
use homeport_settings::SettingsService;
use homeport_stacks::Stacks;
use homeport_storage::{Db, SecretBox};

pub struct AppState {
    pub db: Arc<Db>,
    /// The app data folder, where `homeport.db` lives.
    pub data_dir: std::path::PathBuf,
    pub secrets: Arc<SecretBox>,
    pub settings: SettingsService,
    pub hosts: HostService,
    pub clusters: ClusterService,
    /// Managed, Linked and Synced Stacks, and their deploy history.
    pub stacks: Stacks,
    pub registries: RegistryStore,
    /// Synced Stacks' git repositories and their clones on this machine.
    pub repositories: Arc<Repositories>,
    pub streams: Streams,
    pub terminals: Terminals,
    /// Volumes being browsed, each in a helper container (ADR 0015).
    pub volume_browser: BrowseSessions,
    /// Registry digests, for telling when a newer image is published.
    pub freshness: FreshnessCache,
    /// The update found by the last check, kept for installing it.
    pub update: Mutex<Option<tauri_plugin_updater::Update>>,
}

/// The terminals open from the app, by what they run in.
#[derive(Default)]
pub struct Terminals {
    /// In containers of a Docker Engine.
    pub engine: ExecSessions,
    /// In containerd containers' tasks.
    pub containerd: ContainerdExecSessions,
    /// On Incus Instances: exec sessions and consoles.
    pub incus: IncusTerminals,
    /// In Pods' containers.
    pub pods: PodTerminals,
    /// In Nomad allocations' tasks.
    pub nomad: NomadTerminals,
}
