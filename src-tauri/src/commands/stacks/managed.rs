//! Managed Stacks: their definition and environment, kept by Homeport.

use tauri::State;

use homeport_stacks::{
    store::{ComposeFile, ManagedStack},
    StackOwner,
};

use super::target;
use crate::{app::AppState, error::AppResult};

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
