//! Use cases for managing Hosts: prepares credentials (generating SSH keys),
//! persists through [`HostStore`] and keeps [`HostRegistry`] in step.

mod credentials;
mod import;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;
use homeport_transport as transport;

pub use self::credentials::{BastionKeyInput, CredentialsInput};

use self::credentials::{key_comment, prepare, Current};
use crate::{
    registry::{HostRegistry, HostStatus},
    store::{HostDraft, HostStore},
    BastionAuth, Host, HostConnection, HostSecrets, SshAuth,
};

#[derive(Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HostInput {
    pub name: String,
    pub connection: HostConnection,
    pub credentials: Option<CredentialsInput>,
    /// Keys typed for the connection's `imported` bastions.
    #[serde(default)]
    pub bastion_keys: Vec<BastionKeyInput>,
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HostView {
    pub host: Host,
    pub status: HostStatus,
}

/// What the user needs to authorize Homeport's key on an SSH server.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SshSetup {
    pub public_key: String,
    pub fingerprint: String,
    /// Appends the key to `~/.ssh/authorized_keys`, run on the server.
    pub install_command: String,
    /// Lets the SSH user reach the Docker socket, run on the server.
    pub docker_group_command: String,
    /// Every server the key must be authorised on, as `user@host`: the
    /// bastions that use the Host's key, in order, then the Host.
    pub servers: Vec<String>,
}

pub struct HostService {
    pub store: Arc<HostStore>,
    pub registry: Arc<HostRegistry>,
}

impl HostService {
    pub fn list(&self) -> AppResult<Vec<HostView>> {
        Ok(self
            .store
            .list()?
            .into_iter()
            .map(|h| self.view(h))
            .collect())
    }

    pub fn get(&self, id: &str) -> AppResult<HostView> {
        Ok(self.view(self.store.get(id)?))
    }

    pub fn create(&self, input: HostInput) -> AppResult<HostView> {
        let (connection, secrets) = prepare(
            &input.name,
            input.connection,
            input.credentials,
            input.bastion_keys,
            None,
        )?;
        let host = self.store.create(HostDraft {
            name: input.name,
            connection,
            secrets,
        })?;
        self.registry.restart(&host.id);
        self.get(&host.id)
    }

    pub fn update(&self, id: &str, input: HostInput) -> AppResult<HostView> {
        let current = self.store.get(id)?;
        let (connection, secrets) = prepare(
            &input.name,
            input.connection,
            input.credentials,
            input.bastion_keys,
            Some(Current {
                connection: &current.connection,
                secrets: self.store.secrets(id).ok().flatten(),
            }),
        )?;
        self.store.update(
            id,
            HostDraft {
                name: input.name,
                connection,
                secrets,
            },
        )?;
        self.registry.restart(id);
        self.get(id)
    }

    pub fn delete(&self, id: &str) -> AppResult<()> {
        self.registry.remove(id);
        self.store.delete(id)
    }

    pub fn set_enabled(&self, id: &str, enabled: bool) -> AppResult<HostView> {
        self.store.set_enabled(id, enabled)?;
        self.registry.restart(id);
        self.get(id)
    }

    /// Trusts the SSH server key the user just confirmed, then reconnects.
    /// `bastion` is the address of the bastion that presented it, `None` for
    /// the Host's own server.
    pub fn trust_host_key(
        &self,
        id: &str,
        key: &str,
        bastion: Option<&str>,
    ) -> AppResult<HostView> {
        if transport::ssh::fingerprint(key).is_none() {
            return Err(AppError::validation(msg::NOT_A_PUBLIC_KEY));
        }
        match bastion {
            Some(address) => self.store.set_bastion_key(id, address, key.trim())?,
            None => self.store.set_host_key(id, Some(key.trim()))?,
        };
        self.registry.restart(id);
        self.get(id)
    }

    pub fn reconnect(&self, id: &str) -> AppResult<HostView> {
        self.store.get(id)?;
        self.registry.restart(id);
        self.get(id)
    }

    /// Replaces an SSH Host's key with a newly generated one, keeping its
    /// bastions' own keys.
    pub fn regenerate_ssh_key(&self, id: &str) -> AppResult<HostView> {
        let current = self.store.get(id)?;
        let HostConnection::Ssh {
            host,
            port,
            user,
            socket_path,
            bastions,
            ..
        } = current.connection
        else {
            return Err(AppError::validation(msg::KEYS_ONLY_FOR_SSH));
        };
        let bastion_keys = if bastions.iter().any(|b| b.auth == BastionAuth::Imported) {
            match self.store.secrets(id)? {
                Some(HostSecrets::Ssh { bastions, .. }) => bastions,
                _ => Default::default(),
            }
        } else {
            Default::default()
        };
        let key = transport::ssh::generate_key(&key_comment(&current.name));
        self.store.update(
            id,
            HostDraft {
                name: current.name,
                connection: HostConnection::Ssh {
                    host,
                    port,
                    user,
                    socket_path,
                    auth: SshAuth::Generated,
                    public_key: Some(key.public_key),
                    bastions,
                },
                secrets: Some(HostSecrets::Ssh {
                    private_key: Some(key.private_key),
                    passphrase: None,
                    bastions: bastion_keys,
                }),
            },
        )?;
        self.registry.restart(id);
        self.get(id)
    }

    pub fn ssh_setup(&self, id: &str) -> AppResult<SshSetup> {
        let host = self.store.get(id)?;
        let HostConnection::Ssh {
            public_key: Some(public_key),
            user,
            host: server,
            bastions,
            ..
        } = host.connection
        else {
            return Err(AppError::validation(msg::NO_MANAGED_KEY));
        };
        let servers = bastions
            .iter()
            .filter(|b| b.auth == BastionAuth::SameAsHost)
            .map(|b| format!("{}@{}", b.user, b.host))
            .chain(std::iter::once(format!("{user}@{server}")))
            .collect();
        Ok(SshSetup {
            fingerprint: transport::ssh::fingerprint(&public_key).unwrap_or_default(),
            install_command: transport::ssh::authorized_keys_command(&public_key),
            docker_group_command: format!("sudo usermod -aG docker {user}"),
            public_key,
            servers,
        })
    }

    fn view(&self, host: Host) -> HostView {
        HostView {
            status: self.registry.status(&host.id),
            host,
        }
    }
}

#[cfg(test)]
mod tests {
    use homeport_storage::{Db, SecretBox};

    use super::*;
    use homeport_core::error::ErrorCode;

    use crate::registry::{EngineEvent, Notifier};

    struct Silent;
    impl Notifier for Silent {
        fn host_status(&self, _: &str, _: &HostStatus) {}
        fn engine_event(&self, _: &str, _: &EngineEvent) {}
    }

    fn service() -> HostService {
        let store = Arc::new(HostStore::new(
            Arc::new(Db::open_in_memory().unwrap()),
            Arc::new(SecretBox::from_key(&[5; 32])),
        ));
        HostService {
            registry: HostRegistry::new(store.clone(), Arc::new(Silent)),
            store,
        }
    }

    fn ssh(auth: SshAuth, credentials: Option<CredentialsInput>) -> HostInput {
        HostInput {
            name: "prod".into(),
            connection: HostConnection::Ssh {
                // An unroutable address keeps the supervisor from reaching anything.
                host: "192.0.2.1".into(),
                port: 22,
                user: "deploy".into(),
                socket_path: "/var/run/docker.sock".into(),
                auth,
                public_key: None,
                bastions: vec![],
            },
            credentials,
            bastion_keys: vec![],
        }
    }

    fn public_key(view: &HostView) -> Option<String> {
        match &view.host.connection {
            HostConnection::Ssh { public_key, .. } => public_key.clone(),
            _ => None,
        }
    }

    #[tokio::test]
    async fn ssh_hosts_get_a_generated_key_and_setup_instructions() {
        let svc = service();
        let view = svc.create(ssh(SshAuth::Generated, None)).unwrap();
        let key = public_key(&view).unwrap();
        assert!(key.starts_with("ssh-ed25519 "));
        assert!(key.ends_with("-prod"));

        let setup = svc.ssh_setup(&view.host.id).unwrap();
        assert!(setup.install_command.contains(&key));
        assert_eq!(setup.docker_group_command, "sudo usermod -aG docker deploy");
        assert!(setup.fingerprint.starts_with("SHA256:"));

        let Some(HostSecrets::Ssh {
            private_key: Some(private_key),
            ..
        }) = svc.store.secrets(&view.host.id).unwrap()
        else {
            panic!("expected SSH secrets");
        };
        assert_eq!(
            transport::ssh::public_key_of(&private_key, None).unwrap(),
            key
        );
    }

    #[tokio::test]
    async fn editing_an_ssh_host_keeps_its_key_until_regenerated() {
        let svc = service();
        let id = svc.create(ssh(SshAuth::Generated, None)).unwrap().host.id;
        let original = public_key(&svc.get(&id).unwrap());

        let mut renamed = ssh(SshAuth::Generated, None);
        renamed.name = "production".into();
        assert_eq!(public_key(&svc.update(&id, renamed).unwrap()), original);

        let regenerated = public_key(&svc.regenerate_ssh_key(&id).unwrap());
        assert!(regenerated.is_some());
        assert_ne!(regenerated, original);
    }

    #[tokio::test]
    async fn imported_keys_show_their_public_half() {
        let generated = transport::ssh::generate_key("mine");
        let view = service()
            .create(ssh(
                SshAuth::Imported,
                Some(CredentialsInput::SshKey {
                    private_key: generated.private_key,
                    passphrase: Some(String::new()),
                }),
            ))
            .unwrap();
        assert_eq!(public_key(&view), Some(generated.public_key));
    }

    #[tokio::test]
    async fn agent_hosts_store_no_secrets() {
        let svc = service();
        let id = svc.create(ssh(SshAuth::Agent, None)).unwrap().host.id;
        assert_eq!(svc.store.secrets(&id).unwrap(), None);
        assert!(svc.ssh_setup(&id).is_err());
    }

    #[tokio::test]
    async fn unusable_credentials_are_rejected_up_front() {
        let svc = service();
        let bad_key = ssh(
            SshAuth::Imported,
            Some(CredentialsInput::SshKey {
                private_key: "nope".into(),
                passphrase: None,
            }),
        );
        assert!(matches!(svc.create(bad_key), Err(ref e) if e.is(ErrorCode::Validation)));

        let bad_tls = HostInput {
            name: "tls".into(),
            connection: HostConnection::Tls {
                address: "h:2376".into(),
            },
            credentials: Some(CredentialsInput::Tls {
                ca_pem: "x".into(),
                cert_pem: "x".into(),
                key_pem: "x".into(),
            }),
            bastion_keys: vec![],
        };
        assert!(matches!(svc.create(bad_tls), Err(ref e) if e.is(ErrorCode::Validation)));
    }

    #[tokio::test]
    async fn trusting_requires_a_real_key() {
        let svc = service();
        let id = svc.create(ssh(SshAuth::Agent, None)).unwrap().host.id;
        assert!(svc.trust_host_key(&id, "garbage", None).is_err());
        let key = transport::ssh::generate_key("server").public_key;
        assert_eq!(
            svc.trust_host_key(&id, &key, None).unwrap().host.host_key,
            Some(key)
        );
    }

    fn bastion(host: &str, auth: BastionAuth) -> crate::Bastion {
        crate::Bastion {
            host: host.into(),
            port: 22,
            user: "me".into(),
            auth,
            public_key: None,
            host_key: None,
        }
    }

    fn behind(
        auth: SshAuth,
        bastions: Vec<crate::Bastion>,
        keys: Vec<BastionKeyInput>,
    ) -> HostInput {
        let mut input = ssh(auth, None);
        if let HostConnection::Ssh { bastions: b, .. } = &mut input.connection {
            *b = bastions;
        }
        input.bastion_keys = keys;
        input
    }

    fn bastions_of(view: &HostView) -> Vec<crate::Bastion> {
        match &view.host.connection {
            HostConnection::Ssh { bastions, .. } => bastions.clone(),
            _ => vec![],
        }
    }

    fn key_input(index: u32, private_key: &str) -> BastionKeyInput {
        BastionKeyInput {
            index,
            private_key: private_key.into(),
            passphrase: None,
        }
    }

    #[tokio::test]
    async fn bastions_with_their_own_key_keep_it_sealed_by_address() {
        let svc = service();
        let jump_key = transport::ssh::generate_key("jump");
        let view = svc
            .create(behind(
                SshAuth::Agent,
                vec![
                    bastion("jump", BastionAuth::Imported),
                    bastion("inner", BastionAuth::SameAsHost),
                ],
                vec![key_input(0, &jump_key.private_key)],
            ))
            .unwrap();
        let bastions = bastions_of(&view);
        assert_eq!(
            bastions[0].public_key.as_deref(),
            Some(jump_key.public_key.as_str())
        );
        assert_eq!(bastions[1].public_key, None);
        let Some(HostSecrets::Ssh {
            private_key,
            bastions: keys,
            ..
        }) = svc.store.secrets(&view.host.id).unwrap()
        else {
            panic!("expected SSH secrets");
        };
        assert_eq!(private_key, None);
        assert_eq!(keys["jump:22"].private_key, jump_key.private_key);

        // Moved after another bastion and renamed, its key isn't retyped.
        let updated = svc
            .update(
                &view.host.id,
                behind(
                    SshAuth::Agent,
                    vec![
                        bastion("first", BastionAuth::Agent),
                        bastion("jump", BastionAuth::Imported),
                    ],
                    vec![],
                ),
            )
            .unwrap();
        assert_eq!(
            bastions_of(&updated)[1].public_key.as_deref(),
            Some(jump_key.public_key.as_str())
        );
    }

    #[tokio::test]
    async fn a_bastion_using_its_own_key_needs_one() {
        let svc = service();
        let err = svc
            .create(behind(
                SshAuth::Agent,
                vec![
                    bastion("first", BastionAuth::Agent),
                    bastion("jump", BastionAuth::Imported),
                ],
                vec![],
            ))
            .unwrap_err();
        assert!(err.is(ErrorCode::Validation));
        assert_eq!(err.message, "Bastion 2 needs its private key.");
    }

    #[tokio::test]
    async fn adding_a_bastion_key_keeps_the_hosts_own_key() {
        let svc = service();
        let id = svc.create(ssh(SshAuth::Generated, None)).unwrap().host.id;
        let original = public_key(&svc.get(&id).unwrap());
        let jump_key = transport::ssh::generate_key("jump");
        let view = svc
            .update(
                &id,
                behind(
                    SshAuth::Generated,
                    vec![bastion("jump", BastionAuth::Imported)],
                    vec![key_input(0, &jump_key.private_key)],
                ),
            )
            .unwrap();
        assert_eq!(public_key(&view), original);
        let Some(HostSecrets::Ssh {
            private_key: Some(_),
            bastions,
            ..
        }) = svc.store.secrets(&id).unwrap()
        else {
            panic!("expected the Host's key and the bastion's");
        };
        assert!(bastions.contains_key("jump:22"));

        // Regenerating the Host's key leaves the bastion's alone.
        svc.regenerate_ssh_key(&id).unwrap();
        let Some(HostSecrets::Ssh { bastions, .. }) = svc.store.secrets(&id).unwrap() else {
            panic!("expected SSH secrets");
        };
        assert!(bastions.contains_key("jump:22"));
    }

    #[tokio::test]
    async fn bastion_keys_are_trusted_on_the_bastion_and_the_form_cannot_set_them() {
        let svc = service();
        let mut forged = bastion("jump", BastionAuth::SameAsHost);
        forged.host_key = Some("ssh-ed25519 FORGED".into());
        let id = svc
            .create(behind(SshAuth::Agent, vec![forged], vec![]))
            .unwrap()
            .host
            .id;
        assert_eq!(bastions_of(&svc.get(&id).unwrap())[0].host_key, None);

        let key = transport::ssh::generate_key("jump").public_key;
        let view = svc.trust_host_key(&id, &key, Some("jump:22")).unwrap();
        assert_eq!(bastions_of(&view)[0].host_key, Some(key));
        assert_eq!(view.host.host_key, None);
        assert!(svc
            .trust_host_key(&id, &key_of("x"), Some("gone:22"))
            .is_err());
    }

    fn key_of(comment: &str) -> String {
        transport::ssh::generate_key(comment).public_key
    }

    #[tokio::test]
    async fn the_setup_lists_every_server_the_generated_key_logs_in_to() {
        let svc = service();
        let id = svc
            .create(behind(
                SshAuth::Generated,
                vec![
                    bastion("jump", BastionAuth::SameAsHost),
                    bastion("inner", BastionAuth::Agent),
                ],
                vec![],
            ))
            .unwrap()
            .host
            .id;
        assert_eq!(
            svc.ssh_setup(&id).unwrap().servers,
            ["me@jump", "deploy@192.0.2.1"]
        );
    }
}
