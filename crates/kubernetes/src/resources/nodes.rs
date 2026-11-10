//! Nodes: the machines a Cluster runs Pods on.

use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::Node;
use serde::Serialize;

use crate::metrics::quantity;

use super::common::{annotations, conditions, created, labels, ConditionView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNodeView {
    pub name: String,
    /// Its Ready condition is true.
    pub ready: bool,
    /// Cordoned: no new Pod is scheduled on it.
    pub unschedulable: bool,
    /// From its `node-role.kubernetes.io/<role>` labels.
    pub roles: Vec<String>,
    /// The kubelet's version.
    pub version: String,
    pub os: String,
    pub arch: String,
    pub internal_ip: Option<String>,
    /// Allocatable CPU and memory, as Kubernetes writes them (`4`, `16302048Ki`).
    pub cpu: Option<String>,
    pub memory: Option<String>,
    /// The same in millicores and bytes, to set what it uses against.
    pub cpu_millicores: Option<u64>,
    pub memory_bytes: Option<u64>,
    pub taints: u32,
    pub created: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NodeTaint {
    pub key: String,
    pub value: Option<String>,
    pub effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNodeDetail {
    pub view: ClusterNodeView,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    pub conditions: Vec<ConditionView>,
    pub capacity: BTreeMap<String, String>,
    pub allocatable: BTreeMap<String, String>,
    pub taints: Vec<NodeTaint>,
    pub os_image: Option<String>,
    pub kernel: Option<String>,
    pub container_runtime: Option<String>,
    /// `InternalIP 10.0.0.4`, `Hostname node-a`…
    pub addresses: Vec<String>,
}

const ROLE: &str = "node-role.kubernetes.io/";

pub fn view(n: &Node) -> ClusterNodeView {
    let status = n.status.as_ref();
    let info = status.and_then(|s| s.node_info.as_ref());
    let mut roles: Vec<String> = n
        .metadata
        .labels
        .iter()
        .flatten()
        .filter_map(|(k, _)| k.strip_prefix(ROLE).map(str::to_owned))
        .filter(|r| !r.is_empty())
        .collect();
    roles.sort();
    let allocatable = status.and_then(|s| s.allocatable.as_ref());
    ClusterNodeView {
        name: n.metadata.name.clone().unwrap_or_default(),
        ready: status
            .and_then(|s| s.conditions.as_ref())
            .into_iter()
            .flatten()
            .any(|c| c.type_ == "Ready" && c.status == "True"),
        unschedulable: n.spec.as_ref().and_then(|s| s.unschedulable) == Some(true),
        roles,
        version: info.map(|i| i.kubelet_version.clone()).unwrap_or_default(),
        os: info.map(|i| i.operating_system.clone()).unwrap_or_default(),
        arch: info.map(|i| i.architecture.clone()).unwrap_or_default(),
        internal_ip: status
            .and_then(|s| s.addresses.as_ref())
            .into_iter()
            .flatten()
            .find(|a| a.type_ == "InternalIP")
            .map(|a| a.address.clone()),
        cpu: allocatable.and_then(|a| a.get("cpu")).map(|q| q.0.clone()),
        memory: allocatable
            .and_then(|a| a.get("memory"))
            .map(|q| q.0.clone()),
        cpu_millicores: allocatable
            .and_then(|a| a.get("cpu"))
            .and_then(|q| quantity::whole_millicores(&q.0)),
        memory_bytes: allocatable
            .and_then(|a| a.get("memory"))
            .and_then(|q| quantity::bytes(&q.0)),
        taints: n
            .spec
            .as_ref()
            .and_then(|s| s.taints.as_ref())
            .map_or(0, |t| t.len() as u32),
        created: created(&n.metadata),
    }
}

pub fn detail(n: &Node) -> ClusterNodeDetail {
    let status = n.status.as_ref();
    let info = status.and_then(|s| s.node_info.as_ref());
    let quantities =
        |m: Option<&BTreeMap<String, k8s_openapi::apimachinery::pkg::api::resource::Quantity>>| {
            m.into_iter()
                .flatten()
                .map(|(k, v)| (k.clone(), v.0.clone()))
                .collect()
        };
    ClusterNodeDetail {
        view: view(n),
        labels: labels(&n.metadata),
        annotations: annotations(&n.metadata),
        conditions: conditions!(status.and_then(|s| s.conditions.as_ref())),
        capacity: quantities(status.and_then(|s| s.capacity.as_ref())),
        allocatable: quantities(status.and_then(|s| s.allocatable.as_ref())),
        taints: n
            .spec
            .as_ref()
            .and_then(|s| s.taints.as_ref())
            .into_iter()
            .flatten()
            .map(|t| NodeTaint {
                key: t.key.clone(),
                value: t.value.clone(),
                effect: t.effect.clone(),
            })
            .collect(),
        os_image: info.map(|i| i.os_image.clone()),
        kernel: info.map(|i| i.kernel_version.clone()),
        container_runtime: info.map(|i| i.container_runtime_version.clone()),
        addresses: status
            .and_then(|s| s.addresses.as_ref())
            .into_iter()
            .flatten()
            .map(|a| format!("{} {}", a.type_, a.address))
            .collect(),
    }
}
