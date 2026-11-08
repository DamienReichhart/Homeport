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
    /// and Incus only answer on their socket, on this computer or through SSH.
    pub fn validate(self, connection: &HostConnection) -> AppResult<()> {
        match (self, connection) {
            (Self::Docker, _) => Ok(()),
            (
                Self::Containerd | Self::Incus,
                HostConnection::Local { .. } | HostConnection::Ssh { .. },
            ) => Ok(()),
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
            bastions: vec![],
        }
    }

    fn bastion(host: &str, port: u16, user: &str, auth: BastionAuth) -> Bastion {
        Bastion {
            host: host.into(),
            port,
            user: user.into(),
            auth,
            public_key: None,
            host_key: None,
        }
    }

    fn behind(bastions: Vec<Bastion>) -> HostConnection {
        let HostConnection::Ssh {
            host,
            port,
            user,
            socket_path,
            auth,
            public_key,
            ..
        } = ssh("server", "deploy", 22, "/var/run/docker.sock")
        else {
            unreachable!()
        };
        HostConnection::Ssh {
            host,
            port,
            user,
            socket_path,
            auth,
            public_key,
            bastions,
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
    fn bastions_need_a_server_a_user_a_port_and_their_own_address() {
        let ok = behind(vec![
            bastion("jump", 22, "me", BastionAuth::SameAsHost),
            bastion("inner", 2222, "me", BastionAuth::Agent),
        ]);
        assert!(ok.validate().is_ok());
        for (bastions, n) in [
            (vec![bastion(" ", 22, "me", BastionAuth::Agent)], "1"),
            (
                vec![
                    bastion("jump", 22, "me", BastionAuth::Agent),
                    bastion("inner", 22, "", BastionAuth::Agent),
                ],
                "2",
            ),
            (vec![bastion("jump", 0, "me", BastionAuth::Agent)], "1"),
        ] {
            let err = behind(bastions).validate().unwrap_err();
            assert!(err.is(ErrorCode::Validation));
            assert_eq!(err.params.get("n").map(|p| p.to_string()), Some(n.into()));
        }
        // The same server twice, or the Host itself as a bastion.
        for bastions in [
            vec![
                bastion("jump", 22, "a", BastionAuth::Agent),
                bastion("jump", 22, "b", BastionAuth::Agent),
            ],
            vec![bastion("server", 22, "a", BastionAuth::Agent)],
        ] {
            assert!(behind(bastions).validate().is_err());
        }
    }

    #[test]
    fn bastions_with_their_own_key_need_secrets_even_with_ssh_agent() {
        let HostConnection::Ssh {
            host,
            port,
            user,
            socket_path,
            public_key,
            ..
        } = ssh("server", "deploy", 22, "/var/run/docker.sock")
        else {
            unreachable!()
        };
        let agent = |bastions| HostConnection::Ssh {
            host: host.clone(),
            port,
            user: user.clone(),
            socket_path: socket_path.clone(),
            auth: SshAuth::Agent,
            public_key: public_key.clone(),
            bastions,
        };
        assert!(!agent(vec![]).needs_secrets());
        assert!(!agent(vec![bastion("jump", 22, "me", BastionAuth::SameAsHost)]).needs_secrets());
        assert!(agent(vec![bastion("jump", 22, "me", BastionAuth::Imported)]).needs_secrets());
    }

    #[test]
    fn secrets_stored_before_bastions_read_with_none() {
        let stored = r#"{"kind":"ssh","private_key":"KEY","passphrase":null}"#;
        let s: HostSecrets = serde_json::from_str(stored).unwrap();
        assert!(s == HostSecrets::ssh_key("KEY".into(), None));
    }

    #[test]
    fn ipv6_addresses_are_bracketed() {
        assert_eq!(ssh_address("10.0.0.1", 22), "10.0.0.1:22");
        assert_eq!(ssh_address("fd00::1", 2222), "[fd00::1]:2222");
    }

    #[test]
    fn engine_kinds_are_lowercase_names() {
        for (kind, name) in [
            (EngineKind::Docker, "docker"),
            (EngineKind::Containerd, "containerd"),
            (EngineKind::Incus, "incus"),
        ] {
            assert_eq!(serde_json::to_value(kind).unwrap(), serde_json::json!(name));
            assert_eq!(
                serde_json::from_value::<EngineKind>(name.into()).unwrap(),
                kind
            );
            assert_eq!(EngineKind::parse(name), Some(kind));
            assert_eq!(kind.as_str(), name);
        }
        assert_eq!(EngineKind::parse("lxd"), None);
        assert_eq!(EngineKind::default(), EngineKind::Docker);
    }

    #[test]
    fn every_engine_is_accepted_over_the_connections_it_answers_on() {
        let local = HostConnection::Local {
            socket_path: "/run/containerd/containerd.sock".into(),
        };
        let tcp = HostConnection::Tcp {
            address: "10.0.0.2:2375".into(),
        };
        let remote = ssh("server", "deploy", 22, "/run/containerd/containerd.sock");
        for c in [&local, &tcp, &remote] {
            assert!(EngineKind::Docker.validate(c).is_ok());
        }
        assert!(EngineKind::Containerd.validate(&local).is_ok());
        assert!(EngineKind::Containerd.validate(&remote).is_ok());
        let err = EngineKind::Containerd.validate(&tcp).unwrap_err();
        assert!(err.is(ErrorCode::Validation));
        assert!(err.message.contains("socket"), "{err:?}");
        let incus = HostConnection::Local {
            socket_path: "/var/lib/incus/unix.socket".into(),
        };
        assert!(EngineKind::Incus.validate(&incus).is_ok());
        assert!(EngineKind::Incus.validate(&remote).is_ok());
        let err = EngineKind::Incus.validate(&tcp).unwrap_err();
        assert!(err.is(ErrorCode::Validation));
        assert!(err.message.contains("Incus"), "{err:?}");
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
