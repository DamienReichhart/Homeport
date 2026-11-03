//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use std::sync::{Arc, Mutex};

use crate::services::{
    crypto::SecretBox, docker::exec::ExecSessions, hosts::service::HostService,
    settings::SettingsService, stacks::store::StackStore, streams::Streams,
};

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
