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
        nomad_terminals: Default::default(),
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
    let ClusterSecrets::Kubernetes(secrets) = desktop.clusters.store.secrets(&cluster.id).unwrap()
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
async fn the_preview_names_what_comes_back_without_its_secrets() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("homeport.backup");
    fill(&computer(dir.path(), 1));
    // Backed up while the keychain had another key: nothing opens.
    let locked = computer(dir.path(), 2);
    let outcome = backup_to(&locked, &file, PASS).unwrap();
    assert!(!outcome.without_secrets.is_empty());
    let preview = Backup::read(&file, PASS).unwrap().contents();
    for name in ["prod", "shop", "infra", "ghcr.io", "kind-dev"] {
        assert!(
            preview.without_secrets.iter().any(|n| n == name),
            "{name} in {:?}",
            preview.without_secrets
        );
    }

    // A backup from before the list was kept still names its Hosts.
    let older = r#"{"version":1,"createdAt":"2026-01-01T00:00:00Z","appVersion":"0.12.0",
            "backup":true,
            "hosts":[{"id":"h1","name":"tls","connection":{"kind":"tls","address":"h:2376"},
                      "enabled":true,"hostKey":null,"secrets":null},
                     {"id":"h2","name":"lab","connection":{"kind":"tcp","address":"lab:2375"},
                      "enabled":true,"hostKey":null,"secrets":null}]}"#;
    std::fs::write(&file, bundle::seal(PASS, older.as_bytes()).unwrap()).unwrap();
    let preview = Backup::read(&file, PASS).unwrap().contents();
    assert_eq!(preview.without_secrets, ["tls"]);
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
