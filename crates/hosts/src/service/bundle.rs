//! Exporting Hosts to a passphrase-protected file and importing such a file
//! (ADR 0013). The passphrase is used, never stored or logged.

use std::path::Path;

use serde::{Deserialize, Serialize};

use homeport_core::error::{local_file, AppError, AppResult, ErrorCode};

use super::HostService;
use crate::{
    bundle::{ExportedHost, Payload},
    import::{free_name, ImportFailure, ImportOutcome},
    messages as msg,
    store::HostDraft,
    HostConnection,
};

/// Larger files aren't exports: a thousand Hosts weigh well under a megabyte.
const MAX_BUNDLE_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ExportOutcome {
    pub exported: u32,
    /// Hosts written without their secrets, which couldn't be opened.
    pub without_secrets: Vec<String>,
}

/// A Host in an export file, as the import preview shows it.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BundleHost {
    /// Its place in the file, which the import refers to.
    pub index: u32,
    pub name: String,
    /// `name`, or a free variant when a Host already has it.
    pub suggested_name: String,
    pub name_taken: bool,
    pub connection: HostConnection,
    /// It needs secrets but was exported without them.
    pub secrets_missing: bool,
}

#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BundleChoice {
    pub index: u32,
    pub name: String,
}

impl HostService {
    /// Writes the Hosts `ids`, with their secrets and trusted keys, to `path`.
    pub fn export_hosts(
        &self,
        ids: &[String],
        path: &Path,
        passphrase: &str,
    ) -> AppResult<ExportOutcome> {
        if ids.is_empty() {
            return Err(AppError::validation(msg::EXPORT_NOTHING));
        }
        let mut without_secrets = Vec::new();
        let mut hosts = Vec::with_capacity(ids.len());
        for id in ids {
            let mut host = self.exported(id, &mut without_secrets)?;
            host.id = None;
            hosts.push(host);
        }
        let sealed = Payload::new(hosts).seal(passphrase)?;
        std::fs::write(path, sealed).map_err(|e| local_file(path, e))?;
        Ok(ExportOutcome {
            exported: ids.len() as u32,
            without_secrets,
        })
    }

    /// Every Host as a backup holds it, ids included; the names of those
    /// whose secrets couldn't be opened are added to `without_secrets`.
    pub fn backup_hosts(&self, without_secrets: &mut Vec<String>) -> AppResult<Vec<ExportedHost>> {
        self.store
            .list()?
            .iter()
            .map(|host| self.exported(&host.id, without_secrets))
            .collect()
    }

    /// Host `id` with its opened secrets, or without them (named in
    /// `without_secrets`) when they can't be opened.
    fn exported(&self, id: &str, without_secrets: &mut Vec<String>) -> AppResult<ExportedHost> {
        let host = self.store.get(id)?;
        let secrets = if host.connection.needs_secrets() {
            match self.store.secrets(id) {
                Ok(secrets) => secrets,
                Err(e)
                    if e.is(ErrorCode::CredentialsLost) || e.is(ErrorCode::KeychainUnavailable) =>
                {
                    without_secrets.push(host.name.clone());
                    None
                }
                Err(e) => return Err(e),
            }
        } else {
            None
        };
        Ok(ExportedHost {
            id: Some(host.id),
            name: host.name,
            connection: host.connection,
            engine: host.engine,
            enabled: host.enabled,
            host_key: host.host_key,
            secrets,
        })
    }

    /// The Hosts in an export file, checked against the existing ones.
    pub fn preview_bundle(&self, path: &Path, passphrase: &str) -> AppResult<Vec<BundleHost>> {
        let payload = read_bundle(path, passphrase)?;
        let taken: Vec<String> = self.store.list()?.into_iter().map(|h| h.name).collect();
        Ok(payload
            .hosts
            .into_iter()
            .enumerate()
            .map(|(i, h)| {
                let suggested_name = free_name(&h.name, &taken);
                BundleHost {
                    index: i as u32,
                    name_taken: suggested_name != h.name,
                    secrets_missing: h.connection.needs_secrets() && h.secrets.is_none(),
                    name: h.name,
                    suggested_name,
                    connection: h.connection,
                }
            })
            .collect())
    }

    /// Creates the chosen Hosts of an export file, under the names chosen,
    /// sealing their secrets under this computer's key. Their trusted server
    /// keys come along, so they connect without a trust prompt.
    pub fn import_bundle(
        &self,
        path: &Path,
        passphrase: &str,
        choices: Vec<BundleChoice>,
    ) -> AppResult<ImportOutcome> {
        let payload = read_bundle(path, passphrase)?;
        let mut outcome = ImportOutcome {
            created: vec![],
            failed: vec![],
        };
        for choice in choices {
            let Some(host) = payload.hosts.get(choice.index as usize).cloned() else {
                continue;
            };
            let source = host.name.clone();
            match self.create_exported(host, &choice.name) {
                Ok(id) => outcome.created.push(self.get(&id)?),
                Err(error) => outcome.failed.push(ImportFailure { source, error }),
            }
        }
        Ok(outcome)
    }

    fn create_exported(&self, host: ExportedHost, name: &str) -> AppResult<String> {
        let created = self.store.create_imported(HostDraft {
            name: name.to_owned(),
            connection: host.connection,
            engine: host.engine,
            secrets: host.secrets,
        })?;
        if let Some(key) = &host.host_key {
            self.store.set_host_key(&created.id, Some(key))?;
        }
        if !host.enabled {
            self.store.set_enabled(&created.id, false)?;
        }
        self.registry.restart(&created.id);
        Ok(created.id)
    }
}

fn read_bundle(path: &Path, passphrase: &str) -> AppResult<Payload> {
    let size = std::fs::metadata(path)
        .map_err(|e| local_file(path, e))?
        .len();
    if size > MAX_BUNDLE_BYTES {
        return Err(AppError::validation(
            homeport_storage::messages::BUNDLE_NOT_A_BUNDLE,
        ));
    }
    let bytes = std::fs::read(path).map_err(|e| local_file(path, e))?;
    Payload::open(passphrase, &bytes)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use homeport_storage::{Db, SecretBox};

    use super::*;
    use crate::{
        service::{BastionKeyInput, CredentialsInput, HostInput},
        store::HostStore,
        Bastion, BastionAuth, EngineKind, HostSecrets, SshAuth,
    };
    use homeport_transport as transport;

    /// A computer with its own database and keychain key.
    fn computer(key: u8) -> HostService {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let store = Arc::new(HostStore::new(
            db.clone(),
            Arc::new(SecretBox::from_key(&[key; 32])),
        ));
        HostService::for_tests(db, store)
    }

    const PASS: &str = "moving to the new laptop";

    fn ssh_input(name: &str, jump_key: &str) -> HostInput {
        HostInput {
            name: name.into(),
            connection: HostConnection::Ssh {
                host: "192.0.2.7".into(),
                port: 22,
                user: "deploy".into(),
                socket_path: "/var/run/docker.sock".into(),
                auth: SshAuth::Generated,
                public_key: None,
                bastions: vec![Bastion {
                    host: "192.0.2.1".into(),
                    port: 22,
                    user: "me".into(),
                    auth: BastionAuth::Imported,
                    public_key: None,
                    host_key: None,
                }],
            },
            engine: EngineKind::Docker,
            credentials: None,
            bastion_keys: vec![BastionKeyInput {
                index: 0,
                private_key: jump_key.into(),
                passphrase: None,
            }],
        }
    }

    #[tokio::test]
    async fn hosts_move_to_another_computer_with_their_keys_and_trust() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("hosts.homeport");
        let old = computer(1);
        let jump = transport::ssh::generate_key("jump");
        let ssh = old
            .create(ssh_input("prod", &jump.private_key))
            .unwrap()
            .host
            .id;
        let server = transport::ssh::generate_key("server").public_key;
        let bastion_server = transport::ssh::generate_key("bastion").public_key;
        old.trust_host_key(&ssh, &server, None).unwrap();
        old.trust_host_key(&ssh, &bastion_server, Some("192.0.2.1:22"))
            .unwrap();
        let lab = old
            .create(HostInput {
                name: "lab".into(),
                connection: HostConnection::Tcp {
                    address: "192.0.2.9:2375".into(),
                },
                engine: EngineKind::Docker,
                credentials: None,
                bastion_keys: vec![],
            })
            .unwrap()
            .host
            .id;
        old.set_enabled(&lab, false).unwrap();

        let exported = old.export_hosts(&[ssh.clone(), lab], &file, PASS).unwrap();
        assert_eq!(exported.exported, 2);
        assert!(exported.without_secrets.is_empty());
        let bytes = std::fs::read(&file).unwrap();
        for clear in ["prod", "192.0.2.7", "deploy", "OPENSSH"] {
            assert!(
                !bytes.windows(clear.len()).any(|w| w == clear.as_bytes()),
                "{clear}"
            );
        }

        let new = computer(2);
        let preview = new.preview_bundle(&file, PASS).unwrap();
        assert_eq!(preview.len(), 2);
        assert!(preview.iter().all(|h| !h.secrets_missing && !h.name_taken));
        let outcome = new
            .import_bundle(
                &file,
                PASS,
                preview
                    .iter()
                    .map(|h| BundleChoice {
                        index: h.index,
                        name: h.suggested_name.clone(),
                    })
                    .collect(),
            )
            .unwrap();
        assert!(outcome.failed.is_empty());
        let prod = outcome
            .created
            .iter()
            .find(|v| v.host.name == "prod")
            .unwrap();
        assert_eq!(prod.host.host_key.as_deref(), Some(server.as_str()));
        let HostConnection::Ssh { bastions, .. } = &prod.host.connection else {
            panic!("expected SSH");
        };
        assert_eq!(
            bastions[0].host_key.as_deref(),
            Some(bastion_server.as_str())
        );
        // The secrets are sealed under the new computer's key and open there.
        let Some(HostSecrets::Ssh {
            private_key: Some(_),
            bastions: keys,
            ..
        }) = new.store.secrets(&prod.host.id).unwrap()
        else {
            panic!("expected the Host's and the bastion's keys");
        };
        assert_eq!(keys["192.0.2.1:22"].private_key, jump.private_key);
        let lab = outcome
            .created
            .iter()
            .find(|v| v.host.name == "lab")
            .unwrap();
        assert!(!lab.host.enabled);

        // Again: names are taken now, so the preview suggests others.
        let again = new.preview_bundle(&file, PASS).unwrap();
        assert!(again.iter().all(|h| h.name_taken));
        assert_eq!(again[0].suggested_name, format!("{} (2)", again[0].name));
    }

    #[tokio::test]
    async fn hosts_whose_secrets_are_lost_are_exported_without_them() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("hosts.homeport");
        let db = Arc::new(Db::open_in_memory().unwrap());
        let before = HostStore::new(db.clone(), Arc::new(SecretBox::from_key(&[1; 32])));
        let id = before
            .create(HostDraft {
                name: "tls".into(),
                connection: HostConnection::Tls {
                    address: "h:2376".into(),
                },
                engine: EngineKind::Docker,
                secrets: Some(HostSecrets::Tls {
                    ca_pem: "CA".into(),
                    cert_pem: "CERT".into(),
                    key_pem: "KEY".into(),
                }),
            })
            .unwrap()
            .id;
        let store = Arc::new(HostStore::new(
            db.clone(),
            Arc::new(SecretBox::from_key(&[2; 32])),
        ));
        let svc = HostService::for_tests(db, store);
        let outcome = svc.export_hosts(&[id], &file, PASS).unwrap();
        assert_eq!(outcome.without_secrets, ["tls"]);

        let other = computer(3);
        let preview = other.preview_bundle(&file, PASS).unwrap();
        assert!(preview[0].secrets_missing);
        let imported = other
            .import_bundle(
                &file,
                PASS,
                vec![BundleChoice {
                    index: 0,
                    name: "tls".into(),
                }],
            )
            .unwrap();
        assert_eq!(imported.created.len(), 1);
        assert_eq!(
            other.store.secrets(&imported.created[0].host.id).unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn the_wrong_passphrase_imports_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("hosts.homeport");
        let svc = computer(1);
        let id = svc
            .create(HostInput {
                name: "tls".into(),
                connection: HostConnection::Local {
                    socket_path: "/var/run/docker.sock".into(),
                },
                engine: EngineKind::Docker,
                credentials: None::<CredentialsInput>,
                bastion_keys: vec![],
            })
            .unwrap()
            .host
            .id;
        svc.export_hosts(&[id], &file, PASS).unwrap();
        let err = computer(2)
            .preview_bundle(&file, "not the passphrase")
            .unwrap_err();
        assert_eq!(err.code, ErrorCode::BundleUnreadable);
        assert!(svc.export_hosts(&[], &file, PASS).is_err());
    }
}
