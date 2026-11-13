//! Files inside a container: browsing, downloading and uploading.

use std::path::{Path, PathBuf};

use homeport_core::error::AppResult;
use homeport_engine::containers::files::{self, FileEntry};

use crate::app::AppState;

pub async fn files_list(
    state: &AppState,
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
/// directory saved as a `.tar` archive.
pub async fn files_download(
    state: &AppState,
    host_id: String,
    container_id: String,
    path: String,
    destination: String,
) -> AppResult<bool> {
    let docker = state.hosts.registry.docker(&host_id)?;
    files::download(&docker, &container_id, &path, Path::new(&destination)).await
}

/// [`files_download`], telling `progress` how many bytes came, of how
/// many when the Engine says (a file's size).
pub async fn files_download_counted(
    state: &AppState,
    host_id: String,
    container_id: String,
    path: String,
    destination: String,
    progress: impl Fn(u64, Option<u64>),
) -> AppResult<bool> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let destination = Path::new(&destination);
    files::download_counted(&docker, &container_id, &path, destination, progress).await
}

pub async fn files_upload(
    state: &AppState,
    host_id: String,
    container_id: String,
    directory: String,
    sources: Vec<String>,
) -> AppResult<()> {
    let docker = state.hosts.registry.docker(&host_id)?;
    let sources: Vec<PathBuf> = sources.into_iter().map(PathBuf::from).collect();
    files::upload(&docker, &container_id, &directory, &sources).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;

    use super::*;
    use crate::api::engine::testing::{assert_all, code, host};
    use crate::app::testing::app;

    #[tokio::test]
    async fn a_host_that_isnt_connected_is_refused() {
        let app = app();
        let s = &app.state;
        let c = || "c1".to_owned();
        let saved = app.dir.path().join("saved").display().to_string();
        let sources = vec![app.dir.path().join("source").display().to_string()];
        let results = [
            ("list", code(files_list(s, host(), c(), "/".into()).await)),
            (
                "download",
                code(files_download(s, host(), c(), "/etc".into(), saved.clone()).await),
            ),
            (
                "download_counted",
                code(
                    files_download_counted(s, host(), c(), "/etc".into(), saved.clone(), |_, _| {})
                        .await,
                ),
            ),
            (
                "upload",
                code(files_upload(s, host(), c(), "/tmp".into(), sources).await),
            ),
        ];
        assert_all(ErrorCode::HostNotConnected, &results);
        // Refused before anything is written on this machine.
        assert!(!std::path::Path::new(&saved).exists());
    }
}
