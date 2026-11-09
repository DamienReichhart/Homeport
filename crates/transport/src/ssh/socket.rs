//! Why the server won't open its Engine socket: it asks the server, since a
//! refused channel doesn't say.

use std::time::Duration;

use russh::client::Handle;

use homeport_core::{
    error::{AppError, ErrorCode},
    host::EngineKind,
};

use super::{dialer::SshTarget, host_key::ServerKeyCheck};
use crate::{
    messages as msg,
    reach::{containerd_group_commands, incus_group_command, is_restricted_incus_socket},
};

/// How long the server may take to describe its Engine socket.
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);

/// What the server says about its Engine socket after refusing to open it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RemoteSocket {
    /// Readable and writable: sshd itself refused the forward.
    Usable,
    /// There, but the user may not use it.
    Denied,
    /// No socket, but the Engine is installed.
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

/// A POSIX shell check of `socket`, printing one of [`RemoteSocket`]'s
/// answers; `engine` decides which programs mean it's installed.
fn probe_script(socket: &str, engine: EngineKind) -> String {
    let q = format!("'{}'", socket.replace('\'', r"'\''"));
    let installed = match engine {
        EngineKind::Containerd => "command -v containerd >/dev/null 2>&1 || command -v k3s >/dev/null 2>&1",
        EngineKind::Incus => "command -v incusd >/dev/null 2>&1 || command -v incus >/dev/null 2>&1 || [ -x /usr/libexec/incus/incusd ] || [ -x /opt/incus/bin/incusd ]",
        _ => "command -v dockerd >/dev/null 2>&1 || command -v docker >/dev/null 2>&1 || command -v podman >/dev/null 2>&1",
    };
    format!(
        "if [ -S {q} ]; then if [ -r {q} ] && [ -w {q} ]; then echo usable; else echo denied; fi; \
         elif {installed}; then echo stopped; else echo missing; fi"
    )
}

/// Runs [`probe_script`] on the server; `None` when it can't run or answers oddly.
pub(super) async fn probe_socket(
    session: &Handle<ServerKeyCheck>,
    socket: &str,
    engine: EngineKind,
) -> Option<RemoteSocket> {
    let run = async {
        let mut channel = session.channel_open_session().await.ok()?;
        channel
            .exec(true, probe_script(socket, engine))
            .await
            .ok()?;
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
    if t.engine == EngineKind::Incus {
        return incus_unavailable(t, probe, prohibited).with_cause(e);
    }
    if t.engine == EngineKind::Containerd {
        let error = match probe {
            _ if prohibited => forwarding_disabled(host),
            Some(RemoteSocket::Usable) => forwarding_disabled(host),
            Some(RemoteSocket::Missing) => AppError::new(
                ErrorCode::EngineNotInstalled,
                msg::SSH_CONTAINERD_NOT_INSTALLED.with("host", host),
            )
            .with_hint(msg::SSH_CONTAINERD_NOT_INSTALLED_HINT.with("host", host)),
            Some(RemoteSocket::Stopped) => AppError::new(
                ErrorCode::EngineNotRunning,
                msg::SSH_CONTAINERD_NOT_RUNNING
                    .with("host", host)
                    .with("path", socket),
            )
            .with_hint(msg::SSH_CONTAINERD_NOT_RUNNING_HINT.with("host", host))
            .with_command("sudo systemctl enable --now containerd"),
            Some(RemoteSocket::Denied) | None => AppError::new(
                ErrorCode::PermissionDenied,
                msg::SSH_CONTAINERD_PERMISSION_DENIED
                    .with("user", user)
                    .with("host", host)
                    .with("path", socket),
            )
            .with_hint(msg::SSH_CONTAINERD_PERMISSION_DENIED_HINT.with("host", host))
            .with_command(containerd_group_commands(user))
            .with_command("sudo systemctl restart containerd"),
        };
        return error.with_cause(e);
    }
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

/// [`socket_unavailable`] for an Incus Host.
fn incus_unavailable(t: &SshTarget, probe: Option<RemoteSocket>, prohibited: bool) -> AppError {
    let (host, user, socket) = (&t.host, &t.user, &t.socket_path);
    let restricted = is_restricted_incus_socket(socket);
    match probe {
        _ if prohibited => forwarding_disabled(host),
        Some(RemoteSocket::Usable) => forwarding_disabled(host),
        Some(RemoteSocket::Missing) => AppError::new(
            ErrorCode::EngineNotInstalled,
            msg::SSH_INCUS_NOT_INSTALLED.with("host", host),
        )
        .with_hint(msg::SSH_INCUS_NOT_INSTALLED_HINT.with("host", host)),
        Some(RemoteSocket::Stopped) => AppError::new(
            ErrorCode::EngineNotRunning,
            msg::SSH_INCUS_NOT_RUNNING
                .with("host", host)
                .with("path", socket),
        )
        .with_hint(msg::SSH_INCUS_NOT_RUNNING_HINT.with("host", host))
        .with_command(if restricted {
            "sudo systemctl enable --now incus-user.socket"
        } else {
            "sudo systemctl enable --now incus.socket incus.service"
        }),
        Some(RemoteSocket::Denied) | None => AppError::new(
            ErrorCode::PermissionDenied,
            msg::SSH_INCUS_PERMISSION_DENIED
                .with("user", user)
                .with("host", host)
                .with("path", socket),
        )
        .with_hint(
            msg::SSH_INCUS_PERMISSION_DENIED_HINT
                .with("host", host)
                .with("user", user),
        )
        .with_command(incus_group_command(restricted, user)),
    }
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
            bastions: vec![],
            engine: EngineKind::Docker,
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
        let script = probe_script("/tmp/it's.sock", EngineKind::Docker);
        assert!(script.contains(r"'/tmp/it'\''s.sock'"), "{script}");
        assert_eq!(RemoteSocket::parse("denied\n"), Some(RemoteSocket::Denied));
        assert_eq!(RemoteSocket::parse("bash: syntax error"), None);
    }

    #[test]
    fn containerd_hosts_are_probed_and_explained_as_containerd() {
        let script = probe_script("/run/containerd/containerd.sock", EngineKind::Containerd);
        assert!(script.contains("command -v containerd"), "{script}");
        assert!(!script.contains("dockerd"), "{script}");
        let mut t = target();
        t.engine = EngineKind::Containerd;
        t.socket_path = "/run/containerd/containerd.sock".into();
        for (probe, code, command) in [
            (
                Some(RemoteSocket::Stopped),
                ErrorCode::EngineNotRunning,
                "systemctl enable --now containerd",
            ),
            (
                Some(RemoteSocket::Denied),
                ErrorCode::PermissionDenied,
                "usermod -aG containerd deploy",
            ),
            (
                None,
                ErrorCode::PermissionDenied,
                "usermod -aG containerd deploy",
            ),
        ] {
            let e = socket_unavailable(&t, probe, &refused());
            assert_eq!(e.code, code, "{probe:?}");
            let hint = e.hint.unwrap();
            assert!(!hint.text.contains("docker group"), "{hint:?}");
            assert!(
                hint.commands.join("\n").contains(command),
                "{probe:?}: {hint:?}"
            );
        }
        let missing = socket_unavailable(&t, Some(RemoteSocket::Missing), &refused());
        assert_eq!(missing.code, ErrorCode::EngineNotInstalled);
        assert!(missing.message.contains("containerd"), "{missing:?}");
    }

    #[test]
    fn incus_hosts_are_probed_and_explained_as_incus() {
        let script = probe_script("/var/lib/incus/unix.socket", EngineKind::Incus);
        assert!(script.contains("command -v incusd"), "{script}");
        assert!(!script.contains("dockerd"), "{script}");
        let mut t = target();
        t.engine = EngineKind::Incus;
        t.socket_path = "/var/lib/incus/unix.socket".into();
        for (probe, code, command) in [
            (
                Some(RemoteSocket::Stopped),
                ErrorCode::EngineNotRunning,
                "systemctl enable --now incus.socket",
            ),
            (
                Some(RemoteSocket::Denied),
                ErrorCode::PermissionDenied,
                "usermod -aG incus-admin deploy",
            ),
        ] {
            let e = socket_unavailable(&t, probe, &refused());
            assert_eq!(e.code, code, "{probe:?}");
            let hint = e.hint.unwrap();
            assert!(!hint.text.contains("docker group"), "{hint:?}");
            assert!(hint.commands.join("\n").contains(command), "{hint:?}");
        }
        t.socket_path = "/var/lib/incus/unix.socket.user".into();
        let e = socket_unavailable(&t, Some(RemoteSocket::Denied), &refused());
        assert!(e.hint.unwrap().commands[0].contains("-aG incus deploy"));
        let missing = socket_unavailable(&t, Some(RemoteSocket::Missing), &refused());
        assert_eq!(missing.code, ErrorCode::EngineNotInstalled);
        assert!(missing.message.contains("Incus"), "{missing:?}");
    }
}
