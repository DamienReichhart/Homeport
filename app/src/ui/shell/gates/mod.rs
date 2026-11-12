//! The gates: what stands before a page until the selection can show it,
//! and says what to do meanwhile (`HostGate`, `ClusterGate`, `SwarmGate`,
//! `ContainerdGate`, `IncusGate`, `HostCapabilityGuard`,
//! `UnsupportedEngine` and `ClusterKindGuard` in React).
//!
//! [`decide`] is the whole decision, pure: a route and the [`World`] give a
//! [`Verdict`]. `view` draws the verdicts that aren't `Open`.

#[cfg(test)]
mod tests;
pub mod view;

use homeport_clusters::{model::ClusterKind, registry::ClusterStatus, service::ClusterView};
use homeport_core::error::AppError;
use homeport_hosts::{registry::HostStatus, service::HostView};

use super::{
    page::World,
    pickers::model,
    registry::{self, Capability, Gate},
    router::Route,
    selection::Selection,
    status::{cluster, engine, host},
};
use crate::t;

/// What an open page is about: what its gate let through.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subject {
    /// The app as a whole.
    App,
    /// The selected Host, connected.
    Host { host_id: String },
    /// A containerd Host and the namespace its pages show.
    Containerd { host_id: String, namespace: String },
    /// An Incus Host and the project its pages show.
    Incus { host_id: String, project: String },
    /// The selected Cluster, connected, and the Namespace its pages show
    /// (`None` for every one).
    Cluster {
        cluster_id: String,
        namespace: Option<String>,
    },
    /// A Swarm one of whose managers answers.
    Swarm { swarm_id: String },
}

/// What a gate decided.
#[derive(Debug, Clone)]
pub enum Verdict<'a> {
    /// The page shows.
    Open(Subject),
    /// Something it needs is still being read.
    Loading,
    /// The Host or Cluster named so is connecting.
    Connecting(&'a str),
    /// No Host is selected: the welcome, with what to add.
    Welcome,
    /// The selected Host's Engine doesn't offer what the page requires.
    Unsupported {
        host: &'a HostView,
        requires: Capability,
    },
    /// The selected Host isn't connected.
    HostDown(&'a HostView),
    /// The Clusters couldn't be read.
    ClustersFailed(&'a AppError),
    /// No Cluster is selected.
    NoCluster,
    /// The selected Cluster is of another kind than the page is for.
    WrongKind {
        cluster: &'a ClusterView,
        wanted: ClusterKind,
    },
    /// The selected Cluster isn't connected.
    ClusterDown(&'a ClusterView),
    /// No Host belongs to the Swarm any more.
    SwarmGone,
    /// No manager of the Swarm answers.
    SwarmFailed(&'a AppError),
}

impl PartialEq for Verdict<'_> {
    /// Two verdicts are the same when they say the same of the same Host
    /// or Cluster.
    fn eq(&self, other: &Self) -> bool {
        use Verdict as V;
        match (self, other) {
            (V::Open(a), V::Open(b)) => a == b,
            (V::Connecting(a), V::Connecting(b)) => a == b,
            (
                V::Unsupported {
                    host: a,
                    requires: x,
                },
                V::Unsupported {
                    host: b,
                    requires: y,
                },
            ) => a.host.id == b.host.id && x == y,
            (V::HostDown(a), V::HostDown(b)) => a.host.id == b.host.id && a.status == b.status,
            (V::ClustersFailed(a), V::ClustersFailed(b))
            | (V::SwarmFailed(a), V::SwarmFailed(b)) => a == b,
            (
                V::WrongKind {
                    cluster: a,
                    wanted: x,
                },
                V::WrongKind {
                    cluster: b,
                    wanted: y,
                },
            ) => a.cluster.id == b.cluster.id && x == y,
            (V::ClusterDown(a), V::ClusterDown(b)) => a == b,
            (V::Loading, V::Loading)
            | (V::Welcome, V::Welcome)
            | (V::NoCluster, V::NoCluster)
            | (V::SwarmGone, V::SwarmGone) => true,
            _ => false,
        }
    }
}

/// What a page that isn't connected says and offers.
#[derive(Debug, Clone, PartialEq)]
pub struct Down<'a> {
    /// `name: status`.
    pub title: String,
    /// The classified cause, shown with its hint, its Details and a Retry.
    pub error: Option<&'a AppError>,
    /// With a cause: when Homeport tries again on its own.
    pub retrying_in: Option<u32>,
    /// Without a cause: the reason, in a line.
    pub detail: Option<String>,
    /// Without a cause, unless it is disabled: a Retry now button.
    pub retry_now: bool,
}

/// What `HostGate` showed for a Host that isn't connected.
pub fn host_down(view: &HostView) -> Down<'_> {
    let summary = host::describe(&view.status);
    let error = host::error(&view.status);
    Down {
        title: t!(
            "core.gate.statusTitle",
            name = view.host.name.as_str(),
            status = summary.label
        ),
        error,
        retrying_in: match &view.status {
            HostStatus::Unreachable { retry_in_secs, .. } => Some(*retry_in_secs),
            _ => None,
        },
        detail: if error.is_some() {
            None
        } else {
            summary.detail
        },
        retry_now: error.is_none() && !matches!(view.status, HostStatus::Disabled),
    }
}

/// What `ClusterGate` showed for a Cluster that isn't connected.
pub fn cluster_down(view: &ClusterView) -> Down<'_> {
    let summary = cluster::describe(&view.status, view.cluster.kind);
    let error = view.status.error();
    Down {
        title: t!(
            "core.gate.statusTitle",
            name = view.cluster.name.as_str(),
            status = summary.label
        ),
        error,
        retrying_in: match &view.status {
            ClusterStatus::Unreachable { retry_in_secs, .. } => Some(*retry_in_secs),
            _ => None,
        },
        detail: if error.is_some() {
            None
        } else {
            summary.detail
        },
        retry_now: error.is_none() && !matches!(view.status, ClusterStatus::Disabled),
    }
}

/// `HostCapabilityGuard`: the selected Host is known not to offer what the
/// feature requires.
pub fn lacks(host: Option<&HostView>, requires: Option<Capability>) -> Option<Capability> {
    let (host, requires) = (host?, requires?);
    (engine::capability_known(host, requires) && !engine::supports(Some(host), requires))
        .then_some(requires)
}

/// `HostGate`: the selected Host, connected and offering `requires`.
fn host_gate(world: &World, requires: Option<Capability>) -> Result<&HostView, Verdict<'_>> {
    if world.hosts.is_pending() {
        return Err(Verdict::Loading);
    }
    let Some(view) = world.host() else {
        return Err(Verdict::Welcome);
    };
    if let Some(requires) = lacks(Some(view), requires) {
        return Err(Verdict::Unsupported {
            host: view,
            requires,
        });
    }
    match &view.status {
        HostStatus::Connected { .. } => Ok(view),
        HostStatus::Connecting => Err(Verdict::Connecting(&view.host.name)),
        _ => Err(Verdict::HostDown(view)),
    }
}

/// `ClusterGate`: the selected Cluster, connected.
fn cluster_gate(world: &World) -> Result<&ClusterView, Verdict<'_>> {
    if world.clusters.is_pending() {
        return Err(Verdict::Loading);
    }
    // As React's: a list that couldn't be read is said, whatever an
    // earlier read left on screen.
    if let Some(error) = world.clusters.error() {
        return Err(Verdict::ClustersFailed(error));
    }
    let Some(view) = world.cluster() else {
        return Err(Verdict::NoCluster);
    };
    match &view.status {
        ClusterStatus::Connected { .. } => Ok(view),
        ClusterStatus::Connecting => Err(Verdict::Connecting(&view.cluster.name)),
        _ => Err(Verdict::ClusterDown(view)),
    }
}

fn of_cluster<'a>(world: &'a World, view: &ClusterView) -> Verdict<'a> {
    Verdict::Open(Subject::Cluster {
        cluster_id: view.cluster.id.clone(),
        namespace: model::cluster_namespace(view, &world.settings.cluster_namespaces),
    })
}

/// `SwarmGate`: the Swarm `swarm_id`, once one of its managers answers.
fn swarm_gate<'a>(world: &'a World, swarm_id: &str) -> Verdict<'a> {
    if world.swarms.is_pending() {
        return Verdict::Loading;
    }
    if !world.swarm_list().iter().any(|swarm| swarm.id == swarm_id) {
        return Verdict::SwarmGone;
    }
    match world.swarm.error() {
        Some(error) => Verdict::SwarmFailed(error),
        None => Verdict::Open(Subject::Swarm {
            swarm_id: swarm_id.to_owned(),
        }),
    }
}

/// What stands before the page of `route`, given what is selected and
/// known: the guards of its feature first (the Engine, the kind of
/// Cluster), then its gate.
pub fn decide<'a>(route: &Route, world: &'a World) -> Verdict<'a> {
    let feature = registry::feature(route.feature());
    if let Some(requires) = lacks(world.host(), feature.requires) {
        if let Some(host) = world.host() {
            return Verdict::Unsupported { host, requires };
        }
    }
    if let (Some(cluster), Some(wanted)) = (world.cluster(), feature.cluster_kind) {
        if cluster.cluster.kind != wanted {
            return Verdict::WrongKind { cluster, wanted };
        }
    }
    let host = |requires| {
        host_gate(world, requires).map(|view| Subject::Host {
            host_id: view.host.id.clone(),
        })
    };
    let subject = match feature.gate {
        Gate::Open => Ok(Subject::App),
        Gate::Selection if matches!(world.selection, Some(Selection::Cluster { .. })) => {
            return cluster_gate(world)
                .map_or_else(|closed| closed, |view| of_cluster(world, view));
        }
        Gate::Selection => host(None),
        Gate::Host => host(feature.requires),
        Gate::Containerd => host_gate(world, Some(Capability::Containerd)).and_then(|view| {
            // A list that couldn't be read is an empty one: `default` shows.
            let known = world.containerd.data().map(|list| list.as_slice());
            let failed = world.containerd.error().map(|_| [].as_slice());
            model::containerd_namespace(
                &view.host.id,
                &world.settings.containerd_namespaces,
                known.or(failed),
            )
            .map(|namespace| Subject::Containerd {
                host_id: view.host.id.clone(),
                namespace,
            })
            .ok_or(Verdict::Loading)
        }),
        Gate::Incus => host_gate(world, Some(Capability::Incus)).and_then(|view| {
            let known = world.incus.data().map(|list| list.as_slice());
            let failed = world.incus.error().map(|_| [].as_slice());
            model::incus_project(
                &view.host.id,
                &world.settings.incus_projects,
                known.or(failed),
            )
            .map(|project| Subject::Incus {
                host_id: view.host.id.clone(),
                project,
            })
            .ok_or(Verdict::Loading)
        }),
        Gate::Cluster => {
            return cluster_gate(world)
                .map_or_else(|closed| closed, |view| of_cluster(world, view));
        }
        Gate::Swarm => return swarm_gate(world, route.swarm_id().unwrap_or_default()),
    };
    subject.map_or_else(|closed| closed, Verdict::Open)
}
