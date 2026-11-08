//! A type of object a Cluster serves, named the way its API names it, so an
//! action or a page can reach any of them: the kinds Homeport shows on its
//! own pages and those it only finds through discovery (CRDs included).

use kube::{
    api::{ApiResource, DynamicObject, GroupVersionKind},
    Api,
};
use serde::{Deserialize, Serialize};

use super::ResourceKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ApiType {
    /// Empty for the core group (Pods, Services…).
    pub group: String,
    pub version: String,
    pub kind: String,
    /// The plural of its API resource, as RBAC names it (`deployments`).
    pub plural: String,
    /// Whether its objects live in a Namespace.
    pub namespaced: bool,
}

impl ApiType {
    /// `apps/v1`, or `v1` for the core group.
    pub fn api_version(&self) -> String {
        if self.group.is_empty() {
            self.version.clone()
        } else {
            format!("{}/{}", self.group, self.version)
        }
    }

    /// `deployments.apps`, or `pods` for the core group: what kubectl takes.
    pub fn resource(&self) -> String {
        if self.group.is_empty() {
            self.plural.clone()
        } else {
            format!("{}.{}", self.plural, self.group)
        }
    }

    pub fn api_resource(&self) -> ApiResource {
        ApiResource::from_gvk_with_plural(
            &GroupVersionKind::gvk(&self.group, &self.version, &self.kind),
            &self.plural,
        )
    }

    /// Its dynamic API in `namespace`, or across every Namespace (and for
    /// a type that isn't in one) with `None`.
    pub fn api(&self, client: kube::Client, namespace: Option<&str>) -> Api<DynamicObject> {
        let ar = self.api_resource();
        match namespace.filter(|_| self.namespaced) {
            Some(ns) => Api::namespaced_with(client, ns, &ar),
            None => Api::all_with(client, &ar),
        }
    }
}

impl ResourceKind {
    /// Its API type.
    pub fn api_type(self) -> ApiType {
        let (group, version) = match self {
            Self::Deployment | Self::StatefulSet | Self::DaemonSet => ("apps", "v1"),
            Self::Job | Self::CronJob => ("batch", "v1"),
            Self::Ingress => ("networking.k8s.io", "v1"),
            Self::Namespace
            | Self::Pod
            | Self::Service
            | Self::ConfigMap
            | Self::Secret
            | Self::Node
            | Self::Event => ("", "v1"),
        };
        ApiType {
            group: group.into(),
            version: version.into(),
            kind: self.kind().into(),
            plural: self.plural().into(),
            namespaced: self.namespaced(),
        }
    }
}
