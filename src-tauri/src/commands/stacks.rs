//! Stacks on a Host, and deploying Managed Stacks.

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::containers;
use homeport_stacks::{
    self as stacks,
    compose::{self, ComposeAction, ComposeEvent},
    definition::{self, ComposeCheck},
    history::{self, DeployDetail, DeploySummary},
    lifecycle::StackAction,
    linked::{
        self,
        files::LinkedFiles,
        folder::{self, FolderProject},
        LinkedStack,
    },
    runtime::{self, ComposeJob, Recorder, SourceKind, StackSource, StackTarget},
    store::{ComposeFile, ManagedStack},
    Stack,
};

use crate::{app::AppState, commands::registries, error::AppResult};

/// Every Stack on a Host: those running there plus undeployed Managed and
/// Linked Stacks. Stacks started from compose files on this machine are
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
/// the rules of its kind of Stack.
#[tauri::command]
#[specta::specta]
pub fn stacks_check(files: Vec<String>, kind: SourceKind) -> ComposeCheck {
    let files: Vec<&str> = files.iter().map(String::as_str).collect();
    definition::check(&files, kind)
}

/// Creates a Managed Stack. Using the name of an External Stack adopts it.
#[tauri::command]
#[specta::specta]
pub fn stacks_create(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    files: Vec<ComposeFile>,
    env: String,
) -> AppResult<ManagedStack> {
    state.hosts.store.get(&host_id)?;
    state.stacks.create(&host_id, &name, &files, &env)
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

/// Records a Linked Stack for the folder `working_dir` and its compose
/// files. Using the name of an External Stack takes it over.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_create(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
    working_dir: String,
    config_files: Vec<String>,
) -> AppResult<LinkedStack> {
    state.hosts.store.get(&host_id)?;
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
        .create(&host_id, &name, &working_dir, &config_files, &fingerprint)
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

/// Sets the Compose profiles a Managed or Linked Stack deploys; they apply
/// from its next compose command.
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
    runtime::act(&target(&state, &host_id)?, &name, action).await
}

/// Removes a Stack's containers and networks through the Engine API.
#[tauri::command]
#[specta::specta]
pub async fn stacks_remove(
    state: State<'_, AppState>,
    host_id: String,
    name: String,
) -> AppResult<()> {
    runtime::remove(&target(&state, &host_id)?, &name).await
}

/// Runs a compose command for the Managed or Linked Stack `id`, streaming
/// its output.
#[tauri::command]
#[specta::specta]
pub async fn stacks_compose_subscribe(
    state: State<'_, AppState>,
    kind: SourceKind,
    id: String,
    action: ComposeAction,
    on_event: Channel<ComposeEvent>,
) -> AppResult<StreamId> {
    let job = job(&state, kind, &id, action).await?;
    Ok(state
        .streams
        .spawn(runtime::run(job, move |e| on_event.send(e).is_ok())))
}

/// The compose job for `action` on the Managed or Linked Stack `id`, its
/// deploys recorded in its history.
async fn job(
    state: &AppState,
    kind: SourceKind,
    id: &str,
    action: ComposeAction,
) -> AppResult<ComposeJob> {
    let stack = resolve(state, kind, id)?;
    Ok(ComposeJob {
        cli: compose::detect().await?,
        source: stack.source,
        target: target(state, &stack.host_id)?,
        logins: registries::logins(state)?,
        project: stack.name,
        action,
        profiles: stack.profiles,
        record: Some(Recorder {
            history: state.history.clone(),
            host_id: stack.host_id,
            stack_id: id.to_owned(),
        }),
    })
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
    let job = job(&state, SourceKind::Managed, &stack.id, ComposeAction::Up).await?;
    Ok(state
        .streams
        .spawn(runtime::run(job, move |e| on_event.send(e).is_ok())))
}

/// A Managed or Linked Stack, ready to run.
struct Resolved {
    host_id: String,
    name: String,
    source: StackSource,
    profiles: Vec<String>,
}

/// The Host, name, source and profiles of the Managed or Linked Stack `id`.
fn resolve(state: &AppState, kind: SourceKind, id: &str) -> AppResult<Resolved> {
    Ok(match kind {
        SourceKind::Managed => {
            let stack = state.stacks.get(id)?;
            Resolved {
                source: StackSource::Managed {
                    env: state.stacks.env(id)?,
                    files: stack.files,
                },
                host_id: stack.host_id,
                name: stack.name,
                profiles: stack.profiles,
            }
        }
        SourceKind::Linked => {
            let stack = state.linked.get(id)?;
            Resolved {
                source: StackSource::Linked {
                    working_dir: stack.working_dir.into(),
                    config_files: stack.config_files,
                },
                host_id: stack.host_id,
                name: stack.name,
                profiles: stack.profiles,
            }
        }
    })
}

/// Where the Stacks of `host_id` run: its Compose projects.
fn target(state: &AppState, host_id: &str) -> AppResult<StackTarget> {
    let host = state.hosts.store.get(host_id)?;
    Ok(StackTarget::Host {
        name: host.name,
        connection: host.connection,
        dialer: state.hosts.registry.dialer(host_id)?,
        docker: state.hosts.registry.docker(host_id)?,
    })
}
