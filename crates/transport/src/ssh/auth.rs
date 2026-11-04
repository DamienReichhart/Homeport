//! Logging in: with a stored private key or the keys of the running ssh-agent.

use std::sync::Arc;

use russh::{
    client::Handle,
    keys::{decode_secret_key, PrivateKey},
};

use homeport_core::{
    error::{AppError, ErrorCode},
    host::{HostSecrets, SshAuth},
};

use super::{
    dialer::SshTarget,
    host_key::ServerKeyCheck,
    keys::{authorized_keys_command, key_unreadable},
};
use crate::{messages as msg, reach, ConnectError};

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
                .map_err(key_unreadable),
            _ => Err(ConnectError::Invalid(
                AppError::new(ErrorCode::SshKeyInvalid, msg::SSH_KEY_MISSING)
                    .with_hint(msg::SSH_KEY_MISSING_HINT),
            )),
        }
    }
}

pub(super) fn auth_refused(t: &SshTarget) -> AppError {
    let error = AppError::new(
        ErrorCode::SshAuthFailed,
        msg::SSH_AUTH_REFUSED
            .with("user", &t.user)
            .with("host", &t.host),
    );
    match &t.credentials {
        Credentials::Key(key) => match key.public_key().to_openssh() {
            Ok(public_key) => error
                .with_hint(
                    msg::SSH_AUTH_REFUSED_KEY_HINT
                        .with("user", &t.user)
                        .with("host", &t.host),
                )
                .with_command(authorized_keys_command(&public_key)),
            Err(_) => error.with_hint(msg::SSH_AUTH_REFUSED_PUBLIC_KEY_HINT.with("host", &t.host)),
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
