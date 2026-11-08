//! A snapshotter's snapshots in a namespace: where containerd's disk goes.
//! Each comes with its usage, its parent, and what uses it: a container's
//! own (by its `snapshot_key`), or an image's layers (from the snapshot its
//! config's garbage-collection label names, down through the parents).

use std::collections::{HashMap, HashSet};

use futures_util::StreamExt;
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    api::containerd::services::{
        containers::v1::ListContainersRequest,
        content::v1::InfoRequest,
        snapshots::v1::{Info, Kind, ListSnapshotsRequest, RemoveSnapshotRequest, UsageRequest},
    },
    client::{call, stream},
    containers::timestamp,
    error, images, introspection, messages as msg, Containerd,
};

/// The label on an image's config naming the snapshot of its top layer.
const GC_SNAPSHOT: &str = "containerd.io/gc.ref.snapshot.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotKind {
    /// A layer, read-only, that others build on.
    Committed,
    /// A container's writable layer.
    Active,
    /// A read-only mount of another snapshot.
    View,
    Unknown,
}

impl From<Kind> for SnapshotKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Committed => Self::Committed,
            Kind::Active => Self::Active,
            Kind::View => Self::View,
            Kind::Unknown => Self::Unknown,
        }
    }
}

/// What uses a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SnapshotUser {
    Container { id: String },
    Image { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdSnapshot {
    /// Its key: a container's id, or a layer's chain ID.
    pub key: String,
    /// The snapshot it's built on, empty for a base layer.
    pub parent: String,
    pub kind: SnapshotKind,
    /// Bytes on disk, as `ctr snapshot usage` counts them; `None` when the
    /// snapshotter couldn't say.
    pub size: Option<i64>,
    pub inodes: Option<i64>,
    /// RFC 3339.
    pub created_at: Option<String>,
    pub labels: HashMap<String, String>,
    pub users: Vec<SnapshotUser>,
    /// How many snapshots are built on it.
    pub children: u32,
}

/// The snapshots of one snapshotter, and every snapshotter the Host has.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdSnapshots {
    pub snapshotter: String,
    pub snapshotters: Vec<String>,
    pub snapshots: Vec<ContainerdSnapshot>,
    /// Their usage added up.
    pub total: i64,
}

async fn infos(client: &Containerd, namespace: &str, snapshotter: &str) -> AppResult<Vec<Info>> {
    let mut responses = client
        .snapshots()
        .list(stream(
            namespace,
            ListSnapshotsRequest {
                snapshotter: snapshotter.to_owned(),
                filters: vec![],
            },
        )?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner();
    let mut all = Vec::new();
    while let Some(response) = responses.next().await {
        all.extend(response.map_err(|s| error::status(&s))?.info);
    }
    Ok(all)
}

/// Every snapshot in `parents`' chain from `top` down, `top` included.
fn chain<'a>(top: &'a str, parents: &'a HashMap<String, String>) -> Vec<&'a str> {
    let mut chain = vec![];
    let mut seen = HashSet::new();
    let mut next = Some(top);
    while let Some(key) = next.filter(|k| !k.is_empty() && seen.insert(*k)) {
        chain.push(key);
        next = parents.get(key).map(String::as_str);
    }
    chain
}

/// What uses each snapshot: containers by their snapshot key, and images
/// by the chain under the snapshot their config names.
pub(crate) fn users_of(
    parents: &HashMap<String, String>,
    containers: &[(String, String)],
    images: &[(String, String)],
) -> HashMap<String, Vec<SnapshotUser>> {
    let mut users: HashMap<String, Vec<SnapshotUser>> = HashMap::new();
    for (id, key) in containers {
        users
            .entry(key.clone())
            .or_default()
            .push(SnapshotUser::Container { id: id.clone() });
    }
    for (name, top) in images {
        for key in chain(top, parents) {
            users
                .entry(key.to_owned())
                .or_default()
                .push(SnapshotUser::Image { name: name.clone() });
        }
    }
    users
}

/// The snapshot of each image's top layer in `snapshotter`, by image name.
async fn image_tops(
    client: &Containerd,
    namespace: &str,
    snapshotter: &str,
) -> AppResult<Vec<(String, String)>> {
    let info = introspection::describe(client).await?;
    let mut tops = Vec::new();
    for image in images::records(client, namespace).await? {
        let Some(config) = images::describe(client, namespace, &image, &info)
            .await
            .config
        else {
            continue;
        };
        let labels = client
            .content()
            .info(call(namespace, InfoRequest { digest: config })?)
            .await
            .ok()
            .and_then(|r| r.into_inner().info)
            .map(|i| i.labels)
            .unwrap_or_default();
        if let Some(top) = labels.get(&format!("{GC_SNAPSHOT}{snapshotter}")) {
            tops.push((image.name, top.clone()));
        }
    }
    Ok(tops)
}

/// The containers of `namespace` on `snapshotter`, with their snapshot keys.
async fn container_keys(
    client: &Containerd,
    namespace: &str,
    snapshotter: &str,
) -> AppResult<Vec<(String, String)>> {
    Ok(client
        .containers()
        .list(call(namespace, ListContainersRequest::default())?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner()
        .containers
        .into_iter()
        .filter(|c| c.snapshotter == snapshotter && !c.snapshot_key.is_empty())
        .map(|c| (c.id, c.snapshot_key))
        .collect())
}

/// The snapshots of `snapshotter` (the Host's default when `None`) in
/// `namespace`, largest first, with their usage and users.
pub async fn list(
    client: &Containerd,
    namespace: &str,
    snapshotter: Option<&str>,
) -> AppResult<ContainerdSnapshots> {
    let info = introspection::describe(client).await?;
    let snapshotter = snapshotter
        .filter(|s| !s.is_empty())
        .unwrap_or(info.default_snapshotter())
        .to_owned();
    let infos = infos(client, namespace, &snapshotter).await?;
    let parents: HashMap<String, String> = infos
        .iter()
        .map(|i| (i.name.clone(), i.parent.clone()))
        .collect();
    let mut children: HashMap<&str, u32> = HashMap::new();
    for parent in parents.values().filter(|p| !p.is_empty()) {
        *children.entry(parent.as_str()).or_default() += 1;
    }
    let users = users_of(
        &parents,
        &container_keys(client, namespace, &snapshotter).await?,
        &image_tops(client, namespace, &snapshotter).await?,
    );
    let mut snapshots = Vec::with_capacity(infos.len());
    for i in &infos {
        let usage = client
            .snapshots()
            .usage(call(
                namespace,
                UsageRequest {
                    snapshotter: snapshotter.clone(),
                    key: i.name.clone(),
                },
            )?)
            .await
            .ok()
            .map(tonic::Response::into_inner);
        snapshots.push(ContainerdSnapshot {
            key: i.name.clone(),
            parent: i.parent.clone(),
            kind: i.kind().into(),
            size: usage.map(|u| u.size),
            inodes: usage.map(|u| u.inodes),
            created_at: timestamp(i.created_at.as_ref()),
            labels: i.labels.clone(),
            users: users.get(&i.name).cloned().unwrap_or_default(),
            children: children.get(i.name.as_str()).copied().unwrap_or(0),
        });
    }
    snapshots.sort_by(|a, b| b.size.cmp(&a.size).then_with(|| a.key.cmp(&b.key)));
    Ok(ContainerdSnapshots {
        total: snapshots.iter().filter_map(|s| s.size).sum(),
        snapshotter,
        snapshotters: info.snapshotters,
        snapshots,
    })
}

/// Removes a snapshot nothing uses and nothing is built on.
pub async fn remove(
    client: &Containerd,
    namespace: &str,
    snapshotter: &str,
    key: &str,
) -> AppResult<()> {
    let listed = list(client, namespace, Some(snapshotter)).await?;
    let Some(snapshot) = listed.snapshots.iter().find(|s| s.key == key) else {
        return Err(AppError::not_found(
            msg::SNAPSHOT_NOT_FOUND.with("key", key),
        ));
    };
    if snapshot.children > 0 {
        let message = match snapshot.children {
            1 => msg::SNAPSHOT_HAS_CHILD.with("key", key),
            count => msg::SNAPSHOT_HAS_CHILDREN
                .with("key", key)
                .with("count", count),
        };
        return Err(AppError::new(ErrorCode::InUse, message).with_hint(msg::SNAPSHOT_IN_USE_HINT));
    }
    if !snapshot.users.is_empty() {
        let names: Vec<String> = snapshot
            .users
            .iter()
            .map(|u| match u {
                SnapshotUser::Container { id } => id.clone(),
                SnapshotUser::Image { name } => name.clone(),
            })
            .collect();
        return Err(AppError::new(
            ErrorCode::InUse,
            msg::SNAPSHOT_USED
                .with("key", key)
                .with("users", names.join(", ")),
        )
        .with_hint(msg::SNAPSHOT_IN_USE_HINT));
    }
    client
        .snapshots()
        .remove(call(
            namespace,
            RemoveSnapshotRequest {
                snapshotter: snapshotter.to_owned(),
                key: key.to_owned(),
            },
        )?)
        .await
        .map_err(|s| error::status(&s))?;
    Ok(())
}

#[cfg(test)]
mod tests;
