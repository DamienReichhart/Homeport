//! Turning the credentials a user typed into what the store keeps: generated
//! or imported SSH keys, and TLS material checked before it's saved.

use serde::Deserialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;
use homeport_transport::{self as transport, ConnectError};

use crate::{HostConnection, HostSecrets, SshAuth};

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

/// Turns user input into what the store keeps: generates or reads SSH keys,
/// checks TLS material, and carries keys over when an update omits them.
pub(super) fn prepare(
    name: &str,
    connection: HostConnection,
    credentials: Option<CredentialsInput>,
    current: Option<&HostConnection>,
) -> AppResult<(HostConnection, Option<HostSecrets>)> {
    // The transport's own message, as a plain validation failure.
    let invalid = |e: ConnectError| AppError {
        code: ErrorCode::Validation,
        hint: None,
        detail: None,
        ..AppError::from(e)
    };
    match connection {
        HostConnection::Tls { address } => {
            let secrets = match credentials {
                Some(CredentialsInput::Tls {
                    ca_pem,
                    cert_pem,
                    key_pem,
                }) => {
                    transport::tls::TlsDialer::new(
                        &address, &address, &ca_pem, &cert_pem, &key_pem,
                    )
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
            ..
        } => {
            let kept_public_key = match current {
                Some(HostConnection::Ssh {
                    auth: current_auth,
                    public_key,
                    ..
                }) if *current_auth == auth => public_key.clone(),
                _ => None,
            };
            let (public_key, secrets) = match (auth, credentials) {
                (SshAuth::Agent, _) => (None, None),
                (
                    _,
                    Some(CredentialsInput::SshKey {
                        private_key,
                        passphrase,
                    }),
                ) => {
                    let passphrase = passphrase.filter(|p| !p.is_empty());
                    let public_key =
                        transport::ssh::public_key_of(&private_key, passphrase.as_deref())
                            .map_err(invalid)?;
                    (
                        Some(public_key),
                        Some(HostSecrets::Ssh {
                            private_key,
                            passphrase,
                        }),
                    )
                }
                (SshAuth::Generated, None) if kept_public_key.is_none() => {
                    let key = transport::ssh::generate_key(&key_comment(name));
                    (
                        Some(key.public_key),
                        Some(HostSecrets::Ssh {
                            private_key: key.private_key,
                            passphrase: None,
                        }),
                    )
                }
                (_, None) => (kept_public_key, None),
                (_, Some(_)) => return Err(AppError::validation(msg::SSH_NEEDS_PRIVATE_KEY)),
            };
            Ok((
                HostConnection::Ssh {
                    host,
                    port,
                    user,
                    socket_path,
                    auth,
                    public_key,
                },
                secrets,
            ))
        }
        other => Ok((other, None)),
    }
}
