//! Synced Stacks: compose files from a git repository (ADR 0014).

use homeport_core::{error::AppResult, streams::StreamId};
use homeport_git::cli as git;
use homeport_stacks::{
    compose::{ComposeAction, ComposeEvent},
    runtime::SourceKind,
    synced::{SyncReview, SyncedDraft, SyncedFiles, SyncedStack},
    StackOwner,
};

use super::deploy::spawn;
use super::target;
use crate::app::AppState;

/// Creates a Synced Stack on a Host or a Swarm from a repository's branch or
/// tag: fetches it into the repository's clone and checks the compose file
/// before anything is stored.
pub async fn stacks_synced_create(
    state: &AppState,
    owner: StackOwner,
    draft: SyncedDraft,
    env: String,
) -> AppResult<SyncedStack> {
    target::known(state, &owner)?;
    let engine = target::engine(state, &owner)?;
    state
        .stacks
        .refuse_taking_over(&engine, &owner, &draft.name, None)
        .await?;
    let cli = git::detect().await?;
    state.stacks.synced.create(&cli, &owner, &draft, &env).await
}

pub fn stacks_synced_get(state: &AppState, id: String) -> AppResult<SyncedStack> {
    state.stacks.synced.store.get(&id)
}

/// A Synced Stack's files at its fetched commit, read-only: no command
/// writes them, since they change only through git.
pub async fn stacks_synced_files(state: &AppState, id: String) -> AppResult<SyncedFiles> {
    let stack = state.stacks.synced.store.get(&id)?;
    let cli = git::detect().await?;
    state.stacks.synced.files(&cli, &stack).await
}

/// The decrypted environment of a Synced Stack, for editing.
pub fn stacks_synced_get_env(state: &AppState, id: String) -> AppResult<String> {
    state.stacks.synced.store.env(&id)
}

/// Replaces a Synced Stack's environment, the one part of its definition
/// that's Homeport's; its compose files only change through git.
pub fn stacks_synced_set_env(state: &AppState, id: String, env: String) -> AppResult<SyncedStack> {
    state.stacks.synced.store.set_env(&id, &env)
}

/// "Sync": fetches a Synced Stack's branch or tag, and says what deploying
/// the new commit would change since the deployed one.
pub async fn stacks_synced_sync(state: &AppState, id: String) -> AppResult<SyncReview> {
    let cli = git::detect().await?;
    state.stacks.synced.sync(&cli, &id).await
}

/// Deploys `commit` of a Synced Stack, the one its sync review showed, with
/// `action` (`up` or `rebuild`), streaming the output.
pub async fn stacks_synced_deploy_subscribe(
    state: &AppState,
    id: String,
    commit: String,
    action: ComposeAction,
    on_event: impl Fn(ComposeEvent) -> bool + Send + Sync + 'static,
) -> AppResult<StreamId> {
    spawn(
        state,
        SourceKind::Synced,
        id,
        action,
        Some(commit),
        on_event,
    )
    .await
}

/// Turns a Synced Stack's polling and auto-deploy on or off; auto-deploy
/// needs polling.
pub fn stacks_synced_set_sync(
    state: &AppState,
    id: String,
    poll: bool,
    auto_deploy: bool,
) -> AppResult<SyncedStack> {
    state.stacks.synced.store.set_sync(&id, poll, auto_deploy)
}

/// Forgets a Synced Stack: its record, history and worktree go, its
/// containers are left alone and it becomes External.
pub async fn stacks_synced_forget(state: &AppState, id: String) -> AppResult<()> {
    let cli = git::detect().await.ok();
    state.stacks.synced.forget(cli.as_ref(), &id).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;
    use homeport_git::RefKind;
    use homeport_stacks::{compose::ComposeAction, synced::SyncedDraft, StackOwner};

    use super::super::fixtures;
    use crate::app::testing;

    #[tokio::test]
    async fn creating_needs_a_known_owner_and_its_engine() {
        let app = testing::app();
        let draft = SyncedDraft {
            name: "synced".into(),
            repository_id: "gone".into(),
            ref_kind: RefKind::Branch,
            ref_name: "main".into(),
            compose_path: "compose.yaml".into(),
        };
        let gone = StackOwner::Host("gone".into());
        let e = super::stacks_synced_create(&app.state, gone, draft.clone(), String::new())
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        let owner = fixtures::host(&app.state);
        let e = super::stacks_synced_create(&app.state, owner, draft, String::new())
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::HostNotConnected);
    }

    #[tokio::test]
    async fn a_synced_stacks_record_is_read_changed_and_forgotten() {
        let app = testing::app();
        let stack = fixtures::synced(&app.state, &fixtures::host(&app.state));
        let id = stack.id.clone();
        assert_eq!(
            super::stacks_synced_get(&app.state, id.clone()).unwrap(),
            stack
        );
        assert_eq!(
            super::stacks_synced_get_env(&app.state, id.clone()).unwrap(),
            "KEY=value"
        );
        let changed = super::stacks_synced_set_env(&app.state, id.clone(), "A=1".into()).unwrap();
        assert!(changed.has_env);
        assert_eq!(
            super::stacks_synced_get_env(&app.state, id.clone()).unwrap(),
            "A=1"
        );

        let polled = super::stacks_synced_set_sync(&app.state, id.clone(), true, true).unwrap();
        assert!(polled.poll && polled.auto_deploy);
        // Auto-deploy needs polling.
        let off = super::stacks_synced_set_sync(&app.state, id.clone(), false, true);
        assert!(off.map_or(true, |s| !s.auto_deploy));

        super::stacks_synced_forget(&app.state, id.clone())
            .await
            .unwrap();
        let e = super::stacks_synced_get(&app.state, id).unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn what_needs_the_clone_or_the_engine_fails_without_them() {
        let app = testing::app();
        let stack = fixtures::synced(&app.state, &fixtures::host(&app.state));
        // The repository was never cloned on this computer.
        assert!(super::stacks_synced_files(&app.state, stack.id.clone())
            .await
            .is_err());
        let e = super::stacks_synced_files(&app.state, "gone".into())
            .await
            .unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
        assert!(super::stacks_synced_sync(&app.state, "gone".into())
            .await
            .is_err());
        let started = super::stacks_synced_deploy_subscribe(
            &app.state,
            stack.id.clone(),
            "0123abc".into(),
            ComposeAction::Up,
            |_| true,
        )
        .await;
        assert!(started.is_err());
        let stored = app.state.stacks.synced.store.get(&stack.id).unwrap();
        assert_eq!(stored.deployed_commit, None);
    }
}
