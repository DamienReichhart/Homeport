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

/// How asking one agent went.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AgentOutcome {
    /// Nothing answered where it lives.
    Unavailable,
    /// It answered, but the server accepted none of its keys.
    Refused,
    Accepted,
}

/// What to do once the first agents of a list of `agents` gave `outcomes`.
#[derive(Debug, PartialEq, Eq)]
enum NextAgent {
    /// Ask the agent at this place in the list.
    Try(usize),
    /// Stop: whether a key was accepted.
    Done(bool),
    /// None of them is running.
    NoneRunning,
}

/// Asks each agent in turn until one holds a key the server accepts, so a
/// key in any of them works (on Windows: the OpenSSH agent, then Pageant).
fn next_agent(outcomes: &[AgentOutcome], agents: usize) -> NextAgent {
    if outcomes.contains(&AgentOutcome::Accepted) {
        NextAgent::Done(true)
    } else if outcomes.len() < agents {
        NextAgent::Try(outcomes.len())
    } else if outcomes.iter().all(|o| *o == AgentOutcome::Unavailable) {
        NextAgent::NoneRunning
    } else {
        NextAgent::Done(false)
    }
}

/// On macOS and Linux, the agent at `$SSH_AUTH_SOCK`.
#[cfg(unix)]
pub(super) async fn authenticate_with_agent(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
    label: &str,
) -> Result<bool, ConnectError> {
    let mut outcomes = Vec::new();
    let mut unavailable = None;
    let mut failure = None;
    loop {
        let attempt = match next_agent(&outcomes, 1) {
            NextAgent::Done(true) => return Ok(true),
            NextAgent::Done(false) => return failure.map_or(Ok(false), Err),
            NextAgent::NoneRunning => {
                let cause = unavailable.expect("an unavailable agent left its error");
                return Err(ConnectError::Unauthorized(agent_unavailable(&cause)));
            }
            NextAgent::Try(_) => match russh::keys::agent::client::AgentClient::connect_env().await
            {
                Ok(agent) => try_agent_identities(session, user, label, agent).await,
                Err(e) => {
                    unavailable = Some(e);
                    outcomes.push(AgentOutcome::Unavailable);
                    continue;
                }
            },
        };
        outcomes.push(record(attempt, &mut failure));
    }
}

/// The outcome of asking a running agent, keeping a failure to report if
/// no other agent does better.
fn record(attempt: Result<bool, ConnectError>, failure: &mut Option<ConnectError>) -> AgentOutcome {
    match attempt {
        Ok(true) => AgentOutcome::Accepted,
        Ok(false) => AgentOutcome::Refused,
        Err(e) => {
            *failure = Some(e);
            AgentOutcome::Refused
        }
    }
}

/// The Windows OpenSSH agent's pipe.
#[cfg(windows)]
const OPENSSH_AGENT_PIPE: &str = r"\\.\pipe\openssh-ssh-agent";

/// On Windows, the OpenSSH Authentication Agent first, then PuTTY's Pageant
/// (or a tool that stands in for it, like KeeAgent or gpg-agent).
#[cfg(windows)]
pub(super) async fn authenticate_with_agent(
    session: &mut Handle<ServerKeyCheck>,
    user: &str,
    label: &str,
) -> Result<bool, ConnectError> {
    use russh::keys::agent::client::AgentClient;

    let mut outcomes = Vec::new();
    let mut unavailable = None;
    let mut failure = None;
    loop {
        let attempt = match next_agent(&outcomes, 2) {
            NextAgent::Done(true) => return Ok(true),
            NextAgent::Done(false) => return failure.map_or(Ok(false), Err),
            NextAgent::NoneRunning => {
                let cause = unavailable.expect("an unavailable agent left its error");
                return Err(ConnectError::Unauthorized(agent_unavailable(&cause)));
            }
            NextAgent::Try(0) => match AgentClient::connect_named_pipe(OPENSSH_AGENT_PIPE).await {
                Ok(agent) => try_agent_identities(session, user, label, agent).await,
                Err(e) => {
                    unavailable = Some(e);
                    outcomes.push(AgentOutcome::Unavailable);
                    continue;
                }
            },
            NextAgent::Try(_) => match AgentClient::connect_pageant().await {
                Ok(agent) => try_agent_identities(session, user, label, agent).await,
                Err(e) => {
                    unavailable = Some(e);
                    outcomes.push(AgentOutcome::Unavailable);
                    continue;
                }
            },
        };
        outcomes.push(record(attempt, &mut failure));
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use AgentOutcome::*;

    #[test]
    fn the_first_agent_is_asked_first() {
        assert_eq!(next_agent(&[], 2), NextAgent::Try(0));
        assert_eq!(next_agent(&[], 1), NextAgent::Try(0));
    }

    #[test]
    fn an_accepted_key_ends_the_search() {
        assert_eq!(next_agent(&[Accepted], 2), NextAgent::Done(true));
        assert_eq!(
            next_agent(&[Unavailable, Accepted], 2),
            NextAgent::Done(true)
        );
        assert_eq!(next_agent(&[Refused, Accepted], 2), NextAgent::Done(true));
    }

    #[test]
    fn the_next_agent_is_asked_when_one_is_missing_or_holds_no_accepted_key() {
        assert_eq!(next_agent(&[Unavailable], 2), NextAgent::Try(1));
        assert_eq!(next_agent(&[Refused], 2), NextAgent::Try(1));
    }

    #[test]
    fn running_agents_without_an_accepted_key_mean_the_server_refused() {
        assert_eq!(
            next_agent(&[Refused, Unavailable], 2),
            NextAgent::Done(false)
        );
        assert_eq!(
            next_agent(&[Unavailable, Refused], 2),
            NextAgent::Done(false)
        );
        assert_eq!(next_agent(&[Refused], 1), NextAgent::Done(false));
    }

    #[test]
    fn no_agent_running_is_reported_as_such() {
        assert_eq!(
            next_agent(&[Unavailable, Unavailable], 2),
            NextAgent::NoneRunning
        );
        assert_eq!(next_agent(&[Unavailable], 1), NextAgent::NoneRunning);
    }
}
