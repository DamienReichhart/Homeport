//! A local containerd socket that didn't answer: how to start containerd,
//! or let this user use its socket.

use std::{error::Error as StdError, io};

use homeport_core::error::{AppError, ErrorCode};

use super::io_cause;
use crate::messages as msg;

/// Who runs the containerd behind a local socket, from where it lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Provider {
    /// containerd's own service.
    System,
    /// k3s, which runs a containerd of its own.
    K3s,
    /// Rootless containerd, inside RootlessKit's namespace.
    Rootless,
}

fn provider(socket_path: &str) -> Provider {
    if socket_path.contains("/k3s/") {
        Provider::K3s
    } else if socket_path.starts_with("/run/user/") {
        Provider::Rootless
    } else {
        Provider::System
    }
}

/// Why the containerd socket at `socket_path` didn't answer.
pub fn local_containerd(socket_path: &str, error: &(dyn StdError + 'static)) -> AppError {
    let kind = io_cause(error).map(io::Error::kind);
    let path = socket_path;
    let classified = match kind {
        Some(io::ErrorKind::PermissionDenied) => permission_denied(path),
        Some(io::ErrorKind::TimedOut) => AppError::new(
            ErrorCode::Timeout,
            msg::LOCAL_CONTAINERD_TIMEOUT.with("path", path),
        )
        .with_hint(msg::LOCAL_TIMEOUT_HINT),
        Some(io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused) | None => {
            not_running(path)
        }
        Some(_) => AppError::new(
            ErrorCode::EngineNotRunning,
            msg::LOCAL_CONTAINERD_UNREACHABLE.with("path", path),
        )
        .with_hint(msg::LOCAL_CONTAINERD_UNREACHABLE_HINT),
    };
    classified.with_cause(error)
}

fn not_running(path: &str) -> AppError {
    let error = AppError::new(
        ErrorCode::EngineNotRunning,
        msg::LOCAL_CONTAINERD_NOT_RUNNING.with("path", path),
    );
    match provider(path) {
        Provider::System => error
            .with_hint(msg::LOCAL_CONTAINERD_NOT_RUNNING_HINT)
            .with_command("sudo systemctl enable --now containerd"),
        Provider::K3s => error
            .with_hint(msg::LOCAL_CONTAINERD_NOT_RUNNING_K3S_HINT)
            .with_command("sudo systemctl enable --now k3s"),
        Provider::Rootless => error.with_hint(msg::LOCAL_CONTAINERD_ROOTLESS_HINT),
    }
}

fn permission_denied(path: &str) -> AppError {
    AppError::new(
        ErrorCode::PermissionDenied,
        msg::LOCAL_CONTAINERD_PERMISSION_DENIED.with("path", path),
    )
    .with_hint(msg::CONTAINERD_GID_HINT)
    .with_command(containerd_group_commands("\"$USER\""))
    .with_command("sudo systemctl restart containerd")
}

/// Makes a `containerd` group holding `user` and prints its ID, for
/// `[grpc] gid` in containerd's `config.toml`.
pub(crate) fn containerd_group_commands(user: &str) -> String {
    format!(
        "sudo groupadd -f containerd\nsudo usermod -aG containerd {user}\ngetent group containerd | cut -d: -f3"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stopped_containerd_says_who_starts_it() {
        let missing = io::Error::from(io::ErrorKind::NotFound);
        let system = local_containerd("/run/containerd/containerd.sock", &missing);
        assert_eq!(system.code, ErrorCode::EngineNotRunning);
        assert!(system.message.contains("containerd"), "{system:?}");
        assert_eq!(
            system.hint.unwrap().commands,
            ["sudo systemctl enable --now containerd"]
        );
        let k3s = local_containerd("/run/k3s/containerd/containerd.sock", &missing);
        assert_eq!(
            k3s.hint.unwrap().commands,
            ["sudo systemctl enable --now k3s"]
        );
        let rootless = local_containerd("/run/user/1000/containerd/containerd.sock", &missing);
        assert!(rootless.hint.unwrap().text.contains("RootlessKit"));
    }

    #[test]
    fn a_refused_socket_points_to_containerds_grpc_gid() {
        let e = local_containerd(
            "/run/containerd/containerd.sock",
            &io::Error::from(io::ErrorKind::PermissionDenied),
        );
        assert_eq!(e.code, ErrorCode::PermissionDenied);
        let hint = e.hint.unwrap();
        assert!(hint.text.contains("[grpc]"), "{hint:?}");
        assert!(!hint.text.contains("docker group"), "{hint:?}");
        assert!(hint.commands[0].contains("usermod -aG containerd"));
    }
}
