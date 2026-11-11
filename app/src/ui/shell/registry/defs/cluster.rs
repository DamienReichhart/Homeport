//! The features of a Cluster: Kubernetes' and Nomad's.

use homeport_clusters::model::ClusterKind;

use super::nav;
use crate::{
    t,
    ui::{
        shell::{
            registry::{Feature, FeatureId, Gate, PageShortcut, Run, Scope},
            router::Route,
        },
        theme::Icon,
    },
};

/// A feature of a Cluster of a kind, with its sidebar entry and no command.
macro_rules! of {
    ($id:expr, $kind:expr, $routes:expr, ($route:expr, $label:expr, $icon:expr $(,)?), $gate:expr, $shortcuts:expr $(,)?) => {
        Feature {
            id: $id,
            routes: $routes,
            nav: nav($route, $label, $icon, Scope::Cluster),
            requires: None,
            cluster_kind: Some($kind),
            gate: $gate,
            commands: &[],
            shortcuts: $shortcuts,
        }
    };
}

pub static WORKLOADS: Feature = of!(
    FeatureId::Workloads,
    ClusterKind::Kubernetes,
    &["workloads/:kind?", "workloads/:kind/:namespace/:name/:tab?"],
    (
        Route::Workloads { kind: None },
        || t!("workloads.nav"),
        Icon::Layers
    ),
    Gate::Cluster,
    &[PageShortcut::search(|route| {
        matches!(route, Route::Workloads { .. })
    })],
);

pub static PODS: Feature = of!(
    FeatureId::Pods,
    ClusterKind::Kubernetes,
    &["pods", "pods/:namespace/:name/:tab?"],
    (Route::Pods, || t!("pods.nav"), Icon::Container),
    Gate::Cluster,
    &[PageShortcut::search(|route| matches!(route, Route::Pods))],
);

pub static NETWORK: Feature = of!(
    FeatureId::Network,
    ClusterKind::Kubernetes,
    &[
        "services",
        "services/:namespace/:name/:tab?",
        "ingresses",
        "ingresses/:namespace/:name/:tab?",
    ],
    (Route::Services, || t!("network.nav"), Icon::Waypoints),
    Gate::Cluster,
    &[],
);

pub static CONFIG: Feature = of!(
    FeatureId::Config,
    ClusterKind::Kubernetes,
    &[
        "configmaps",
        "configmaps/:namespace/:name/:tab?",
        "secrets",
        "secrets/:namespace/:name/:tab?",
    ],
    (Route::ConfigMaps, || t!("config.nav"), Icon::FileCog),
    Gate::Cluster,
    &[],
);

pub static NODES: Feature = of!(
    FeatureId::Nodes,
    ClusterKind::Kubernetes,
    &["nodes", "nodes/:name/:tab?"],
    (Route::Nodes, || t!("nodes.nav"), Icon::Server),
    Gate::Cluster,
    &[],
);

pub static CLUSTER_EVENTS: Feature = of!(
    FeatureId::ClusterEvents,
    ClusterKind::Kubernetes,
    &["cluster-events"],
    (
        Route::ClusterEvents,
        || t!("clusterevents.nav"),
        Icon::Activity
    ),
    Gate::Cluster,
    &[],
);

pub static API_RESOURCES: Feature = of!(
    FeatureId::ApiResources,
    ClusterKind::Kubernetes,
    &[
        "api-resources",
        "api-resources/:group/:version/:plural",
        "api-resources/:group/:version/:plural/:namespace/:name/:tab?",
    ],
    (
        Route::ApiResources { api: None },
        || t!("apiresources.nav"),
        Icon::Blocks,
    ),
    Gate::Cluster,
    &[PageShortcut::search(|route| {
        matches!(route, Route::ApiResources { .. })
    })],
);

pub static RBAC: Feature = of!(
    FeatureId::Rbac,
    ClusterKind::Kubernetes,
    &[
        "rbac/:tab?",
        "rbac/clusterroles/:name",
        "rbac/roles/:namespace/:name",
        "rbac/users/:name",
        "rbac/groups/:name",
        "rbac/serviceaccounts/:namespace/:name",
    ],
    (
        Route::Rbac { tab: None },
        || t!("rbac.nav"),
        Icon::ShieldCheck
    ),
    Gate::Cluster,
    &[PageShortcut::search(|route| matches!(
        route,
        Route::Rbac { .. }
    ))],
);

pub static APPLY: Feature = of!(
    FeatureId::Apply,
    ClusterKind::Kubernetes,
    &["apply"],
    (Route::Apply, || t!("apply.nav"), Icon::FileCodeCorner),
    Gate::Cluster,
    &[],
);

pub static HELM: Feature = of!(
    FeatureId::Helm,
    ClusterKind::Kubernetes,
    &["helm", "helm/repositories", "helm/:namespace/:name/:tab?"],
    (Route::HelmReleases, || t!("helm.nav"), Icon::ShipWheel),
    Gate::Cluster,
    &[
        PageShortcut::search(|route| {
            matches!(route, Route::HelmReleases | Route::HelmRepositories)
        }),
        PageShortcut::action(
            |route| matches!(route, Route::HelmReleases),
            || t!("helm.install.open"),
            Run::Request("install"),
        ),
    ],
);

/// Not behind the Cluster's gate: it lists the forwards to every Cluster.
pub static PORT_FORWARDS: Feature = of!(
    FeatureId::PortForwards,
    ClusterKind::Kubernetes,
    &["port-forwards"],
    (
        Route::PortForwards,
        || t!("portforwards.nav"),
        Icon::ArrowRightLeft,
    ),
    Gate::Open,
    &[],
);

pub static NOMAD_JOBS: Feature = of!(
    FeatureId::NomadJobs,
    ClusterKind::Nomad,
    &[
        "nomad/jobs",
        "nomad/jobs/:namespace/:jobId",
        "nomad/allocations/:allocId/:tab?",
    ],
    (Route::NomadJobs, || t!("nomad.jobs.nav"), Icon::Briefcase),
    Gate::Cluster,
    &[PageShortcut::search(|route| matches!(
        route,
        Route::NomadJobs
    ))],
);

pub static NOMAD_NODES: Feature = of!(
    FeatureId::NomadNodes,
    ClusterKind::Nomad,
    &["nomad/nodes", "nomad/nodes/:nodeId"],
    (Route::NomadNodes, || t!("nomad.nodes.nav"), Icon::Server),
    Gate::Cluster,
    &[],
);
