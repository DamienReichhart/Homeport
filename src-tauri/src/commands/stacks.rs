//! Stacks on a Host: finding them, Managed, Linked and Synced Stacks'
//! definitions, deploying through the runtime, their deploy history and what
//! a redeploy would change.

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::containers;
use homeport_git::cli as git;
use homeport_stacks::{
    self as stacks,
    compose::{self, ComposeAction, ComposeEvent},
    definition::{self, ComposeCheck},
    diff::{self, Candidate, DeployDiff},
    history::{self, DeployDetail, DeploySummary},
    lifecycle::StackAction,
    linked::{
        self,
        files::LinkedFiles,
        folder::{self, FolderProject},
        LinkedStack,
    },
    runtime::{self, ComposeJob, Reach, Recorder, SourceKind, StackSource, StackTarget},
    store::{ComposeFile, ManagedStack},
    synced::{SyncReview, SyncedDraft, SyncedFiles, SyncedStack},
    Stack, StackOwner,
};

use crate::{app::AppState, commands::registries, error::AppResult};

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
        &state.stacks,
        &state.linked,
        &state.synced.store,
        &state.linked_files,
    )
}

#[tauri::command]
#[specta::specta]
pub fn stacks_get(state: State<'_, AppState>, id: String) -> AppResult<ManagedStack> {
    state.stacks.get(&id)
}

/// The decrypted environment of a Managed Stack, for editing.
#[tauri::command]
#[specta::specta]
pub fn stacks_get_env(state: State<'_, AppState>, id: String) -> AppResult<String> {
    state.stacks.env(&id)
}

/// Checks a definition made of `files` (their YAML, merged in order) against
/// the rules of its kind of Stack, and of a Swarm's when `swarm`.
#[tauri::command]
#[specta::specta]
pub fn stacks_check(files: Vec<String>, kind: SourceKind, swarm: bool) -> ComposeCheck {
    let files: Vec<&str> = files.iter().map(String::as_str).collect();
    definition::check_for(&files, kind, swarm)
}

/// Checks a new Stack's Host or Swarm is one Homeport knows.
fn known(state: &AppState, owner: &StackOwner) -> AppResult<()> {
    match owner {
        StackOwner::Host(id) => state.hosts.store.get(id).map(drop),
        StackOwner::Swarm(id) => state.hosts.registry.manager_host(id).map(drop),
    }
}

/// Creates a Managed Stack on a Host or a Swarm. Using the name of an
/// External Stack adopts it.
#[tauri::command]
#[specta::specta]
pub fn stacks_create(
    state: State<'_, AppState>,
    owner: StackOwner,
    name: String,
    files: Vec<ComposeFile>,
    env: String,
) -> AppResult<ManagedStack> {
    known(&state, &owner)?;
    state.stacks.create(&owner, &name, &files, &env)
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
    Ok(match kind {
        SourceKind::Managed => state.stacks.set_registry_auth(&id, on)?.registry_auth,
        SourceKind::Linked => state.linked.set_registry_auth(&id, on)?.registry_auth,
        SourceKind::Synced => state.synced.store.set_registry_auth(&id, on)?.registry_auth,
    })
}

#[tauri::command]
#[specta::specta]
pub fn stacks_update(
    state: State<'_, AppState>,
    id: String,
    files: Vec<ComposeFile>,
    env: String,
) -> AppResult<ManagedStack> {
    state.stacks.update(&id, &files, &env)
}

/// Forgets a Managed Stack's definition; its containers keep running and it
/// becomes an External Stack.
#[tauri::command]
#[specta::specta]
pub fn stacks_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.stacks.delete(&id)
}

/// What Compose would make of a folder picked for a new Linked Stack.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_inspect(path: String) -> AppResult<FolderProject> {
    folder::inspect(std::path::Path::new(&path))
}

/// Records a Linked Stack of a Host or a Swarm for the folder `working_dir`
/// and its compose files. Using the name of an External Stack takes it over.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_create(
    state: State<'_, AppState>,
    owner: StackOwner,
    name: String,
    working_dir: String,
    config_files: Vec<String>,
) -> AppResult<LinkedStack> {
    known(&state, &owner)?;
    // The files are checked again here: the webview only names them.
    let found = folder::inspect(std::path::Path::new(&working_dir))?;
    if config_files.is_empty()
        || config_files
            .iter()
            .any(|f| !found.files.iter().any(|g| &g.path == f))
    {
        return Err(linked::files::not_a_stack_file(
            config_files.first().map_or("", String::as_str),
        ));
    }
    let fingerprint = linked::files::files_fingerprint(&config_files);
    state
        .linked
        .create(&owner, &name, &working_dir, &config_files, &fingerprint)
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
    let stack = state.linked.get(&id)?;
    let mut files = linked::files::read(&stack)?;
    if acknowledge && files.changed_outside {
        state.linked.set_fingerprint(&id, &files.fingerprint)?;
        files.changed_outside = false;
    }
    Ok(files)
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
    let stack = state.linked.get(&id)?;
    linked::files::write(&stack, &path, &content, expected.as_deref())?;
    let stack = state
        .linked
        .set_fingerprint(&id, &linked::files::files_fingerprint(&stack.config_files))?;
    linked::files::read(&stack)
}

/// Sets which compose files of its folder a Linked Stack uses, in order.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_set_files(
    state: State<'_, AppState>,
    id: String,
    config_files: Vec<String>,
) -> AppResult<LinkedFiles> {
    let stack = state.linked.get(&id)?;
    linked::files::validate_choice(&stack.working_dir, &config_files)?;
    let fingerprint = linked::files::files_fingerprint(&config_files);
    let stack = state.linked.set_files(&id, &config_files, &fingerprint)?;
    linked::files::read(&stack)
}

/// Forgets a Linked Stack: its record goes, its folder and containers stay.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.linked.delete(&id)
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
    known(&state, &owner)?;
    let cli = git::detect().await?;
    state.synced.create(&cli, &owner, &draft, &env).await
}

#[tauri::command]
#[specta::specta]
pub fn stacks_synced_get(state: State<'_, AppState>, id: String) -> AppResult<SyncedStack> {
    state.synced.store.get(&id)
}

/// A Synced Stack's files at its fetched commit, read-only: no command
/// writes them, since they change only through git.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_files(state: State<'_, AppState>, id: String) -> AppResult<SyncedFiles> {
    let stack = state.synced.store.get(&id)?;
    let cli = git::detect().await?;
    state.synced.files(&cli, &stack).await
}

/// The decrypted environment of a Synced Stack, for editing.
#[tauri::command]
#[specta::specta]
pub fn stacks_synced_get_env(state: State<'_, AppState>, id: String) -> AppResult<String> {
    state.synced.store.env(&id)
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
    state.synced.store.set_env(&id, &env)
}

/// "Sync": fetches a Synced Stack's branch or tag, and says what deploying
/// the new commit would change since the deployed one.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_sync(state: State<'_, AppState>, id: String) -> AppResult<SyncReview> {
    let cli = git::detect().await?;
    state.synced.sync(&cli, &id).await
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
    state.synced.store.set_sync(&id, poll, auto_deploy)
}

/// Forgets a Synced Stack: its record, history and worktree go, its
/// containers are left alone and it becomes External.
#[tauri::command]
#[specta::specta]
pub async fn stacks_synced_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let cli = git::detect().await.ok();
    state.synced.forget(cli.as_ref(), &id).await
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
    Ok(match kind {
        SourceKind::Managed => state.stacks.set_profiles(&id, &profiles)?.profiles,
        SourceKind::Linked => state.linked.set_profiles(&id, &profiles)?.profiles,
        SourceKind::Synced => state.synced.store.set_profiles(&id, &profiles)?.profiles,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn stacks_act(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    action: StackAction,
) -> AppResult<()> {
    runtime::act(&target(&state, &StackOwner::from(&host_id))?, &name, action).await
}

/// Removes a Stack's containers and networks through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn stacks_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    runtime::remove(&target(&state, &StackOwner::from(&host_id))?, &name).await
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
    let (job, commit) = job(state, kind, &id, action, commit.as_deref()).await?;
    let synced = state.synced.clone();
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

/// The compose job for `action` on the Managed, Linked or Synced Stack `id`,
/// its deploys recorded in its history, with the commit a Synced Stack uses.
async fn job(
    state: &AppState,
    kind: SourceKind,
    id: &str,
    action: ComposeAction,
    commit: Option<&str>,
) -> AppResult<(ComposeJob, Option<String>)> {
    recorded_job(state, kind, id, action, commit, false).await
}

/// [`job`], recorded as started by auto-deploy when `automatic`.
pub(crate) async fn recorded_job(
    state: &AppState,
    kind: SourceKind,
    id: &str,
    action: ComposeAction,
    commit: Option<&str>,
    automatic: bool,
) -> AppResult<(ComposeJob, Option<String>)> {
    let stack = resolve(state, kind, id, action, commit).await?;
    let cli = if stack.owner.is_swarm() {
        compose::stack::detect().await?
    } else {
        compose::detect().await?
    };
    let job = ComposeJob {
        cli,
        source: stack.source,
        target: target(state, &stack.owner)?,
        logins: registries::logins(state)?,
        project: stack.name,
        action,
        profiles: stack.profiles,
        record: Some(Recorder {
            history: state.history.clone(),
            owner: stack.owner,
            stack_id: id.to_owned(),
            commit: stack.commit.clone(),
            automatic,
        }),
        registry_auth: stack.registry_auth,
    };
    Ok((job, stack.commit))
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
    let (stored_files, stored_env, profiles) = match kind {
        SourceKind::Managed => {
            let stack = state.stacks.get(&id)?;
            let env = match &candidate {
                Some(Candidate { env: Some(_), .. }) => String::new(),
                _ => state.stacks.env(&id)?,
            };
            (stack.files, env, stack.profiles)
        }
        SourceKind::Linked => {
            let stack = state.linked.get(&id)?;
            let files = match &candidate {
                Some(_) => vec![],
                None => linked::files::read(&stack)?
                    .files
                    .into_iter()
                    .map(|f| ComposeFile::new(f.path, f.content))
                    .collect(),
            };
            (files, String::new(), stack.profiles)
        }
        SourceKind::Synced => {
            let stack = state.synced.store.get(&id)?;
            // A redeploy runs the deployed commit again; a first deploy, the fetched one.
            let commit = stack
                .deployed_commit
                .as_ref()
                .or(stack.fetched_commit.as_ref());
            let files = match (&candidate, commit) {
                (None, Some(commit)) => {
                    let cli = git::detect().await?;
                    let text = state
                        .synced
                        .compose_file(&cli, &stack.repository_id, commit, &stack.compose_path)
                        .await?;
                    let path = state.synced.worktree(&stack.id).join(&stack.compose_path);
                    vec![ComposeFile::new(path.to_string_lossy(), text)]
                }
                _ => vec![],
            };
            (files, state.synced.store.env(&id)?, stack.profiles)
        }
    };
    let (files, env) = match candidate {
        Some(c) => (c.files, c.env.unwrap_or(stored_env)),
        None => (stored_files, stored_env),
    };
    let latest = state.history.latest_successful(&id)?;
    let deployed_env = match &latest {
        // An environment the keychain can't open compares as empty.
        Some(d) => state.history.env(&d.summary.id).unwrap_or_default(),
        None => String::new(),
    };
    let deployed = latest.as_ref().map(|d| {
        (
            d.summary.started_at.as_str(),
            diff::Definition {
                files: &d.files,
                env: &deployed_env,
                profiles: &d.profiles,
            },
        )
    });
    Ok(diff::diff(
        deployed,
        diff::Definition {
            files: &files,
            env: &env,
            profiles: &profiles,
        },
    ))
}

/// A Stack's recorded deploys, newest first.
#[tauri::command]
#[specta::specta]
pub fn stacks_history(state: State<'_, AppState>, id: String) -> AppResult<Vec<DeploySummary>> {
    state.history.list(&id)
}

/// One recorded deploy, with its definition and the names of its variables.
#[tauri::command]
#[specta::specta]
pub fn stacks_deploy(state: State<'_, AppState>, id: String) -> AppResult<DeployDetail> {
    Ok(DeployDetail {
        deploy: state.history.get(&id)?,
        env_keys: state.history.env_keys(&id)?,
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
    let stack = history::restore(&state.history, &state.stacks, &deploy_id)?;
    let (job, _) = job(
        &state,
        SourceKind::Managed,
        &stack.id,
        ComposeAction::Up,
        None,
    )
    .await?;
    Ok(state
        .streams
        .spawn(runtime::run(job, move |e| on_event.send(e).is_ok())))
}

/// A Managed, Linked or Synced Stack, ready to run.
struct Resolved {
    owner: StackOwner,
    registry_auth: bool,
    name: String,
    source: StackSource,
    profiles: Vec<String>,
    /// The commit a Synced Stack runs.
    commit: Option<String>,
}

/// The Host, name, source and profiles of the Managed, Linked or Synced
/// Stack `id`; a Synced Stack's commit is checked out for `action` first.
async fn resolve(
    state: &AppState,
    kind: SourceKind,
    id: &str,
    action: ComposeAction,
    commit: Option<&str>,
) -> AppResult<Resolved> {
    Ok(match kind {
        SourceKind::Managed => {
            let stack = state.stacks.get(id)?;
            Resolved {
                source: StackSource::Managed {
                    env: state.stacks.env(id)?,
                    files: stack.files,
                },
                owner: stack.owner,
                registry_auth: stack.registry_auth,
                name: stack.name,
                profiles: stack.profiles,
                commit: None,
            }
        }
        SourceKind::Linked => {
            let stack = state.linked.get(id)?;
            Resolved {
                source: StackSource::Linked {
                    working_dir: stack.working_dir.into(),
                    config_files: stack.config_files,
                },
                owner: stack.owner,
                registry_auth: stack.registry_auth,
                name: stack.name,
                profiles: stack.profiles,
                commit: None,
            }
        }
        SourceKind::Synced => {
            let stack = state.synced.store.get(id)?;
            let cli = git::detect().await?;
            let (source, commit) = state.synced.source(&cli, &stack, action, commit).await?;
            Resolved {
                source,
                owner: stack.owner,
                registry_auth: stack.registry_auth,
                name: stack.name,
                profiles: stack.profiles,
                commit: Some(commit),
            }
        }
    })
}

/// How commands reach the Host `host_id`.
fn reach(state: &AppState, host_id: &str) -> AppResult<Reach> {
    let host = state.hosts.store.get(host_id)?;
    Ok(Reach {
        name: host.name,
        connection: host.connection,
        dialer: Some(state.hosts.registry.dialer(host_id)?),
        docker: state.hosts.registry.docker(host_id)?,
    })
}

/// Where the Stacks of `owner` run: a Host's Compose projects, or a Swarm's
/// swarm stacks through one of its connected managers.
pub(crate) fn target(state: &AppState, owner: &StackOwner) -> AppResult<StackTarget> {
    Ok(match owner {
        StackOwner::Host(host_id) => StackTarget::Host(reach(state, host_id)?),
        StackOwner::Swarm(swarm_id) => {
            let manager = reach(state, &state.hosts.registry.manager_host(swarm_id)?)?;
            StackTarget::Swarm {
                swarm_id: swarm_id.clone(),
                name: format!("the Swarm of {}", manager.name),
                manager,
            }
        }
    })
}
