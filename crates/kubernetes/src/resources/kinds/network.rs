//! Services and Ingresses: how what runs on a Cluster is reached.

use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        core::v1::Service,
        networking::v1::{Ingress, IngressBackend},
    },
    apimachinery::pkg::util::intstr::IntOrString,
};
use serde::Serialize;

use crate::resources::common::{annotations, created, labels, selector_string};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterServiceView {
    pub namespace: String,
    pub name: String,
    /// `ClusterIP`, `NodePort`, `LoadBalancer` or `ExternalName`.
    #[serde(rename = "type")]
    pub kind: String,
    pub cluster_ip: Option<String>,
    /// Load balancers' and external addresses, or an ExternalName's name.
    pub external: Vec<String>,
    /// `80/TCP`, `443:30443/TCP` (with a node port), as kubectl shows them.
    pub ports: Vec<String>,
    /// The labels of the Pods it sends traffic to, as `key=value,…`; empty
    /// when it selects none.
    pub selector: String,
    pub created: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterServicePort {
    pub name: Option<String>,
    pub port: i32,
    /// The Pods' port, by number or by the name of a container port.
    pub target_port: String,
    pub node_port: Option<i32>,
    pub protocol: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterServiceDetail {
    pub view: ClusterServiceView,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    pub ports: Vec<ClusterServicePort>,
    pub session_affinity: Option<String>,
}

fn int_or_string(v: &IntOrString) -> String {
    match v {
        IntOrString::Int(n) => n.to_string(),
        IntOrString::String(s) => s.clone(),
    }
}

pub fn service(s: &Service) -> ClusterServiceView {
    let spec = s.spec.as_ref();
    let mut external: Vec<String> = s
        .status
        .as_ref()
        .and_then(|st| st.load_balancer.as_ref()?.ingress.clone())
        .into_iter()
        .flatten()
        .filter_map(|i| i.ip.or(i.hostname))
        .collect();
    external.extend(
        spec.and_then(|s| s.external_ips.clone())
            .into_iter()
            .flatten(),
    );
    if let Some(name) = spec.and_then(|s| s.external_name.clone()) {
        external.push(name);
    }
    ClusterServiceView {
        namespace: s.metadata.namespace.clone().unwrap_or_default(),
        name: s.metadata.name.clone().unwrap_or_default(),
        kind: spec
            .and_then(|s| s.type_.clone())
            .unwrap_or_else(|| "ClusterIP".into()),
        cluster_ip: spec
            .and_then(|s| s.cluster_ip.clone())
            .filter(|ip| !ip.is_empty() && ip != "None"),
        external,
        ports: spec
            .and_then(|s| s.ports.as_ref())
            .into_iter()
            .flatten()
            .map(|p| {
                let protocol = p.protocol.as_deref().unwrap_or("TCP");
                match p.node_port {
                    Some(node) => format!("{}:{node}/{protocol}", p.port),
                    None => format!("{}/{protocol}", p.port),
                }
            })
            .collect(),
        selector: spec
            .and_then(|s| s.selector.as_ref())
            .map(selector_string)
            .unwrap_or_default(),
        created: created(&s.metadata),
    }
}

pub fn service_detail(s: &Service) -> ClusterServiceDetail {
    let spec = s.spec.as_ref();
    ClusterServiceDetail {
        view: service(s),
        labels: labels(&s.metadata),
        annotations: annotations(&s.metadata),
        ports: spec
            .and_then(|s| s.ports.as_ref())
            .into_iter()
            .flatten()
            .map(|p| ClusterServicePort {
                name: p.name.clone(),
                port: p.port,
                target_port: p
                    .target_port
                    .as_ref()
                    .map_or_else(|| p.port.to_string(), int_or_string),
                node_port: p.node_port,
                protocol: p.protocol.clone().unwrap_or_else(|| "TCP".into()),
            })
            .collect(),
        session_affinity: spec.and_then(|s| s.session_affinity.clone()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IngressView {
    pub namespace: String,
    pub name: String,
    pub class: Option<String>,
    pub hosts: Vec<String>,
    /// The addresses its controller published.
    pub addresses: Vec<String>,
    pub tls: bool,
    pub created: Option<String>,
}

/// Where one path of an Ingress sends traffic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IngressPath {
    /// `*` for every host.
    pub host: String,
    pub path: String,
    pub path_type: String,
    /// The Service it sends to, when it sends to one.
    pub service: Option<String>,
    /// That Service's port, by number or name.
    pub port: Option<String>,
    /// Another backend (a resource), described.
    pub resource: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IngressTls {
    pub hosts: Vec<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IngressDetail {
    pub view: IngressView,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    /// Every rule's paths, the default backend's first (host `*`, path `*`).
    pub paths: Vec<IngressPath>,
    pub tls: Vec<IngressTls>,
}

fn backend(host: &str, path: &str, path_type: &str, b: &IngressBackend) -> IngressPath {
    let service = b.service.as_ref();
    IngressPath {
        host: host.to_owned(),
        path: path.to_owned(),
        path_type: path_type.to_owned(),
        service: service.map(|s| s.name.clone()),
        port: service.and_then(|s| {
            let port = s.port.as_ref()?;
            port.number
                .map(|n| n.to_string())
                .or_else(|| port.name.clone())
        }),
        resource: b
            .resource
            .as_ref()
            .map(|r| format!("{} {}", r.kind, r.name)),
    }
}

pub fn ingress(i: &Ingress) -> IngressView {
    let spec = i.spec.as_ref();
    let mut hosts: Vec<String> = spec
        .and_then(|s| s.rules.as_ref())
        .into_iter()
        .flatten()
        .filter_map(|r| r.host.clone())
        .collect();
    hosts.dedup();
    IngressView {
        namespace: i.metadata.namespace.clone().unwrap_or_default(),
        name: i.metadata.name.clone().unwrap_or_default(),
        class: spec.and_then(|s| s.ingress_class_name.clone()).or_else(|| {
            i.metadata
                .annotations
                .as_ref()?
                .get("kubernetes.io/ingress.class")
                .cloned()
        }),
        hosts,
        addresses: i
            .status
            .as_ref()
            .and_then(|s| s.load_balancer.as_ref()?.ingress.clone())
            .into_iter()
            .flatten()
            .filter_map(|lb| lb.ip.or(lb.hostname))
            .collect(),
        tls: spec
            .and_then(|s| s.tls.as_ref())
            .is_some_and(|t| !t.is_empty()),
        created: created(&i.metadata),
    }
}

pub fn ingress_detail(i: &Ingress) -> IngressDetail {
    let spec = i.spec.as_ref();
    let mut paths = vec![];
    if let Some(default) = spec.and_then(|s| s.default_backend.as_ref()) {
        paths.push(backend("*", "*", "", default));
    }
    for rule in spec.and_then(|s| s.rules.as_ref()).into_iter().flatten() {
        let host = rule.host.as_deref().unwrap_or("*");
        for p in rule.http.iter().flat_map(|h| &h.paths) {
            paths.push(backend(
                host,
                p.path.as_deref().unwrap_or("/"),
                &p.path_type,
                &p.backend,
            ));
        }
    }
    IngressDetail {
        view: ingress(i),
        labels: labels(&i.metadata),
        annotations: annotations(&i.metadata),
        paths,
        tls: spec
            .and_then(|s| s.tls.as_ref())
            .into_iter()
            .flatten()
            .map(|t| IngressTls {
                hosts: t.hosts.clone().unwrap_or_default(),
                secret: t.secret_name.clone(),
            })
            .collect(),
    }
}
