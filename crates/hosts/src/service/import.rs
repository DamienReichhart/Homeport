//! Previewing and importing Hosts from `~/.ssh/config` and Docker contexts.
//! Keys and certificates are read again at import, on this side, so they
//! never travel through the frontend.

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::{BastionKeyInput, CredentialsInput, HostInput, HostService};
use crate::{
    import::{
        docker_context,
        ssh_config::{self, DEFAULT_SOCKET},
        Env, ImportChoice, ImportDraft, ImportFailure, ImportOutcome,
    },
    messages as msg, HostConnection,
};

impl HostService {
    fn env() -> AppResult<Env> {
        Env::current().ok_or_else(|| AppError::new(ErrorCode::NotFound, msg::NO_HOME_FOLDER))
    }

    fn host_names(&self) -> AppResult<Vec<String>> {
        Ok(self.store.list()?.into_iter().map(|h| h.name).collect())
    }

    /// The servers of `~/.ssh/config`, as drafts.
    pub fn preview_ssh_config(&self) -> AppResult<Vec<ImportDraft>> {
        self.preview_ssh_config_in(&Self::env()?)
    }

    pub fn preview_ssh_config_in(&self, env: &Env) -> AppResult<Vec<ImportDraft>> {
        let config = ssh_config::read(env)?;
        Ok(ssh_config::drafts(&config, env, &self.host_names()?))
    }

    /// Creates a Host for each chosen alias, with its key and bastions' keys.
    pub fn import_ssh_config(&self, choices: Vec<ImportChoice>) -> AppResult<ImportOutcome> {
        self.import_ssh_config_in(&Self::env()?, choices)
    }

    pub fn import_ssh_config_in(
        &self,
        env: &Env,
        choices: Vec<ImportChoice>,
    ) -> AppResult<ImportOutcome> {
        let config = ssh_config::read(env)?;
        Ok(self.import_each(choices, |choice| {
            let plan = ssh_config::plan(&config, env, &choice.source)?;
            let passphrase = choice.passphrase.clone().filter(|p| !p.is_empty());
            let mut connection = plan.connection;
            set_socket(&mut connection, choice.socket_path.as_deref());
            Ok(HostInput {
                name: choice.name.clone(),
                connection,
                credentials: plan.key.map(|k| CredentialsInput::SshKey {
                    private_key: k.text,
                    passphrase: passphrase.clone(),
                }),
                bastion_keys: plan
                    .bastion_keys
                    .into_iter()
                    .map(|(i, k)| BastionKeyInput {
                        index: i as u32,
                        private_key: k.text,
                        passphrase: passphrase.clone(),
                    })
                    .collect(),
            })
        }))
    }

    /// The Docker CLI's contexts, as drafts.
    pub fn preview_docker_contexts(&self) -> AppResult<Vec<ImportDraft>> {
        self.preview_docker_contexts_in(&Self::env()?)
    }

    pub fn preview_docker_contexts_in(&self, env: &Env) -> AppResult<Vec<ImportDraft>> {
        Ok(docker_context::drafts(
            docker_context::read(env),
            &self.store.list()?,
        ))
    }

    /// Creates a Host for each chosen context, with its TLS material.
    pub fn import_docker_contexts(&self, choices: Vec<ImportChoice>) -> AppResult<ImportOutcome> {
        self.import_docker_contexts_in(&Self::env()?, choices)
    }

    pub fn import_docker_contexts_in(
        &self,
        env: &Env,
        choices: Vec<ImportChoice>,
    ) -> AppResult<ImportOutcome> {
        let mut contexts = docker_context::read(env);
        Ok(self.import_each(choices, |choice| {
            let index = contexts
                .iter()
                .position(|c| c.name == choice.source)
                .ok_or_else(|| {
                    AppError::validation(msg::IMPORT_SOURCE_GONE.with("source", &choice.source))
                })?;
            let plan = contexts.swap_remove(index).plan?;
            let mut connection = plan.connection;
            set_socket(&mut connection, choice.socket_path.as_deref());
            Ok(HostInput {
                name: choice.name.clone(),
                connection,
                credentials: plan.tls.map(|t| CredentialsInput::Tls {
                    ca_pem: t.ca_pem,
                    cert_pem: t.cert_pem,
                    key_pem: t.key_pem,
                }),
                bastion_keys: vec![],
            })
        }))
    }

    /// Creates each chosen entry, carrying on past the ones that fail.
    fn import_each(
        &self,
        choices: Vec<ImportChoice>,
        mut input: impl FnMut(&ImportChoice) -> AppResult<HostInput>,
    ) -> ImportOutcome {
        let mut outcome = ImportOutcome {
            created: vec![],
            failed: vec![],
        };
        for choice in choices {
            match input(&choice).and_then(|i| self.create(i)) {
                Ok(view) => outcome.created.push(view),
                Err(error) => outcome.failed.push(ImportFailure {
                    source: choice.source,
                    error,
                }),
            }
        }
        outcome
    }
}

/// Puts the socket the user typed in the preview on an SSH connection.
fn set_socket(connection: &mut HostConnection, socket: Option<&str>) {
    if let HostConnection::Ssh { socket_path, .. } = connection {
        *socket_path = socket
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(DEFAULT_SOCKET)
            .to_owned();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use homeport_storage::{Db, SecretBox};

    use super::*;
    use crate::{
        registry::{EngineEvent, HostRegistry, HostStatus, Notifier},
        store::HostStore,
        BastionAuth, HostSecrets, SshAuth,
    };
    use homeport_transport as transport;

    struct Silent;
    impl Notifier for Silent {
        fn host_status(&self, _: &str, _: &HostStatus) {}
        fn engine_event(&self, _: &str, _: &EngineEvent) {}
    }

    fn service() -> HostService {
        let store = Arc::new(HostStore::new(
            Arc::new(Db::open_in_memory().unwrap()),
            Arc::new(SecretBox::from_key(&[3; 32])),
        ));
        HostService {
            registry: HostRegistry::new(store.clone(), Arc::new(Silent)),
            store,
        }
    }

    fn home(config: &str) -> (tempfile::TempDir, Env) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".ssh")).unwrap();
        std::fs::write(dir.path().join(".ssh/config"), config).unwrap();
        let env = Env {
            home: dir.path().into(),
            local_user: "me".into(),
            docker_config: Some(dir.path().join("docker")),
        };
        (dir, env)
    }

    fn choice(source: &str) -> ImportChoice {
        ImportChoice {
            source: source.into(),
            name: source.into(),
            socket_path: None,
            passphrase: None,
        }
    }

    #[tokio::test]
    async fn ssh_config_entries_import_with_sealed_keys_and_bastions() {
        let (dir, env) = home(
            "Host app\n  HostName 192.0.2.10\n  User deploy\n  IdentityFile ~/.ssh/app\n  ProxyJump edge\n\
             Host edge\n  HostName 192.0.2.1\n  IdentityFile ~/.ssh/edge\n",
        );
        let app = transport::ssh::generate_key("app");
        let edge = transport::ssh::generate_key("edge");
        std::fs::write(dir.path().join(".ssh/app"), &app.private_key).unwrap();
        std::fs::write(dir.path().join(".ssh/edge"), &edge.private_key).unwrap();
        let config_before = std::fs::read(dir.path().join(".ssh/config")).unwrap();

        let svc = service();
        let mut picked = choice("app");
        picked.socket_path = Some("/run/user/1000/docker.sock".into());
        let outcome = svc.import_ssh_config_in(&env, vec![picked]).unwrap();
        assert!(outcome.failed.is_empty(), "{:?}", outcome.failed);
        let host = &outcome.created[0].host;
        let HostConnection::Ssh {
            auth,
            public_key,
            socket_path,
            bastions,
            ..
        } = &host.connection
        else {
            panic!("expected SSH");
        };
        assert_eq!(*auth, SshAuth::Imported);
        assert_eq!(public_key.as_deref(), Some(app.public_key.as_str()));
        assert_eq!(socket_path, "/run/user/1000/docker.sock");
        assert_eq!(bastions[0].auth, BastionAuth::Imported);
        assert_eq!(
            bastions[0].public_key.as_deref(),
            Some(edge.public_key.as_str())
        );
        let Some(HostSecrets::Ssh {
            private_key,
            bastions: keys,
            ..
        }) = svc.store.secrets(&host.id).unwrap()
        else {
            panic!("expected sealed SSH secrets");
        };
        assert_eq!(private_key.as_deref(), Some(app.private_key.as_str()));
        assert_eq!(keys["192.0.2.1:22"].private_key, edge.private_key);
        assert_eq!(
            std::fs::read(dir.path().join(".ssh/config")).unwrap(),
            config_before
        );

        // Importing again: the name is taken, so the preview says so and the
        // import fails rather than creating a duplicate.
        let drafts = svc.preview_ssh_config_in(&env).unwrap();
        assert!(
            drafts
                .iter()
                .find(|d| d.source == "app")
                .unwrap()
                .name_taken
        );
        let again = svc.import_ssh_config_in(&env, vec![choice("app")]).unwrap();
        assert!(again.created.is_empty());
        assert_eq!(again.failed[0].source, "app");
        assert_eq!(svc.store.list().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn an_encrypted_key_imports_with_its_passphrase_only() {
        let (dir, env) = home("Host box\n  HostName 192.0.2.2\n  IdentityFile ~/.ssh/enc\n");
        std::fs::write(
            dir.path().join(".ssh/enc"),
            include_str!("../../../transport/src/ssh/testdata/encrypted_ed25519"),
        )
        .unwrap();
        let svc = service();
        let failed = svc.import_ssh_config_in(&env, vec![choice("box")]).unwrap();
        assert_eq!(failed.failed.len(), 1);
        let mut picked = choice("box");
        picked.passphrase = Some("secret".into());
        let ok = svc.import_ssh_config_in(&env, vec![picked]).unwrap();
        assert_eq!(ok.created.len(), 1, "{:?}", ok.failed);
    }

    #[tokio::test]
    async fn docker_contexts_import_with_their_tls_material_sealed() {
        let (dir, env) = home("");
        let meta = dir.path().join("docker/contexts/meta/abc");
        std::fs::create_dir_all(&meta).unwrap();
        std::fs::write(
            meta.join("meta.json"),
            r#"{"Name":"lab","Endpoints":{"docker":{"Host":"tcp://10.0.0.2:2375"}}}"#,
        )
        .unwrap();
        let svc = service();
        let drafts = svc.preview_docker_contexts_in(&env).unwrap();
        assert_eq!(drafts.len(), 1);
        let outcome = svc
            .import_docker_contexts_in(&env, vec![choice("lab"), choice("gone")])
            .unwrap();
        assert_eq!(outcome.created[0].host.name, "lab");
        assert_eq!(outcome.failed[0].source, "gone");
        // Now a Host, it can't be picked again.
        let drafts = svc.preview_docker_contexts_in(&env).unwrap();
        assert!(drafts[0].problem.is_some());
    }
}
