//! What the shell knows of the Hosts, the Clusters and what is selected:
//! one picture, taken from the data layer after each change, that the
//! gates, the sidebar, the palette and the pages all read.

use std::sync::Arc;

use homeport_clusters::service::ClusterView;
use homeport_containerd::namespaces::ContainerdNamespace;
use homeport_engine::swarm::SwarmSummary;
use homeport_hosts::{registry::SwarmView, service::HostView};
use homeport_incus::projects::IncusProject;
use homeport_kubernetes::resources::kinds::pods::NamespaceView;
use homeport_nomad::namespaces::NomadNamespaces;
use homeport_settings::Settings;

use crate::{
    api::app::Capabilities,
    data::Resource,
    ui::shell::selection::{self, Candidate, Selection},
};

/// A list read through the data layer.
pub type Listed<T> = Resource<Arc<Vec<T>>>;

/// A newer version of Homeport, as the bar at the top says it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub version: String,
    /// A package manager installs it, not Homeport.
    pub via_package_manager: bool,
}

#[derive(Debug, Clone)]
pub struct World {
    pub hosts: Listed<HostView>,
    pub clusters: Listed<ClusterView>,
    pub swarms: Listed<SwarmView>,
    /// The Host or Cluster the pages show, after falling back.
    pub selection: Option<Selection>,
    /// The Settings, for what they remember: namespaces and projects.
    pub settings: Settings,
    /// The selected containerd Host's namespaces.
    pub containerd: Listed<ContainerdNamespace>,
    /// The selected Incus Host's projects.
    pub incus: Listed<IncusProject>,
    /// The selected Kubernetes Cluster's Namespaces.
    pub namespaces: Listed<NamespaceView>,
    /// The selected Nomad Cluster's namespaces.
    pub nomad: Resource<Arc<NomadNamespaces>>,
    /// What a manager says of the Swarm whose page shows.
    pub swarm: Resource<Arc<SwarmSummary>>,
    pub capabilities: Option<Arc<Capabilities>>,
    pub update: Option<Update>,
}

impl World {
    pub fn new(settings: Settings) -> Self {
        Self {
            hosts: Resource::Idle,
            clusters: Resource::Idle,
            swarms: Resource::Idle,
            selection: None,
            settings,
            containerd: Resource::Idle,
            incus: Resource::Idle,
            namespaces: Resource::Idle,
            nomad: Resource::Idle,
            swarm: Resource::Idle,
            capabilities: None,
            update: None,
        }
    }

    pub fn host_list(&self) -> &[HostView] {
        self.hosts.data().map_or(&[], |list| list.as_slice())
    }

    pub fn cluster_list(&self) -> &[ClusterView] {
        self.clusters.data().map_or(&[], |list| list.as_slice())
    }

    pub fn swarm_list(&self) -> &[SwarmView] {
        self.swarms.data().map_or(&[], |list| list.as_slice())
    }

    /// The selected Host, `None` when a Cluster or nothing is selected.
    pub fn host_id(&self) -> Option<&str> {
        match &self.selection {
            Some(Selection::Host { id }) => Some(id),
            _ => None,
        }
    }

    pub fn host(&self) -> Option<&HostView> {
        let id = self.host_id()?;
        self.host_list().iter().find(|view| view.host.id == id)
    }

    /// The selected Cluster, `None` when a Host or nothing is selected.
    pub fn cluster_id(&self) -> Option<&str> {
        match &self.selection {
            Some(Selection::Cluster { id }) => Some(id),
            _ => None,
        }
    }

    pub fn cluster(&self) -> Option<&ClusterView> {
        let id = self.cluster_id()?;
        self.cluster_list()
            .iter()
            .find(|view| view.cluster.id == id)
    }

    /// The Hosts the selection may fall back to; `None` while they load.
    pub fn host_candidates(&self) -> Option<Vec<Candidate>> {
        self.hosts.data().map(|list| {
            list.iter()
                .map(|view| {
                    let usable = crate::ui::shell::status::host::is_connected(&view.status);
                    Candidate::new(view.host.id.clone(), usable)
                })
                .collect()
        })
    }

    pub fn cluster_candidates(&self) -> Option<Vec<Candidate>> {
        self.clusters.data().map(|list| {
            list.iter()
                .map(|view| Candidate::new(view.cluster.id.clone(), view.status.is_connected()))
                .collect()
        })
    }

    /// The selection `chosen` leads to among what is known now.
    pub fn resolve(&self, chosen: Option<&Selection>) -> Option<Selection> {
        selection::resolve(
            chosen,
            self.host_candidates().as_deref(),
            self.cluster_candidates().as_deref(),
        )
    }
}
