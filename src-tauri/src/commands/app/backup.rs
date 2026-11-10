//! Backing up everything Homeport holds to one passphrase-protected file,
//! and restoring it on this or another computer.

use std::path::Path;

use tauri::State;

use homeport_backup::{BackupContents, BackupOutcome, Data};

use crate::{app::AppState, error::AppResult};

/// What a backup reads and a restore replaces.
fn data(state: &AppState) -> Data<'_> {
    Data {
        db: &state.db,
        data_dir: &state.data_dir,
        secrets: &state.secrets,
        settings: &state.settings,
        hosts: &state.hosts,
        clusters: &state.clusters,
        stacks: &state.stacks,
        linked: &state.linked,
        synced: &state.synced,
        registries: &state.registries,
        repositories: &state.repositories,
    }
}

/// Backs up everything Homeport holds to `path`, sealed with `passphrase`.
#[tauri::command]
#[specta::specta]
pub async fn data_backup(
    state: State<'_, AppState>,
    path: String,
    passphrase: String,
) -> AppResult<BackupOutcome> {
    homeport_backup::write(&data(&state), Path::new(&path), &passphrase)
}

/// What the backup at `path` holds, for the restore's confirmation.
#[tauri::command]
#[specta::specta]
pub async fn data_restore_preview(path: String, passphrase: String) -> AppResult<BackupContents> {
    homeport_backup::preview(Path::new(&path), &passphrase)
}

/// Replaces everything Homeport holds with the backup at `path`, after
/// copying the database to `homeport.db.before-restore-<time>` next to it,
/// then reconnects to the restored Hosts and Clusters.
#[tauri::command]
#[specta::specta]
pub async fn data_restore(
    state: State<'_, AppState>,
    path: String,
    passphrase: String,
) -> AppResult<BackupContents> {
    homeport_backup::restore(&data(&state), Path::new(&path), &passphrase).await
}
