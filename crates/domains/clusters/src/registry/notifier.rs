//! What the registry reports as it happens.

use super::ClusterStatus;
use homeport_kubernetes::streams::forward::ForwardEvent;

/// Receives each Cluster's status changes; the app's (`app::events::Events`)
/// sends them on the window's event channel, and nothing else is one.
pub trait ClusterNotifier: Send + Sync + 'static {
    /// A Cluster's status changed. **Runs with the registry's lock held**,
    /// so statuses are told in the order the entries took them: it must
    /// not block, wait on anything or call the registry back. A call back
    /// would wait on the lock for ever; a build with debug assertions
    /// panics instead. Hand the status on (a channel) and return.
    fn cluster_status(&self, cluster_id: &str, status: &ClusterStatus);

    /// A port forward failed, or ended because its target is gone.
    fn port_forward(&self, _event: &ForwardEvent) {}
}
