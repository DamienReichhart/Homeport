//! What one git command is run with (ADR 0014): a configuration isolated
//! from the user's, and for a remote its credentials and the trusted server
//! key, all of which go away with the [`Session`].

use std::path::{Path, PathBuf};

use base64::Engine;
use tokio::process::Command;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{agent, messages as msg, store::Secret};

/// A repository as one command reaches it.
#[derive(Debug, Clone, Copy)]
pub struct Remote<'a> {
    pub url: &'a str,
    pub secret: &'a Secret,
    /// The trusted `known_hosts` line of an SSH server; `None` trusts the
    /// key it presents, and [`Session::learned_host_key`] says which.
    pub host_key: Option<&'a str>,
}

pub(crate) struct Session {
    dir: tempfile::TempDir,
    agent: Option<agent::Agent>,
    /// `(key, value)` pairs passed as `GIT_CONFIG_KEY_n`/`GIT_CONFIG_VALUE_n`.
    config: Vec<(String, String)>,
    ssh_command: Option<String>,
    trusts_first_key: bool,
}

/// A fresh folder only this user can enter, removed when dropped.
fn private_dir() -> std::io::Result<tempfile::TempDir> {
    let dir = tempfile::Builder::new().prefix("homeport-git-").tempdir()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))?;
    }
    Ok(dir)
}

impl Session {
    /// For a command that doesn't leave this machine.
    pub(crate) fn local() -> AppResult<Self> {
        let dir = private_dir()?;
        std::fs::write(dir.path().join("gitconfig"), "")?;
        Ok(Self {
            dir,
            agent: None,
            config: base_config(),
            ssh_command: None,
            trusts_first_key: false,
        })
    }

    /// For a command that reaches `remote`.
    pub(crate) async fn remote(remote: &Remote<'_>) -> AppResult<Self> {
        let mut session = Self::local()?;
        match remote.secret {
            Secret::None => {}
            Secret::Token { username, token } => {
                let basic =
                    base64::engine::general_purpose::STANDARD.encode(format!("{username}:{token}"));
                session.config.push((
                    "http.extraHeader".into(),
                    format!("Authorization: Basic {basic}"),
                ));
            }
            Secret::SshKey { private_key } => {
                session.agent =
                    Some(agent::start(private_key).await.map_err(|e| {
                        AppError::new(ErrorCode::Io, msg::AGENT_FAILED).with_cause(&e)
                    })?);
            }
        }
        let known_hosts = session.known_hosts();
        let line = remote.host_key.map(|l| format!("{}\n", l.trim()));
        std::fs::write(&known_hosts, line.as_deref().unwrap_or(""))?;
        session.trusts_first_key = remote.host_key.is_none();
        session.ssh_command = Some(ssh_command(&known_hosts, session.trusts_first_key));
        Ok(session)
    }

    fn known_hosts(&self) -> PathBuf {
        self.dir.path().join("known_hosts")
    }

    /// Sets the command's environment: never the user's git configuration,
    /// credential helpers, prompts or SSH agent.
    pub(crate) fn apply(&self, command: &mut Command) {
        command
            .env("GIT_CONFIG_GLOBAL", self.dir.path().join("gitconfig"))
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_CONFIG_COUNT", self.config.len().to_string())
            // English messages, so failures can be told apart.
            .env("LC_ALL", "C")
            .env("LANGUAGE", "C")
            .env_remove("GIT_ASKPASS")
            .env_remove("SSH_ASKPASS")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("SSH_AUTH_SOCK");
        for (i, (key, value)) in self.config.iter().enumerate() {
            command
                .env(format!("GIT_CONFIG_KEY_{i}"), key)
                .env(format!("GIT_CONFIG_VALUE_{i}"), value);
        }
        match &self.ssh_command {
            Some(ssh) => {
                command
                    .env("GIT_SSH_COMMAND", ssh)
                    .env("GIT_SSH_VARIANT", "ssh");
            }
            None => {
                command.env_remove("GIT_SSH_COMMAND");
            }
        }
        if let Some(agent) = &self.agent {
            command.env("SSH_AUTH_SOCK", &agent.socket);
        }
    }

    /// The `known_hosts` line `ssh` wrote for a server it saw for the first
    /// time, once the command succeeded.
    pub(crate) fn learned_host_key(&self) -> Option<String> {
        if !self.trusts_first_key {
            return None;
        }
        let text = std::fs::read_to_string(self.known_hosts()).ok()?;
        let lines: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
        (!lines.is_empty()).then(|| lines.join("\n"))
    }
}

/// Configuration every command gets, ahead of the repository's own.
fn base_config() -> Vec<(String, String)> {
    [
        // Clears every helper set elsewhere, so nothing is asked or cached.
        ("credential.helper", ""),
        ("core.askPass", ""),
        // A repository's symbolic links are checked out as plain files, so
        // a compose file can't point compose at a file elsewhere on this machine.
        ("core.symlinks", "false"),
        // Files are checked out as the repository holds them, whatever the
        // system's git does (Git for Windows turns autocrlf on): a script
        // copied into a Linux image must keep its line endings. A
        // repository's own .gitattributes still applies.
        ("core.autocrlf", "false"),
        // Gives up on a server that stops sending, instead of hanging.
        ("http.lowSpeedLimit", "1"),
        ("http.lowSpeedTime", "60"),
    ]
    .map(|(k, v)| (k.to_owned(), v.to_owned()))
    .to_vec()
}

/// The `ssh` git runs: the user's configuration and keys left out, only the
/// agent's key offered, and only the trusted server key accepted.
fn ssh_command(known_hosts: &Path, trust_first: bool) -> String {
    let known = known_hosts.to_string_lossy();
    let mut args = vec![
        ssh_program(),
        "-F".into(),
        "none".into(),
        "-o".into(),
        "IdentityFile=none".into(),
        "-o".into(),
        "BatchMode=yes".into(),
        "-o".into(),
        "ConnectTimeout=20".into(),
        "-o".into(),
        format!("UserKnownHostsFile={known}"),
        "-o".into(),
        format!("GlobalKnownHostsFile={known}"),
        "-o".into(),
        "HashKnownHosts=no".into(),
        "-o".into(),
        "UpdateHostKeys=no".into(),
        "-o".into(),
        format!(
            "StrictHostKeyChecking={}",
            if trust_first { "accept-new" } else { "yes" }
        ),
    ];
    args.iter_mut().for_each(|a| *a = shell_quote(a));
    args.join(" ")
}

/// The `ssh` program: on Windows the system's OpenSSH, whose agent can be a
/// named pipe, when it's there.
fn ssh_program() -> String {
    if cfg!(windows) {
        let system = std::env::var_os("SystemRoot")
            .map_or_else(|| PathBuf::from(r"C:\Windows"), PathBuf::from)
            .join(r"System32\OpenSSH\ssh.exe");
        if system.is_file() {
            return system.to_string_lossy().replace('\\', "/");
        }
    }
    "ssh".into()
}

/// Quotes `arg` for the POSIX shell git runs `GIT_SSH_COMMAND` with.
fn shell_quote(arg: &str) -> String {
    if !arg.is_empty()
        && arg
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_=./:,@".contains(c))
    {
        return arg.to_owned();
    }
    format!("'{}'", arg.replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_of(command: &Command) -> Vec<(String, Option<String>)> {
        command
            .as_std()
            .get_envs()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().into_owned(),
                    v.map(|v| v.to_string_lossy().into_owned()),
                )
            })
            .collect()
    }

    fn value(env: &[(String, Option<String>)], key: &str) -> Option<String> {
        env.iter()
            .find(|(k, _)| k == key)
            .and_then(|(_, v)| v.clone())
    }

    #[tokio::test]
    async fn tokens_go_in_a_header_through_the_environment_only() {
        let secret = Secret::Token {
            username: "bot".into(),
            token: "hunter2".into(),
        };
        let session = Session::remote(&Remote {
            url: "https://example.com/r.git",
            secret: &secret,
            host_key: None,
        })
        .await
        .unwrap();
        let mut command = Command::new("git");
        session.apply(&mut command);
        let env = env_of(&command);
        let n: usize = value(&env, "GIT_CONFIG_COUNT").unwrap().parse().unwrap();
        let config: Vec<(String, String)> = (0..n)
            .map(|i| {
                (
                    value(&env, &format!("GIT_CONFIG_KEY_{i}")).unwrap(),
                    value(&env, &format!("GIT_CONFIG_VALUE_{i}")).unwrap(),
                )
            })
            .collect();
        assert!(config.contains(&("credential.helper".into(), String::new())));
        assert!(config.contains(&("core.autocrlf".into(), "false".into())));
        let header = config
            .iter()
            .find(|(k, _)| k == "http.extraHeader")
            .unwrap();
        // base64("bot:hunter2")
        assert_eq!(header.1, "Authorization: Basic Ym90Omh1bnRlcjI=");
        assert_eq!(value(&env, "GIT_TERMINAL_PROMPT").as_deref(), Some("0"));
        assert!(command
            .as_std()
            .get_args()
            .all(|a| !a.to_string_lossy().contains("hunter2")));
        // The user's agent is never offered.
        assert!(env.contains(&("SSH_AUTH_SOCK".into(), None)));
        // Nothing secret is on disk.
        for entry in std::fs::read_dir(session.dir.path()).unwrap() {
            let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
            assert!(!text.contains("hunter2") && !text.contains("Ym90"));
        }
    }

    #[test]
    fn ssh_trusts_the_first_key_only_when_none_is_stored() {
        let path = Path::new("/tmp/my dir/known_hosts");
        let first = ssh_command(path, true);
        assert!(first.contains("StrictHostKeyChecking=accept-new"));
        assert!(first.contains("'UserKnownHostsFile=/tmp/my dir/known_hosts'"));
        assert!(first.contains("IdentityFile=none") && first.contains("-F none"));
        assert!(ssh_command(path, false).contains("StrictHostKeyChecking=yes"));
        assert_eq!(shell_quote("it's"), r"'it'\''s'");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn deploy_keys_are_served_by_an_agent_and_a_stored_host_key_is_written() {
        let key = homeport_transport::ssh::generate_key("t");
        let secret = Secret::SshKey {
            private_key: key.private_key.clone(),
        };
        let line = "github.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl";
        let session = Session::remote(&Remote {
            url: "git@github.com:o/r.git",
            secret: &secret,
            host_key: Some(line),
        })
        .await
        .unwrap();
        let mut command = Command::new("git");
        session.apply(&mut command);
        let env = env_of(&command);
        let socket = value(&env, "SSH_AUTH_SOCK").unwrap();
        assert!(Path::new(&socket).exists());
        assert!(value(&env, "GIT_SSH_COMMAND")
            .unwrap()
            .contains("StrictHostKeyChecking=yes"));
        assert_eq!(
            std::fs::read_to_string(session.known_hosts())
                .unwrap()
                .trim(),
            line
        );
        assert_eq!(session.learned_host_key(), None);
        for entry in std::fs::read_dir(session.dir.path()).unwrap() {
            let text = std::fs::read_to_string(entry.unwrap().path()).unwrap_or_default();
            assert!(!text.contains("PRIVATE KEY"));
        }
    }
}
