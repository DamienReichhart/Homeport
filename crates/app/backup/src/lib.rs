//! Backing up everything Homeport holds — Hosts, Clusters, Stacks, registry
//! logins, repositories, settings and their secrets — to one passphrase-protected
//! file, and restoring it on this or another computer (ADR 0013's format).
//!
//! Every domain crate backs up and restores its own section; this crate,
//! which sits on all of them, puts the sections in one file. Each section is domain JSON, not table rows, so
//! a backup from an older Homeport restores into a newer one; one from a
//! newer Homeport is refused. Engine event and deploy histories aren't
//! kept.

use std::path::Path;

use serde::{Deserialize, Serialize};

use homeport_clusters::{
    service::ClusterService,
    store::backup::{self as clusters, BackedUpCluster},
};
use homeport_core::{
    error::{local_file, AppError, AppResult},
    local_files,
};
use homeport_git::{
    store::backup::{self as repositories, BackedUpRepository},
    Repositories,
};
use homeport_hosts::{
    bundle::{self as hosts, ExportedHost, PAYLOAD_VERSION},
    service::HostService,
};
use homeport_registries::{
    store::backup::{self as registries, BackedUpLogin},
    RegistryStore,
};
use homeport_settings::{Settings, SettingsService};
use homeport_stacks::{
    linked::LinkedStore,
    store::{
        backup::{self as stacks, StacksBackup, Stores},
        StackStore,
    },
    synced::SyncedStacks,
};
use homeport_storage::{crypto::bundle, db::sqlite_error, Db, SecretBox};

pub mod messages;

use messages as msg;

/// Everything Homeport holds: what a backup reads and a restore replaces.
pub struct Data<'a> {
    pub db: &'a Db,
    /// Where the database is, and where a restore leaves its copy of it.
    pub data_dir: &'a Path,
    pub secrets: &'a SecretBox,
    pub settings: &'a SettingsService,
    pub hosts: &'a HostService,
    pub clusters: &'a ClusterService,
    pub stacks: &'a StackStore,
    pub linked: &'a LinkedStore,
    pub synced: &'a SyncedStacks,
    pub registries: &'a RegistryStore,
    pub repositories: &'a Repositories,
}

/// Larger files aren't backups: they hold compose files and keys, not data.
const MAX_BACKUP_BYTES: u64 = 64 * 1024 * 1024;

/// The payload of a backup: the Hosts export's (ADR 0013), marked as a
/// backup and with every section. A Homeport that knows only Hosts exports
/// imports its Hosts and skips the rest.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Backup {
    version: u32,
    created_at: String,
    app_version: String,
    /// Tells a backup of everything from an export of some Hosts.
    #[serde(default)]
    backup: bool,
    #[serde(default)]
    hosts: Vec<ExportedHost>,
    #[serde(default)]
    clusters: Vec<BackedUpCluster>,
    #[serde(default)]
    registries: Vec<BackedUpLogin>,
    #[serde(default)]
    repositories: Vec<BackedUpRepository>,
    #[serde(default)]
    stacks: StacksBackup,
    #[serde(default)]
    settings: Option<Settings>,
    /// What was backed up without its secret, which couldn't be opened, by
    /// name: restoring it brings it back without one. Backups from before
    /// 1.0 don't have it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    without_secrets: Vec<String>,
}

/// What a backup holds, as the restore's confirmation lists it.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BackupContents {
    /// When it was written, in UTC (RFC 3339).
    pub created_at: String,
    /// The Homeport that wrote it.
    pub app_version: String,
    /// Its Hosts' names.
    pub hosts: Vec<String>,
    /// Its Clusters' names.
    pub clusters: Vec<String>,
    pub stacks: u32,
    pub registries: u32,
    pub repositories: u32,
    pub settings: bool,
    /// What it holds without its secret, by name: Hosts, Clusters, Stacks,
    /// registry logins (left out altogether) and repositories. Restoring
    /// replaces the secrets they have here with none.
    pub without_secrets: Vec<String>,
}

/// What a backup wrote.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BackupOutcome {
    pub contents: BackupContents,
    /// What was written without its secret, which couldn't be opened:
    /// Hosts, Clusters, Stacks, registries and repositories, by name.
    pub without_secrets: Vec<String>,
}

impl Backup {
    fn contents(&self) -> BackupContents {
        BackupContents {
            created_at: self.created_at.clone(),
            app_version: self.app_version.clone(),
            hosts: self.hosts.iter().map(|h| h.name.clone()).collect(),
            clusters: self.clusters.iter().map(|c| c.name.clone()).collect(),
            stacks: self.stacks.count() as u32,
            registries: self.registries.len() as u32,
            repositories: self.repositories.len() as u32,
            settings: self.settings.is_some(),
            without_secrets: self.without_secrets(),
        }
    }

    /// What the backup says it holds without secrets, and the Hosts that
    /// need secrets but have none, which backups from before the list say.
    fn without_secrets(&self) -> Vec<String> {
        let mut names = self.without_secrets.clone();
        for host in &self.hosts {
            if host.connection.needs_secrets()
                && host.secrets.is_none()
                && !names.contains(&host.name)
            {
                names.push(host.name.clone());
            }
        }
        names
    }

    fn read(path: &Path, passphrase: &str) -> AppResult<Self> {
        let size = std::fs::metadata(path)
            .map_err(|e| local_file(path, e))?
            .len();
        if size > MAX_BACKUP_BYTES {
            return Err(AppError::validation(
                homeport_storage::messages::BUNDLE_NOT_A_BUNDLE,
            ));
        }
        let bytes = std::fs::read(path).map_err(|e| local_file(path, e))?;
        let json = bundle::open(passphrase, &bytes)?;
        // The version first, so a newer payload is reported as such.
        #[derive(Deserialize)]
        struct Version {
            version: u32,
        }
        let Version { version } = serde_json::from_slice(&json)?;
        if version > PAYLOAD_VERSION {
            return Err(bundle::too_new());
        }
        let backup: Self = serde_json::from_slice(&json)?;
        if !backup.backup {
            return Err(
                AppError::validation(msg::BACKUP_IS_AN_EXPORT.with("path", path))
                    .with_hint(msg::BACKUP_IS_AN_EXPORT_HINT),
            );
        }
        Ok(backup)
    }
}

/// Everything Homeport holds, with its secrets opened.
fn collect(data: &Data<'_>) -> AppResult<(Backup, Vec<String>)> {
    let mut without_secrets = Vec::new();
    let hosts = data.hosts.backup_hosts(&mut without_secrets)?;
    let host_ids: Vec<String> = hosts.iter().filter_map(|h| h.id.clone()).collect();
    let stacks = stacks::backup(
        &Stores {
            managed: data.stacks,
            linked: data.linked,
            synced: &data.synced.store,
        },
        &host_ids,
        &mut without_secrets,
    )?;
    let clusters = data.clusters.store.backup(&mut without_secrets)?;
    let registries = data.registries.backup(&mut without_secrets)?;
    let repositories = data.repositories.store.backup(&mut without_secrets)?;
    let backup = Backup {
        version: PAYLOAD_VERSION,
        created_at: hosts::now_rfc3339(),
        app_version: env!("CARGO_PKG_VERSION").into(),
        backup: true,
        hosts,
        clusters,
        registries,
        repositories,
        stacks,
        settings: Some(data.settings.get()?),
        without_secrets: without_secrets.clone(),
    };
    Ok((backup, without_secrets))
}

/// Replaces everything Homeport holds with `backup`'s, in one transaction:
/// on any failure, nothing changes. Secrets are sealed with this
/// computer's key.
fn replace(data: &Data<'_>, backup: &Backup) -> AppResult<()> {
    let secrets = data.secrets;
    data.db.transaction(|tx| {
        // Stacks first: Synced Stacks keep their repository from being deleted.
        tx.execute_batch(
            "DELETE FROM synced_stacks;
             DELETE FROM linked_stacks;
             DELETE FROM stacks;
             DELETE FROM stack_deploys;
             DELETE FROM engine_events;
             DELETE FROM hosts;
             DELETE FROM clusters;
             DELETE FROM git_repositories;
             DELETE FROM registries;",
        )
        .map_err(sqlite_error)?;
        hosts::restore(tx, secrets, &backup.hosts)?;
        clusters::restore(tx, secrets, &backup.clusters)?;
        registries::restore(tx, secrets, &backup.registries)?;
        repositories::restore(tx, secrets, &backup.repositories)?;
        stacks::restore(tx, secrets, &backup.stacks)?;
        if let Some(settings) = &backup.settings {
            SettingsService::restore(tx, settings)?;
        }
        Ok(())
    })
}

/// Backs up everything Homeport holds to `path`, sealed with `passphrase`.
/// Written beside the file first: a failure keeps the backup that was there.
pub fn write(data: &Data<'_>, path: &Path, passphrase: &str) -> AppResult<BackupOutcome> {
    let (backup, without_secrets) = collect(data)?;
    let sealed = bundle::seal(passphrase, &serde_json::to_vec(&backup)?)?;
    local_files::write_replacing(path, &sealed)?;
    Ok(BackupOutcome {
        contents: backup.contents(),
        without_secrets,
    })
}

/// What the backup at `path` holds, for the restore's confirmation.
pub fn preview(path: &Path, passphrase: &str) -> AppResult<BackupContents> {
    Ok(Backup::read(path, passphrase)?.contents())
}

/// Replaces everything Homeport holds with the backup at `path`, after
/// copying the database to `homeport.db.before-restore-<time>` next to it,
/// then reconnects to the restored Hosts and Clusters.
pub async fn restore(data: &Data<'_>, path: &Path, passphrase: &str) -> AppResult<BackupContents> {
    let backup = Backup::read(path, passphrase)?;
    let stamp = hosts::now_rfc3339().replace(':', "-");
    let copy = data
        .data_dir
        .join(format!("homeport.db.before-restore-{stamp}"));
    data.db.copy_to(&copy)?;
    // Whatever was still being written belongs to the data being replaced.
    data.hosts.events.flush().await;
    replace(data, &backup)?;
    data.hosts.registry.reload()?;
    data.clusters.forwards.stop_all();
    data.clusters.registry.reload()?;
    Ok(backup.contents())
}

#[cfg(test)]
mod tests;
