//! A Swarm's services and their tasks.

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_engine::{
    containers::logs::{LogEvent, LogOptions},
    swarm::{
        services::{self, ServiceView},
        spec::{self, ServiceChange, ServiceSpecInput},
        tasks::{self, TaskFilter, TaskView},
    },
};

use crate::app::AppState;

/// Every service of a Swarm, with its running and desired tasks.
pub async fn swarm_services_list(
    state: &AppState,
    swarm_id: String,
) -> AppResult<Vec<ServiceView>> {
    services::list(&state.hosts.registry.manager(&swarm_id)?).await
}

pub async fn swarm_services_inspect(
    state: &AppState,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    services::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Sets a replicated service's replicas, from the service as read at `version`.
pub async fn swarm_services_scale(
    state: &AppState,
    swarm_id: String,
    id: String,
    version: u64,
    replicas: u64,
) -> AppResult<()> {
    services::scale(
        &state.hosts.registry.manager(&swarm_id)?,
        &id,
        version,
        replicas,
    )
    .await
}

/// Puts back a service's previous spec.
pub async fn swarm_services_rollback(
    state: &AppState,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::rollback(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

pub async fn swarm_services_remove(
    state: &AppState,
    swarm_id: String,
    id: String,
) -> AppResult<()> {
    services::remove(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// Streams the logs of every task of a service, each line labelled with its
/// task and node.
pub async fn swarm_service_logs_subscribe(
    state: &AppState,
    swarm_id: String,
    id: String,
    options: LogOptions,
    on_event: impl Fn(LogEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.manager(&swarm_id)?;
    Ok(state
        .streams
        .spawn(services::stream_logs(docker, id, options, on_event)))
}

/// The tasks of a Swarm, of one of its services or of one of its nodes,
/// newest first, each with the Host its node is, if it's one.
pub async fn swarm_tasks_list(
    state: &AppState,
    swarm_id: String,
    filter: TaskFilter,
) -> AppResult<Vec<SwarmTask>> {
    let tasks = tasks::list(&state.hosts.registry.manager(&swarm_id)?, &filter).await?;
    let hosts = state.hosts.registry.node_hosts();
    Ok(tasks
        .into_iter()
        .map(|task| SwarmTask {
            host_id: task.node_id.as_ref().and_then(|n| hosts.get(n).cloned()),
            task,
        })
        .collect())
}

pub async fn swarm_tasks_inspect(
    state: &AppState,
    swarm_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    tasks::inspect(&state.hosts.registry.manager(&swarm_id)?, &id).await
}

/// A task of a Swarm, and the Host its node is when Homeport connects to it.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwarmTask {
    #[serde(flatten)]
    pub task: TaskView,
    pub host_id: Option<String>,
}

/// Creates a service from the Create service form, returning its ID.
pub async fn swarm_services_create(
    state: &AppState,
    swarm_id: String,
    input: ServiceSpecInput,
) -> AppResult<String> {
    spec::create(&state.hosts.registry.manager(&swarm_id)?, &input).await
}

/// Changes a service's image, environment or replicas, from the service as
/// read at `version`; the Swarm rolls its tasks over.
pub async fn swarm_services_update(
    state: &AppState,
    swarm_id: String,
    id: String,
    version: u64,
    change: ServiceChange,
) -> AppResult<()> {
    spec::update(
        &state.hosts.registry.manager(&swarm_id)?,
        &id,
        version,
        &change,
    )
    .await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, swarm};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_swarm_without_a_connected_manager_is_unreachable() {
        let app = app();
        let s = &app.state;
        let id = || "svc1".to_owned();
        let input = ServiceSpecInput {
            name: "web".into(),
            image: "nginx:1".into(),
            mode: spec::ModeInput::Replicated { replicas: 2 },
            env: Vec::new(),
            command: None,
            ports: Vec::new(),
            networks: Vec::new(),
            mounts: Vec::new(),
            secrets: Vec::new(),
            configs: Vec::new(),
            restart: None,
        };
        let logs = LogOptions {
            tail: None,
            timestamps: true,
            since: None,
            stdout: true,
            stderr: true,
            follow: true,
        };
        let change = ServiceChange::default();
        let results = [
            ("list", code(swarm_services_list(s, swarm()).await)),
            (
                "inspect",
                code(swarm_services_inspect(s, swarm(), id()).await),
            ),
            (
                "scale",
                code(swarm_services_scale(s, swarm(), id(), 1, 3).await),
            ),
            (
                "rollback",
                code(swarm_services_rollback(s, swarm(), id()).await),
            ),
            (
                "remove",
                code(swarm_services_remove(s, swarm(), id()).await),
            ),
            (
                "logs",
                code(swarm_service_logs_subscribe(s, swarm(), id(), logs, |_| true).await),
            ),
            (
                "tasks_list",
                code(swarm_tasks_list(s, swarm(), TaskFilter::default()).await),
            ),
            (
                "tasks_inspect",
                code(swarm_tasks_inspect(s, swarm(), id()).await),
            ),
            (
                "create",
                code(swarm_services_create(s, swarm(), input).await),
            ),
            (
                "update",
                code(swarm_services_update(s, swarm(), id(), 1, change).await),
            ),
        ];
        assert_all(ErrorCode::SwarmUnreachable, &results);
    }
}
