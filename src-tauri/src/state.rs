//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use std::sync::{Arc, Mutex};

use homeport_core::streams::Streams;
use homeport_engine::containers::exec::ExecSessions;
use homeport_hosts::service::HostService;
use homeport_stacks::store::StackStore;
use homeport_storage::SecretBox;

use crate::services::settings::SettingsService;

pub struct AppState {
    pub secrets: Arc<SecretBox>,
    pub settings: SettingsService,
    pub hosts: HostService,
    pub stacks: StackStore,
    pub streams: Streams,
    pub exec: ExecSessions,
    /// The update found by the last check, kept for installing it.
    pub update: Mutex<Option<tauri_plugin_updater::Update>>,
}
