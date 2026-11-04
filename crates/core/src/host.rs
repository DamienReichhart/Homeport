//! What a Host is: an Engine Homeport connects to, and how to reach it (see
//! `GLOSSARY.md`). Shared by the crates that store, reach and supervise Hosts.

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult, Message};
use crate::messages as msg;

pub type HostId = String;

/// How to reach a Host. Never contains secret material; that lives in [`HostSecrets`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(
    tag = "kind",
    rename_all = "lowercase",
    rename_all_fields = "camelCase"
)]
pub enum HostConnection {
    /// A Unix socket or Windows named pipe on this machine.
    Local { socket_path: String },
    /// Plain, unauthenticated TCP (`host:port`).
    Tcp { address: String },
    /// TCP with mutual TLS (`host:port`); certificates are in [`HostSecrets::Tls`].
    Tls { address: String },
    /// The remote Engine socket, reached through SSH.
    Ssh {
        host: String,
        port: u16,
        user: String,
        socket_path: String,
        auth: SshAuth,
        /// OpenSSH public key for `generated` and `imported` auth, shown to the user.
        public_key: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum SshAuth {
    /// A keypair Homeport generated for this Host.
    Generated,
    /// A private key the user supplied.
    Imported,
    /// Keys offered by the running ssh-agent.
    Agent,
}

/// Secret material for a Host, stored sealed and never sent to the frontend.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum HostSecrets {
    Tls {
        ca_pem: String,
        cert_pem: String,
        key_pem: String,
    },
    Ssh {
        private_key: String,
        passphrase: Option<String>,
    },
}

impl std::fmt::Debug for HostSecrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HostSecrets(..)")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: HostId,
    pub name: String,
    pub connection: HostConnection,
    pub enabled: bool,
    /// The SSH server key trusted on first use, in OpenSSH format.
    pub host_key: Option<String>,
    pub created_at: String,
}

impl HostConnection {
    /// Rejects settings that can never connect, with a message fit for the user.
    pub fn validate(&self) -> AppResult<()> {
        let invalid = |message: Message| Err(AppError::validation(message));
        match self {
            Self::Local { socket_path } if socket_path.trim().is_empty() => {
                invalid(msg::SOCKET_PATH_MISSING)
            }
            Self::Tcp { address } | Self::Tls { address } if !is_host_port(address) => {
                invalid(msg::ADDRESS_INVALID)
            }
            Self::Ssh { host, .. } if host.trim().is_empty() => invalid(msg::SSH_HOST_MISSING),
            Self::Ssh { user, .. } if user.trim().is_empty() => invalid(msg::SSH_USER_MISSING),
            Self::Ssh { port: 0, .. } => invalid(msg::SSH_PORT_INVALID),
            Self::Ssh { socket_path, .. } if !socket_path.starts_with('/') => {
                invalid(msg::REMOTE_SOCKET_NOT_ABSOLUTE)
            }
            _ => Ok(()),
        }
    }

    /// Whether this kind of connection needs [`HostSecrets`].
    pub fn needs_secrets(&self) -> bool {
        match self {
            Self::Tls { .. } => true,
            Self::Ssh { auth, .. } => *auth != SshAuth::Agent,
            Self::Local { .. } | Self::Tcp { .. } => false,
        }
    }
}

fn is_host_port(address: &str) -> bool {
    match address.rsplit_once(':') {
        Some((host, port)) => !host.is_empty() && port.parse::<u16>().is_ok_and(|p| p > 0),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorCode;

    fn ssh(host: &str, user: &str, port: u16, socket_path: &str) -> HostConnection {
        HostConnection::Ssh {
            host: host.into(),
            port,
            user: user.into(),
            socket_path: socket_path.into(),
            auth: SshAuth::Generated,
            public_key: None,
        }
    }

    #[test]
    fn accepts_well_formed_connections() {
        for c in [
            HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            HostConnection::Tcp {
                address: "10.0.0.2:2375".into(),
            },
            HostConnection::Tls {
                address: "docker.example.com:2376".into(),
            },
            ssh("server", "deploy", 22, "/var/run/docker.sock"),
        ] {
            assert!(c.validate().is_ok(), "{c:?}");
        }
    }

    #[test]
    fn rejects_unusable_connections() {
        for c in [
            HostConnection::Local {
                socket_path: " ".into(),
            },
            HostConnection::Tcp {
                address: "10.0.0.2".into(),
            },
            HostConnection::Tls {
                address: "host:notaport".into(),
            },
            ssh("", "deploy", 22, "/var/run/docker.sock"),
            ssh("server", "", 22, "/var/run/docker.sock"),
            ssh("server", "deploy", 0, "/var/run/docker.sock"),
            ssh("server", "deploy", 22, "docker.sock"),
        ] {
            assert!(
                matches!(c.validate(), Err(ref e) if e.is(ErrorCode::Validation)),
                "{c:?}"
            );
        }
    }

    #[test]
    fn json_shape_is_tagged_by_kind() {
        let json = serde_json::to_value(HostConnection::Tcp {
            address: "a:1".into(),
        })
        .unwrap();
        assert_eq!(json, serde_json::json!({ "kind": "tcp", "address": "a:1" }));
    }
}
