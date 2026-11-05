//! Stacks on a Host, and deploying Managed Stacks.

use tauri::{ipc::Channel, State};

use homeport_core::streams::StreamId;
use homeport_engine::containers;
use homeport_stacks::{
    self as stacks,
    compose::{self, ComposeAction, ComposeEvent},
    definition::{self, ComposeCheck},
    lifecycle::StackAction,
    linked::{self, files::LinkedFiles},
    runtime::{self, ComposeJob, SourceKind, StackSource, StackTarget},
    store::ManagedStack,
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
    compose: String,
    env: String,
) -> AppResult<ManagedStack> {
    state.hosts.store.get(&host_id)?;
    state.stacks.create(&host_id, &name, &compose, &env)
}

#[tauri::command]
#[specta::specta]
pub fn stacks_update(
    state: State<'_, AppState>,
    id: String,
    compose: String,
    env: String,
) -> AppResult<ManagedStack> {
    state.stacks.update(&id, &compose, &env)
}

/// Forgets a Managed Stack's definition; its containers keep running and it
/// becomes an External Stack.
#[tauri::command]
#[specta::specta]
pub fn stacks_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.stacks.delete(&id)
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

/// Forgets a Linked Stack: its record goes, its folder and containers stay.
#[tauri::command]
#[specta::specta]
pub fn stacks_linked_forget(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.linked.delete(&id)
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
    let (host_id, project, source) = source(&state, kind, &id)?;
    let job = ComposeJob {
        cli: compose::detect().await?,
        source,
        target: target(&state, &host_id)?,
        logins: registries::logins(&state)?,
        project,
        action,
    };
    Ok(state
        .streams
        .spawn(runtime::run(job, move |e| on_event.send(e).is_ok())))
}

/// The Host, name and source of the Managed or Linked Stack `id`.
fn source(
    state: &AppState,
    kind: SourceKind,
    id: &str,
) -> AppResult<(String, String, StackSource)> {
    Ok(match kind {
        SourceKind::Managed => {
            let stack = state.stacks.get(id)?;
            let source = StackSource::Managed {
                env: state.stacks.env(id)?,
                compose: stack.compose,
            };
            (stack.host_id, stack.name, source)
        }
        SourceKind::Linked => {
            let stack = state.linked.get(id)?;
            let source = StackSource::Linked {
                working_dir: stack.working_dir.into(),
                config_files: stack.config_files,
            };
            (stack.host_id, stack.name, source)
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
