//! Backing up everything Homeport holds to one passphrase-protected file,
//! and restoring it on this or another computer.

use std::path::Path;

use homeport_backup::{BackupContents, BackupOutcome, Data};
use homeport_core::error::AppResult;

use crate::app::AppState;

/// What a backup reads and a restore replaces.
fn data(state: &AppState) -> Data<'_> {
    Data {
        db: &state.db,
        data_dir: &state.data_dir,
        secrets: &state.secrets,
        settings: &state.settings,
        hosts: &state.hosts,
        clusters: &state.clusters,
        stacks: &state.stacks.managed,
        linked: &state.stacks.linked,
        synced: &state.stacks.synced,
        registries: &state.registries,
        repositories: &state.repositories,
    }
}

/// Backs up everything Homeport holds to `path`, sealed with `passphrase`.
pub async fn data_backup(
    state: &AppState,
    path: String,
    passphrase: String,
) -> AppResult<BackupOutcome> {
    homeport_backup::write(&data(state), Path::new(&path), &passphrase)
}

/// What the backup at `path` holds, for the restore's confirmation.
pub async fn data_restore_preview(path: String, passphrase: String) -> AppResult<BackupContents> {
    homeport_backup::preview(Path::new(&path), &passphrase)
}

/// Replaces everything Homeport holds with the backup at `path`, after
/// copying the database to `homeport.db.before-restore-<time>` next to it,
/// then reconnects to the restored Hosts and Clusters.
pub async fn data_restore(
    state: &AppState,
    path: String,
    passphrase: String,
) -> AppResult<BackupContents> {
    homeport_backup::restore(&data(state), Path::new(&path), &passphrase).await
}

#[cfg(test)]
mod tests {
    use homeport_core::error::ErrorCode;
    use homeport_settings::Settings;

    use crate::app::testing;

    const PASSPHRASE: &str = "correct horse battery staple";

    #[tokio::test]
    async fn a_backup_is_previewed_then_restored() {
        let app = testing::app();
        let path = app.dir.path().join("homeport.backup");
        let path = path.to_string_lossy().into_owned();
        let weekly = Settings {
            keep_events_days: 7,
            ..app.state.settings.get().unwrap()
        };
        app.state.settings.update(weekly).unwrap();

        let outcome = super::data_backup(&app.state, path.clone(), PASSPHRASE.to_owned())
            .await
            .unwrap();
        assert!(outcome.contents.settings);
        assert!(outcome.contents.hosts.is_empty());

        let preview = super::data_restore_preview(path.clone(), PASSPHRASE.to_owned())
            .await
            .unwrap();
        assert_eq!(preview.created_at, outcome.contents.created_at);

        // What changed since the backup is replaced by what it holds.
        let monthly = Settings {
            keep_events_days: 30,
            ..app.state.settings.get().unwrap()
        };
        app.state.settings.update(monthly).unwrap();
        let restored = super::data_restore(&app.state, path, PASSPHRASE.to_owned())
            .await
            .unwrap();
        assert!(restored.settings);
        assert_eq!(app.state.settings.get().unwrap().keep_events_days, 7);
    }

    #[tokio::test]
    async fn the_wrong_passphrase_opens_nothing() {
        let app = testing::app();
        let path = app.dir.path().join("homeport.backup");
        let path = path.to_string_lossy().into_owned();
        super::data_backup(&app.state, path.clone(), PASSPHRASE.to_owned())
            .await
            .unwrap();

        let wrong = "not the passphrase".to_owned();
        let preview = super::data_restore_preview(path.clone(), wrong.clone()).await;
        assert_eq!(preview.unwrap_err().code, ErrorCode::BundleUnreadable);
        let restore = super::data_restore(&app.state, path, wrong).await;
        assert_eq!(restore.unwrap_err().code, ErrorCode::BundleUnreadable);
    }
}
