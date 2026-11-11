//! A route as the path the React router knew it by, and back. The app has
//! no address bar: paths name routes in the palette's items, in logs and in
//! the tests that hold the routes to the manifests' patterns.

use super::route::{ApiKind, Namespaced, Route, Tab};

/// A segment as it is written in a path: `/` and `%` can't stand for
/// themselves there.
fn encode(segment: &str) -> String {
    segment.replace('%', "%25").replace('/', "%2F")
}

fn decode(segment: &str) -> String {
    segment.replace("%2F", "/").replace("%25", "%")
}

fn join(parts: &[&str], tab: Option<&Tab>) -> String {
    let mut path = String::new();
    let tab = tab.and_then(|tab| tab.as_deref());
    for part in parts.iter().copied().chain(tab) {
        path.push('/');
        path.push_str(&encode(part));
    }
    path
}

impl Route {
    /// The path of this route: `/containers/abc/logs`.
    pub fn path(&self) -> String {
        let fixed = |path: &str| path.to_owned();
        match self {
            Self::Overview => fixed("/"),
            Self::Board => fixed("/board"),
            Self::Containers => fixed("/containers"),
            Self::Container { id, tab } => join(&["containers", id], Some(tab)),
            Self::Images => fixed("/images"),
            Self::Volumes => fixed("/volumes"),
            Self::Networks => fixed("/networks"),
            Self::Stacks => fixed("/stacks"),
            Self::NewStack => fixed("/stacks/new"),
            Self::Stack { name } => join(&["stacks", name], None),
            Self::PodmanPods => fixed("/podman-pods"),
            Self::PodmanPod { id } => join(&["podman-pods", id], None),
            Self::Quadlets => fixed("/quadlets"),
            Self::Quadlet { name } => join(&["quadlets", name], None),
            Self::ContainerdContainers => fixed("/containerd/containers"),
            Self::ContainerdContainer { id, tab } => {
                join(&["containerd", "containers", id], Some(tab))
            }
            Self::ContainerdImages => fixed("/containerd/images"),
            Self::ContainerdSnapshots => fixed("/containerd/snapshots"),
            Self::ContainerdNamespaces => fixed("/containerd/namespaces"),
            Self::IncusInstances => fixed("/incus/instances"),
            Self::IncusInstance { name, tab } => join(&["incus", "instances", name], Some(tab)),
            Self::IncusImages => fixed("/incus/images"),
            Self::IncusProfiles => fixed("/incus/profiles"),
            Self::IncusProfile { name } => join(&["incus", "profiles", name], None),
            Self::IncusStorage { pool } => join(&["incus", "storage"], Some(pool)),
            Self::IncusNetworks => fixed("/incus/networks"),
            Self::IncusNetwork { name } => join(&["incus", "networks", name], None),
            Self::Swarm { swarm_id, tab } => join(&["swarms", swarm_id], Some(tab)),
            Self::SwarmService {
                swarm_id,
                service_id,
                tab,
            } => join(&["swarms", swarm_id, "services", service_id], Some(tab)),
            Self::SwarmStack { swarm_id, name } => {
                join(&["swarms", swarm_id, "stacks", name], None)
            }
            Self::Workloads { kind } => join(&["workloads"], Some(kind)),
            Self::Workload { kind, object, tab } => join(
                &["workloads", kind, &object.namespace, &object.name],
                Some(tab),
            ),
            Self::Pods => fixed("/pods"),
            Self::Pod { object, tab } => namespaced("pods", object, tab),
            Self::Services => fixed("/services"),
            Self::Service { object, tab } => namespaced("services", object, tab),
            Self::Ingresses => fixed("/ingresses"),
            Self::Ingress { object, tab } => namespaced("ingresses", object, tab),
            Self::ConfigMaps => fixed("/configmaps"),
            Self::ConfigMap { object, tab } => namespaced("configmaps", object, tab),
            Self::Secrets => fixed("/secrets"),
            Self::Secret { object, tab } => namespaced("secrets", object, tab),
            Self::Nodes => fixed("/nodes"),
            Self::Node { name, tab } => join(&["nodes", name], Some(tab)),
            Self::ClusterEvents => fixed("/cluster-events"),
            Self::ApiResources { api: None } => fixed("/api-resources"),
            Self::ApiResources { api: Some(api) } => join(
                &["api-resources", &api.group, &api.version, &api.plural],
                None,
            ),
            Self::ApiObject { api, object, tab } => join(
                &[
                    "api-resources",
                    &api.group,
                    &api.version,
                    &api.plural,
                    &object.namespace,
                    &object.name,
                ],
                Some(tab),
            ),
            Self::Rbac { tab } => join(&["rbac"], Some(tab)),
            Self::ClusterRole { name } => join(&["rbac", "clusterroles", name], None),
            Self::Role { object } => {
                join(&["rbac", "roles", &object.namespace, &object.name], None)
            }
            Self::RbacUser { name } => join(&["rbac", "users", name], None),
            Self::RbacGroup { name } => join(&["rbac", "groups", name], None),
            Self::ServiceAccount { object } => join(
                &["rbac", "serviceaccounts", &object.namespace, &object.name],
                None,
            ),
            Self::Apply => fixed("/apply"),
            Self::HelmReleases => fixed("/helm"),
            Self::HelmRepositories => fixed("/helm/repositories"),
            Self::HelmRelease { object, tab } => namespaced("helm", object, tab),
            Self::PortForwards => fixed("/port-forwards"),
            Self::NomadJobs => fixed("/nomad/jobs"),
            Self::NomadJob { namespace, job_id } => {
                join(&["nomad", "jobs", namespace, job_id], None)
            }
            Self::NomadAllocation { alloc_id, tab } => {
                join(&["nomad", "allocations", alloc_id], Some(tab))
            }
            Self::NomadNodes => fixed("/nomad/nodes"),
            Self::NomadNode { node_id } => join(&["nomad", "nodes", node_id], None),
            Self::Events => fixed("/events"),
            Self::Settings => fixed("/settings"),
            Self::Hosts => fixed("/hosts"),
            Self::AddHost => fixed("/hosts/new"),
            Self::Host { host_id } => join(&["hosts", host_id], None),
            Self::Clusters => fixed("/clusters"),
            Self::AddCluster => fixed("/clusters/new"),
            Self::Cluster { cluster_id } => join(&["clusters", cluster_id], None),
        }
    }

    /// The route a path names, as the React router matched it: a fixed
    /// segment wins over a parameter. `None` for a path no feature has.
    pub fn parse(path: &str) -> Option<Self> {
        let path = path.split(['?', '#']).next().unwrap_or_default();
        let owned: Vec<String> = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(decode)
            .collect();
        let segments: Vec<&str> = owned.iter().map(String::as_str).collect();
        let s = |text: &str| text.to_owned();
        let tab = |tab: &[&str]| tab.first().map(|tab| s(tab));
        Some(match segments.as_slice() {
            [] => Self::Overview,
            ["board"] => Self::Board,
            ["containers"] => Self::Containers,
            ["containers", id, rest @ ..] if rest.len() <= 1 => Self::Container {
                id: s(id),
                tab: tab(rest),
            },
            ["images"] => Self::Images,
            ["volumes"] => Self::Volumes,
            ["networks"] => Self::Networks,
            ["stacks"] => Self::Stacks,
            ["stacks", "new"] => Self::NewStack,
            ["stacks", name] => Self::Stack { name: s(name) },
            ["podman-pods"] => Self::PodmanPods,
            ["podman-pods", id] => Self::PodmanPod { id: s(id) },
            ["quadlets"] => Self::Quadlets,
            ["quadlets", name] => Self::Quadlet { name: s(name) },
            ["containerd", "containers"] => Self::ContainerdContainers,
            ["containerd", "containers", id, rest @ ..] if rest.len() <= 1 => {
                Self::ContainerdContainer {
                    id: s(id),
                    tab: tab(rest),
                }
            }
            ["containerd", "images"] => Self::ContainerdImages,
            ["containerd", "snapshots"] => Self::ContainerdSnapshots,
            ["containerd", "namespaces"] => Self::ContainerdNamespaces,
            ["incus", "instances"] => Self::IncusInstances,
            ["incus", "instances", name, rest @ ..] if rest.len() <= 1 => Self::IncusInstance {
                name: s(name),
                tab: tab(rest),
            },
            ["incus", "images"] => Self::IncusImages,
            ["incus", "profiles"] => Self::IncusProfiles,
            ["incus", "profiles", name] => Self::IncusProfile { name: s(name) },
            ["incus", "storage", rest @ ..] if rest.len() <= 1 => {
                Self::IncusStorage { pool: tab(rest) }
            }
            ["incus", "networks"] => Self::IncusNetworks,
            ["incus", "networks", name] => Self::IncusNetwork { name: s(name) },
            ["swarms", id, "services", service, rest @ ..] if rest.len() <= 1 => {
                Self::SwarmService {
                    swarm_id: s(id),
                    service_id: s(service),
                    tab: tab(rest),
                }
            }
            ["swarms", id, "stacks", name] => Self::SwarmStack {
                swarm_id: s(id),
                name: s(name),
            },
            ["swarms", id, rest @ ..] if rest.len() <= 1 => Self::Swarm {
                swarm_id: s(id),
                tab: tab(rest),
            },
            ["events"] => Self::Events,
            ["settings"] => Self::Settings,
            ["hosts"] => Self::Hosts,
            ["hosts", "new"] => Self::AddHost,
            ["hosts", id] => Self::Host { host_id: s(id) },
            ["clusters"] => Self::Clusters,
            ["clusters", "new"] => Self::AddCluster,
            ["clusters", id] => Self::Cluster { cluster_id: s(id) },
            other => return Self::of_a_cluster(other),
        })
    }

    /// The routes of the Cluster pages.
    fn of_a_cluster(segments: &[&str]) -> Option<Self> {
        let s = |text: &str| text.to_owned();
        let tab = |tab: &[&str]| tab.first().map(|tab| s(tab));
        let object = |namespace: &str, name: &str| Namespaced::new(namespace, name);
        Some(match segments {
            ["workloads", rest @ ..] if rest.len() <= 1 => Self::Workloads { kind: tab(rest) },
            ["workloads", kind, namespace, name, rest @ ..] if rest.len() <= 1 => Self::Workload {
                kind: s(kind),
                object: object(namespace, name),
                tab: tab(rest),
            },
            ["pods"] => Self::Pods,
            ["services"] => Self::Services,
            ["ingresses"] => Self::Ingresses,
            ["configmaps"] => Self::ConfigMaps,
            ["secrets"] => Self::Secrets,
            [list @ ("pods" | "services" | "ingresses" | "configmaps" | "secrets"), namespace, name, rest @ ..]
                if rest.len() <= 1 =>
            {
                let (object, tab) = (object(namespace, name), tab(rest));
                match *list {
                    "pods" => Self::Pod { object, tab },
                    "services" => Self::Service { object, tab },
                    "ingresses" => Self::Ingress { object, tab },
                    "configmaps" => Self::ConfigMap { object, tab },
                    _ => Self::Secret { object, tab },
                }
            }
            ["nodes"] => Self::Nodes,
            ["nodes", name, rest @ ..] if rest.len() <= 1 => Self::Node {
                name: s(name),
                tab: tab(rest),
            },
            ["cluster-events"] => Self::ClusterEvents,
            ["api-resources"] => Self::ApiResources { api: None },
            ["api-resources", group, version, plural, rest @ ..] => {
                let api = ApiKind {
                    group: s(group),
                    version: s(version),
                    plural: s(plural),
                };
                match rest {
                    [] => Self::ApiResources { api: Some(api) },
                    [namespace, name, rest @ ..] if rest.len() <= 1 => Self::ApiObject {
                        api,
                        object: object(namespace, name),
                        tab: tab(rest),
                    },
                    _ => return None,
                }
            }
            ["rbac", "clusterroles", name] => Self::ClusterRole { name: s(name) },
            ["rbac", "roles", namespace, name] => Self::Role {
                object: object(namespace, name),
            },
            ["rbac", "users", name] => Self::RbacUser { name: s(name) },
            ["rbac", "groups", name] => Self::RbacGroup { name: s(name) },
            ["rbac", "serviceaccounts", namespace, name] => Self::ServiceAccount {
                object: object(namespace, name),
            },
            ["rbac", rest @ ..] if rest.len() <= 1 => Self::Rbac { tab: tab(rest) },
            ["apply"] => Self::Apply,
            ["helm"] => Self::HelmReleases,
            ["helm", "repositories"] => Self::HelmRepositories,
            ["helm", namespace, name, rest @ ..] if rest.len() <= 1 => Self::HelmRelease {
                object: object(namespace, name),
                tab: tab(rest),
            },
            ["port-forwards"] => Self::PortForwards,
            ["nomad", "jobs"] => Self::NomadJobs,
            ["nomad", "jobs", namespace, job] => Self::NomadJob {
                namespace: s(namespace),
                job_id: s(job),
            },
            ["nomad", "allocations", alloc, rest @ ..] if rest.len() <= 1 => {
                Self::NomadAllocation {
                    alloc_id: s(alloc),
                    tab: tab(rest),
                }
            }
            ["nomad", "nodes"] => Self::NomadNodes,
            ["nomad", "nodes", node] => Self::NomadNode { node_id: s(node) },
            _ => return None,
        })
    }
}

fn namespaced(list: &str, object: &Namespaced, tab: &Tab) -> String {
    join(&[list, &object.namespace, &object.name], Some(tab))
}
