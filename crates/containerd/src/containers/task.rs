//! A container's task, its running process: starting it, signalling it,
//! stopping it within a grace period, pausing and resuming it.

use std::time::Duration;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::{
    get,
    signals::{self, signal_number},
    task, TaskAction, NERDCTL_LOG_URI,
};
use crate::{
    grpc::{
        api::containerd::{
            services::{
                snapshots::v1::MountsRequest,
                tasks::v1::{
                    CreateTaskRequest, DeleteTaskRequest, KillRequest, PauseTaskRequest,
                    ResumeTaskRequest, StartRequest, WaitRequest,
                },
            },
            v1::types::Status,
        },
        client::{call, stream},
        error,
    },
    messages as msg, Containerd,
};

/// The signal an image asks to be stopped with.
const STOP_SIGNAL: &str = "io.containerd.image.config.stop-signal";

/// How long a stop waits after SIGKILL before saying the task won't die.
pub(super) const KILL_GRACE: Duration = Duration::from_secs(10);

/// Starts a container's task: creates it on the container's snapshot, with
/// nerdctl's logging when nerdctl made it and no I/O otherwise, then starts
/// it. A stopped task is deleted first, as `ctr task start` would need.
pub(super) async fn start(client: &Containerd, namespace: &str, id: &str) -> AppResult<()> {
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

pub(super) async fn delete_task(client: &Containerd, namespace: &str, id: &str) -> AppResult<()> {
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
pub(super) async fn exited_within(
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

pub(super) async fn resume(client: &Containerd, namespace: &str, id: &str) -> AppResult<()> {
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
