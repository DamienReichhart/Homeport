//! Backing up everything Homeport holds — Hosts, Clusters, Stacks, registry
//! logins, repositories, settings and their secrets — to one passphrase-protected
//! file, and restoring it on this or another computer (ADR 0013's format).
//!
//! Domain crates don't depend on each other (ADR 0005), so only the shell
//! combines their sections. Each section is domain JSON, not table rows, so
//! a backup from an older Homeport restores into a newer one; one from a
//! newer Homeport is refused. Engine event and deploy histories aren't
//! kept.

use std::path::Path;

use serde::{Deserialize, Serialize};
use tauri::State;

use homeport_clusters::store::backup::{self as clusters, BackedUpCluster};
use homeport_core::error::{local_file, AppError, AppResult};
use homeport_git::store::backup::{self as repositories, BackedUpRepository};
use homeport_hosts::bundle::{self as hosts, ExportedHost, PAYLOAD_VERSION};
use homeport_registries::store::backup::{self as registries, BackedUpLogin};
use homeport_settings::{Settings, SettingsService};
use homeport_stacks::backup::{self as stacks, StacksBackup, Stores};
use homeport_storage::{crypto::bundle, db::sqlite_error};

use crate::{app::AppState, error::AppResult as ShellResult, messages as msg};

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
        }
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
fn collect(state: &AppState) -> AppResult<(Backup, Vec<String>)> {
    let mut without_secrets = Vec::new();
    let hosts = state.hosts.backup_hosts(&mut without_secrets)?;
    let host_ids: Vec<String> = hosts.iter().filter_map(|h| h.id.clone()).collect();
    let stacks = stacks::backup(
        &Stores {
            managed: &state.stacks,
            linked: &state.linked,
            synced: &state.synced.store,
        },
        &host_ids,
        &mut without_secrets,
    )?;
    let clusters = state.clusters.store.backup(&mut without_secrets)?;
    let registries = state.registries.backup(&mut without_secrets)?;
    let repositories = state.repositories.store.backup(&mut without_secrets)?;
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
        settings: Some(state.settings.get()?),
    };
    Ok((backup, without_secrets))
}

/// Replaces everything Homeport holds with `backup`'s, in one transaction:
/// on any failure, nothing changes. Secrets are sealed with this
/// computer's key.
fn replace(state: &AppState, backup: &Backup) -> AppResult<()> {
    let secrets = &*state.secrets;
    state.db.transaction(|tx| {
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

fn backup_to(state: &AppState, path: &Path, passphrase: &str) -> AppResult<BackupOutcome> {
    let (backup, without_secrets) = collect(state)?;
    let sealed = bundle::seal(passphrase, &serde_json::to_vec(&backup)?)?;
    std::fs::write(path, sealed).map_err(|e| local_file(path, e))?;
    Ok(BackupOutcome {
        contents: backup.contents(),
        without_secrets,
    })
}

async fn restore_from(
    state: &AppState,
    path: &Path,
    passphrase: &str,
) -> AppResult<BackupContents> {
    let backup = Backup::read(path, passphrase)?;
    let stamp = hosts::now_rfc3339().replace(':', "-");
    let copy = state
        .data_dir
        .join(format!("homeport.db.before-restore-{stamp}"));
    state.db.copy_to(&copy)?;
    // Whatever was still being written belongs to the data being replaced.
    state.hosts.events.flush().await;
    replace(state, &backup)?;
    state.hosts.registry.reload()?;
    state.clusters.forwards.stop_all();
    state.clusters.registry.reload()?;
    Ok(backup.contents())
}

/// Backs up everything Homeport holds to `path`, sealed with `passphrase`.
#[tauri::command]
#[specta::specta]
pub async fn data_backup(
    state: State<'_, AppState>,
    path: String,
    passphrase: String,
) -> ShellResult<BackupOutcome> {
    backup_to(&state, Path::new(&path), &passphrase)
}

/// What the backup at `path` holds, for the restore's confirmation.
#[tauri::command]
#[specta::specta]
pub async fn data_restore_preview(path: String, passphrase: String) -> ShellResult<BackupContents> {
    Ok(Backup::read(Path::new(&path), &passphrase)?.contents())
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
) -> ShellResult<BackupContents> {
    restore_from(&state, Path::new(&path), &passphrase).await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use homeport_clusters::{
        registry::{ClusterNotifier, ClusterStatus},
        service::ClusterService,
        store::{ClusterDraft, ClusterStore},
        ClusterConnection, ClusterSecrets, KubernetesConnection, KubernetesSecrets,
    };
    use homeport_core::{
        error::ErrorCode,
        host::{HostConnection, HostSecrets},
    };
    use homeport_git::{
        ops::RefKind,
        store::{RepoAuth, RepositoryDraft, RepositoryStore},
        Repositories,
    };
    use homeport_hosts::{
        events::EventStore,
        registry::{EngineEvent, HostRegistry, HostStatus, Notifier},
        service::HostService,
        store::{HostDraft, HostStore},
    };
    use homeport_registries::{RegistryDraft, RegistryStore};
    use homeport_settings::Theme;
    use homeport_stacks::{
        history::DeployHistory,
        linked::{FileCache, LinkedStore},
        store::{ComposeFile, StackStore},
        synced::{
            store::{SyncedDraft, SyncedStore},
            SyncedStacks,
        },
    };
    use homeport_storage::{Db, SecretBox};

    use super::*;

    const PASS: &str = "a long enough passphrase";

    struct Silent;
    impl Notifier for Silent {
        fn host_status(&self, _: &str, _: &HostStatus) {}
        fn engine_event(&self, _: &str, _: &EngineEvent) {}
    }
    impl ClusterNotifier for Silent {
        fn cluster_status(&self, _: &str, _: &ClusterStatus) {}
    }

    /// A computer: its own data folder, database and keychain key.
    fn computer(dir: &Path, key: u8) -> AppState {
        let db = Arc::new(Db::open(&dir.join("homeport.db")).unwrap());
        let secrets = Arc::new(SecretBox::from_key(&[key; 32]));
        let store = Arc::new(HostStore::new(db.clone(), secrets.clone()));
        let events = EventStore::new(db.clone());
        let registry = HostRegistry::new(store.clone(), events.clone(), Arc::new(Silent));
        let repositories = Arc::new(Repositories::new(
            RepositoryStore::new(db.clone(), secrets.clone()),
            dir.join("repositories"),
        ));
        let cluster_store = Arc::new(ClusterStore::new(db.clone(), secrets.clone()));
        AppState {
            clusters: ClusterService::new(cluster_store, Arc::new(Silent)),
            settings: SettingsService::new(db.clone(), &dir.join("settings.json")).unwrap(),
            hosts: HostService {
                store,
                registry,
                events,
            },
            stacks: StackStore::new(db.clone(), secrets.clone()),
            linked: LinkedStore::new(db.clone()),
            linked_files: FileCache::new(),
            history: Arc::new(DeployHistory::new(db.clone(), secrets.clone())),
            registries: RegistryStore::new(db.clone(), secrets.clone()),
            synced: Arc::new(SyncedStacks::new(
                SyncedStore::new(db.clone(), secrets.clone()),
                repositories.clone(),
                dir.join("synced-stacks"),
            )),
            repositories,
            secrets,
            streams: Default::default(),
            exec: Default::default(),
            containerd_exec: Default::default(),
            incus_terminals: Default::default(),
            pod_terminals: Default::default(),
            volume_browser: Default::default(),
            freshness: Default::default(),
            update: Default::default(),
            db,
            data_dir: dir.to_owned(),
        }
    }

    /// Fills `state` with one of everything; returns the TLS Host's id.
    fn fill(state: &AppState) -> String {
        let tls = state
            .hosts
            .store
            .create(HostDraft {
                name: "prod".into(),
                connection: HostConnection::Tls {
                    address: "prod:2376".into(),
                },
                engine: homeport_hosts::EngineKind::Docker,
                secrets: Some(HostSecrets::Tls {
                    ca_pem: "CA".into(),
                    cert_pem: "CERT".into(),
                    key_pem: "KEY".into(),
                }),
            })
            .unwrap()
            .id;
        let compose = [ComposeFile::new(
            "compose.yaml",
            "services:\n  web:\n    image: nginx\n",
        )];
        state
            .stacks
            .create(&tls, "shop", &compose, "DB_PASSWORD=hunter2")
            .unwrap();
        state
            .linked
            .create(
                &tls,
                "blog",
                "/srv/blog",
                &["/srv/blog/compose.yaml".into()],
                "f1",
            )
            .unwrap();
        let repo = state
            .repositories
            .store
            .create(&RepositoryDraft {
                url: "https://git.example.com/acme/infra.git".into(),
                auth: RepoAuth::HttpsToken,
                username: Some("deploy".into()),
                token: Some("glpat-secret".into()),
            })
            .unwrap();
        state
            .synced
            .store
            .create(
                &tls,
                &SyncedDraft {
                    name: "infra".into(),
                    repository_id: repo.id,
                    ref_kind: RefKind::Branch,
                    ref_name: "main".into(),
                    compose_path: "deploy/compose.yaml".into(),
                },
                "TOKEN=abc",
                "0123456789abcdef0123456789abcdef01234567",
            )
            .unwrap();
        state
            .registries
            .save(
                None,
                RegistryDraft {
                    address: "ghcr.io".into(),
                    username: "acme".into(),
                    secret: Some("ghp_secret".into()),
                },
            )
            .unwrap();
        state
            .clusters
            .store
            .create(ClusterDraft {
                name: "kind-dev".into(),
                connection: ClusterConnection::Kubernetes(KubernetesConnection {
                    server: "https://127.0.0.1:6443".into(),
                    ..Default::default()
                }),
                secrets: ClusterSecrets::Kubernetes(KubernetesSecrets {
                    client_key: Some("CLUSTER KEY".into()),
                    ..Default::default()
                }),
            })
            .unwrap();
        state
            .settings
            .update(Settings {
                theme: Theme::Dark,
                keep_events_days: 7,
                ..Default::default()
            })
            .unwrap();
        tls
    }

    #[tokio::test]
    async fn a_backup_restores_everything_on_another_computer() {
        let (a, b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let file = a.path().join("homeport.backup");
        let laptop = computer(a.path(), 1);
        let tls = fill(&laptop);
        let outcome = backup_to(&laptop, &file, PASS).unwrap();
        assert_eq!(outcome.contents.hosts, ["prod"]);
        assert_eq!(outcome.contents.stacks, 3);
        assert!(outcome.without_secrets.is_empty());

        // The new computer has a key of its own, and data of its own.
        let desktop = computer(b.path(), 2);
        let old = desktop
            .hosts
            .store
            .create(HostDraft {
                name: "old".into(),
                connection: HostConnection::Tcp {
                    address: "old:2375".into(),
                },
                engine: homeport_hosts::EngineKind::Docker,
                secrets: None,
            })
            .unwrap();
        let restored = restore_from(&desktop, &file, PASS).await.unwrap();
        assert_eq!(restored.registries, 1);
        assert_eq!(restored.clusters, ["kind-dev"]);
        let cluster = &desktop.clusters.store.list().unwrap()[0];
        let ClusterSecrets::Kubernetes(secrets) =
            desktop.clusters.store.secrets(&cluster.id).unwrap()
        else {
            panic!("not Kubernetes credentials");
        };
        assert_eq!(secrets.client_key.as_deref(), Some("CLUSTER KEY"));

        let hosts = desktop.hosts.store.list().unwrap();
        assert_eq!(hosts.iter().map(|h| &h.name).collect::<Vec<_>>(), ["prod"]);
        assert!(desktop.hosts.store.get(&old.id).is_err());
        assert_eq!(hosts[0].id, tls, "Stacks keep their Host");
        assert!(matches!(
            desktop.hosts.store.secrets(&tls).unwrap(),
            Some(HostSecrets::Tls { key_pem, .. }) if key_pem == "KEY"
        ));
        let shop = &desktop.stacks.list(&tls).unwrap()[0];
        assert_eq!(desktop.stacks.env(&shop.id).unwrap(), "DB_PASSWORD=hunter2");
        assert_eq!(shop.files[0].name, "compose.yaml");
        assert_eq!(
            desktop.linked.list(&tls).unwrap()[0].working_dir,
            "/srv/blog"
        );
        let infra = &desktop.synced.store.list(&tls).unwrap()[0];
        assert_eq!(desktop.synced.store.env(&infra.id).unwrap(), "TOKEN=abc");
        let access = desktop
            .repositories
            .store
            .access(&infra.repository_id)
            .unwrap();
        assert_eq!(access.repository.username.as_deref(), Some("deploy"));
        let login = desktop.registries.credentials("ghcr.io").unwrap().unwrap();
        assert_eq!(login.secret, "ghp_secret");
        let settings = desktop.settings.get().unwrap();
        assert_eq!(
            (settings.theme, settings.keep_events_days),
            (Theme::Dark, 7)
        );

        // The data it replaced is kept next to the database.
        let copies: Vec<_> = std::fs::read_dir(b.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("homeport.db.before-restore-")
            })
            .collect();
        assert_eq!(copies.len(), 1);
    }

    #[tokio::test]
    async fn a_failed_restore_changes_nothing() {
        let (a, b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let file = a.path().join("homeport.backup");
        let laptop = computer(a.path(), 1);
        fill(&laptop);
        backup_to(&laptop, &file, PASS).unwrap();

        let desktop = computer(b.path(), 2);
        let mine = fill(&desktop);
        let wrong = restore_from(&desktop, &file, "not the passphrase at all").await;
        assert_eq!(wrong.unwrap_err().code, ErrorCode::BundleUnreadable);
        assert_eq!(desktop.hosts.store.list().unwrap()[0].id, mine);

        // A backup that breaks halfway (a Stack of a Host it doesn't hold)
        // rolls back whatever it had written.
        let mut broken = collect(&laptop).unwrap().0;
        broken.stacks.linked[0].host_id = "nowhere".into();
        assert!(replace(&desktop, &broken).is_err());
        assert_eq!(desktop.hosts.store.list().unwrap()[0].id, mine);
        assert_eq!(desktop.stacks.list(&mine).unwrap().len(), 1);
    }

    #[tokio::test]
    async fn backups_from_an_older_homeport_restore_and_newer_ones_are_refused() {
        let dir = tempfile::tempdir().unwrap();
        let state = computer(dir.path(), 1);
        let file = dir.path().join("old.backup");
        // Before registries, repositories and Synced Stacks' polling existed.
        let older = r#"{"version":1,"createdAt":"2026-01-01T00:00:00Z","appVersion":"0.12.0",
            "backup":true,
            "hosts":[{"id":"h1","name":"lab","connection":{"kind":"tcp","address":"lab:2375"},
                      "enabled":true,"hostKey":null,"secrets":null}],
            "stacks":{"managed":[{"id":"s1","hostId":"h1","name":"web",
                      "files":[{"name":"compose.yaml","content":"services: {}"}]}]}}"#;
        std::fs::write(&file, bundle::seal(PASS, older.as_bytes()).unwrap()).unwrap();
        let contents = restore_from(&state, &file, PASS).await.unwrap();
        assert_eq!((contents.hosts.len(), contents.stacks), (1, 1));
        assert_eq!(state.stacks.list("h1").unwrap()[0].name, "web");
        assert_eq!(state.settings.get().unwrap().keep_events_days, 30);

        let newer = r#"{"version":2,"backup":true}"#;
        std::fs::write(&file, bundle::seal(PASS, newer.as_bytes()).unwrap()).unwrap();
        let refused = restore_from(&state, &file, PASS).await.unwrap_err();
        assert_eq!(refused.code, ErrorCode::BundleTooNew);
        assert_eq!(state.hosts.store.list().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn an_export_of_hosts_is_not_a_backup() {
        let dir = tempfile::tempdir().unwrap();
        let state = computer(dir.path(), 1);
        let id = fill(&state);
        let file = dir.path().join("hosts.homeport");
        state.hosts.export_hosts(&[id], &file, PASS).unwrap();
        let refused = restore_from(&state, &file, PASS).await.unwrap_err();
        assert_eq!(refused.code, ErrorCode::Validation);
        assert_eq!(
            state
                .stacks
                .list(&state.hosts.store.list().unwrap()[0].id)
                .unwrap()
                .len(),
            1
        );
    }
}
