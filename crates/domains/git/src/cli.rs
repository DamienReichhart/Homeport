//! Finding the `git` program and running it with a [`Session`]'s isolated
//! configuration and credentials.

use std::{path::Path, process::Stdio, time::Duration};

use serde::Serialize;
use tokio::process::Command;

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    programs,
};

use crate::{
    access::{Remote, Session},
    error::{self, Context},
    messages as msg,
};

/// The `git` program found on this computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GitCli {
    pub program: String,
    /// As `git --version` prints it, without the `git version ` prefix.
    pub version: String,
}

/// How long a command that reaches a server may take.
const NETWORK_TIMEOUT: Duration = Duration::from_secs(600);
/// How long a command on this machine may take.
const LOCAL_TIMEOUT: Duration = Duration::from_secs(120);

/// Finds `git`, and says how to install it when it isn't there.
pub async fn detect() -> AppResult<GitCli> {
    let missing = || {
        let (hint, command) = if cfg!(target_os = "macos") {
            (msg::INSTALL_MACOS_HINT, "xcode-select --install")
        } else if cfg!(windows) {
            (msg::INSTALL_WINDOWS_HINT, "winget install --id Git.Git -e")
        } else {
            (msg::INSTALL_LINUX_HINT, "sudo apt-get install git")
        };
        AppError::new(ErrorCode::GitMissing, msg::MISSING)
            .with_hint(hint)
            .with_command(command)
    };
    let program = programs::find("git").ok_or_else(missing)?;
    let out = Command::new(&program)
        .arg("--version")
        .env("PATH", programs::search_path())
        .env("LC_ALL", "C")
        .stdin(Stdio::null())
        .output()
        .await
        .map_err(|_| missing())?;
    let text = String::from_utf8_lossy(&out.stdout);
    let version = text.trim().trim_start_matches("git version ").to_owned();
    if !out.status.success() || version.is_empty() {
        // Apple's stub asks to install the command line tools instead.
        return Err(missing());
    }
    Ok(GitCli {
        program: program.to_string_lossy().into_owned(),
        version,
    })
}

/// What a command that succeeded printed.
pub(crate) struct Output {
    pub(crate) stdout: Vec<u8>,
}

impl Output {
    pub(crate) fn text(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }
}

/// Why a command failed: before it ran, or what it printed when it did.
pub(crate) enum Failure {
    Error(AppError),
    Exited { stderr: String },
}

impl GitCli {
    /// Runs `git <args>` on this machine, in `cwd`.
    pub(crate) async fn local(&self, cwd: &Path, args: &[&str]) -> AppResult<Output> {
        let session = Session::local()?;
        self.output(&session, cwd, args, LOCAL_TIMEOUT)
            .await
            .map_err(|f| match f {
                Failure::Error(e) => e,
                Failure::Exited { stderr } => error::failure(args[0], &stderr, None),
            })
    }

    /// Runs `git <args>` on this machine, and says how it failed.
    pub(crate) async fn local_raw(&self, cwd: &Path, args: &[&str]) -> Result<Output, Failure> {
        let session = Session::local().map_err(Failure::Error)?;
        self.output(&session, cwd, args, LOCAL_TIMEOUT).await
    }

    /// Runs `git <args>` reaching `remote`, in `cwd`. Returns what it printed
    /// and the server key it trusted for the first time, if any; a failure
    /// is classified from what git printed (see [`error::failure`]).
    pub(crate) async fn remote(
        &self,
        remote: &Remote<'_>,
        cwd: &Path,
        args: &[&str],
    ) -> AppResult<(Output, Option<String>)> {
        let session = Session::remote(remote).await?;
        match self.output(&session, cwd, args, NETWORK_TIMEOUT).await {
            Ok(out) => Ok((out, session.learned_host_key())),
            Err(Failure::Error(error)) => Err(error),
            Err(Failure::Exited { stderr }) => Err(error::failure(
                args[0],
                &stderr,
                Some(&Context {
                    url: remote.url,
                    secret: remote.secret,
                }),
            )),
        }
    }

    async fn output(
        &self,
        session: &Session,
        cwd: &Path,
        args: &[&str],
        timeout: Duration,
    ) -> Result<Output, Failure> {
        let mut command = Command::new(&self.program);
        command
            .args(args)
            .current_dir(cwd)
            .env("PATH", programs::search_path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        session.apply(&mut command);
        let child = command.spawn().map_err(|e| {
            Failure::Error(
                AppError::new(ErrorCode::GitMissing, msg::SPAWN_FAILED)
                    .with_hint(msg::SPAWN_FAILED_HINT)
                    .with_cause(&e),
            )
        })?;
        let out = match tokio::time::timeout(timeout, child.wait_with_output()).await {
            Ok(out) => out.map_err(|e| Failure::Error(e.into()))?,
            Err(_) => {
                return Err(Failure::Exited {
                    stderr: "fatal: operation timed out".into(),
                })
            }
        };
        if out.status.success() {
            Ok(Output { stdout: out.stdout })
        } else {
            Err(Failure::Exited {
                stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn git_is_found_with_its_version() {
        // CI runners and developer machines have git.
        let cli = detect().await.unwrap();
        assert!(cli.version.chars().next().unwrap().is_ascii_digit());
    }
}
