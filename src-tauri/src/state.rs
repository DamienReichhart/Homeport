//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use std::sync::Arc;

use crate::services::{
    crypto::SecretBox, db::Db, docker::exec::ExecSessions, hosts::service::HostService,
    settings::SettingsService, streams::Streams,
};

pub struct AppState {
    pub secrets: Arc<SecretBox>,
    #[allow(dead_code)] // read by the stack service
    pub db: Arc<Db>,
    pub settings: SettingsService,
    pub hosts: HostService,
    pub streams: Streams,
    pub exec: ExecSessions,
}
