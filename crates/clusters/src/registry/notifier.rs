//! What the registry reports as it happens.

use super::ClusterStatus;
use homeport_kubernetes::forward::ForwardEvent;

/// Receives each Cluster's status changes; the app forwards them to the frontend.
pub trait ClusterNotifier: Send + Sync + 'static {
    fn cluster_status(&self, cluster_id: &str, status: &ClusterStatus);

    /// A port forward failed, or ended because its target is gone.
    fn port_forward(&self, _event: &ForwardEvent) {}
}
