//! The use cases the app exposes, keeping the store and the live
//! connections in step.

use std::sync::Arc;

use serde::Serialize;

use homeport_core::error::AppResult;

use crate::{
    model::Cluster,
    registry::{ClusterRegistry, ClusterStatus},
    store::ClusterStore,
};

/// A Cluster and its connection's status, as the frontend lists them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterView {
    pub cluster: Cluster,
    pub status: ClusterStatus,
}

#[derive(Clone)]
pub struct ClusterService {
    pub store: Arc<ClusterStore>,
    pub registry: Arc<ClusterRegistry>,
}

impl ClusterService {
    pub fn view(&self, cluster: Cluster) -> ClusterView {
        let status = self.registry.status(&cluster.id);
        ClusterView { cluster, status }
    }

    pub fn list(&self) -> AppResult<Vec<ClusterView>> {
        Ok(self
            .store
            .list()?
            .into_iter()
            .map(|c| self.view(c))
            .collect())
    }

    pub fn get(&self, id: &str) -> AppResult<ClusterView> {
        Ok(self.view(self.store.get(id)?))
    }

    pub fn rename(&self, id: &str, name: &str) -> AppResult<ClusterView> {
        let cluster = self.store.rename(id, name)?;
        Ok(self.view(cluster))
    }

    /// Enables or disables a Cluster; its supervisor connects or stops.
    pub fn set_enabled(&self, id: &str, enabled: bool) -> AppResult<ClusterView> {
        let cluster = self.store.set_enabled(id, enabled)?;
        self.registry.restart(id);
        Ok(self.view(cluster))
    }

    /// Forgets a Cluster and its credentials. Nothing changes on the Cluster.
    pub fn remove(&self, id: &str) -> AppResult<()> {
        self.store.delete(id)?;
        self.registry.remove(id);
        Ok(())
    }

    pub fn reconnect(&self, id: &str) -> AppResult<ClusterView> {
        let cluster = self.store.get(id)?;
        self.registry.restart(id);
        Ok(self.view(cluster))
    }
}
