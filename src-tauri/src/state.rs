//! Shared application state, registered once with `app.manage(...)` and
//! injected into commands via `tauri::State<AppState>`.

use std::sync::Arc;

use crate::services::{crypto::SecretBox, db::Db, settings::SettingsService};

pub struct AppState {
    pub secrets: Arc<SecretBox>,
    #[allow(dead_code)] // read by the host and stack services
    pub db: Arc<Db>,
    pub settings: SettingsService,
}
