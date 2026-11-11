//! What only Podman offers, through its own API: pods (containers run into
//! one go through `containers_run`) and Quadlet units, whose services start
//! and stop with `systemctl` where the Host runs commands (ADR 0019). A
//! Host that isn't Podman answers `EngineUnsupported`.

use homeport_core::error::AppResult;
use homeport_engine::{
    containers,
    podman::{
        pods::{self, Pod, PodAction, PodDetail, PodSpec},
        quadlets::{self, QuadletInstall, QuadletUnits, UnitAction, UnitScope},
    },
};

use crate::app::AppState;

pub async fn pods_list(state: &AppState, host_id: String) -> AppResult<Vec<Pod>> {
    pods::list(&state.hosts.registry.podman(&host_id)?).await
}

pub async fn pods_inspect(state: &AppState, host_id: String, id: String) -> AppResult<PodDetail> {
    pods::inspect(&state.hosts.registry.podman(&host_id)?, &id).await
}

pub async fn pods_act(
    state: &AppState,
    host_id: String,
    id: String,
    action: PodAction,
) -> AppResult<()> {
    pods::act(&state.hosts.registry.podman(&host_id)?, &id, action).await
}

/// Removes a pod and its containers; returns how many containers went with it.
pub async fn pods_remove(
    state: &AppState,
    host_id: String,
    id: String,
    force: bool,
) -> AppResult<u32> {
    pods::remove(&state.hosts.registry.podman(&host_id)?, &id, force).await
}

/// Creates a pod; returns its id.
pub async fn pods_create(state: &AppState, host_id: String, spec: PodSpec) -> AppResult<String> {
    pods::create(&state.hosts.registry.podman(&host_id)?, &spec).await
}

/// The Host's Quadlet units, with the containers each created.
pub async fn quadlets_list(state: &AppState, host_id: String) -> AppResult<QuadletUnits> {
    let podman = state.hosts.registry.podman(&host_id)?;
    let containers = containers::list(&state.hosts.registry.docker(&host_id)?).await?;
    quadlets::list(&podman, &containers).await
}

/// A unit's file, as Podman reads it on the Host (Podman 5.8 and later).
pub async fn quadlets_file(state: &AppState, host_id: String, name: String) -> AppResult<String> {
    quadlets::file(&state.hosts.registry.podman(&host_id)?, &name).await
}

/// Installs a unit with its extra files; `replace` edits an installed one.
/// `expected` is the unit file as the edit opened it: a file changed on the
/// Host since is refused, unless `None` overwrites it. Returns where the
/// files went on the Host.
pub async fn quadlets_install(
    state: &AppState,
    host_id: String,
    install: QuadletInstall,
    replace: bool,
    expected: Option<String>,
) -> AppResult<Vec<String>> {
    quadlets::install(
        &state.hosts.registry.podman(&host_id)?,
        &install,
        replace,
        expected.as_deref(),
    )
    .await
}

/// Removes a unit; `force` stops its service first.
pub async fn quadlets_remove(
    state: &AppState,
    host_id: String,
    name: String,
    force: bool,
) -> AppResult<()> {
    quadlets::remove(&state.hosts.registry.podman(&host_id)?, &name, force).await
}

/// Starts, stops or restarts a unit's service with `systemctl`.
pub async fn quadlets_act(
    state: &AppState,
    host_id: String,
    unit: String,
    scope: UnitScope,
    action: UnitAction,
) -> AppResult<()> {
    quadlets::act(
        &state.hosts.registry.podman(&host_id)?,
        &unit,
        scope,
        action,
    )
    .await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, host};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_host_that_isnt_connected_is_refused() {
        let app = app();
        let s = &app.state;
        let id = || "web".to_owned();
        let install = QuadletInstall {
            unit: quadlets::QuadletFile {
                name: "web.container".into(),
                content: "[Container]\nImage=nginx\n".into(),
            },
            extra: Vec::new(),
            app: None,
        };
        let (scope, restart) = (UnitScope::User, UnitAction::Restart);
        let results = [
            ("pods_list", code(pods_list(s, host()).await)),
            ("pods_inspect", code(pods_inspect(s, host(), id()).await)),
            (
                "pods_act",
                code(pods_act(s, host(), id(), PodAction::Start).await),
            ),
            (
                "pods_remove",
                code(pods_remove(s, host(), id(), true).await),
            ),
            (
                "pods_create",
                code(pods_create(s, host(), PodSpec::default()).await),
            ),
            ("quadlets_list", code(quadlets_list(s, host()).await)),
            ("quadlets_file", code(quadlets_file(s, host(), id()).await)),
            (
                "quadlets_install",
                code(quadlets_install(s, host(), install, false, None).await),
            ),
            (
                "quadlets_remove",
                code(quadlets_remove(s, host(), id(), false).await),
            ),
            (
                "quadlets_act",
                code(quadlets_act(s, host(), id(), scope, restart).await),
            ),
        ];
        assert_all(ErrorCode::HostNotConnected, &results);
    }
}
