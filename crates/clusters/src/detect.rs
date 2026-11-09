//! Finds the Kubernetes clusters running on this computer (Docker Desktop,
//! OrbStack, Rancher Desktop, kind, k3d, minikube) among the kubeconfig's
//! contexts, so the first Cluster is one click. Homeport starts or creates
//! none of them.

use std::{net::IpAddr, time::Duration};

use futures_util::future::join_all;
use kube::config::{Kubeconfig, NamedExtension};
use serde::Serialize;

use crate::{
    client,
    kubeconfig::{self, AddedCluster, Source},
    model::{Cluster, ClusterConnection, ClusterKind},
};

/// How long a local API server may take to say its version.
const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

/// What runs a local cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum LocalTool {
    DockerDesktop,
    Orbstack,
    RancherDesktop,
    Kind,
    K3d,
    Minikube,
}

/// A local cluster found in the kubeconfig.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LocalCluster {
    pub tool: LocalTool,
    /// The context it is reached through.
    pub context: String,
    /// The name its Cluster gets when added in one click.
    pub suggested_name: String,
    pub server: String,
    /// Its API server answered, with this version.
    pub server_version: Option<String>,
    /// The Cluster already made from its context.
    pub added: Option<AddedCluster>,
}

/// A kind or k3d cluster whose containers run on a local Host, but that
/// the kubeconfig doesn't reach yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MissingLocalCluster {
    pub tool: LocalTool,
    pub name: String,
    /// The command that writes it into the kubeconfig.
    pub command: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LocalClusters {
    pub found: Vec<LocalCluster>,
    pub missing: Vec<MissingLocalCluster>,
}

/// The tool behind the context `name` reaching `server`, if it's a local
/// one: by its name, and only with a server on this computer or its VM.
pub fn recognise(name: &str, server: &str, minikube: bool) -> Option<LocalTool> {
    let host = server_host(server)?;
    let tool = match name {
        "docker-desktop" | "docker-for-desktop" => LocalTool::DockerDesktop,
        "orbstack" => LocalTool::Orbstack,
        "rancher-desktop" => LocalTool::RancherDesktop,
        _ if minikube || name == "minikube" => LocalTool::Minikube,
        _ if name.starts_with("kind-") && name.len() > 5 => LocalTool::Kind,
        _ if name.starts_with("k3d-") && name.len() > 4 => LocalTool::K3d,
        _ => return None,
    };
    let local = is_loopback(&host)
        || match tool {
            LocalTool::DockerDesktop => host == "kubernetes.docker.internal",
            LocalTool::Orbstack => host == "k8s.orb.local" || host.ends_with(".orb.local"),
            // These run in a VM on a private address of this computer.
            LocalTool::Minikube | LocalTool::RancherDesktop => is_private(&host),
            LocalTool::Kind | LocalTool::K3d => host == "host.docker.internal",
        };
    local.then_some(tool)
}

/// The host part of a server URL, without brackets.
fn server_host(server: &str) -> Option<String> {
    let uri: http::Uri = server.parse().ok()?;
    Some(uri.host()?.trim_matches(['[', ']']).to_ascii_lowercase())
}

fn is_loopback(host: &str) -> bool {
    host == "localhost"
        || host.ends_with(".localhost")
        || host
            .parse::<IpAddr>()
            .is_ok_and(|ip| ip.is_loopback() || ip.is_unspecified())
}

fn is_private(host: &str) -> bool {
    match host.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => ip.is_private() || ip.is_link_local(),
        Ok(IpAddr::V6(ip)) => (ip.segments()[0] & 0xfe00) == 0xfc00,
        Err(_) => false,
    }
}

/// Whether minikube wrote this context or its cluster (its `context_info`
/// extension names `minikube.sigs.k8s.io`).
fn by_minikube(extensions: Option<&Vec<NamedExtension>>) -> bool {
    extensions.into_iter().flatten().any(|e| {
        e.extension
            .get("provider")
            .and_then(|p| p.as_str())
            .is_some_and(|p| p.starts_with("minikube"))
    })
}

/// The name a local cluster's Cluster gets: the tool for one-of-a-kind ones,
/// the context otherwise.
fn suggested_name(tool: LocalTool, context: &str) -> String {
    match tool {
        LocalTool::DockerDesktop => "Docker Desktop".into(),
        LocalTool::Orbstack => "OrbStack".into(),
        LocalTool::RancherDesktop => "Rancher Desktop".into(),
        _ => context.into(),
    }
}

/// The local contexts of `config`, before probing them.
fn candidates(config: &Kubeconfig) -> Vec<(LocalTool, String, String)> {
    config
        .contexts
        .iter()
        .filter_map(|named| {
            let context = named.context.as_ref()?;
            let cluster = config.clusters.iter().find(|c| c.name == context.cluster)?;
            let server = cluster.cluster.as_ref()?.server.clone()?;
            let minikube = by_minikube(context.extensions.as_ref())
                || by_minikube(cluster.cluster.as_ref()?.extensions.as_ref());
            let tool = recognise(&named.name, &server, minikube)?;
            Some((tool, named.name.clone(), server))
        })
        .collect()
}

/// The local clusters this computer's kubeconfig reaches, each probed; an
/// empty list without a kubeconfig.
pub async fn detect(clusters: &[Cluster]) -> Vec<LocalCluster> {
    let Ok(source) = Source::local() else {
        return vec![];
    };
    let probes = candidates(&source.config)
        .into_iter()
        .map(|(tool, context, server)| {
            let source = &source;
            async move {
                LocalCluster {
                    tool,
                    suggested_name: suggested_name(tool, &context),
                    server_version: probe(source, &context).await,
                    added: clusters
                        .iter()
                        .find(|c| {
                            c.kubernetes()
                                .and_then(|k| k.source.as_ref())
                                .is_some_and(|s| s.context == context)
                        })
                        .map(|c| AddedCluster {
                            id: c.id.clone(),
                            name: c.name.clone(),
                        }),
                    context,
                    server,
                }
            }
        });
    join_all(probes).await
}

/// The version of the context's API server, `None` when it doesn't answer.
async fn probe(source: &Source, context: &str) -> Option<String> {
    let (connection, secrets) = kubeconfig::resolve(source, context).ok()?;
    if secrets.exec.is_some() {
        // Local clusters don't sign in with plugins; don't run one to probe.
        return None;
    }
    let cluster = Cluster {
        id: String::new(),
        name: context.to_owned(),
        kind: ClusterKind::Kubernetes,
        connection: ClusterConnection::Kubernetes(connection),
        enabled: true,
        created_at: String::new(),
    };
    let client = client::client(&cluster, &secrets, None).ok()?;
    match tokio::time::timeout(PROBE_TIMEOUT, client.apiserver_version()).await {
        Ok(Ok(info)) => Some(info.git_version),
        _ => None,
    }
}

/// The kind and k3d clusters named by container labels (`io.x-k8s.kind.cluster`,
/// `k3d.cluster`) that no context of the kubeconfig reaches.
pub fn missing<'a>(
    labels: impl IntoIterator<Item = &'a std::collections::HashMap<String, String>>,
    contexts: &[String],
) -> Vec<MissingLocalCluster> {
    let mut missing: Vec<MissingLocalCluster> = vec![];
    for labels in labels {
        let found = [
            ("io.x-k8s.kind.cluster", LocalTool::Kind, "kind-"),
            ("k3d.cluster", LocalTool::K3d, "k3d-"),
        ]
        .into_iter()
        .find_map(|(label, tool, prefix)| Some((labels.get(label)?, tool, prefix)));
        let Some((name, tool, prefix)) = found else {
            continue;
        };
        let context = format!("{prefix}{name}");
        if contexts.contains(&context) || missing.iter().any(|m| &m.name == name && m.tool == tool)
        {
            continue;
        }
        let command = match tool {
            LocalTool::Kind => format!("kind export kubeconfig --name {name}"),
            _ => format!("k3d kubeconfig merge {name} --kubeconfig-merge-default"),
        };
        missing.push(MissingLocalCluster {
            tool,
            name: name.clone(),
            command,
        });
    }
    missing
}

/// The names of this computer's kubeconfig contexts, none without one.
pub fn local_contexts() -> Vec<String> {
    Source::local()
        .map(|s| s.config.contexts.into_iter().map(|c| c.name).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn local_tools_are_recognised_by_context_and_server() {
        for (name, server, tool) in [
            (
                "docker-desktop",
                "https://kubernetes.docker.internal:6443",
                LocalTool::DockerDesktop,
            ),
            (
                "docker-desktop",
                "https://127.0.0.1:6443",
                LocalTool::DockerDesktop,
            ),
            ("orbstack", "https://127.0.0.1:26443", LocalTool::Orbstack),
            (
                "rancher-desktop",
                "https://127.0.0.1:6443",
                LocalTool::RancherDesktop,
            ),
            (
                "rancher-desktop",
                "https://192.168.5.15:6443",
                LocalTool::RancherDesktop,
            ),
            ("kind-dev", "https://127.0.0.1:41235", LocalTool::Kind),
            ("k3d-shop", "https://0.0.0.0:6550", LocalTool::K3d),
            ("minikube", "https://192.168.49.2:8443", LocalTool::Minikube),
            ("kind-ipv6", "https://[::1]:6443", LocalTool::Kind),
        ] {
            assert_eq!(
                recognise(name, server, false),
                Some(tool),
                "{name} {server}"
            );
        }
        assert_eq!(
            recognise("staging", "https://192.168.59.100:8443", true),
            Some(LocalTool::Minikube),
            "a minikube profile, by its extension"
        );
    }

    #[test]
    fn remote_contexts_are_never_local() {
        for (name, server) in [
            ("kind-dev", "https://k8s.example.com:6443"),
            ("k3d-prod", "https://10.0.0.5:6443"),
            ("docker-desktop", "https://203.0.113.7:6443"),
            ("minikube", "https://k8s.example.com"),
            ("prod", "https://127.0.0.1:6443"),
            ("kind-", "https://127.0.0.1:6443"),
            ("eks", "https://ABC.gr7.eu-west-1.eks.amazonaws.com"),
        ] {
            assert_eq!(recognise(name, server, false), None, "{name} {server}");
        }
    }

    #[test]
    fn minikube_marks_its_entries() {
        let config = Kubeconfig::from_yaml(
            r#"clusters:
- name: work
  cluster:
    server: https://192.168.58.2:8443
    extensions:
    - name: cluster_info
      extension: { provider: minikube.sigs.k8s.io, version: v1.34.0 }
- name: prod
  cluster: { server: "https://192.168.1.10:6443" }
contexts:
- name: work
  context: { cluster: work }
- name: prod
  context: { cluster: prod }
"#,
        )
        .unwrap();
        assert_eq!(
            candidates(&config),
            [(
                LocalTool::Minikube,
                "work".to_owned(),
                "https://192.168.58.2:8443".to_owned()
            )]
        );
    }

    #[test]
    fn running_kind_and_k3d_clusters_missing_from_the_kubeconfig_say_how_to_add_them() {
        let label = |k: &str, v: &str| HashMap::from([(k.to_owned(), v.to_owned())]);
        let labels = [
            label("io.x-k8s.kind.cluster", "dev"),
            label("io.x-k8s.kind.cluster", "dev"),
            label("io.x-k8s.kind.cluster", "known"),
            label("k3d.cluster", "shop"),
            label("com.docker.compose.project", "web"),
        ];
        let missing = missing(&labels, &["kind-known".to_owned()]);
        assert_eq!(
            missing,
            [
                MissingLocalCluster {
                    tool: LocalTool::Kind,
                    name: "dev".into(),
                    command: "kind export kubeconfig --name dev".into(),
                },
                MissingLocalCluster {
                    tool: LocalTool::K3d,
                    name: "shop".into(),
                    command: "k3d kubeconfig merge shop --kubeconfig-merge-default".into(),
                },
            ]
        );
    }
}
