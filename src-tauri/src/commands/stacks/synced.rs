//! Synced Stacks: compose files from a git repository (ADR 0014).

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_git::cli as git;
use homeport_stacks::{
    compose::{ComposeAction, ComposeEvent},
    runtime::SourceKind,
    synced::{SyncReview, SyncedDraft, SyncedFiles, SyncedStack},
    StackOwner,
};

use super::deploy::spawn;
use super::target;
use crate::{app::AppState, error::AppResult};

/// Creates a Synced Stack on a Host or a Swarm from a repository's branch or
/// tag: fetches it into the repository's clone and checks the compose file
/// before anything is stored.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_create(
    state: State<'_, AppState>,
    owner: StackOwner,
    draft: SyncedDraft,
    env: String,
) -> AppResult<SyncedStack> {
    target::known(&state, &owner)?;
    let engine = target::engine(&state, &owner)?;
    state
        .stacks
        .refuse_taking_over(&engine, &owner, &draft.name, None)
        .await?;
    let cli = git::detect().await?;
    state.stacks.synced.create(&cli, &owner, &draft, &env).await
}

#[tauri::command]
#[specta::specta]
pub fn stacks_synced_get(state: State<'_, AppState>, id: String) -> AppResult<SyncedStack> {
    state.stacks.synced.store.get(&id)
}

/// A Synced Stack's files at its fetched commit, read-only: no command
/// writes them, since they change only through git.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_files(state: State<'_, AppState>, id: String) -> AppResult<SyncedFiles> {
    let stack = state.stacks.synced.store.get(&id)?;
    let cli = git::detect().await?;
    state.stacks.synced.files(&cli, &stack).await
}

/// The decrypted environment of a Synced Stack, for editing.
#[tauri::command]
#[specta::specta]
pub fn stacks_synced_get_env(state: State<'_, AppState>, id: String) -> AppResult<String> {
    state.stacks.synced.store.env(&id)
}

/// Replaces a Synced Stack's environment, the one part of its definition
/// that's Homeport's; its compose files only change through git.
#[tauri::command]
#[specta::specta]
pub fn stacks_synced_set_env(
    state: State<'_, AppState>,
    id: String,
    env: String,
) -> AppResult<SyncedStack> {
    state.stacks.synced.store.set_env(&id, &env)
}

/// "Sync": fetches a Synced Stack's branch or tag, and says what deploying
/// the new commit would change since the deployed one.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_sync(state: State<'_, AppState>, id: String) -> AppResult<SyncReview> {
    let cli = git::detect().await?;
    state.stacks.synced.sync(&cli, &id).await
}

/// Deploys `commit` of a Synced Stack, the one its sync review showed, with
/// `action` (`up` or `rebuild`), streaming the output.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_deploy_subscribe(
    state: State<'_, AppState>,
    id: String,
    commit: String,
    action: ComposeAction,
    on_event: Channel<ComposeEvent>,
) -> AppResult<StreamId> {
    spawn(
        &state,
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
#[tauri::command]
#[specta::specta]
pub fn stacks_synced_set_sync(
    state: State<'_, AppState>,
    id: String,
    poll: bool,
    auto_deploy: bool,
) -> AppResult<SyncedStack> {
    state.stacks.synced.store.set_sync(&id, poll, auto_deploy)
}

/// Forgets a Synced Stack: its record, history and worktree go, its
/// containers are left alone and it becomes External.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let cli = git::detect().await.ok();
    state.stacks.synced.forget(cli.as_ref(), &id).await
}
