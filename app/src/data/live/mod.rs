//! Keeping what is cached in step with what the backend pushes, instead of
//! polling: what `LiveUpdates` and its hooks did in React
//! (`useHostStatusUpdates`, `useClusterStatusUpdates`,
//! `useEngineEventUpdates`, `useSwarmUpdates`, `useForwardUpdates`,
//! `useSyncedUpdates`). The window gives every [`Event`] to
//! [`Data::on_event`]; saying what happened to the user (a toast for a
//! forward that ended or an automatic deploy) is the shell's.

use std::sync::Arc;

use iced::Task;

use homeport_clusters::{registry::ClusterStatus, service::ClusterView};
use homeport_core::error::{AppError, ErrorCode};
use homeport_engine::swarm::SwarmRole;
use homeport_hosts::{
    events::PAGE,
    registry::{EngineEvent, HostStatus},
    service::HostView,
};

use super::{
    keys::{cluster, host, host::IncusResource, nomad, stacks, swarm},
    store::batch,
    Data, Key, Message,
};
use crate::app::{events::Event, AppState};

#[cfg(test)]
mod tests;

impl Data {
    /// Takes in what the backend just told: statuses are put in the cached
    /// lists, a Host or Cluster that connects has everything it holds read
    /// again, one that goes down has it marked failed with the data kept,
    /// and Engine events make the lists they change out of date.
    pub fn on_event(&mut self, state: &Arc<AppState>, event: &Event) -> Task<Message> {
        match event {
            Event::HostStatus { host_id, status } => self.host_status(state, host_id, status),
            Event::ClusterStatus { cluster_id, status } => {
                self.cluster_status(state, cluster_id, status)
            }
            Event::Engine { host_id, event } => {
                self.change(&host::events(host_id), |list: &Vec<EngineEvent>| {
                    append_event(list, event)
                });
                let mut stale = stale_keys(host_id, &event.kind);
                if let Some(swarm_id) = managed_swarm(&state.hosts.registry.status(host_id)) {
                    stale.extend(stale_swarm_keys(&swarm_id, &event.kind));
                }
                self.invalidate_later(stale)
            }
            Event::PortForward(_) => self.invalidate(state, &cluster::port_forwards()),
            Event::HostsChanged => batch([
                self.invalidate(state, &host::list()),
                self.invalidate_later([swarm::list()]),
            ]),
            Event::SyncedUpdateAvailable {
                owner, stack_id, ..
            } => batch([
                self.invalidate(state, &stacks::of(owner)),
                self.invalidate(state, &stacks::synced(stack_id)),
            ]),
            Event::SyncedAutoDeployed {
                owner, stack_id, ..
            } => batch([
                self.invalidate(state, &stacks::synced(stack_id)),
                // Its Stacks are in the scope: a deploy changes containers,
                // networks and volumes too.
                self.invalidate(state, &stacks::scope(owner)),
            ]),
            Event::NativeTextChanged => Task::none(),
        }
    }

    fn host_status(
        &mut self,
        state: &Arc<AppState>,
        host_id: &str,
        status: &HostStatus,
    ) -> Task<Message> {
        let known = self.change(&host::list(), |hosts: &Vec<HostView>| {
            with_host_status(hosts, host_id, status)
        });
        let mut tasks = vec![];
        if !known {
            // A Host that was just added, or a list nobody read yet.
            tasks.push(self.invalidate(state, &host::list()));
        }
        let scope = host::scope(host_id);
        if matches!(status, HostStatus::Connected { .. }) {
            tasks.push(self.invalidate(state, &scope));
        } else if let Err(error) = state.hosts.registry.dialer(host_id) {
            // The error a read would get now, with the cause in its hint.
            self.fail(&scope, &error);
        }
        // Which Hosts manage which Swarm follows from their statuses.
        tasks.push(self.invalidate_later([swarm::list()]));
        batch(tasks)
    }

    fn cluster_status(
        &mut self,
        state: &Arc<AppState>,
        cluster_id: &str,
        status: &ClusterStatus,
    ) -> Task<Message> {
        let known = self.change(&cluster::list(), |clusters: &Vec<ClusterView>| {
            with_cluster_status(clusters, cluster_id, status)
        });
        let mut tasks = vec![];
        if !known {
            tasks.push(self.invalidate(state, &cluster::list()));
        }
        // A Cluster is Kubernetes or Nomad; only one of the two holds anything.
        let scopes = [cluster::scope(cluster_id), nomad::scope(cluster_id)];
        if status.is_connected() {
            tasks.extend(scopes.iter().map(|scope| self.invalidate(state, scope)));
        } else {
            let error = cluster_down(state, cluster_id, status);
            for scope in &scopes {
                self.fail(scope, &error);
            }
        }
        batch(tasks)
    }
}

/// `hosts` with `status` given to the Host `host_id`; `None` when it isn't
/// in the list.
fn with_host_status(
    hosts: &[HostView],
    host_id: &str,
    status: &HostStatus,
) -> Option<Vec<HostView>> {
    hosts.iter().any(|view| view.host.id == host_id).then(|| {
        hosts
            .iter()
            .map(|view| HostView {
                host: view.host.clone(),
                status: if view.host.id == host_id {
                    status.clone()
                } else {
                    view.status.clone()
                },
            })
            .collect()
    })
}

/// `clusters` with `status` given to the Cluster `cluster_id`; `None` when
/// it isn't in the list.
fn with_cluster_status(
    clusters: &[ClusterView],
    cluster_id: &str,
    status: &ClusterStatus,
) -> Option<Vec<ClusterView>> {
    clusters
        .iter()
        .any(|view| view.cluster.id == cluster_id)
        .then(|| {
            clusters
                .iter()
                .cloned()
                .map(|mut view| {
                    if view.cluster.id == cluster_id {
                        view.status = status.clone();
                    }
                    view
                })
                .collect()
        })
}

/// Why nothing can be read from a Cluster in `status`: the error a read
/// would get now, or one made from the status when the registry already
/// moved on.
fn cluster_down(state: &AppState, cluster_id: &str, status: &ClusterStatus) -> AppError {
    let registry = &state.clusters.registry;
    let refused = [
        registry.client(cluster_id).err(),
        registry.nomad(cluster_id).err(),
    ];
    if let Some(error) = refused
        .into_iter()
        .flatten()
        .find(|error| error.is(ErrorCode::ClusterNotConnected))
    {
        return error;
    }
    let error = AppError::new(
        ErrorCode::ClusterNotConnected,
        homeport_clusters::messages::NOT_CONNECTED_UNKNOWN,
    );
    match status.error() {
        Some(cause) => AppError {
            hint: cause.hint.clone(),
            detail: Some(cause.message.clone()),
            ..error
        },
        None => error.with_hint(homeport_clusters::messages::NOT_CONNECTED_HINT),
    }
}

/// The Swarm a Host in `status` is a manager of.
fn managed_swarm(status: &HostStatus) -> Option<String> {
    let membership = status.swarm()?;
    (membership.role == SwarmRole::Manager)
        .then(|| membership.swarm_id.clone())
        .flatten()
}

/// What tells two events of a Host apart: when, what, and to what.
fn same_event(a: &EngineEvent, b: &EngineEvent) -> bool {
    a.time_nano == b.time_nano
        && a.kind == b.kind
        && a.action == b.action
        && a.actor_id == b.actor_id
}

/// `list` with `event` appended, keeping the latest page; `None` when it is
/// already there (a reconnection asks the Engine again from the last kept
/// event).
pub fn append_event(list: &[EngineEvent], event: &EngineEvent) -> Option<Vec<EngineEvent>> {
    if list.iter().any(|known| same_event(known, event)) {
        return None;
    }
    let keep = (PAGE as usize).saturating_sub(1);
    let mut appended = list[list.len().saturating_sub(keep)..].to_vec();
    appended.push(event.clone());
    Some(appended)
}

/// Which cached lists an Engine event of `kind` makes out of date.
/// containerd's events (`task`, `snapshot`, `namespace`, `content`, and its
/// own `container` and `image`) refresh its pages in every namespace, and
/// Incus's lifecycle events (named after the object: `instance`, `image`,
/// `profile`…) its pages in every project.
pub fn stale_keys(host_id: &str, kind: &str) -> Vec<Key> {
    let incus = |resource| host::incus(host_id, resource, None);
    match kind {
        "instance" | "instance-snapshot" | "instance-backup" => vec![
            incus(IncusResource::Instances),
            host::incus_projects(host_id),
            // Profiles, volumes and networks say which Instances use them.
            incus(IncusResource::Profiles),
            incus(IncusResource::Storage),
            incus(IncusResource::Networks),
        ],
        "image-alias" => vec![incus(IncusResource::Images)],
        "profile" => vec![incus(IncusResource::Profiles)],
        "storage-pool" | "storage-volume" | "storage-volume-snapshot" => {
            vec![incus(IncusResource::Storage)]
        }
        "network-acl" | "network-zone" => vec![incus(IncusResource::Networks)],
        "project" => vec![host::incus_projects(host_id)],
        "task" => vec![host::containerd_containers(host_id, None)],
        "snapshot" => vec![host::containerd_snapshots(host_id, None)],
        "namespace" => vec![host::containerd_namespaces(host_id)],
        "content" => vec![host::containerd_images(host_id, None)],
        "container" => vec![
            host::containerd_containers(host_id, None),
            host::containerd_snapshots(host_id, None),
            host::containerd_namespaces(host_id),
            host::containers(host_id),
            host::stacks(host_id),
            host::system(host_id),
            host::volumes(host_id),
            host::networks(host_id),
            // A pod's and a Quadlet unit's status are their containers'.
            host::pods(host_id),
            host::quadlets(host_id),
        ],
        "pod" => vec![host::pods(host_id)],
        "image" => vec![
            host::images(host_id),
            host::system(host_id),
            host::containerd_images(host_id, None),
            host::containerd_namespaces(host_id),
            incus(IncusResource::Images),
            host::incus_projects(host_id),
        ],
        "volume" => vec![
            host::volumes(host_id),
            host::volume_sizes(host_id),
            host::system(host_id),
        ],
        "network" => vec![host::networks(host_id), incus(IncusResource::Networks)],
        _ => vec![],
    }
}

/// Which of a Swarm's cached lists an Engine event of `kind` on one of its
/// managers makes out of date.
pub fn stale_swarm_keys(swarm_id: &str, kind: &str) -> Vec<Key> {
    match kind {
        "node" => vec![
            swarm::list(),
            swarm::summary(swarm_id),
            swarm::nodes(swarm_id),
            swarm::tasks(swarm_id),
        ],
        "service" => vec![
            swarm::summary(swarm_id),
            swarm::services(swarm_id),
            swarm::tasks(swarm_id),
            swarm::stacks(swarm_id),
        ],
        "container" => vec![swarm::tasks(swarm_id), swarm::services(swarm_id)],
        "secret" => vec![swarm::secrets(swarm_id), swarm::stacks(swarm_id)],
        "config" => vec![swarm::configs(swarm_id), swarm::stacks(swarm_id)],
        _ => vec![],
    }
}
