//! Stacks on a Host: finding them, Managed, Linked and Synced Stacks'
//! definitions, deploying through the runtime, their deploy history and what
//! a redeploy would change.

pub mod repositories;
pub(crate) mod target;

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::containers;
use homeport_git::cli as git;
use homeport_stacks::{
    self as stacks,
    compose::{ComposeAction, ComposeEvent},
    definition::{self, ComposeCheck},
    diff::{Candidate, DeployDiff},
    history::{self, DeployDetail, DeploySummary},
    lifecycle::StackAction,
    linked::{
        files::LinkedFiles,
        folder::{self, FolderProject},
        LinkedStack,
    },
    runtime::{self, SourceKind},
    store::{ComposeFile, ManagedStack},
    synced::{SyncReview, SyncedDraft, SyncedFiles, SyncedStack},
    Stack, StackOwner,
};

use crate::{app::AppState, error::AppResult};

/// Every Stack on a Host: those running there plus undeployed Managed,
/// Linked and Synced Stacks. Stacks started from compose files on this machine are
/// recorded as Linked the first time they're seen.
#[tauri::command]
#[specta::specta]
pub async fn stacks_list(state: State<'_, AppState>, host_id: String) -> AppResult<Vec<Stack>> {
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

#[tauri::command]
#[specta::specta]
pub fn stacks_get(state: State<'_, AppState>, id: String) -> AppResult<ManagedStack> {
    state.stacks.managed.get(&id)
}

/// The decrypted environment of a Managed Stack, for editing.
#[tauri::command]
#[specta::specta]
pub fn stacks_get_env(state: State<'_, AppState>, id: String) -> AppResult<String> {
    state.stacks.managed.env(&id)
}

/// Checks a definition made of `files` (their YAML, merged in order) against
/// the rules of its kind of Stack, and of a Swarm's when `swarm`.
#[tauri::command]
#[specta::specta]
pub fn stacks_check(files: Vec<String>, kind: SourceKind, swarm: bool) -> ComposeCheck {
    let files: Vec<&str> = files.iter().map(String::as_str).collect();
    definition::check_for(&files, kind, swarm)
}

/// Creates a Managed Stack on a Host or a Swarm. With `adopt`, it takes
/// over the External Stack of that name; otherwise a name that runs there
/// outside Homeport is refused, since deploying would replace it.
#[tauri::command]
#[specta::specta]
pub async fn stacks_create(
    state: State<'_, AppState>,
    owner: StackOwner,
    name: String,
    files: Vec<ComposeFile>,
    env: String,
    adopt: bool,
) -> AppResult<ManagedStack> {
    target::known(&state, &owner)?;
    if !adopt {
        let engine = target::engine(&state, &owner)?;
        state
            .stacks
            .refuse_taking_over(&engine, &owner, &name, None)
            .await?;
    }
    state.stacks.managed.create(&owner, &name, &files, &env)
}

/// Whether deploying a Stack to a Swarm sends the docker CLI's own registry
/// logins (`--with-registry-auth`).
#[tauri::command]
#[specta::specta]
pub fn stacks_set_registry_auth(
    state: State<'_, AppState>,
    kind: SourceKind,
    id: String,
    on: bool,
) -> AppResult<bool> {
    state.stacks.set_registry_auth(kind, &id, on)
}

#[tauri::command]
#[specta::specta]
pub fn stacks_update(
    state: State<'_, AppState>,
    id: String,
    files: Vec<ComposeFile>,
    env: String,
) -> AppResult<ManagedStack> {
    state.stacks.managed.update(&id, &files, &env)
}

/// Forgets a Managed Stack's definition; its containers keep running and it
/// becomes an External Stack.
#[tauri::command]
#[specta::specta]
pub fn stacks_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.stacks.managed.delete(&id)
}

/// What Compose would make of a folder picked for a new Linked Stack.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_inspect(path: String) -> AppResult<FolderProject> {
    folder::inspect(std::path::Path::new(&path))
}

/// Records a Linked Stack of a Host or a Swarm for the folder `working_dir`
/// and its compose files. It may take the name of an External Stack only
/// when compose started that one from this folder: it is the same project.
#[tauri::command]
#[specta::specta]
pub async fn stacks_linked_create(
    state: State<'_, AppState>,
    owner: StackOwner,
    name: String,
    working_dir: String,
    config_files: Vec<String>,
) -> AppResult<LinkedStack> {
    target::known(&state, &owner)?;
    let engine = target::engine(&state, &owner)?;
    state
        .stacks
        .link(&engine, &owner, &name, &working_dir, &config_files)
        .await
}

/// A Linked Stack's compose files, read from its folder. `acknowledge` notes
/// them as seen, so they no longer count as changed outside Homeport.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_read(
    state: State<'_, AppState>,
    id: String,
    acknowledge: bool,
) -> AppResult<LinkedFiles> {
    state.stacks.linked_files(&id, acknowledge)
}

/// Writes one compose file of a Linked Stack in place. `expected` is the
/// fingerprint of the file the edit started from; `None` overwrites it even
/// when it changed outside Homeport.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_write(
    state: State<'_, AppState>,
    id: String,
    path: String,
    content: String,
    expected: Option<String>,
) -> AppResult<LinkedFiles> {
    state
        .stacks
        .write_linked_file(&id, &path, &content, expected.as_deref())
}

/// Sets which compose files of its folder a Linked Stack uses, in order.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_set_files(
    state: State<'_, AppState>,
    id: String,
    config_files: Vec<String>,
) -> AppResult<LinkedFiles> {
    state.stacks.set_linked_files(&id, &config_files)
}

/// Forgets a Linked Stack: its record goes, its folder and containers stay.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.stacks.linked.delete(&id)
}

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

/// Sets the Compose profiles a Managed, Linked or Synced Stack deploys; they
/// apply from its next compose command.
#[tauri::command]
#[specta::specta]
pub fn stacks_set_profiles(
    state: State<'_, AppState>,
    kind: SourceKind,
    id: String,
    profiles: Vec<String>,
) -> AppResult<Vec<String>> {
    state.stacks.set_profiles(kind, &id, &profiles)
}

#[tauri::command]
#[specta::specta]
pub async fn stacks_act(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    action: StackAction,
) -> AppResult<()> {
    runtime::act(
        &target::of(&state, &StackOwner::from(&host_id))?,
        &name,
        action,
    )
    .await
}

/// Removes a Stack's containers and networks through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn stacks_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    runtime::remove(&target::of(&state, &StackOwner::from(&host_id))?, &name).await
}

/// Runs a compose command for the Managed, Linked or Synced Stack `id`,
/// streaming its output. A Synced Stack's deploy that succeeds is noted as
/// its deployed commit.
#[tauri::command]
#[specta::specta]
pub async fn stacks_compose_subscribe(
    state: State<'_, AppState>,
    kind: SourceKind,
    id: String,
    action: ComposeAction,
    on_event: Channel<ComposeEvent>,
) -> AppResult<StreamId> {
    spawn(&state, kind, id, action, None, on_event).await
}

/// Starts the compose job for `action` on the Stack `id`, `commit` for a
/// Synced Stack's reviewed one; a Synced Stack's deploy that succeeds is
/// noted as its deployed commit.
async fn spawn(
    state: &AppState,
    kind: SourceKind,
    id: String,
    action: ComposeAction,
    commit: Option<String>,
    on_event: Channel<ComposeEvent>,
) -> AppResult<StreamId> {
    let (job, commit) = target::job(state, kind, &id, action, commit.as_deref(), false).await?;
    let synced = state.stacks.synced.clone();
    Ok(state.streams.spawn(runtime::run(job, move |e| {
        if let (Some(commit), ComposeEvent::Exit { success: true, .. }) = (&commit, &e) {
            if action.deploys() {
                if let Err(e) = synced.store.set_deployed(&id, commit) {
                    e.logged("noting a Synced Stack's deployed commit");
                }
            }
        }
        on_event.send(e).is_ok()
    })))
}

/// What redeploying the Managed, Linked or Synced Stack `id` would change:
/// its latest successful deploy against `candidate`, or against its stored
/// definition (a Linked Stack's files as they are now, a Synced Stack's
/// compose file at its fetched commit) when there's none. Environment values
/// are compared here and never sent back.
#[tauri::command]
#[specta::specta]
pub async fn stacks_diff(
    state: State<'_, AppState>,
    kind: SourceKind,
    id: String,
    candidate: Option<Candidate>,
) -> AppResult<DeployDiff> {
    state.stacks.diff(kind, &id, candidate).await
}

/// A Stack's recorded deploys, newest first.
#[tauri::command]
#[specta::specta]
pub fn stacks_history(state: State<'_, AppState>, id: String) -> AppResult<Vec<DeploySummary>> {
    state.stacks.history.list(&id)
}

/// One recorded deploy, with its definition and the names of its variables.
#[tauri::command]
#[specta::specta]
pub fn stacks_deploy(state: State<'_, AppState>, id: String) -> AppResult<DeployDetail> {
    Ok(DeployDetail {
        deploy: state.stacks.history.get(&id)?,
        env_keys: state.stacks.history.env_keys(&id)?,
    })
}

/// Rolls a Managed Stack back to the recorded deploy `deploy_id`: puts its
/// definition, environment and profiles back, then redeploys, streaming the
/// output.
#[tauri::command]
#[specta::specta]
pub async fn stacks_rollback_subscribe(
    state: State<'_, AppState>,
    deploy_id: String,
    on_event: Channel<ComposeEvent>,
) -> AppResult<StreamId> {
    let stack = history::restore(&state.stacks.history, &state.stacks.managed, &deploy_id)?;
    let (job, _) = target::job(
        &state,
        SourceKind::Managed,
        &stack.id,
        ComposeAction::Up,
        None,
        false,
    )
    .await?;
    Ok(state
        .streams
        .spawn(runtime::run(job, move |e| on_event.send(e).is_ok())))
}
