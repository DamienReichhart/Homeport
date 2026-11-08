//! What the registry reports as it happens.

use super::ClusterStatus;

/// Receives each Cluster's status changes; the app forwards them to the frontend.
pub trait ClusterNotifier: Send + Sync + 'static {
    fn cluster_status(&self, cluster_id: &str, status: &ClusterStatus);
}
