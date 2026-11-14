//! Files inside a container: browsing, downloading and uploading.

use std::path::{Path, PathBuf};

use tauri::State;

use homeport_engine::containers::files::{self, FileEntry};

use crate::{app::AppState, error::AppResult};

#[tauri::command]
#[specta::specta]
pub async fn files_list(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    path: String,
) -> AppResult<Vec<FileEntry>> {
    files::list(
        &state.hosts.registry.docker(&host_id)?,
        &container_id,
        &path,
    )
    .await
}

/// Saves a container path to `destination`; returns `true` when it was a
/// directory saved as a `.tar` archive. The file is written beside
/// `destination` and put in its place only once whole (see
/// `homeport_engine::containers::files::download_counted`): the folder
/// must be writable, a link at `destination` is replaced rather than
/// written through, and an empty archive is an error.
#[tauri::command]
#[specta::specta]
pub async fn files_download(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    path: String,
    destination: String,
) -> AppResult<bool> {
    let docker = state.hosts.registry.docker(&host_id)?;
    files::download(&docker, &container_id, &path, Path::new(&destination)).await
}

#[tauri::command]
#[specta::specta]
pub async fn files_upload(
    state: State<'_, AppState>,
    host_id: String,
    container_id: String,
    directory: String,
    sources: Vec<String>,
) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let sources: Vec<PathBuf> = sources.into_iter().map(PathBuf::from).collect();
    files::upload(&docker, &container_id, &directory, &sources).await
}
