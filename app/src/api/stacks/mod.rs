//! Stacks on a Host: finding them, Managed, Linked and Synced Stacks'
//! definitions, deploying through the runtime, their deploy history and what
//! a redeploy would change.

pub mod deploy;
#[cfg(test)]
mod fixtures;
pub mod linked;
pub mod managed;
pub mod polling;
pub mod repositories;
pub mod synced;
pub(crate) mod target;

use homeport_core::error::AppResult;
use homeport_engine::containers;
use homeport_stacks::{
    self as stacks,
    model::definition::{self, ComposeCheck},
    runtime::{self, lifecycle::StackAction, SourceKind},
    Stack, StackOwner,
};

use crate::app::AppState;

/// Every Stack on a Host: those running there plus undeployed Managed,
/// Linked and Synced Stacks. Stacks started from compose files on this machine are
/// recorded as Linked the first time they're seen.
pub async fn stacks_list(state: &AppState, host_id: String) -> AppResult<Vec<Stack>> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let running = containers::list(&docker).await?;
    stacks::list(
        &host_id,
        &running,
        &state.stacks.managed,
        &state.stacks.linked,
        &state.stacks.synced.store,
        &state.stacks.linked_files,
    )
}

/// Checks a definition made of `files` (their YAML, merged in order) against
/// the rules of its kind of Stack, and of a Swarm's when `swarm`.
pub fn stacks_check(files: Vec<String>, kind: SourceKind, swarm: bool) -> ComposeCheck {
    let files: Vec<&str> = files.iter().map(String::as_str).collect();
    definition::check_for(&files, kind, swarm)
}

/// Whether deploying a Stack to a Swarm sends the docker CLI's own registry
/// logins (`--with-registry-auth`).
pub fn stacks_set_registry_auth(
    state: &AppState,
    kind: SourceKind,
    id: String,
    on: bool,
) -> AppResult<bool> {
    state.stacks.set_registry_auth(kind, &id, on)
}

/// Sets the Compose profiles a Managed, Linked or Synced Stack deploys; they
/// apply from its next compose command.
pub fn stacks_set_profiles(
    state: &AppState,
    kind: SourceKind,
    id: String,
    profiles: Vec<String>,
) -> AppResult<Vec<String>> {
    state.stacks.set_profiles(kind, &id, &profiles)
}

pub async fn stacks_act(
    state: &AppState,
    host_id: String,
    name: String,
    action: StackAction,
) -> AppResult<()> {
    runtime::act(
        &target::of(state, &StackOwner::from(&host_id))?,
        &name,
        action,
    )
    .await
}

/// Removes a Stack's containers and networks through the Engine API.
pub async fn stacks_remove(state: &AppState, host_id: String, name: String) -> AppResult<()> {
    runtime::remove(&target::of(state, &StackOwner::from(&host_id))?, &name).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;
    use homeport_stacks::runtime::{lifecycle::StackAction, SourceKind};

    use super::fixtures;
    use crate::app::testing;

    #[tokio::test]
    async fn listing_needs_the_hosts_engine() {
        let app = testing::app();
        let owner = fixtures::host(&app.state);
        let e = super::stacks_list(&app.state, owner.id().to_owned())
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::HostNotConnected);
    }

    #[test]
    fn a_definition_is_checked_against_its_kind() {
        let good = super::stacks_check(vec![fixtures::COMPOSE.into()], SourceKind::Managed, false);
        assert!(good.is_deployable(), "{:?}", good.errors);
        let bad = super::stacks_check(vec!["services: [".into()], SourceKind::Managed, false);
        assert!(!bad.errors.is_empty());
    }

    #[tokio::test]
    async fn profiles_and_registry_auth_are_stored_on_the_stack() {
        let app = testing::app();
        let stack = fixtures::managed(&app.state, &fixtures::host(&app.state));
        let profiles = super::stacks_set_profiles(
            &app.state,
            SourceKind::Managed,
            stack.id.clone(),
            vec!["debug".into()],
        )
        .unwrap();
        assert_eq!(profiles, ["debug"]);
        let on = super::stacks_set_registry_auth(
            &app.state,
            SourceKind::Managed,
            stack.id.clone(),
            true,
        )
        .unwrap();
        assert!(on);
        let stored = app.state.stacks.managed.get(&stack.id).unwrap();
        assert_eq!(stored.profiles, ["debug"]);
        assert!(stored.registry_auth);
        let e = super::stacks_set_profiles(&app.state, SourceKind::Linked, "gone".into(), vec![])
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn acting_on_a_stack_needs_a_known_connected_host() {
        let app = testing::app();
        let e = super::stacks_act(&app.state, "gone".into(), "web".into(), StackAction::Stop)
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        let e = super::stacks_remove(&app.state, "gone".into(), "web".into())
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        let host = fixtures::host(&app.state).id().to_owned();
        let e = super::stacks_act(&app.state, host.clone(), "web".into(), StackAction::Start)
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::HostNotConnected);
        let e = super::stacks_remove(&app.state, host, "web".into())
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::HostNotConnected);
    }
}
