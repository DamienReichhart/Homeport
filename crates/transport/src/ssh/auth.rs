//! Logging in: with a stored private key or the keys of the running ssh-agent.

use std::sync::Arc;

use russh::{
    client::Handle,
    keys::{decode_secret_key, PrivateKey},
};

use homeport_core::{
    error::{AppError, ErrorCode},
    host::{BastionAuth, HostSecrets, SshAuth, SshKeySecret},
};

use super::{
    host_key::ServerKeyCheck,
    keys::{authorized_keys_command, key_unreadable},
};
use crate::{messages as msg, reach, ConnectError};

/// How Homeport logs in to one SSH server: the Host or a bastion.
#[derive(Clone)]
pub enum Credentials {
    Key(Arc<PrivateKey>),
    Agent,
}

impl Credentials {
    /// The Host's own credentials.
    pub fn from_secrets(
        auth: SshAuth,
        secrets: Option<&HostSecrets>,
    ) -> Result<Self, ConnectError> {
        match (auth, secrets) {
            (SshAuth::Agent, _) => Ok(Self::Agent),
            (
                _,
                Some(HostSecrets::Ssh {
                    private_key: Some(private_key),
                    passphrase,
                    ..
                }),
            ) => Self::from_key(private_key, passphrase.as_deref()),
            _ => Err(key_missing()),
        }
    }

    /// A bastion's credentials: the Host's, the agent's, or its own key
    /// found in `secrets` by its address.
    pub fn for_bastion(
        auth: BastionAuth,
        address: &str,
        host: &Self,
        secrets: Option<&HostSecrets>,
    ) -> Result<Self, ConnectError> {
        match auth {
            BastionAuth::SameAsHost => Ok(host.clone()),
            BastionAuth::Agent => Ok(Self::Agent),
            BastionAuth::Imported => match secrets {
                Some(HostSecrets::Ssh { bastions, .. }) => match bastions.get(address) {
                    Some(SshKeySecret {
                        private_key,
                        passphrase,
                    }) => Self::from_key(private_key, passphrase.as_deref()),
                    None => Err(key_missing()),
                },
                _ => Err(key_missing()),
            },
        }
    }

    fn from_key(private_key: &str, passphrase: Option<&str>) -> Result<Self, ConnectError> {
        decode_secret_key(private_key, passphrase)
            .map(|k| Self::Key(Arc::new(k)))
            .map_err(key_unreadable)
    }
}

fn key_missing() -> ConnectError {
    ConnectError::Invalid(
        AppError::new(ErrorCode::SshKeyInvalid, msg::SSH_KEY_MISSING)
            .with_hint(msg::SSH_KEY_MISSING_HINT),
    )
}

/// The server at `host` refused every key `credentials` offered for `user`.
pub(super) fn auth_refused(user: &str, host: &str, credentials: &Credentials) -> AppError {
    let error = AppError::new(
        ErrorCode::SshAuthFailed,
        msg::SSH_AUTH_REFUSED.with("user", user).with("host", host),
    );
    match credentials {
        Credentials::Key(key) => match key.public_key().to_openssh() {
            Ok(public_key) => error
                .with_hint(
                    msg::SSH_AUTH_REFUSED_KEY_HINT
                        .with("user", user)
                        .with("host", host),
                )
                .with_command(authorized_keys_command(&public_key)),
            Err(_) => error.with_hint(msg::SSH_AUTH_REFUSED_PUBLIC_KEY_HINT.with("host", host)),
        },
        Credentials::Agent => error
            .with_hint(msg::SSH_AUTH_REFUSED_AGENT_HINT)
            .with_command("ssh-add"),
    }
}

fn agent_unavailable(e: &(dyn std::error::Error + 'static)) -> AppError {
    let error = AppError::new(ErrorCode::SshAgentUnavailable, msg::SSH_AGENT_UNAVAILABLE);
    let error = if cfg!(windows) {
        error
            .with_hint(msg::SSH_AGENT_UNAVAILABLE_WINDOWS_HINT)
            .with_command("Get-Service ssh-agent | Set-Service -StartupType Automatic\nStart-Service ssh-agent")
            .with_command("ssh-add")
    } else {
        error
            .with_hint(msg::SSH_AGENT_UNAVAILABLE_UNIX_HINT)
            .with_command("eval \"$(ssh-agent -s)\"\nssh-add")
    };
    error.with_cause(e)
}

fn agent_failed(e: &(dyn std::error::Error + 'static)) -> AppError {
    AppError::new(ErrorCode::SshAgentUnavailable, msg::SSH_AGENT_FAILED)
        .with_hint(msg::SSH_AGENT_FAILED_HINT)
        .with_cause(e)
}

#[cfg(unix)]
pub(super) async fn authenticate_with_agent(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
    label: &str,
) -> Result<bool, ConnectError> {
    let agent = russh::keys::agent::client::AgentClient::connect_env()
        .await
        .map_err(|e| ConnectError::Unauthorized(agent_unavailable(&e)))?;
    try_agent_identities(session, user, label, agent).await
}

#[cfg(windows)]
pub(super) async fn authenticate_with_agent(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
    label: &str,
) -> Result<bool, ConnectError> {
    let agent =
        russh::keys::agent::client::AgentClient::connect_named_pipe(r"\\.\pipe\openssh-ssh-agent")
            .await
            .map_err(|e| ConnectError::Unauthorized(agent_unavailable(&e)))?;
    try_agent_identities(session, user, label, agent).await
}

async fn try_agent_identities<S>(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
    label: &str,
    mut agent: russh::keys::agent::client::AgentClient<S>,
) -> Result<bool, ConnectError>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    let identities = agent
        .request_identities()
        .await
        .map_err(|e| ConnectError::Unauthorized(agent_failed(&e)))?;
    let hash = session
        .best_supported_rsa_hash()
        .await
        .map_err(|e| ConnectError::from(reach::ssh(label, &e)))?
        .flatten();
    for identity in identities {
        let key = identity.public_key().into_owned();
        let result = session
            .authenticate_publickey_with(user, key, hash, &mut agent)
            .await
            .map_err(|e| ConnectError::Unauthorized(agent_failed(&e)))?;
        if result.success() {
            return Ok(true);
        }
    }
    Ok(false)
}
