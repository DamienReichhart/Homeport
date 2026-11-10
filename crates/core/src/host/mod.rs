//! What a Host is: an Engine Homeport connects to, and how to reach it (see
//! `GLOSSARY.md`). Shared by the crates that store, reach and supervise Hosts.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult, Message};
use crate::messages as msg;

pub type HostId = String;

/// The label of the helper containers Homeport runs on a Host to open
/// volumes (ADR 0015). Containers with it, and what they do, are hidden.
pub const HELPER_LABEL: &str = "com.homeport.helper";

/// The kind of Engine a Host points to, which decides what Homeport can
/// show and do on it. Named `engine` everywhere: `kind` is the connection's.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type,
)]
#[serde(rename_all = "lowercase")]
pub enum EngineKind {
    /// An Engine speaking the Docker Engine API: Docker, or Podman through
    /// its compatible API. Which one is read on every connect.
    #[default]
    Docker,
    /// containerd, through its own gRPC API (ADR 0020).
    Containerd,
    /// Incus, through its REST API (ADR 0021).
    Incus,
}

impl EngineKind {
    /// Its stored and serialized name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Docker => "docker",
            Self::Containerd => "containerd",
            Self::Incus => "incus",
        }
    }

    /// Its name for the user, in messages.
    pub fn label(self) -> &'static str {
        match self {
            Self::Docker => "a Docker-compatible Engine",
            Self::Containerd => "containerd",
            Self::Incus => "Incus",
        }
    }

    /// Reads a stored name; `None` for one this version doesn't know.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "docker" => Some(Self::Docker),
            "containerd" => Some(Self::Containerd),
            "incus" => Some(Self::Incus),
            _ => None,
        }
    }

    /// Refuses connections a kind of Engine isn't served over: containerd
    /// only answers on its socket, on this computer or through SSH, and
    /// Incus on its socket or over HTTPS.
    pub fn validate(self, connection: &HostConnection) -> AppResult<()> {
        match (self, connection) {
            (Self::Docker, _) => Ok(()),
            (
                Self::Containerd | Self::Incus,
                HostConnection::Local { .. } | HostConnection::Ssh { .. },
            ) => Ok(()),
            (Self::Incus, HostConnection::Tls { .. }) => Ok(()),
            (Self::Containerd, _) => Err(AppError::validation(msg::CONTAINERD_NEEDS_SOCKET)),
            (Self::Incus, _) => Err(AppError::validation(msg::INCUS_NEEDS_SOCKET)),
        }
    }
}

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
        /// The servers to go through first, in order (ADR 0012). Empty for a
        /// direct connection; migration 0005 added it to stored Hosts.
        bastions: Vec<Bastion>,
    },
}

/// An SSH server an SSH Host is reached through (`ProxyJump`, ADR 0012).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Bastion {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub auth: BastionAuth,
    /// OpenSSH public key of an `imported` key, shown to the user.
    pub public_key: Option<String>,
    /// The bastion's server key, trusted on first use. Kept by Homeport:
    /// whatever the frontend sends here is replaced by the stored one.
    pub host_key: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum BastionAuth {
    /// The Host's own key or agent.
    SameAsHost,
    /// A private key of its own, sealed with the Host's secrets.
    Imported,
    /// Keys offered by the running ssh-agent.
    Agent,
}

impl Bastion {
    /// `host:port`, which tells the bastions of a Host apart.
    pub fn address(&self) -> String {
        ssh_address(&self.host, self.port)
    }
}

/// `host:port` of an SSH server, with IPv6 addresses in brackets.
pub fn ssh_address(host: &str, port: u16) -> String {
    if host.contains(':') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

/// A private key and its passphrase, as typed or imported.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SshKeySecret {
    pub private_key: String,
    pub passphrase: Option<String>,
}

impl std::fmt::Debug for SshKeySecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SshKeySecret(..)")
    }
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
        /// The Host's own key; `None` when it uses ssh-agent and only its
        /// bastions have keys.
        private_key: Option<String>,
        passphrase: Option<String>,
        /// The keys of `imported` bastions, by [`Bastion::address`].
        #[serde(default)]
        bastions: BTreeMap<String, SshKeySecret>,
    },
}

impl HostSecrets {
    /// SSH secrets holding only the Host's own key.
    pub fn ssh_key(private_key: String, passphrase: Option<String>) -> Self {
        Self::Ssh {
            private_key: Some(private_key),
            passphrase,
            bastions: BTreeMap::new(),
        }
    }
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
    /// The kind of Engine it points to; migration 0017 made stored Hosts Docker-compatible.
    pub engine: EngineKind,
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
            Self::Ssh {
                host,
                port,
                bastions,
                ..
            } => validate_bastions(&ssh_address(host, *port), bastions),
            _ => Ok(()),
        }
    }

    /// Whether this kind of connection needs [`HostSecrets`].
    pub fn needs_secrets(&self) -> bool {
        match self {
            Self::Tls { .. } => true,
            Self::Ssh { auth, bastions, .. } => {
                *auth != SshAuth::Agent || bastions.iter().any(|b| b.auth == BastionAuth::Imported)
            }
            Self::Local { .. } | Self::Tcp { .. } => false,
        }
    }
}

/// Each bastion needs a server and a user, and no two hops may share an
/// address: trusted keys and bastion keys are found by it.
fn validate_bastions(host_address: &str, bastions: &[Bastion]) -> AppResult<()> {
    let mut seen = std::collections::HashSet::from([host_address.to_owned()]);
    for (i, b) in bastions.iter().enumerate() {
        let n = i + 1;
        let invalid = |message: Message| Err(AppError::validation(message.with("n", n)));
        if b.host.trim().is_empty() {
            return invalid(msg::BASTION_HOST_MISSING);
        }
        if b.user.trim().is_empty() {
            return invalid(msg::BASTION_USER_MISSING);
        }
        if b.port == 0 {
            return invalid(msg::BASTION_PORT_INVALID);
        }
        if !seen.insert(b.address()) {
            return Err(AppError::validation(
                msg::BASTION_REPEATED.with("address", b.address()),
            ));
        }
    }
    Ok(())
}

fn is_host_port(address: &str) -> bool {
    match address.rsplit_once(':') {
        Some((host, port)) => !host.is_empty() && port.parse::<u16>().is_ok_and(|p| p > 0),
        None => false,
    }
}

#[cfg(test)]
mod tests;
