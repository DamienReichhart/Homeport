//! SSH Hosts: key handling and streams to the remote Engine socket over
//! `direct-streamlocal` channels (ADR 0002). Private keys stay in memory.

use std::{io, sync::Arc, time::Duration};

use russh::{
    client::{self, Handle},
    keys::{
        decode_secret_key,
        ssh_key::{
            private::{Ed25519Keypair, KeypairData},
            LineEnding,
        },
        HashAlg, PrivateKey, PrivateKeyWithHashAlg, PublicKey, PublicKeyOrCertificate,
    },
};
use tokio::sync::Mutex;

use super::{BoxFuture, BoxStream, ConnectError, Dialer, CONNECT_TIMEOUT};
use crate::services::hosts::{HostSecrets, SshAuth};

/// A freshly generated Ed25519 keypair in OpenSSH formats.
pub struct GeneratedKey {
    pub private_key: String,
    pub public_key: String,
}

/// Generates the per-Host keypair. The comment is reduced to characters that
/// are safe inside the single-quoted `authorized_keys` command.
pub fn generate_key(comment: &str) -> GeneratedKey {
    let mut seed = [0u8; 32];
    getrandom::fill(&mut seed).expect("OS random number generator failed");
    let comment: String = comment
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || "@._-".contains(c) {
                c
            } else {
                '-'
            }
        })
        .collect();
    let key = PrivateKey::new(
        KeypairData::Ed25519(Ed25519Keypair::from_seed(&seed)),
        comment,
    )
    .expect("an Ed25519 keypair is always valid");
    GeneratedKey {
        private_key: key
            .to_openssh(LineEnding::LF)
            .expect("Ed25519 keys always encode")
            .to_string(),
        public_key: key
            .public_key()
            .to_openssh()
            .expect("Ed25519 keys always encode"),
    }
}

/// Reads a user-supplied private key and returns its public half in OpenSSH format.
pub fn public_key_of(private_key: &str, passphrase: Option<&str>) -> Result<String, ConnectError> {
    let key = decode_secret_key(private_key, passphrase)
        .map_err(|e| ConnectError::Invalid(format!("The private key can't be read: {e}")))?;
    key.public_key()
        .to_openssh()
        .map_err(|e| ConnectError::Invalid(e.to_string()))
}

/// The one-liner a user pastes on the server to authorize `public_key`.
pub fn authorized_keys_command(public_key: &str) -> String {
    format!(
        "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo '{}' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys",
        public_key.trim()
    )
}

/// `SHA256:…` fingerprint of an OpenSSH public key, as `ssh-keygen -l` shows it.
pub fn fingerprint(openssh_public_key: &str) -> Option<String> {
    PublicKey::from_openssh(openssh_public_key)
        .ok()
        .map(|k| k.fingerprint(HashAlg::Sha256).to_string())
}

/// How a server key compares to the one trusted on first use.
#[derive(Debug, PartialEq, Eq)]
pub enum HostKeyCheck {
    Trusted,
    Unknown,
    Mismatch,
}

pub fn check_host_key(trusted: Option<&str>, presented: &PublicKey) -> HostKeyCheck {
    match trusted.map(PublicKey::from_openssh) {
        None => HostKeyCheck::Unknown,
        Some(Ok(trusted)) if trusted.key_data() == presented.key_data() => HostKeyCheck::Trusted,
        Some(_) => HostKeyCheck::Mismatch,
    }
}

pub enum Credentials {
    Key(Arc<PrivateKey>),
    Agent,
}

impl Credentials {
    pub fn from_secrets(auth: SshAuth, secrets: Option<HostSecrets>) -> Result<Self, ConnectError> {
        match (auth, secrets) {
            (SshAuth::Agent, _) => Ok(Self::Agent),
            (
                _,
                Some(HostSecrets::Ssh {
                    private_key,
                    passphrase,
                }),
            ) => decode_secret_key(&private_key, passphrase.as_deref())
                .map(|k| Self::Key(Arc::new(k)))
                .map_err(|e| ConnectError::Invalid(format!("The private key can't be read: {e}"))),
            _ => Err(ConnectError::Invalid(
                "The SSH private key is missing.".into(),
            )),
        }
    }
}

pub struct SshTarget {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub socket_path: String,
    pub credentials: Credentials,
    pub trusted_host_key: Option<String>,
}

/// Keeps one SSH session per Host and opens a channel per Engine connection.
pub struct SshDialer {
    target: SshTarget,
    session: Mutex<Option<Handle<ServerKeyCheck>>>,
}

impl SshDialer {
    pub fn new(target: SshTarget) -> Self {
        Self {
            target,
            session: Mutex::new(None),
        }
    }

    async fn open_session(&self) -> Result<Handle<ServerKeyCheck>, ConnectError> {
        let t = &self.target;
        let config = Arc::new(client::Config {
            keepalive_interval: Some(Duration::from_secs(15)),
            keepalive_max: 3,
            ..Default::default()
        });
        let presented = Arc::new(std::sync::Mutex::new(None));
        let handler = ServerKeyCheck {
            trusted: t.trusted_host_key.clone(),
            presented: presented.clone(),
        };
        let connecting = client::connect(config, (t.host.as_str(), t.port), handler);
        let result = tokio::time::timeout(CONNECT_TIMEOUT, connecting)
            .await
            .map_err(|_| ConnectError::Unreachable("SSH connection timed out".into()))?;

        let mut session = match result {
            Ok(session) => session,
            Err(e) => {
                let presented = presented.lock().expect("lock poisoned").take();
                return Err(match presented {
                    Some(key) => host_key_error(t.trusted_host_key.as_deref(), &key),
                    None => ConnectError::Unreachable(e.to_string()),
                });
            }
        };

        let authenticated = match &t.credentials {
            Credentials::Key(key) => {
                let hash = session
                    .best_supported_rsa_hash()
                    .await
                    .map_err(|e| ConnectError::Unreachable(e.to_string()))?
                    .flatten();
                session
                    .authenticate_publickey(&t.user, PrivateKeyWithHashAlg::new(key.clone(), hash))
                    .await
                    .map_err(|e| ConnectError::Unreachable(e.to_string()))?
                    .success()
            }
            Credentials::Agent => authenticate_with_agent(&mut session, &t.user).await?,
        };
        if !authenticated {
            return Err(ConnectError::Unauthorized(format!(
                "The server refused the key for {}@{}. Add the public key to its authorized_keys.",
                t.user, t.host
            )));
        }
        Ok(session)
    }

    async fn open_channel(&self) -> Result<BoxStream, ConnectError> {
        let mut session = self.session.lock().await;
        if session.as_ref().is_none_or(|s| s.is_closed()) {
            *session = Some(self.open_session().await?);
        }
        let channel = session
            .as_ref()
            .expect("session was just opened")
            .channel_open_direct_streamlocal(self.target.socket_path.as_str())
            .await
            .map_err(|e| {
                ConnectError::Unreachable(format!(
                    "The SSH server couldn't open {}: {e}. Check the path and that {} can use Docker.",
                    self.target.socket_path, self.target.user
                ))
            })?;
        Ok(Box::new(channel.into_stream()))
    }
}

impl Dialer for SshDialer {
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
        Box::pin(async move { self.open_channel().await.map(drop) })
    }

    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
        Box::pin(async move {
            self.open_channel()
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::ConnectionRefused, e.to_string()))
        })
    }
}

fn host_key_error(trusted: Option<&str>, presented: &PublicKey) -> ConnectError {
    let key = presented.to_openssh().unwrap_or_default();
    let fingerprint = presented.fingerprint(HashAlg::Sha256).to_string();
    match trusted.and_then(fingerprint_of_trusted) {
        Some(expected_fingerprint) => ConnectError::HostKeyMismatch {
            key,
            fingerprint,
            expected_fingerprint,
        },
        None => ConnectError::HostKeyUntrusted { key, fingerprint },
    }
}

fn fingerprint_of_trusted(trusted: &str) -> Option<String> {
    fingerprint(trusted)
}

#[cfg(unix)]
async fn authenticate_with_agent(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
) -> Result<bool, ConnectError> {
    let agent = russh::keys::agent::client::AgentClient::connect_env()
        .await
        .map_err(|e| ConnectError::Unauthorized(format!("No ssh-agent is reachable: {e}")))?;
    try_agent_identities(session, user, agent).await
}

#[cfg(windows)]
async fn authenticate_with_agent(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
) -> Result<bool, ConnectError> {
    let agent =
        russh::keys::agent::client::AgentClient::connect_named_pipe(r"\\.\pipe\openssh-ssh-agent")
            .await
            .map_err(|e| ConnectError::Unauthorized(format!("No ssh-agent is reachable: {e}")))?;
    try_agent_identities(session, user, agent).await
}

async fn try_agent_identities<S>(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
    mut agent: russh::keys::agent::client::AgentClient<S>,
) -> Result<bool, ConnectError>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    let identities = agent
        .request_identities()
        .await
        .map_err(|e| ConnectError::Unauthorized(format!("ssh-agent failed: {e}")))?;
    let hash = session
        .best_supported_rsa_hash()
        .await
        .map_err(|e| ConnectError::Unreachable(e.to_string()))?
        .flatten();
    for identity in identities {
        let key = identity.public_key().into_owned();
        let result = session
            .authenticate_publickey_with(user, key, hash, &mut agent)
            .await
            .map_err(|e| ConnectError::Unauthorized(format!("ssh-agent failed: {e}")))?;
        if result.success() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Accepts only the trusted server key, remembering what was presented so a
/// rejection can be reported as "untrusted" or "changed".
pub struct ServerKeyCheck {
    trusted: Option<String>,
    presented: Arc<std::sync::Mutex<Option<PublicKey>>>,
}

impl client::Handler for ServerKeyCheck {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        let key = match server_public_key {
            PublicKeyOrCertificate::PublicKey { key, .. } => key.clone(),
            PublicKeyOrCertificate::Certificate(cert) => {
                PublicKey::new(cert.public_key().clone(), "")
            }
        };
        let trusted = check_host_key(self.trusted.as_deref(), &key) == HostKeyCheck::Trusted;
        if !trusted {
            *self.presented.lock().expect("lock poisoned") = Some(key);
        }
        Ok(trusted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_keys_are_ed25519_with_a_safe_comment() {
        let key = generate_key("homeport@my laptop-'prod'");
        assert!(key.public_key.starts_with("ssh-ed25519 "));
        assert!(key.public_key.ends_with(" homeport@my-laptop--prod-"));
        assert!(key.private_key.contains("BEGIN OPENSSH PRIVATE KEY"));
        assert_eq!(
            public_key_of(&key.private_key, None).unwrap(),
            key.public_key
        );
    }

    #[test]
    fn each_generated_key_is_unique() {
        assert_ne!(generate_key("a").public_key, generate_key("a").public_key);
    }

    #[test]
    fn authorized_keys_command_appends_the_key_with_safe_permissions() {
        assert_eq!(
            authorized_keys_command("ssh-ed25519 AAAAC3Nza homeport@laptop\n"),
            "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo 'ssh-ed25519 AAAAC3Nza homeport@laptop' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys"
        );
    }

    #[test]
    fn unreadable_private_keys_are_rejected() {
        assert!(matches!(
            public_key_of("not a key", None),
            Err(ConnectError::Invalid(_))
        ));
    }

    #[test]
    fn host_keys_are_trusted_only_when_they_match() {
        let a = generate_key("a").public_key;
        let b = generate_key("b").public_key;
        let presented = PublicKey::from_openssh(&a).unwrap();
        assert_eq!(check_host_key(None, &presented), HostKeyCheck::Unknown);
        assert_eq!(check_host_key(Some(&a), &presented), HostKeyCheck::Trusted);
        assert_eq!(check_host_key(Some(&b), &presented), HostKeyCheck::Mismatch);
    }

    #[test]
    fn fingerprints_use_sha256() {
        let fp = fingerprint(&generate_key("a").public_key).unwrap();
        assert!(fp.starts_with("SHA256:"));
        assert_eq!(fingerprint("garbage"), None);
    }
}
