//! Use cases for managing Hosts: prepares credentials (generating SSH keys),
//! persists through [`HostStore`] and keeps [`HostRegistry`] in step.

mod bundle;
mod credentials;
mod import;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::messages as msg;
use homeport_transport as transport;

pub use self::{
    bundle::{BundleChoice, BundleHost, ExportOutcome},
    credentials::{BastionKeyInput, CredentialsInput},
};

use self::credentials::{key_comment, prepare, Current};
use crate::{
    events::EventStore,
    registry::{HostRegistry, HostStatus},
    store::{HostDraft, HostStore},
    BastionAuth, EngineKind, Host, HostConnection, HostSecrets, SshAuth,
};

#[derive(Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HostInput {
    pub name: String,
    pub connection: HostConnection,
    /// The kind of Engine it points to; Docker-compatible when left out.
    #[serde(default)]
    pub engine: EngineKind,
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
    /// Lets the SSH user reach the Engine's socket (Docker's, or Incus's),
    /// run on the server.
    pub docker_group_command: String,
    /// Every server the key must be authorised on, as `user@host`: the
    /// bastions that use the Host's key, in order, then the Host.
    pub servers: Vec<String>,
}

pub struct HostService {
    pub store: Arc<HostStore>,
    pub registry: Arc<HostRegistry>,
    /// Each Host's Engine events, kept across restarts.
    pub events: Arc<EventStore>,
}

impl HostService {
    /// A service over `store` with nothing to report to, for tests.
    #[cfg(test)]
    pub(crate) fn for_tests(db: Arc<homeport_storage::Db>, store: Arc<HostStore>) -> Self {
        struct Silent;
        impl crate::registry::Notifier for Silent {
            fn host_status(&self, _: &str, _: &HostStatus) {}
            fn engine_event(&self, _: &str, _: &crate::registry::EngineEvent) {}
        }
        let events = EventStore::new(db);
        Self {
            registry: HostRegistry::new(store.clone(), events.clone(), Arc::new(Silent)),
            store,
            events,
        }
    }

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
            input.engine,
            input.connection,
            input.credentials,
            input.bastion_keys,
            None,
        )?;
        let host = self.store.create(HostDraft {
            name: input.name,
            connection,
            engine: input.engine,
            secrets,
        })?;
        self.registry.restart(&host.id);
        self.get(&host.id)
    }

    pub fn update(&self, id: &str, input: HostInput) -> AppResult<HostView> {
        let current = self.store.get(id)?;
        let (connection, secrets) = prepare(
            &input.name,
            input.engine,
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
                engine: input.engine,
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
                engine: current.engine,
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
            socket_path,
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
            docker_group_command: match host.engine {
                // The restricted socket is the incus group's (ADR 0021).
                EngineKind::Incus if socket_path.ends_with(".user") => {
                    format!("sudo usermod -aG incus {user}")
                }
                EngineKind::Incus => format!("sudo usermod -aG incus-admin {user}"),
                _ => format!("sudo usermod -aG docker {user}"),
            },
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
mod tests;
