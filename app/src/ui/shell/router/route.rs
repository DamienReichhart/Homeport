//! Every page of the app, as the routes of the React features' manifests
//! (`src/features/*/manifest.ts`): one variant per route pattern, with the
//! ids and the tab its path carried.

use crate::ui::shell::registry::FeatureId;

/// A tab of a detail page, as its path named it; `None` for the first one.
pub type Tab = Option<String>;

/// An object of a Kubernetes Namespace.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespaced {
    pub namespace: String,
    pub name: String,
}

impl Namespaced {
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }
}

/// A kind of the Kubernetes API: `apps/v1/deployments`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApiKind {
    pub group: String,
    pub version: String,
    pub plural: String,
}

/// Where the window is.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Route {
    Overview,
    Board,
    Containers,
    Container {
        id: String,
        tab: Tab,
    },
    Images,
    Volumes,
    Networks,
    Stacks,
    NewStack,
    Stack {
        name: String,
    },
    PodmanPods,
    PodmanPod {
        id: String,
    },
    Quadlets,
    Quadlet {
        name: String,
    },
    ContainerdContainers,
    ContainerdContainer {
        id: String,
        tab: Tab,
    },
    ContainerdImages,
    ContainerdSnapshots,
    ContainerdNamespaces,
    IncusInstances,
    IncusInstance {
        name: String,
        tab: Tab,
    },
    IncusImages,
    IncusProfiles,
    IncusProfile {
        name: String,
    },
    IncusStorage {
        pool: Option<String>,
    },
    IncusNetworks,
    IncusNetwork {
        name: String,
    },
    Swarm {
        swarm_id: String,
        tab: Tab,
    },
    SwarmService {
        swarm_id: String,
        service_id: String,
        tab: Tab,
    },
    SwarmStack {
        swarm_id: String,
        name: String,
    },
    Workloads {
        kind: Option<String>,
    },
    Workload {
        kind: String,
        object: Namespaced,
        tab: Tab,
    },
    Pods,
    Pod {
        object: Namespaced,
        tab: Tab,
    },
    Services,
    Service {
        object: Namespaced,
        tab: Tab,
    },
    Ingresses,
    Ingress {
        object: Namespaced,
        tab: Tab,
    },
    ConfigMaps,
    ConfigMap {
        object: Namespaced,
        tab: Tab,
    },
    Secrets,
    Secret {
        object: Namespaced,
        tab: Tab,
    },
    Nodes,
    Node {
        name: String,
        tab: Tab,
    },
    ClusterEvents,
    ApiResources {
        api: Option<ApiKind>,
    },
    ApiObject {
        api: ApiKind,
        object: Namespaced,
        tab: Tab,
    },
    Rbac {
        tab: Tab,
    },
    ClusterRole {
        name: String,
    },
    Role {
        object: Namespaced,
    },
    RbacUser {
        name: String,
    },
    RbacGroup {
        name: String,
    },
    ServiceAccount {
        object: Namespaced,
    },
    Apply,
    HelmReleases,
    HelmRepositories,
    HelmRelease {
        object: Namespaced,
        tab: Tab,
    },
    PortForwards,
    NomadJobs,
    NomadJob {
        namespace: String,
        job_id: String,
    },
    NomadAllocation {
        alloc_id: String,
        tab: Tab,
    },
    NomadNodes,
    NomadNode {
        node_id: String,
    },
    Events,
    Settings,
    Hosts,
    AddHost,
    Host {
        host_id: String,
    },
    Clusters,
    AddCluster,
    Cluster {
        cluster_id: String,
    },
}

impl Route {
    /// The feature whose page this is.
    pub fn feature(&self) -> FeatureId {
        use FeatureId as F;
        match self {
            Self::Overview => F::Overview,
            Self::Board => F::Board,
            Self::Containers | Self::Container { .. } => F::Containers,
            Self::Images => F::Images,
            Self::Volumes => F::Volumes,
            Self::Networks => F::Networks,
            Self::Stacks | Self::NewStack | Self::Stack { .. } => F::Stacks,
            Self::PodmanPods | Self::PodmanPod { .. } => F::PodmanPods,
            Self::Quadlets | Self::Quadlet { .. } => F::Quadlets,
            Self::ContainerdContainers | Self::ContainerdContainer { .. } => {
                F::ContainerdContainers
            }
            Self::ContainerdImages => F::ContainerdImages,
            Self::ContainerdSnapshots => F::ContainerdSnapshots,
            Self::ContainerdNamespaces => F::ContainerdNamespaces,
            Self::IncusInstances | Self::IncusInstance { .. } => F::IncusInstances,
            Self::IncusImages => F::IncusImages,
            Self::IncusProfiles | Self::IncusProfile { .. } => F::IncusProfiles,
            Self::IncusStorage { .. } => F::IncusStorage,
            Self::IncusNetworks | Self::IncusNetwork { .. } => F::IncusNetworks,
            Self::Swarm { .. } | Self::SwarmService { .. } | Self::SwarmStack { .. } => F::Swarm,
            Self::Workloads { .. } | Self::Workload { .. } => F::Workloads,
            Self::Pods | Self::Pod { .. } => F::Pods,
            Self::Services | Self::Service { .. } | Self::Ingresses | Self::Ingress { .. } => {
                F::Network
            }
            Self::ConfigMaps | Self::ConfigMap { .. } | Self::Secrets | Self::Secret { .. } => {
                F::Config
            }
            Self::Nodes | Self::Node { .. } => F::Nodes,
            Self::ClusterEvents => F::ClusterEvents,
            Self::ApiResources { .. } | Self::ApiObject { .. } => F::ApiResources,
            Self::Rbac { .. }
            | Self::ClusterRole { .. }
            | Self::Role { .. }
            | Self::RbacUser { .. }
            | Self::RbacGroup { .. }
            | Self::ServiceAccount { .. } => F::Rbac,
            Self::Apply => F::Apply,
            Self::HelmReleases | Self::HelmRepositories | Self::HelmRelease { .. } => F::Helm,
            Self::PortForwards => F::PortForwards,
            Self::NomadJobs | Self::NomadJob { .. } | Self::NomadAllocation { .. } => F::NomadJobs,
            Self::NomadNodes | Self::NomadNode { .. } => F::NomadNodes,
            Self::Events => F::Events,
            Self::Settings => F::Settings,
            Self::Hosts | Self::AddHost | Self::Host { .. } => F::Hosts,
            Self::Clusters | Self::AddCluster | Self::Cluster { .. } => F::Clusters,
        }
    }

    /// The Swarm a Swarm page is about.
    pub fn swarm_id(&self) -> Option<&str> {
        match self {
            Self::Swarm { swarm_id, .. }
            | Self::SwarmService { swarm_id, .. }
            | Self::SwarmStack { swarm_id, .. } => Some(swarm_id),
            _ => None,
        }
    }
}
