//! The resources several features read: the Hosts, the Clusters, and each
//! Host's Engine lists (`hostQueries` and `resourceQueries` in React).
//! Engine events make the lists out of date (`super::live`), so none is
//! polled. What only one feature reads is a [`Query`] of its own, filed
//! under one of these keys.

use std::{collections::HashMap, time::Duration};

use homeport_clusters::service::ClusterView;
use homeport_engine::{
    containers::Container,
    images::{freshness::FreshnessResult, Image},
    networks::NetworkView,
    podman::pods::Pod,
    system::{DiskUsage, EngineInfo},
    volumes::VolumeView,
};
use homeport_hosts::{events::PAGE, registry::EngineEvent, service::HostView};
use homeport_stacks::Stack;

use super::{
    keys::{cluster, host},
    Query,
};
use crate::api;

/// The Hosts, with their statuses; status events keep them in step.
pub fn hosts() -> Query<Vec<HostView>> {
    Query::new(host::list(), |state| async move {
        api::hosts::hosts_list(&state)
    })
}

/// The Clusters, with their statuses; status events keep them in step.
pub fn clusters() -> Query<Vec<ClusterView>> {
    Query::new(cluster::list(), |state| async move {
        api::clusters::clusters_list(&state)
    })
}

pub fn containers(host_id: &str) -> Query<Vec<Container>> {
    let id = host_id.to_owned();
    Query::new(host::containers(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::containers::containers_list(&state, id).await }
    })
}

pub fn images(host_id: &str) -> Query<Vec<Image>> {
    let id = host_id.to_owned();
    Query::new(host::images(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::images::images_list(&state, id).await }
    })
}

pub fn volumes(host_id: &str) -> Query<Vec<VolumeView>> {
    let id = host_id.to_owned();
    Query::new(host::volumes(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::volumes::volumes_list(&state, id).await }
    })
}

/// Each volume's size, `None` when its driver doesn't report one. Slow on a
/// large Host, so it's apart from the list and good for a minute; volume
/// events and the Volumes page's refresh ask again.
pub fn volume_sizes(host_id: &str) -> Query<HashMap<String, Option<u64>>> {
    let id = host_id.to_owned();
    Query::new(host::volume_sizes(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::volumes::volumes_sizes(&state, id).await }
    })
    .stale_after(Duration::from_secs(60))
}

pub fn networks(host_id: &str) -> Query<Vec<NetworkView>> {
    let id = host_id.to_owned();
    Query::new(host::networks(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::networks::networks_list(&state, id).await }
    })
}

/// Every Stack on a Host: those running there, and the Managed, Linked and
/// Synced Stacks not deployed yet.
pub fn stacks(host_id: &str) -> Query<Vec<Stack>> {
    let id = host_id.to_owned();
    Query::new(host::stacks(host_id), move |state| {
        let id = id.clone();
        async move { api::stacks::stacks_list(&state, id).await }
    })
}

/// What the Host's Engine is and runs.
pub fn system_info(host_id: &str) -> Query<EngineInfo> {
    let id = host_id.to_owned();
    Query::new(host::system_info(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::system::system_info(&state, id).await }
    })
}

/// The disk the Engine's images, containers, volumes and build cache use.
pub fn disk_usage(host_id: &str) -> Query<DiskUsage> {
    let id = host_id.to_owned();
    Query::new(host::disk_usage(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::system::system_disk_usage(&state, id).await }
    })
}

/// Whether a newer image is published for each container's image and each
/// tag. The backend caches registry answers, so this is cheap to read
/// again; a registry that says no is an answer, not retried.
pub fn freshness(host_id: &str) -> Query<Vec<FreshnessResult>> {
    let id = host_id.to_owned();
    Query::new(host::freshness(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::images::images_freshness(&state, id, false).await }
    })
    .stale_after(Duration::from_secs(300))
    .no_retry()
}

/// A Podman Host's pods; container events make them out of date.
pub fn pods(host_id: &str) -> Query<Vec<Pod>> {
    let id = host_id.to_owned();
    Query::new(host::pods(host_id), move |state| {
        let id = id.clone();
        async move { api::engine::podman::pods_list(&state, id).await }
    })
}

/// The latest page of a Host's kept Engine events, oldest first. Live
/// events are appended to it, so it is never read again by age.
pub fn events(host_id: &str) -> Query<Vec<EngineEvent>> {
    let id = host_id.to_owned();
    Query::new(host::events(host_id), move |state| {
        let id = id.clone();
        async move { api::hosts::hosts_events(&state, id, Some(PAGE), None).await }
    })
    .never_stale()
}
