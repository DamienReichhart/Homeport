//! Use cases for managing Hosts: prepares credentials (generating SSH keys),
//! persists through [`HostStore`] and keeps [`HostRegistry`] in step.

mod credentials;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;
use homeport_transport as transport;

pub use self::credentials::CredentialsInput;

use self::credentials::{key_comment, prepare};
use crate::{
    registry::{HostRegistry, HostStatus},
    store::{HostDraft, HostStore},
    Host, HostConnection, HostSecrets, SshAuth,
};

#[derive(Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HostInput {
    pub name: String,
    pub connection: HostConnection,
    pub credentials: Option<CredentialsInput>,
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
        let (connection, secrets) =
            prepare(&input.name, input.connection, input.credentials, None)?;
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
            Some(&current.connection),
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
    pub fn trust_host_key(&self, id: &str, key: &str) -> AppResult<HostView> {
        if transport::ssh::fingerprint(key).is_none() {
            return Err(AppError::validation(msg::NOT_A_PUBLIC_KEY));
        }
        self.store.set_host_key(id, Some(key.trim()))?;
        self.registry.restart(id);
        self.get(id)
    }

    pub fn reconnect(&self, id: &str) -> AppResult<HostView> {
        self.store.get(id)?;
        self.registry.restart(id);
        self.get(id)
    }

    /// Replaces an SSH Host's key with a newly generated one.
    pub fn regenerate_ssh_key(&self, id: &str) -> AppResult<HostView> {
        let current = self.store.get(id)?;
        let HostConnection::Ssh {
            host,
            port,
            user,
            socket_path,
            ..
        } = current.connection
        else {
            return Err(AppError::validation(msg::KEYS_ONLY_FOR_SSH));
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
                },
                secrets: Some(HostSecrets::Ssh {
                    private_key: key.private_key,
                    passphrase: None,
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
            ..
        } = host.connection
        else {
            return Err(AppError::validation(msg::NO_MANAGED_KEY));
        };
        Ok(SshSetup {
            fingerprint: transport::ssh::fingerprint(&public_key).unwrap_or_default(),
            install_command: transport::ssh::authorized_keys_command(&public_key),
            docker_group_command: format!("sudo usermod -aG docker {user}"),
            public_key,
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
            },
            credentials,
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

        let Some(HostSecrets::Ssh { private_key, .. }) = svc.store.secrets(&view.host.id).unwrap()
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
        };
        assert!(matches!(svc.create(bad_tls), Err(ref e) if e.is(ErrorCode::Validation)));
    }

    #[tokio::test]
    async fn trusting_requires_a_real_key() {
        let svc = service();
        let id = svc.create(ssh(SshAuth::Agent, None)).unwrap().host.id;
        assert!(svc.trust_host_key(&id, "garbage").is_err());
        let key = transport::ssh::generate_key("server").public_key;
        assert_eq!(
            svc.trust_host_key(&id, &key).unwrap().host.host_key,
            Some(key)
        );
    }
}
