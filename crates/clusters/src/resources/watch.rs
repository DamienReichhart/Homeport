//! Watches that tell a page its list changed, so it reads it again instead
//! of polling. Only metadata is streamed; changes are batched.

use std::{fmt::Debug, pin::pin, time::Duration};

use futures_util::StreamExt;
use k8s_openapi::api::{
    apps::v1::{DaemonSet, Deployment, StatefulSet},
    batch::v1::{CronJob, Job},
    core::v1::{ConfigMap, Event, Namespace, Node, Pod, Secret, Service},
    networking::v1::Ingress,
};
use kube::{
    api::{Api, PartialObjectMeta},
    runtime::{watcher, WatchStreamExt},
    Resource,
};
use serde::de::DeserializeOwned;
use tokio::time::Instant;

use super::ResourceKind;

/// Changes closer together than this are told as one.
const BATCH: Duration = Duration::from_millis(500);

/// Calls `changed` whenever the resources of `kind` in `namespace` (every
/// one with `None`) change, at most every half second, until it returns
/// `false`. Retries with backoff when the watch breaks.
pub async fn watch(
    client: kube::Client,
    kind: ResourceKind,
    namespace: Option<String>,
    mut changed: impl FnMut() -> bool + Send,
) {
    let ns = namespace.as_deref();
    let c = &mut changed;
    match kind {
        ResourceKind::Namespace => run(Api::<PartialObjectMeta<Namespace>>::all(client), c).await,
        ResourceKind::Node => run(Api::<PartialObjectMeta<Node>>::all(client), c).await,
        ResourceKind::Deployment => run(scoped::<Deployment>(client, ns), c).await,
        ResourceKind::StatefulSet => run(scoped::<StatefulSet>(client, ns), c).await,
        ResourceKind::DaemonSet => run(scoped::<DaemonSet>(client, ns), c).await,
        ResourceKind::Job => run(scoped::<Job>(client, ns), c).await,
        ResourceKind::CronJob => run(scoped::<CronJob>(client, ns), c).await,
        ResourceKind::Pod => run(scoped::<Pod>(client, ns), c).await,
        ResourceKind::Service => run(scoped::<Service>(client, ns), c).await,
        ResourceKind::Ingress => run(scoped::<Ingress>(client, ns), c).await,
        ResourceKind::ConfigMap => run(scoped::<ConfigMap>(client, ns), c).await,
        ResourceKind::Secret => run(scoped::<Secret>(client, ns), c).await,
        ResourceKind::Event => run(scoped::<Event>(client, ns), c).await,
    }
}

/// The metadata-only API of `K` in `namespace`, or across every Namespace.
fn scoped<K>(client: kube::Client, namespace: Option<&str>) -> Api<PartialObjectMeta<K>>
where
    K: Resource<Scope = k8s_openapi::NamespaceResourceScope>,
    K::DynamicType: Default,
{
    match namespace {
        Some(ns) => Api::namespaced(client, ns),
        None => Api::all(client),
    }
}

async fn run<K>(api: Api<K>, changed: &mut (impl FnMut() -> bool + Send))
where
    K: Resource + Clone + DeserializeOwned + Debug + Send + 'static,
{
    let mut stream = pin!(watcher(api, watcher::Config::default()).default_backoff());
    // The page read the list itself: the watch's first listing isn't a change.
    let mut listed = false;
    let mut due: Option<Instant> = None;
    loop {
        tokio::select! {
            event = stream.next() => match event {
                None => return,
                Some(Ok(watcher::Event::InitDone)) => {
                    // A later listing follows a broken watch: what it missed changed.
                    if listed {
                        due.get_or_insert_with(|| Instant::now() + BATCH);
                    }
                    listed = true;
                }
                Some(Ok(watcher::Event::Apply(_) | watcher::Event::Delete(_))) if listed => {
                    due.get_or_insert_with(|| Instant::now() + BATCH);
                }
                Some(Ok(_)) => {}
                Some(Err(e)) => log::debug!("watch interrupted: {e}"),
            },
            _ = tokio::time::sleep_until(due.unwrap_or_else(Instant::now)), if due.is_some() => {
                due = None;
                if !changed() {
                    return;
                }
            }
        }
    }
}
