//! Turning the credentials a user typed into what the store keeps: generated
//! or imported SSH keys, and TLS material checked before it's saved.

use serde::Deserialize;

use std::collections::BTreeMap;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;
use homeport_transport::{self as transport, ConnectError};

use crate::{Bastion, BastionAuth, EngineKind, HostConnection, HostSecrets, SshAuth, SshKeySecret};

/// Credentials typed by the user. Absent on update means "keep the stored ones".
#[derive(Deserialize, specta::Type)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum CredentialsInput {
    Tls {
        ca_pem: String,
        cert_pem: String,
        key_pem: String,
    },
    SshKey {
        private_key: String,
        passphrase: Option<String>,
    },
}

pub(super) fn key_comment(host_name: &str) -> String {
    let machine = gethostname::gethostname().to_string_lossy().into_owned();
    format!("homeport@{machine}-{host_name}")
}

/// The private key typed for the `imported` bastion at `index` in the input's
/// list. A bastion with no entry keeps the key stored for its address.
#[derive(Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BastionKeyInput {
    pub index: u32,
    pub private_key: String,
    pub passphrase: Option<String>,
}

/// What the store already holds for a Host being edited.
pub(super) struct Current<'a> {
    pub connection: &'a HostConnection,
    /// Its opened secrets; `None` when it has none or they can't be opened.
    pub secrets: Option<HostSecrets>,
}

/// Turns user input into what the store keeps: generates or reads SSH keys,
/// checks TLS material, and carries keys over when an update omits them.
/// Secrets of `None` mean "keep the stored ones".
pub(super) fn prepare(
    name: &str,
    engine: EngineKind,
    connection: HostConnection,
    credentials: Option<CredentialsInput>,
    bastion_keys: Vec<BastionKeyInput>,
    current: Option<Current<'_>>,
) -> AppResult<(HostConnection, Option<HostSecrets>)> {
    match connection {
        HostConnection::Tls { address } => {
            let secrets = match credentials {
                Some(CredentialsInput::Tls {
                    ca_pem,
                    cert_pem,
                    key_pem,
                }) => {
                    // An Incus Host's "CA" is its own certificate, pinned (ADR 0021).
                    if engine == EngineKind::Incus {
                        transport::tls::TlsDialer::pinned(
                            &address, &address, &ca_pem, &cert_pem, &key_pem,
                        )
                    } else {
                        transport::tls::TlsDialer::new(
                            &address, &address, &ca_pem, &cert_pem, &key_pem,
                        )
                    }
                    .map_err(invalid)?;
                    Some(HostSecrets::Tls {
                        ca_pem,
                        cert_pem,
                        key_pem,
                    })
                }
                Some(_) => return Err(AppError::validation(msg::TLS_NEEDS_CERTIFICATES)),
                None => None,
            };
            Ok((HostConnection::Tls { address }, secrets))
        }
        HostConnection::Ssh {
            host,
            port,
            user,
            socket_path,
            auth,
            bastions,
            ..
        } => {
            let current_connection = current.as_ref().map(|c| c.connection);
            let (public_key, new_host_key) =
                prepare_host_key(name, auth, credentials, current_connection)?;
            let (bastions, new_bastion_keys) =
                prepare_bastions(bastions, bastion_keys, current_connection)?;

            let secrets = if new_host_key.is_none() && new_bastion_keys.is_empty() {
                None
            } else {
                let stored = current.and_then(|c| c.secrets);
                Some(merge_ssh_secrets(
                    auth,
                    &bastions,
                    new_host_key,
                    new_bastion_keys,
                    stored,
                )?)
            };
            Ok((
                HostConnection::Ssh {
                    host,
                    port,
                    user,
                    socket_path,
                    auth,
                    public_key,
                    bastions,
                },
                secrets,
            ))
        }
        other => Ok((other, None)),
    }
}

/// The transport's own message, as a plain validation failure.
fn invalid(e: ConnectError) -> AppError {
    AppError {
        code: ErrorCode::Validation,
        hint: None,
        detail: None,
        ..AppError::from(e)
    }
}

/// Reads an imported key, returning its public half and the secret to keep.
fn read_key(private_key: String, passphrase: Option<String>) -> AppResult<(String, SshKeySecret)> {
    let passphrase = passphrase.filter(|p| !p.is_empty());
    let public_key =
        transport::ssh::public_key_of(&private_key, passphrase.as_deref()).map_err(invalid)?;
    Ok((
        public_key,
        SshKeySecret {
            private_key,
            passphrase,
        },
    ))
}

/// The Host's own key: generated, imported, or kept. Returns its public half
/// and the new secret, if there is one.
fn prepare_host_key(
    name: &str,
    auth: SshAuth,
    credentials: Option<CredentialsInput>,
    current: Option<&HostConnection>,
) -> AppResult<(Option<String>, Option<SshKeySecret>)> {
    let kept_public_key = match current {
        Some(HostConnection::Ssh {
            auth: current_auth,
            public_key,
            ..
        }) if *current_auth == auth => public_key.clone(),
        _ => None,
    };
    match (auth, credentials) {
        (SshAuth::Agent, _) => Ok((None, None)),
        (
            _,
            Some(CredentialsInput::SshKey {
                private_key,
                passphrase,
            }),
        ) => {
            let (public_key, secret) = read_key(private_key, passphrase)?;
            Ok((Some(public_key), Some(secret)))
        }
        (SshAuth::Generated, None) if kept_public_key.is_none() => {
            let key = transport::ssh::generate_key(&key_comment(name));
            Ok((
                Some(key.public_key),
                Some(SshKeySecret {
                    private_key: key.private_key,
                    passphrase: None,
                }),
            ))
        }
        (_, None) => Ok((kept_public_key, None)),
        (_, Some(_)) => Err(AppError::validation(msg::SSH_NEEDS_PRIVATE_KEY)),
    }
}

/// The bastions as the store keeps them, with the keys typed for them by
/// address. A bastion whose key isn't retyped keeps the stored one, which
/// only exists if it already used its own key at the same address.
fn prepare_bastions(
    mut bastions: Vec<Bastion>,
    keys: Vec<BastionKeyInput>,
    current: Option<&HostConnection>,
) -> AppResult<(Vec<Bastion>, BTreeMap<String, SshKeySecret>)> {
    let stored_public_key = |address: &str| match current {
        Some(HostConnection::Ssh { bastions, .. }) => bastions
            .iter()
            .find(|b| b.address() == address && b.auth == BastionAuth::Imported)
            .and_then(|b| b.public_key.clone()),
        _ => None,
    };
    let mut typed: BTreeMap<usize, BastionKeyInput> =
        keys.into_iter().map(|k| (k.index as usize, k)).collect();
    let mut secrets = BTreeMap::new();
    for (i, bastion) in bastions.iter_mut().enumerate() {
        // Trusted keys are Homeport's to set, never the form's.
        bastion.host_key = None;
        let address = bastion.address();
        bastion.public_key = match (bastion.auth, typed.remove(&i)) {
            (BastionAuth::Imported, Some(key)) => {
                let (public_key, secret) = read_key(key.private_key, key.passphrase)?;
                secrets.insert(address, secret);
                Some(public_key)
            }
            (BastionAuth::Imported, None) => match stored_public_key(&address) {
                Some(public_key) => Some(public_key),
                None => {
                    return Err(AppError::validation(
                        msg::BASTION_NEEDS_KEY.with("n", i + 1),
                    ))
                }
            },
            (_, _) => None,
        };
    }
    Ok((bastions, secrets))
}

/// The full SSH secrets once something changed: the new keys, and the stored
/// ones for whatever wasn't retyped. Keys of bastions no longer used are dropped.
fn merge_ssh_secrets(
    auth: SshAuth,
    bastions: &[Bastion],
    host_key: Option<SshKeySecret>,
    mut bastion_keys: BTreeMap<String, SshKeySecret>,
    stored: Option<HostSecrets>,
) -> AppResult<HostSecrets> {
    let (stored_key, stored_passphrase, mut stored_bastions) = match stored {
        Some(HostSecrets::Ssh {
            private_key,
            passphrase,
            bastions,
        }) => (private_key, passphrase, bastions),
        _ => (None, None, BTreeMap::new()),
    };
    let (private_key, passphrase) = match (auth, host_key) {
        (SshAuth::Agent, _) => (None, None),
        (_, Some(key)) => (Some(key.private_key), key.passphrase),
        (_, None) => match stored_key {
            Some(key) => (Some(key), stored_passphrase),
            None => return Err(AppError::validation(msg::NEEDS_CREDENTIALS)),
        },
    };
    for (i, bastion) in bastions.iter().enumerate() {
        let address = bastion.address();
        if bastion.auth != BastionAuth::Imported || bastion_keys.contains_key(&address) {
            continue;
        }
        match stored_bastions.remove(&address) {
            Some(secret) => {
                bastion_keys.insert(address, secret);
            }
            None => {
                return Err(AppError::validation(
                    msg::BASTION_NEEDS_KEY.with("n", i + 1),
                ))
            }
        }
    }
    Ok(HostSecrets::Ssh {
        private_key,
        passphrase,
        bastions: bastion_keys,
    })
}
