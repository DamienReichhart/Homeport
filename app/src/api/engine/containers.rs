//! Containers on a Host: their lifecycle, and what runs inside one.

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_engine::containers::{
    self,
    commit::{self, CommitSpec},
    health::{self, Health},
    logs::{self, LogEvent, LogOptions, LogSource},
    recreate::{self, Loss},
    run::{self, RunSpec},
    stats::{self, StatsSample},
    Container, ContainerAction, Processes,
};
use homeport_engine::podman;

use crate::api::hosts::registries;
use crate::app::AppState;

pub async fn containers_list(state: &AppState, host_id: String) -> AppResult<Vec<Container>> {
    containers::list(&state.hosts.registry.docker(&host_id)?).await
}

pub async fn containers_inspect(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<serde_json::Value> {
    containers::inspect(&state.hosts.registry.docker(&host_id)?, &id).await
}

pub async fn containers_act(
    state: &AppState,
    host_id: String,
    id: String,
    action: ContainerAction,
) -> AppResult<()> {
    containers::act(&state.hosts.registry.docker(&host_id)?, &id, action).await
}

pub async fn containers_remove(
    state: &AppState,
    host_id: String,
    id: String,
    force: bool,
    volumes: bool,
) -> AppResult<()> {
    containers::remove(&state.hosts.registry.docker(&host_id)?, &id, force, volumes).await
}

pub async fn containers_rename(
    state: &AppState,
    host_id: String,
    id: String,
    name: String,
) -> AppResult<()> {
    containers::rename(&state.hosts.registry.docker(&host_id)?, &id, &name).await
}

pub async fn containers_processes(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<Processes> {
    containers::processes(&state.hosts.registry.docker(&host_id)?, &id).await
}

/// Creates and starts a container, in the Podman pod `pod` when given;
/// returns its id. A pod on a Host that isn't Podman is `EngineUnsupported`.
pub async fn containers_run(
    state: &AppState,
    host_id: String,
    spec: RunSpec,
    pod: Option<String>,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let image = spec.image.trim().to_owned();
    let auth = registries::auth_for(state, &image)?;
    let run = async {
        match pod.as_deref().map(str::trim).filter(|p| !p.is_empty()) {
            Some(pod) => {
                let podman = state.hosts.registry.podman(&host_id)?;
                podman::containers::run_in_pod(&docker, &podman, spec, pod, auth).await
            }
            None => run::run(&docker, spec, auth).await,
        }
    };
    run.await.map_err(|e| registries::explain(state, &image, e))
}

/// The Run dialog's form for an existing container, for Duplicate and Edit.
pub async fn containers_run_spec(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<RunSpec> {
    run::spec_for(&state.hosts.registry.docker(&host_id)?, &id).await
}

/// A container's healthcheck, status and last results; `None` without one.
pub async fn containers_health(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<Option<Health>> {
    health::health(&state.hosts.registry.docker(&host_id)?, &id).await
}

/// Commits a container to a new image; returns the image's id.
pub async fn containers_commit(
    state: &AppState,
    host_id: String,
    id: String,
    spec: CommitSpec,
) -> AppResult<String> {
    commit::commit(&state.hosts.registry.docker(&host_id)?, &id, spec).await
}

/// What recreating a container with a changed configuration loses, for Edit's confirmation.
pub async fn containers_edit_losses(
    state: &AppState,
    host_id: String,
    id: String,
) -> AppResult<Vec<Loss>> {
    recreate::losses(&state.hosts.registry.docker(&host_id)?, &id).await
}

/// Recreates a container from `spec` under the same name (Edit); returns the new id.
pub async fn containers_recreate(
    state: &AppState,
    host_id: String,
    id: String,
    spec: RunSpec,
) -> AppResult<String> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let image = spec.image.trim().to_owned();
    let auth = registries::auth_for(state, &image)?;
    recreate::recreate(&docker, &id, spec, auth)
        .await
        .map_err(|e| registries::explain(state, &image, e))
}

pub async fn logs_subscribe(
    state: &AppState,
    host_id: String,
    sources: Vec<LogSource>,
    options: LogOptions,
    on_event: impl Fn(LogEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(logs::stream(docker, sources, options, on_event)))
}

pub async fn stats_subscribe(
    state: &AppState,
    host_id: String,
    container_id: String,
    on_sample: impl Fn(StatsSample) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    let docker = state.hosts.registry.docker(&host_id)?;
    Ok(state
        .streams
        .spawn(stats::stream(docker, container_id, on_sample)))
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, host};
    use crate::app::testing::app;

    fn log_options() -> LogOptions {
        LogOptions {
            tail: Some(10),
            timestamps: false,
            since: None,
            stdout: true,
            stderr: true,
            follow: false,
        }
    }

    #[tokio::test]
    async fn a_host_that_isnt_connected_is_refused() {
        let app = app();
        let s = &app.state;
        let id = || "c1".to_owned();
        let spec = RunSpec::default;
        let sources = vec![LogSource {
            container_id: id(),
            label: None,
        }];
        let pod = Some("pod".to_owned());
        let start = ContainerAction::Start;
        let results = [
            ("list", code(containers_list(s, host()).await)),
            ("inspect", code(containers_inspect(s, host(), id()).await)),
            ("act", code(containers_act(s, host(), id(), start).await)),
            (
                "remove",
                code(containers_remove(s, host(), id(), true, false).await),
            ),
            (
                "rename",
                code(containers_rename(s, host(), id(), "new".into()).await),
            ),
            (
                "processes",
                code(containers_processes(s, host(), id()).await),
            ),
            ("run", code(containers_run(s, host(), spec(), None).await)),
            (
                "run in a pod",
                code(containers_run(s, host(), spec(), pod).await),
            ),
            ("run_spec", code(containers_run_spec(s, host(), id()).await)),
            ("health", code(containers_health(s, host(), id()).await)),
            (
                "commit",
                code(containers_commit(s, host(), id(), CommitSpec::default()).await),
            ),
            (
                "edit_losses",
                code(containers_edit_losses(s, host(), id()).await),
            ),
            (
                "recreate",
                code(containers_recreate(s, host(), id(), spec()).await),
            ),
            (
                "logs",
                code(logs_subscribe(s, host(), sources, log_options(), |_| true).await),
            ),
            (
                "stats",
                code(stats_subscribe(s, host(), id(), |_| true).await),
            ),
        ];
        assert_all(ErrorCode::HostNotConnected, &results);
    }
}
