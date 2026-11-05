//! Why the server won't open its Engine socket: it asks the server, since a
//! refused channel doesn't say.

use std::time::Duration;

use russh::client::Handle;

use homeport_core::error::{AppError, ErrorCode};

use super::{dialer::SshTarget, host_key::ServerKeyCheck};
use crate::messages as msg;

/// How long the server may take to describe its Engine socket.
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);

/// What the server says about its Engine socket after refusing to open it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RemoteSocket {
    /// Readable and writable: sshd itself refused the forward.
    Usable,
    /// There, but the user may not use it.
    Denied,
    /// No socket, but a Docker-compatible Engine is installed.
    Stopped,
    /// No socket and no Engine installed.
    Missing,
}

impl RemoteSocket {
    fn parse(output: &str) -> Option<Self> {
        match output.trim() {
            "usable" => Some(Self::Usable),
            "denied" => Some(Self::Denied),
            "stopped" => Some(Self::Stopped),
            "missing" => Some(Self::Missing),
            _ => None,
        }
    }
}

/// A POSIX shell check of `socket`, printing one of [`RemoteSocket`]'s answers.
fn probe_script(socket: &str) -> String {
    let q = format!("'{}'", socket.replace('\'', r"'\''"));
    format!(
        "if [ -S {q} ]; then if [ -r {q} ] && [ -w {q} ]; then echo usable; else echo denied; fi; \
         elif command -v dockerd >/dev/null 2>&1 || command -v docker >/dev/null 2>&1 || command -v podman >/dev/null 2>&1; \
         then echo stopped; else echo missing; fi"
    )
}

/// Runs [`probe_script`] on the server; `None` when it can't run or answers oddly.
pub(super) async fn probe_socket(
    session: &Handle<ServerKeyCheck>,
    socket: &str,
) -> Option<RemoteSocket> {
    let run = async {
        let mut channel = session.channel_open_session().await.ok()?;
        channel.exec(true, probe_script(socket)).await.ok()?;
        let mut output = Vec::new();
        while let Some(message) = channel.wait().await {
            match message {
                russh::ChannelMsg::Data { data } => output.extend_from_slice(&data),
                russh::ChannelMsg::Eof | russh::ChannelMsg::Close => break,
                _ => {}
            }
        }
        RemoteSocket::parse(&String::from_utf8_lossy(&output))
    };
    tokio::time::timeout(PROBE_TIMEOUT, run)
        .await
        .ok()
        .flatten()
}

/// The server accepted the login but wouldn't open the Engine socket; `probe`
/// is what it said about the socket, if it could be asked.
pub(super) fn socket_unavailable(
    t: &SshTarget,
    probe: Option<RemoteSocket>,
    e: &russh::Error,
) -> AppError {
    let (host, user, socket) = (&t.host, &t.user, &t.socket_path);
    let prohibited = matches!(
        e,
        russh::Error::ChannelOpenFailure(russh::ChannelOpenFailure::AdministrativelyProhibited)
    );
    let error = match probe {
        _ if prohibited => forwarding_disabled(host),
        Some(RemoteSocket::Usable) => forwarding_disabled(host),
        Some(RemoteSocket::Missing) => AppError::new(
            ErrorCode::EngineNotInstalled,
            msg::SSH_ENGINE_NOT_INSTALLED.with("host", host),
        )
        .with_hint(
            msg::SSH_ENGINE_NOT_INSTALLED_HINT
                .with("host", host)
                .with("user", user),
        )
        .with_command("curl -fsSL https://get.docker.com | sh\nsudo systemctl enable --now docker")
        .with_command(format!("sudo usermod -aG docker {user}")),
        Some(RemoteSocket::Stopped) => AppError::new(
            ErrorCode::EngineNotRunning,
            msg::SSH_ENGINE_NOT_RUNNING
                .with("host", host)
                .with("path", socket),
        )
        .with_hint(msg::SSH_ENGINE_NOT_RUNNING_HINT.with("host", host))
        .with_command("sudo systemctl enable --now docker"),
        Some(RemoteSocket::Denied) => AppError::new(
            ErrorCode::PermissionDenied,
            msg::SSH_ENGINE_PERMISSION_DENIED
                .with("user", user)
                .with("host", host),
        )
        .with_hint(
            msg::SSH_ENGINE_PERMISSION_DENIED_HINT
                .with("host", host)
                .with("user", user),
        )
        .with_command(format!("sudo usermod -aG docker {user}")),
        None => AppError::new(
            ErrorCode::RemoteEngineUnavailable,
            msg::SSH_SOCKET_UNAVAILABLE
                .with("user", user)
                .with("path", socket)
                .with("host", host),
        )
        .with_hint(
            msg::SSH_SOCKET_UNAVAILABLE_HINT
                .with("host", host)
                .with("user", user),
        )
        .with_command("sudo systemctl enable --now docker")
        .with_command(format!("sudo usermod -aG docker {user}")),
    };
    error.with_cause(e)
}

fn forwarding_disabled(host: &str) -> AppError {
    AppError::new(
        ErrorCode::RemoteEngineUnavailable,
        msg::SSH_FORWARDING_DISABLED.with("host", host),
    )
    .with_hint(msg::SSH_FORWARDING_DISABLED_HINT.with("host", host))
    .with_command(
        "echo 'AllowStreamLocalForwarding yes' | sudo tee /etc/ssh/sshd_config.d/homeport.conf\nsudo systemctl restart ssh",
    )
}

#[cfg(test)]
mod tests {
    use super::super::auth::Credentials;
    use super::*;

    fn target() -> SshTarget {
        SshTarget {
            label: "prod (deploy@prod.example:22)".into(),
            host: "prod.example".into(),
            port: 22,
            user: "deploy".into(),
            socket_path: "/var/run/docker.sock".into(),
            credentials: Credentials::Agent,
            trusted_host_key: None,
        }
    }

    fn refused() -> russh::Error {
        russh::Error::ChannelOpenFailure(russh::ChannelOpenFailure::ConnectFailed)
    }

    #[test]
    fn the_probe_tells_why_the_remote_socket_cant_be_opened() {
        for (probe, code, command) in [
            (
                Some(RemoteSocket::Missing),
                ErrorCode::EngineNotInstalled,
                "get.docker.com",
            ),
            (
                Some(RemoteSocket::Stopped),
                ErrorCode::EngineNotRunning,
                "systemctl enable --now docker",
            ),
            (
                Some(RemoteSocket::Denied),
                ErrorCode::PermissionDenied,
                "usermod -aG docker deploy",
            ),
            (
                Some(RemoteSocket::Usable),
                ErrorCode::RemoteEngineUnavailable,
                "AllowStreamLocalForwarding yes",
            ),
            (
                None,
                ErrorCode::RemoteEngineUnavailable,
                "usermod -aG docker deploy",
            ),
        ] {
            let e = socket_unavailable(&target(), probe, &refused());
            assert_eq!(e.code, code, "{probe:?}");
            assert!(e.message.contains("prod.example"), "{e:?}");
            let commands = e.hint.unwrap().commands.join("\n");
            assert!(commands.contains(command), "{probe:?}: {commands}");
        }
    }

    #[test]
    fn a_prohibited_forward_means_sshd_forbids_it() {
        let e = socket_unavailable(
            &target(),
            None,
            &russh::Error::ChannelOpenFailure(
                russh::ChannelOpenFailure::AdministrativelyProhibited,
            ),
        );
        assert!(e.hint.unwrap().commands[0].contains("AllowStreamLocalForwarding yes"));
    }

    #[test]
    fn the_probe_quotes_the_socket_path_and_reads_its_answer() {
        let script = probe_script("/tmp/it's.sock");
        assert!(script.contains(r"'/tmp/it'\''s.sock'"), "{script}");
        assert_eq!(RemoteSocket::parse("denied\n"), Some(RemoteSocket::Denied));
        assert_eq!(RemoteSocket::parse("bash: syntax error"), None);
    }
}
