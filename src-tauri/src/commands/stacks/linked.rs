//! Linked Stacks: compose files read in place from a folder on this computer
//! (ADR 0008).

use tauri::State;

use homeport_stacks::{
    linked::{
        files::LinkedFiles,
        folder::{self, FolderProject},
        LinkedStack,
    },
    StackOwner,
};

use super::target;
use crate::{app::AppState, error::AppResult};

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
