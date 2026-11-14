//! What the registry reports as it happens.

use super::ClusterStatus;
use homeport_kubernetes::streams::forward::ForwardEvent;

/// Receives each Cluster's status changes; the app forwards them to the frontend.
/// A status is told while the registry holds its entries, so statuses are
/// told in the order the entries took them: a notifier must not call the
/// registry back (the app's sends on a channel).
pub trait ClusterNotifier: Send + Sync + 'static {
    fn cluster_status(&self, cluster_id: &str, status: &ClusterStatus);

    /// A port forward failed, or ended because its target is gone.
    fn port_forward(&self, _event: &ForwardEvent) {}
}
