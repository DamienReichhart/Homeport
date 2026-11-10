//! The kinds of Kubernetes resources Homeport shows.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ResourceKind {
    Namespace,
    Deployment,
    StatefulSet,
    DaemonSet,
    Job,
    CronJob,
    Pod,
    Service,
    Ingress,
    ConfigMap,
    Secret,
    Node,
    Event,
}

impl ResourceKind {
    /// Its kind as Kubernetes names it.
    pub fn kind(self) -> &'static str {
        match self {
            Self::Namespace => "Namespace",
            Self::Deployment => "Deployment",
            Self::StatefulSet => "StatefulSet",
            Self::DaemonSet => "DaemonSet",
            Self::Job => "Job",
            Self::CronJob => "CronJob",
            Self::Pod => "Pod",
            Self::Service => "Service",
            Self::Ingress => "Ingress",
            Self::ConfigMap => "ConfigMap",
            Self::Secret => "Secret",
            Self::Node => "Node",
            Self::Event => "Event",
        }
    }

    /// The plural of its API resource, as RBAC names it (`deployments`).
    pub fn plural(self) -> &'static str {
        match self {
            Self::Namespace => "namespaces",
            Self::Deployment => "deployments",
            Self::StatefulSet => "statefulsets",
            Self::DaemonSet => "daemonsets",
            Self::Job => "jobs",
            Self::CronJob => "cronjobs",
            Self::Pod => "pods",
            Self::Service => "services",
            Self::Ingress => "ingresses",
            Self::ConfigMap => "configmaps",
            Self::Secret => "secrets",
            Self::Node => "nodes",
            Self::Event => "events",
        }
    }

    /// Whether it lives in a Namespace.
    pub fn namespaced(self) -> bool {
        !matches!(self, Self::Namespace | Self::Node)
    }
}
