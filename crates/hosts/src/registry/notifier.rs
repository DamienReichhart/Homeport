//! What the registry reports as it happens: status changes and Engine events.

use std::collections::HashMap;

use serde::Serialize;

use super::HostStatus;

/// One entry of a Host's Engine event stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EngineEvent {
    /// Unix time in seconds.
    pub time: i64,
    /// `container`, `image`, `volume`, `network`, …
    pub kind: String,
    pub action: String,
    pub actor_id: String,
    pub attributes: HashMap<String, String>,
}

/// Receives what the registry observes; the app forwards it to the frontend.
pub trait Notifier: Send + Sync + 'static {
    fn host_status(&self, host_id: &str, status: &HostStatus);
    fn engine_event(&self, host_id: &str, event: &EngineEvent);
}
