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
    /// Unix time in nanoseconds, which tells apart events of the same second.
    pub time_nano: i64,
    /// `container`, `image`, `volume`, `network`, …
    pub kind: String,
    pub action: String,
    pub actor_id: String,
    pub attributes: HashMap<String, String>,
}

/// Receives what the registry observes; the app's (`app::events::Events`)
/// sends it on the window's event channel, and nothing else is one.
pub trait Notifier: Send + Sync + 'static {
    /// A Host's status changed. **Runs with the registry's lock held**, so
    /// statuses are told in the order the entries took them: it must not
    /// block, wait on anything or call the registry back. A call back
    /// would wait on the lock for ever; a build with debug assertions
    /// panics instead. Hand the status on (a channel) and return.
    fn host_status(&self, host_id: &str, status: &HostStatus);
    /// An Engine event, told without the lock.
    fn engine_event(&self, host_id: &str, event: &EngineEvent);
}
