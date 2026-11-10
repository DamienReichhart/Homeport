//! Databases from every released version open in today's Homeport with
//! everything they held: each fixture of `crates/storage/tests/fixtures/` is
//! migrated, then read back through the stores the app uses, down to every
//! secret. A migration that drops a column's data, or a stored shape today's
//! code no longer reads, fails here.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use homeport_clusters::{
    model::{ClusterConnection, ClusterKind, ClusterSecrets},
    store::ClusterStore,
    AuthMethod,
};
use homeport_core::{
    error::ErrorCode,
    host::{BastionAuth, EngineKind, HostConnection, HostSecrets, SshAuth},
};
use homeport_git::{
    ops::RefKind,
    store::{RepoAuth, RepositoryStore, Secret},
};
use homeport_hosts::{events::EventStore, store::HostStore};
use homeport_registries::RegistryStore;
use homeport_settings::{SettingsService, Theme};
use homeport_stacks::{
    compose::ComposeAction,
    linked::LinkedStore,
    model::owner::StackOwner,
    runtime::SourceKind,
    store::{history::DeployHistory, ComposeFile, StackStore},
    synced::SyncedStore,
};
use homeport_storage::{
    fixtures::{self as fx, since},
    Db, SecretBox,
};

fn fixtures() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../crates/storage/tests/fixtures");
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|e| e == "db"))
        .collect();
    files.sort();
    files
}

/// The schema version a database file is at, from its header.
fn schema_of(path: &Path) -> usize {
    let bytes = std::fs::read(path).unwrap();
    u32::from_be_bytes(bytes[60..64].try_into().unwrap()) as usize
}

/// One fixture, migrated by today's Homeport.
struct Upgraded {
    /// The release it came from, such as `0.9.0`.
    release: String,
    /// The schema it was at before migrating.
    schema: usize,
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
    _dir: tempfile::TempDir,
}

impl Upgraded {
    fn open(fixture: &Path) -> Self {
        let release = fixture.file_stem().unwrap().to_string_lossy().into_owned();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("homeport.db");
        std::fs::copy(fixture, &path).unwrap();
        let schema = schema_of(&path);
        let db = Db::open(&path).unwrap_or_else(|e| panic!("{release} doesn't open: {e:?}"));
        Self {
            release,
            schema,
            db: Arc::new(db),
            secrets: Arc::new(SecretBox::from_key(&fx::KEY)),
            _dir: dir,
        }
    }

    fn has(&self, since: usize) -> bool {
        self.schema >= since
    }
}

#[test]
fn every_release_has_been_checked() {
    assert!(fixtures().len() >= 21, "a fixture per release since 0.1.0");
}

#[test]
fn settings_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let settings = SettingsService::new(up.db.clone(), Path::new("/nonexistent/settings.json"))
            .unwrap()
            .get()
            .unwrap();
        assert_eq!(settings.theme, Theme::Dark, "{}", up.release);
        assert_eq!(
            settings.last_host_id.as_deref(),
            Some("h-local"),
            "{}",
            up.release
        );
        // What later versions added reads as its default.
        assert!(settings.check_for_updates, "{}", up.release);
    }
}

#[test]
fn hosts_and_their_secrets_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let release = &up.release;
        let store = HostStore::new(up.db.clone(), up.secrets.clone());
        let hosts = store.list().unwrap();
        let expected = if up.has(since::ENGINES) { 7 } else { 5 };
        assert_eq!(hosts.len(), expected, "{release}");
        let host = |id: &str| {
            hosts
                .iter()
                .find(|h| h.id == id)
                .unwrap_or_else(|| panic!("{release}: no host {id}"))
        };

        let local = host("h-local");
        assert_eq!(local.name, "laptop");
        assert_eq!(
            local.connection,
            HostConnection::Local {
                socket_path: "/var/run/docker.sock".into()
            }
        );
        assert_eq!(local.engine, EngineKind::Docker, "{release}");
        assert!(local.enabled);
        assert_eq!(local.created_at, fx::CREATED_AT);
        assert_eq!(store.secrets("h-local").unwrap(), None);

        assert_eq!(
            host("h-tcp").connection,
            HostConnection::Tcp {
                address: "192.168.1.20:2375".into()
            }
        );

        assert_eq!(
            host("h-tls").connection,
            HostConnection::Tls {
                address: "prod.example.com:2376".into()
            }
        );
        assert_eq!(
            store.secrets("h-tls").unwrap(),
            Some(HostSecrets::Tls {
                ca_pem: fx::TLS_CA.into(),
                cert_pem: fx::TLS_CERT.into(),
                key_pem: fx::TLS_KEY.into(),
            }),
            "{release}"
        );

        let ssh = host("h-ssh");
        assert_eq!(ssh.host_key.as_deref(), Some(fx::HOST_KEY), "{release}");
        let HostConnection::Ssh {
            host: address,
            port,
            user,
            socket_path,
            auth,
            public_key,
            bastions,
        } = &ssh.connection
        else {
            panic!("{release}: h-ssh is {:?}", ssh.connection);
        };
        assert_eq!(
            (address.as_str(), *port, user.as_str(), socket_path.as_str()),
            ("build.example.com", 2222, "deploy", "/var/run/docker.sock")
        );
        assert_eq!(*auth, SshAuth::Imported);
        assert_eq!(public_key.as_deref(), Some(fx::PUBLIC_KEY));
        let Some(HostSecrets::Ssh {
            private_key,
            passphrase,
            bastions: bastion_keys,
        }) = store.secrets("h-ssh").unwrap()
        else {
            panic!("{release}: h-ssh has no SSH secrets");
        };
        assert_eq!(
            private_key.as_deref(),
            Some(fx::SSH_PRIVATE_KEY),
            "{release}"
        );
        assert_eq!(passphrase.as_deref(), Some(fx::SSH_PASSPHRASE), "{release}");
        if up.has(since::BASTIONS) {
            assert_eq!(bastions.len(), 1, "{release}");
            let bastion = &bastions[0];
            assert_eq!(bastion.address(), "jump.example.com:22");
            assert_eq!(bastion.user, "jump");
            assert_eq!(bastion.auth, BastionAuth::Imported);
            assert_eq!(bastion.host_key.as_deref(), Some(fx::HOST_KEY));
            assert_eq!(
                bastion_keys["jump.example.com:22"].private_key,
                fx::BASTION_PRIVATE_KEY,
                "{release}"
            );
        } else {
            assert!(bastions.is_empty(), "{release}: direct before bastions");
            assert!(bastion_keys.is_empty(), "{release}");
        }

        let agent = host("h-agent");
        assert!(!agent.enabled, "{release}: a disabled Host stays disabled");
        assert!(matches!(
            agent.connection,
            HostConnection::Ssh {
                auth: SshAuth::Agent,
                ..
            }
        ));

        if up.has(since::ENGINES) {
            assert_eq!(host("h-containerd").engine, EngineKind::Containerd);
            assert_eq!(host("h-incus").engine, EngineKind::Incus);
        }
    }
}

#[test]
fn managed_stacks_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let release = &up.release;
        let store = StackStore::new(up.db.clone(), up.secrets.clone());

        let shop = store.get("s-shop").unwrap();
        assert_eq!(shop.owner, StackOwner::Host("h-local".into()), "{release}");
        assert_eq!(shop.name, "shop");
        let mut files = vec![ComposeFile::new("compose.yaml", fx::COMPOSE)];
        if up.has(since::STACK_FILES) {
            files.push(ComposeFile::new("compose.prod.yaml", fx::COMPOSE_PROD));
        }
        assert_eq!(shop.files, files, "{release}");
        assert!(shop.has_env);
        assert_eq!(store.env("s-shop").unwrap(), fx::STACK_ENV, "{release}");
        let profiles: &[&str] = if up.has(since::PROFILES) {
            &["debug"]
        } else {
            &[]
        };
        assert_eq!(shop.profiles, profiles, "{release}");
        assert_eq!(
            (shop.created_at.as_str(), shop.updated_at.as_str()),
            (fx::CREATED_AT, fx::UPDATED_AT),
            "{release}"
        );

        let plain = store.get("s-plain").unwrap();
        assert_eq!(plain.owner, StackOwner::Host("h-tcp".into()));
        assert!(!plain.has_env);
        assert_eq!(store.env("s-plain").unwrap(), "");
        assert_eq!(store.list("h-local").unwrap().len(), 1, "{release}");

        let on_swarms = store.list_on_swarms().unwrap();
        if up.has(since::SWARM_STACKS) {
            assert_eq!(on_swarms.len(), 1, "{release}");
            assert_eq!(on_swarms[0].owner, StackOwner::Swarm("swarm-abc".into()));
            assert!(on_swarms[0].registry_auth);
        } else {
            assert!(on_swarms.is_empty(), "{release}");
        }
    }
}

#[test]
fn registry_logins_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let store = RegistryStore::new(up.db.clone(), up.secrets.clone());
        let logins = store.list().unwrap();
        if !up.has(since::REGISTRIES) {
            assert!(logins.is_empty(), "{}", up.release);
            continue;
        }
        assert_eq!(logins.len(), 1, "{}", up.release);
        assert_eq!(
            (logins[0].address.as_str(), logins[0].username.as_str()),
            ("ghcr.io", "octo")
        );
        let credentials = store.credentials("ghcr.io").unwrap().expect("a login");
        assert_eq!(credentials.secret, fx::REGISTRY_SECRET, "{}", up.release);
    }
}

#[test]
fn linked_stacks_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let store = LinkedStore::new(up.db.clone());
        if !up.has(since::LINKED_STACKS) {
            assert!(store.list("h-local").unwrap().is_empty(), "{}", up.release);
            continue;
        }
        let blog = store.get("l-blog").unwrap();
        assert_eq!(
            blog.owner,
            StackOwner::Host("h-local".into()),
            "{}",
            up.release
        );
        assert_eq!(
            (blog.name.as_str(), blog.working_dir.as_str()),
            ("blog", "/srv/blog")
        );
        assert_eq!(
            blog.config_files,
            ["/srv/blog/compose.yaml", "/srv/blog/compose.override.yaml"]
        );
        assert_eq!(blog.fingerprint, "fp-1");
        let profiles: &[&str] = if up.has(since::PROFILES) {
            &["metrics"]
        } else {
            &[]
        };
        assert_eq!(blog.profiles, profiles, "{}", up.release);
    }
}

#[test]
fn deploy_history_survives() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let release = &up.release;
        let history = DeployHistory::new(up.db.clone(), up.secrets.clone());
        if !up.has(since::DEPLOYS) {
            assert!(history.list("s-shop").unwrap().is_empty(), "{release}");
            continue;
        }
        let deploy = history.get("d-shop").unwrap();
        assert_eq!(deploy.summary.kind, SourceKind::Managed, "{release}");
        assert_eq!(deploy.summary.action, ComposeAction::Up);
        assert_eq!(deploy.summary.started_at, fx::CREATED_AT);
        assert_eq!(deploy.summary.finished_at.as_deref(), Some(fx::UPDATED_AT));
        assert_eq!(
            (deploy.summary.success, deploy.summary.exit_code),
            (Some(true), Some(0))
        );
        assert_eq!(
            deploy.files,
            [ComposeFile::new("compose.yaml", fx::COMPOSE)]
        );
        assert_eq!(deploy.profiles, ["debug"]);
        assert_eq!(history.env("d-shop").unwrap(), fx::DEPLOY_ENV, "{release}");
        assert_eq!(
            history
                .latest_successful("s-shop")
                .unwrap()
                .map(|d| d.summary.id),
            Some("d-shop".into())
        );

        if up.has(since::SYNCED_STACKS) {
            let site = history.get("d-site").unwrap();
            assert_eq!(site.summary.kind, SourceKind::Synced, "{release}");
            assert_eq!(site.summary.action, ComposeAction::Update);
            assert_eq!(
                site.summary.finished_at, None,
                "{release}: still unfinished"
            );
            let commit = up.has(since::DEPLOY_COMMITS).then_some(fx::DEPLOYED_COMMIT);
            assert_eq!(site.summary.commit.as_deref(), commit, "{release}");
            assert_eq!(site.summary.automatic, up.has(since::POLLING), "{release}");
        }
    }
}

#[test]
fn repositories_and_synced_stacks_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let release = &up.release;
        let repositories = RepositoryStore::new(up.db.clone(), up.secrets.clone());
        let synced = SyncedStore::new(up.db.clone(), up.secrets.clone());
        if !up.has(since::REPOSITORIES) {
            assert!(repositories.list().unwrap().is_empty(), "{release}");
            assert!(synced.list_all().unwrap().is_empty(), "{release}");
            continue;
        }

        let ssh = repositories.access("g-ssh").unwrap();
        assert_eq!(
            ssh.repository.url, "git@github.com:acme/infra.git",
            "{release}"
        );
        assert_eq!(ssh.repository.auth, RepoAuth::SshKey);
        assert_eq!(ssh.repository.public_key.as_deref(), Some(fx::PUBLIC_KEY));
        assert!(ssh.repository.key_fingerprint.is_some(), "{release}");
        assert_eq!(ssh.host_key.as_deref(), Some(fx::REPOSITORY_HOST_KEY));
        assert_eq!(
            ssh.secret,
            Secret::SshKey {
                private_key: fx::REPOSITORY_PRIVATE_KEY.into()
            },
            "{release}"
        );
        let https = repositories.access("g-https").unwrap();
        assert_eq!(https.repository.auth, RepoAuth::HttpsToken);
        assert_eq!(
            https.secret,
            Secret::Token {
                username: "octo".into(),
                token: fx::REPOSITORY_TOKEN.into()
            },
            "{release}"
        );

        let site = synced.get("y-site").unwrap();
        assert_eq!(site.owner, StackOwner::Host("h-local".into()), "{release}");
        assert_eq!(site.repository_id, "g-ssh");
        assert_eq!(
            (
                site.ref_kind,
                site.ref_name.as_str(),
                site.compose_path.as_str()
            ),
            (RefKind::Branch, "main", "deploy/compose.yaml")
        );
        assert_eq!(synced.env("y-site").unwrap(), fx::SYNCED_ENV, "{release}");
        assert_eq!(site.profiles, ["cdn"]);
        assert_eq!(site.deployed_commit.as_deref(), Some(fx::DEPLOYED_COMMIT));
        assert_eq!(site.fetched_commit.as_deref(), Some(fx::FETCHED_COMMIT));
        if up.has(since::POLLING) {
            assert!(!site.poll, "{release}: polling stays off");
            assert!(site.auto_deploy, "{release}: auto-deploy stays on");
            assert!(site.update_available);
            assert_eq!(site.last_polled_at.as_deref(), Some(fx::UPDATED_AT));
            let error = site.last_poll_error.expect("the last poll's error");
            assert_eq!(error.code, ErrorCode::Timeout, "{release}");
            assert_eq!(
                site.auto_deploy_failed_commit.as_deref(),
                Some(fx::FETCHED_COMMIT)
            );
            assert!(site.auto_deploy_error.is_some(), "{release}");
        } else {
            // What 0.11 added: polled, never deployed without asking.
            assert!(site.poll && !site.auto_deploy, "{release}");
        }

        let docs = synced.get("y-docs").unwrap();
        assert_eq!(
            (docs.ref_kind, docs.ref_name.as_str()),
            (RefKind::Tag, "v2")
        );
        assert!(!docs.has_env);
        assert_eq!(synced.using("g-https").unwrap().len(), 1, "{release}");
    }
}

#[test]
fn engine_events_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let events = EventStore::new(up.db.clone())
            .page("h-local", 10, None)
            .unwrap();
        if !up.has(since::EVENTS) {
            assert!(events.is_empty(), "{}", up.release);
            continue;
        }
        assert_eq!(events.len(), 2, "{}", up.release);
        assert_eq!(
            (events[0].kind.as_str(), events[0].action.as_str()),
            ("container", "start")
        );
        assert_eq!(events[0].attributes["name"], "web");
        assert_eq!(events[1].actor_id, "nginx:1.27");
    }
}

#[test]
fn clusters_and_their_secrets_survive() {
    for fixture in fixtures() {
        let up = Upgraded::open(&fixture);
        let release = &up.release;
        let store = ClusterStore::new(up.db.clone(), up.secrets.clone());
        let clusters = store.list().unwrap();
        if !up.has(since::CLUSTERS) {
            assert!(clusters.is_empty(), "{release}");
            continue;
        }
        let nomad = fx::has_nomad(release);
        assert_eq!(clusters.len(), if nomad { 3 } else { 2 }, "{release}");

        let kind = store.get("c-kind").unwrap();
        assert_eq!(
            (kind.name.as_str(), kind.kind),
            ("kind-dev", ClusterKind::Kubernetes)
        );
        let ClusterConnection::Kubernetes(connection) = &kind.connection else {
            panic!("{release}: c-kind is {:?}", kind.connection);
        };
        assert_eq!(connection.server, "https://127.0.0.1:6443");
        assert_eq!(connection.namespace.as_deref(), Some("apps"));
        assert_eq!(connection.auth, AuthMethod::Token);
        assert_eq!(
            connection.source.as_ref().map(|s| s.context.as_str()),
            Some("kind-dev")
        );
        let ClusterSecrets::Kubernetes(secrets) = store.secrets("c-kind").unwrap() else {
            panic!("{release}: c-kind's secrets are of another kind");
        };
        assert_eq!(
            secrets.token.as_deref(),
            Some(fx::CLUSTER_TOKEN),
            "{release}"
        );
        assert_eq!(
            secrets.certificate_authority.as_deref(),
            Some(fx::CLUSTER_CA),
            "{release}"
        );

        let open = store.get("c-open").unwrap();
        assert!(
            !open.enabled,
            "{release}: a disabled Cluster stays disabled"
        );
        assert_eq!(open.address(), "https://192.168.49.2:8443");

        if nomad {
            let batch = store.get("c-nomad").unwrap();
            assert_eq!(batch.kind, ClusterKind::Nomad);
            let connection = batch.nomad().expect("a Nomad connection");
            assert_eq!(connection.address, "https://nomad.example.com:4646");
            assert_eq!(connection.region.as_deref(), Some("global"));
            assert!(connection.acl_token);
            let ClusterSecrets::Nomad(secrets) = store.secrets("c-nomad").unwrap() else {
                panic!("{release}: c-nomad's secrets are of another kind");
            };
            assert_eq!(secrets.token.as_deref(), Some(fx::NOMAD_TOKEN), "{release}");
        }
    }
}
