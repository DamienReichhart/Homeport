//! containerd's containers, which are only metadata, joined with their
//! tasks, which are what runs: listing them, and starting, stopping,
//! killing, pausing, resuming and removing them.

mod signals;

use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    api::containerd::{
        services::{
            containers::v1::{
                Container, DeleteContainerRequest, GetContainerRequest, ListContainersRequest,
            },
            snapshots::v1::{Info, MountsRequest, RemoveSnapshotRequest, UpdateSnapshotRequest},
            tasks::v1::{
                CreateTaskRequest, DeleteTaskRequest, GetRequest, KillRequest, ListTasksRequest,
                PauseTaskRequest, ResumeTaskRequest, StartRequest, WaitRequest,
            },
        },
        v1::types::{Process, Status},
    },
    client::{call, stream},
    error, messages as msg, Containerd,
};

pub use signals::signal_number;

/// The labels a container made by Kubernetes' CRI carries.
const KUBERNETES_POD: &str = "io.kubernetes.pod.name";
const KUBERNETES_CONTAINER: &str = "io.kubernetes.container.name";
const KUBERNETES_POD_NAMESPACE: &str = "io.kubernetes.pod.namespace";
/// `sandbox` for a Pod's pause container, `container` for the others.
const CRI_KIND: &str = "io.cri-containerd.kind";
/// nerdctl's name for a container, and where its output goes.
const NERDCTL_NAME: &str = "nerdctl/name";
pub(crate) const NERDCTL_LOG_URI: &str = "nerdctl/log-uri";
/// The signal an image asks to be stopped with.
const STOP_SIGNAL: &str = "io.containerd.image.config.stop-signal";

/// How long a stop waits after SIGKILL before saying the task won't die.
const KILL_GRACE: Duration = Duration::from_secs(10);

/// What a container's task is doing; `none` when it has no task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum TaskState {
    None,
    Created,
    Running,
    Paused,
    Pausing,
    Stopped,
    Unknown,
}

impl From<Status> for TaskState {
    fn from(status: Status) -> Self {
        match status {
            Status::Created => Self::Created,
            Status::Running => Self::Running,
            Status::Paused => Self::Paused,
            Status::Pausing => Self::Pausing,
            Status::Stopped => Self::Stopped,
            Status::Unknown => Self::Unknown,
        }
    }
}

/// Which tool made a container, which decides what Homeport can show of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum CreatedBy {
    /// Kubernetes' CRI: the kubelet undoes stops and removals.
    Kubernetes,
    Nerdctl,
    /// Docker's own containers, in the `moby` namespace.
    Docker,
    /// `ctr` or another client.
    Other,
}

/// A container and its task, as the list shows them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdContainer {
    pub id: String,
    /// nerdctl's name, or the Pod's container's, or the id.
    pub name: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    pub runtime: String,
    pub snapshotter: String,
    pub snapshot_key: String,
    /// RFC 3339.
    pub created_at: Option<String>,
    pub created_by: CreatedBy,
    /// The Kubernetes Pod it belongs to, as `namespace/name`.
    pub pod: Option<String>,
    /// A Pod's pause container, which holds its namespaces.
    pub sandbox: bool,
    pub state: TaskState,
    /// The task's main process.
    pub pid: Option<u32>,
    /// How a stopped task ended.
    pub exit_status: Option<u32>,
    pub exited_at: Option<String>,
}

/// A container with everything containerd holds about it.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdContainerDetail {
    pub container: ContainerdContainer,
    /// Its OCI runtime spec, as containerd stores it.
    pub spec: Option<serde_json::Value>,
    /// The names of its extensions, such as CRI's metadata.
    pub extensions: Vec<String>,
    pub updated_at: Option<String>,
}

/// What can be done to a container's task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum TaskAction {
    Start,
    Stop,
    Pause,
    Resume,
}

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

/// Starts a container's task: creates it on the container's snapshot, with
/// nerdctl's logging when nerdctl made it and no I/O otherwise, then starts
/// it. A stopped task is deleted first, as `ctr task start` would need.
async fn start(client: &Containerd, namespace: &str, id: &str) -> AppResult<()> {
    let container = get(client, namespace, id).await?;
    match task(client, namespace, id).await?.map(|t| t.status()) {
        Some(Status::Running | Status::Pausing) => {
            return Err(AppError::new(
                ErrorCode::AlreadyDone,
                msg::TASK_ALREADY_RUNNING.with("id", id),
            ))
        }
        Some(Status::Paused) => return resume(client, namespace, id).await,
        Some(Status::Created) => {}
        Some(Status::Stopped | Status::Unknown) => delete_task(client, namespace, id).await?,
        None => {}
    }
    if task(client, namespace, id).await?.is_none() {
        let rootfs = if container.snapshot_key.is_empty() {
            vec![]
        } else {
            client
                .snapshots()
                .mounts(call(
                    namespace,
                    MountsRequest {
                        snapshotter: container.snapshotter.clone(),
                        key: container.snapshot_key.clone(),
                    },
                )?)
                .await
                .map_err(|s| error::status(&s))?
                .into_inner()
                .mounts
        };
        let log = container
            .labels
            .get(NERDCTL_LOG_URI)
            .cloned()
            .unwrap_or_default();
        client
            .tasks()
            .create(call(
                namespace,
                CreateTaskRequest {
                    container_id: id.to_owned(),
                    rootfs,
                    stdout: log.clone(),
                    stderr: log,
                    ..Default::default()
                },
            )?)
            .await
            .map_err(|s| error::status(&s))?;
    }
    client
        .tasks()
        .start(call(
            namespace,
            StartRequest {
                container_id: id.to_owned(),
                exec_id: String::new(),
            },
        )?)
        .await
        .map_err(|s| error::status(&s))?;
    Ok(())
}

async fn delete_task(client: &Containerd, namespace: &str, id: &str) -> AppResult<()> {
    match client
        .tasks()
        .delete(call(
            namespace,
            DeleteTaskRequest {
                container_id: id.to_owned(),
            },
        )?)
        .await
    {
        Ok(_) => Ok(()),
        Err(s) if s.code() == tonic::Code::NotFound => Ok(()),
        Err(s) => Err(error::status(&s)),
    }
}

/// Sends `signal` to a container's task, or to every process of it.
pub async fn kill(client: &Containerd, namespace: &str, id: &str, signal: u32) -> AppResult<()> {
    client
        .tasks()
        .kill(call(
            namespace,
            KillRequest {
                container_id: id.to_owned(),
                exec_id: String::new(),
                signal,
                all: false,
            },
        )?)
        .await
        .map_err(|s| match s.code() {
            tonic::Code::NotFound => AppError::new(
                ErrorCode::ContainerNotRunning,
                msg::TASK_NOT_RUNNING.with("id", id),
            )
            .with_detail(error::detail(&s)),
            _ => error::status(&s),
        })?;
    Ok(())
}

/// Sends the signal named `signal` (`SIGHUP`, `hup` or `1`) to a container's task.
pub async fn kill_named(
    client: &Containerd,
    namespace: &str,
    id: &str,
    signal: &str,
) -> AppResult<()> {
    let number = signal_number(signal)
        .ok_or_else(|| AppError::validation(msg::SIGNAL_UNKNOWN.with("signal", signal)))?;
    kill(client, namespace, id, number).await
}

/// Waits for a container's task to exit, at most `limit`; whether it did.
async fn exited_within(
    client: &Containerd,
    namespace: &str,
    id: &str,
    limit: Duration,
) -> AppResult<bool> {
    let mut tasks = client.tasks();
    let wait = tasks.wait(stream(
        namespace,
        WaitRequest {
            container_id: id.to_owned(),
            exec_id: String::new(),
        },
    )?);
    match tokio::time::timeout(limit, wait).await {
        Ok(Ok(_)) => Ok(true),
        Ok(Err(s)) if s.code() == tonic::Code::NotFound => Ok(true),
        Ok(Err(s)) => Err(error::status(&s)),
        Err(_) => Ok(false),
    }
}

/// Stops a container's task: its stop signal (SIGTERM unless the image
/// says otherwise), then SIGKILL once `timeout` has passed.
pub async fn stop(
    client: &Containerd,
    namespace: &str,
    id: &str,
    timeout: Duration,
) -> AppResult<()> {
    let container = get(client, namespace, id).await?;
    match task(client, namespace, id).await?.map(|t| t.status()) {
        None | Some(Status::Stopped) => {
            return Err(AppError::new(
                ErrorCode::AlreadyDone,
                msg::TASK_NOT_RUNNING.with("id", id),
            ))
        }
        // A paused task can't act on SIGTERM until it runs again.
        Some(Status::Paused | Status::Pausing) => resume(client, namespace, id).await?,
        _ => {}
    }
    let signal = container
        .labels
        .get(STOP_SIGNAL)
        .and_then(|s| signal_number(s))
        .unwrap_or(signals::SIGTERM);
    kill(client, namespace, id, signal).await?;
    if exited_within(client, namespace, id, timeout).await? {
        return Ok(());
    }
    kill(client, namespace, id, signals::SIGKILL).await?;
    if exited_within(client, namespace, id, KILL_GRACE).await? {
        return Ok(());
    }
    Err(
        AppError::new(ErrorCode::TaskWontStop, msg::TASK_WONT_STOP.with("id", id))
            .with_hint(msg::TASK_WONT_STOP_HINT),
    )
}

/// Starts, pauses or resumes a container's task; stops it with a 10 s grace.
pub async fn act(
    client: &Containerd,
    namespace: &str,
    id: &str,
    action: TaskAction,
) -> AppResult<()> {
    match action {
        TaskAction::Start => start(client, namespace, id).await,
        TaskAction::Stop => stop(client, namespace, id, Duration::from_secs(10)).await,
        TaskAction::Pause => client
            .tasks()
            .pause(call(
                namespace,
                PauseTaskRequest {
                    container_id: id.to_owned(),
                },
            )?)
            .await
            .map(drop)
            .map_err(|s| not_running_or(&s, id)),
        TaskAction::Resume => resume(client, namespace, id).await,
    }
}

async fn resume(client: &Containerd, namespace: &str, id: &str) -> AppResult<()> {
    client
        .tasks()
        .resume(call(
            namespace,
            ResumeTaskRequest {
                container_id: id.to_owned(),
            },
        )?)
        .await
        .map(drop)
        .map_err(|s| not_running_or(&s, id))
}

fn not_running_or(s: &tonic::Status, id: &str) -> AppError {
    match s.code() {
        tonic::Code::NotFound => AppError::new(
            ErrorCode::ContainerNotRunning,
            msg::TASK_NOT_RUNNING.with("id", id),
        )
        .with_detail(error::detail(s)),
        _ => error::status(s),
    }
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
