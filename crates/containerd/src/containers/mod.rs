//! containerd's containers, which are only metadata, joined with their
//! tasks, which are what runs: listing them, and starting, stopping,
//! killing, pausing, resuming and removing them.

pub mod exec;
pub mod logs;
mod model;
mod signals;
mod task;

use std::collections::HashMap;

pub use signals::signal_number;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use self::task::{delete_task, exited_within, resume, KILL_GRACE};
pub use self::{
    model::{ContainerdContainer, ContainerdContainerDetail, CreatedBy, TaskAction, TaskState},
    task::{act, kill, kill_named, stop},
};
use crate::{
    grpc::{
        api::containerd::{
            services::{
                containers::v1::{
                    Container, DeleteContainerRequest, GetContainerRequest, ListContainersRequest,
                },
                snapshots::v1::{Info, RemoveSnapshotRequest, UpdateSnapshotRequest},
                tasks::v1::{GetRequest, ListTasksRequest},
            },
            v1::types::{Process, Status},
        },
        client::call,
        error,
    },
    messages as msg, Containerd,
};

/// The labels a container made by Kubernetes' CRI carries.
const KUBERNETES_POD: &str = "io.kubernetes.pod.name";

const KUBERNETES_CONTAINER: &str = "io.kubernetes.container.name";

const KUBERNETES_POD_NAMESPACE: &str = "io.kubernetes.pod.namespace";

/// `sandbox` for a Pod's pause container, `container` for the others.
const CRI_KIND: &str = "io.cri-containerd.kind";

/// nerdctl's name for a container, and where its output goes.
const NERDCTL_NAME: &str = "nerdctl/name";

pub(crate) const NERDCTL_LOG_URI: &str = "nerdctl/log-uri";

pub(crate) fn timestamp(t: Option<&prost_types::Timestamp>) -> Option<String> {
    let t = t?;
    if t.seconds <= 0 {
        return None;
    }
    // prost_types prints RFC 3339 for a Timestamp.
    Some(t.to_string())
}

/// What to call a container: nerdctl's name, the Pod's container's, or its id.
fn display_name(id: &str, labels: &HashMap<String, String>) -> String {
    if let Some(name) = labels.get(NERDCTL_NAME).filter(|n| !n.is_empty()) {
        return name.clone();
    }
    match (labels.get(KUBERNETES_POD), labels.get(KUBERNETES_CONTAINER)) {
        (Some(pod), Some(container)) => format!("{pod}/{container}"),
        (Some(pod), None) => pod.clone(),
        _ => id.to_owned(),
    }
}

fn created_by(namespace: &str, labels: &HashMap<String, String>) -> CreatedBy {
    if labels.contains_key(KUBERNETES_POD) || labels.contains_key(CRI_KIND) {
        CreatedBy::Kubernetes
    } else if labels.keys().any(|k| k.starts_with("nerdctl/")) {
        CreatedBy::Nerdctl
    } else if namespace == "moby" {
        CreatedBy::Docker
    } else {
        CreatedBy::Other
    }
}

/// `container` joined with its task, `None` when it has none.
pub(crate) fn summary(
    namespace: &str,
    container: &Container,
    task: Option<&Process>,
) -> ContainerdContainer {
    let labels = &container.labels;
    let pod = labels
        .get(KUBERNETES_POD)
        .map(|pod| match labels.get(KUBERNETES_POD_NAMESPACE) {
            Some(ns) => format!("{ns}/{pod}"),
            None => pod.clone(),
        });
    let state = task.map_or(TaskState::None, |t| t.status().into());
    let stopped = state == TaskState::Stopped;
    ContainerdContainer {
        id: container.id.clone(),
        name: display_name(&container.id, labels),
        image: container.image.clone(),
        labels: labels.clone(),
        runtime: container
            .runtime
            .as_ref()
            .map(|r| r.name.clone())
            .unwrap_or_default(),
        snapshotter: container.snapshotter.clone(),
        snapshot_key: container.snapshot_key.clone(),
        created_at: timestamp(container.created_at.as_ref()),
        created_by: created_by(namespace, labels),
        pod,
        sandbox: labels.get(CRI_KIND).is_some_and(|k| k == "sandbox"),
        state,
        pid: task.map(|t| t.pid).filter(|&pid| pid > 0 && !stopped),
        exit_status: task.filter(|_| stopped).map(|t| t.exit_status),
        exited_at: task
            .filter(|_| stopped)
            .and_then(|t| timestamp(t.exited_at.as_ref())),
    }
}

async fn tasks(client: &Containerd, namespace: &str) -> AppResult<HashMap<String, Process>> {
    Ok(client
        .tasks()
        .list(call(namespace, ListTasksRequest::default())?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner()
        .tasks
        .into_iter()
        // containerd names a task by its container in `id`, and leaves
        // `container_id` empty in lists.
        .map(|t| {
            let id = if t.container_id.is_empty() {
                t.id.clone()
            } else {
                t.container_id.clone()
            };
            (id, t)
        })
        .collect())
}

async fn task(client: &Containerd, namespace: &str, id: &str) -> AppResult<Option<Process>> {
    match client
        .tasks()
        .get(call(
            namespace,
            GetRequest {
                container_id: id.to_owned(),
                exec_id: String::new(),
            },
        )?)
        .await
    {
        Ok(response) => Ok(response.into_inner().process),
        Err(s) if s.code() == tonic::Code::NotFound => Ok(None),
        Err(s) => Err(error::status(&s)),
    }
}

pub(crate) async fn get(client: &Containerd, namespace: &str, id: &str) -> AppResult<Container> {
    client
        .containers()
        .get(call(namespace, GetContainerRequest { id: id.to_owned() })?)
        .await
        .map_err(|s| match s.code() {
            tonic::Code::NotFound => AppError::not_found(msg::CONTAINER_NOT_FOUND.with("id", id))
                .with_detail(error::detail(&s)),
            _ => error::status(&s),
        })?
        .into_inner()
        .container
        .ok_or_else(|| AppError::not_found(msg::CONTAINER_NOT_FOUND.with("id", id)))
}

/// The containers of `namespace`, by name, each with its task.
pub async fn list(client: &Containerd, namespace: &str) -> AppResult<Vec<ContainerdContainer>> {
    let containers = client
        .containers()
        .list(call(namespace, ListContainersRequest::default())?)
        .await
        .map_err(|s| error::status(&s))?
        .into_inner()
        .containers;
    let tasks = tasks(client, namespace).await?;
    let mut list: Vec<ContainerdContainer> = containers
        .iter()
        .map(|c| summary(namespace, c, tasks.get(&c.id)))
        .collect();
    list.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.id.cmp(&b.id)));
    Ok(list)
}

/// One container with its spec and task.
pub async fn inspect(
    client: &Containerd,
    namespace: &str,
    id: &str,
) -> AppResult<ContainerdContainerDetail> {
    let container = get(client, namespace, id).await?;
    let task = task(client, namespace, id).await?;
    let spec = container
        .spec
        .as_ref()
        .and_then(|any| serde_json::from_slice(&any.value).ok());
    let mut extensions: Vec<String> = container.extensions.keys().cloned().collect();
    extensions.sort();
    Ok(ContainerdContainerDetail {
        container: summary(namespace, &container, task.as_ref()),
        spec,
        extensions,
        updated_at: timestamp(container.updated_at.as_ref()),
    })
}

/// The label that makes containerd's garbage collector keep a snapshot
/// nothing else references.
pub(crate) const GC_ROOT: &str = "containerd.io/gc.root";

/// Marks the snapshot `key` as a garbage-collection root, so it outlives
/// the container about to be removed: once nothing references it,
/// containerd's collector would otherwise delete it at its next run.
async fn keep(client: &Containerd, namespace: &str, snapshotter: &str, key: &str) -> AppResult<()> {
    let labels = HashMap::from([(GC_ROOT.to_owned(), jiff::Timestamp::now().to_string())]);
    let updated = client
        .snapshots()
        .update(call(
            namespace,
            UpdateSnapshotRequest {
                snapshotter: snapshotter.to_owned(),
                info: Some(Info {
                    name: key.to_owned(),
                    labels,
                    ..Default::default()
                }),
                update_mask: Some(prost_types::FieldMask {
                    paths: vec![format!("labels.{GC_ROOT}")],
                }),
            },
        )?)
        .await;
    match updated {
        Ok(_) => Ok(()),
        // Nothing left to keep.
        Err(s) if s.code() == tonic::Code::NotFound => Ok(()),
        Err(s) => Err(error::status(&s)),
    }
}

/// Removes a container: its stopped task, the container, and its snapshot
/// unless `keep_snapshot`, which marks the snapshot so containerd keeps it.
/// A running task is refused unless `force`, which kills it first.
pub async fn remove(
    client: &Containerd,
    namespace: &str,
    id: &str,
    force: bool,
    keep_snapshot: bool,
) -> AppResult<()> {
    let container = get(client, namespace, id).await?;
    if let Some(status) = task(client, namespace, id).await?.map(|t| t.status()) {
        if !matches!(status, Status::Stopped | Status::Created | Status::Unknown) {
            if !force {
                return Err(AppError::new(
                    ErrorCode::ContainerRunning,
                    msg::TASK_STILL_RUNNING.with("id", id),
                )
                .with_hint(msg::TASK_STILL_RUNNING_HINT));
            }
            if status == Status::Paused {
                resume(client, namespace, id).await.ok();
            }
            kill(client, namespace, id, signals::SIGKILL).await?;
            if !exited_within(client, namespace, id, KILL_GRACE).await? {
                return Err(AppError::new(
                    ErrorCode::TaskWontStop,
                    msg::TASK_WONT_STOP.with("id", id),
                )
                .with_hint(msg::TASK_WONT_STOP_HINT));
            }
        }
        delete_task(client, namespace, id).await?;
    }
    if keep_snapshot && !container.snapshot_key.is_empty() {
        keep(
            client,
            namespace,
            &container.snapshotter,
            &container.snapshot_key,
        )
        .await?;
    }
    client
        .containers()
        .delete(call(
            namespace,
            DeleteContainerRequest { id: id.to_owned() },
        )?)
        .await
        .map_err(|s| error::status(&s))?;
    if !keep_snapshot && !container.snapshot_key.is_empty() {
        let removed = client
            .snapshots()
            .remove(call(
                namespace,
                RemoveSnapshotRequest {
                    snapshotter: container.snapshotter.clone(),
                    key: container.snapshot_key.clone(),
                },
            )?)
            .await;
        match removed {
            Ok(_) => {}
            Err(s) if s.code() == tonic::Code::NotFound => {}
            Err(s) => return Err(error::status(&s)),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
