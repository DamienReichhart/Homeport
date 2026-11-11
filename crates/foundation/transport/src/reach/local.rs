//! A local socket or named pipe that didn't answer: which product owns it,
//! and how to start, install or get permission to use it.

use std::{error::Error as StdError, io};

use homeport_core::{
    error::{AppError, ErrorCode},
    programs,
};

use super::io_cause;
use crate::messages as msg;

/// The software that usually provides a local Engine socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalEngine {
    DockerEngine,
    DockerRootless,
    DockerDesktop,
    Podman,
    Colima,
    OrbStack,
    RancherDesktop,
    Unknown,
}

impl LocalEngine {
    /// Guesses the product from where its socket lives (see `hosts::detect`).
    pub fn of(socket_path: &str) -> Self {
        let p = socket_path.replace('\\', "/");
        if p.contains("/.docker/desktop/") || p.contains("/.docker/run/") {
            Self::DockerDesktop
        } else if p.contains("/.colima/") {
            Self::Colima
        } else if p.contains("/.orbstack/") {
            Self::OrbStack
        } else if p.contains("/.rd/") {
            Self::RancherDesktop
        } else if p.contains("podman") {
            Self::Podman
        } else if p.ends_with("pipe/docker_engine") {
            Self::DockerDesktop
        } else if p.starts_with("/run/user/") && p.ends_with("/docker.sock") {
            Self::DockerRootless
        } else if p == "/var/run/docker.sock" || p == "/run/docker.sock" {
            if cfg!(target_os = "linux") {
                Self::DockerEngine
            } else {
                Self::DockerDesktop
            }
        } else {
            Self::Unknown
        }
    }
}

/// Why the local socket at `socket_path` didn't answer.
pub fn local_socket(socket_path: &str, error: &(dyn StdError + 'static)) -> AppError {
    let kind = io_cause(error).map(io::Error::kind);
    let classified = match kind {
        Some(io::ErrorKind::PermissionDenied) => permission_denied(socket_path),
        Some(io::ErrorKind::NotFound) if !programs::engine_installed() => {
            not_installed(socket_path)
        }
        Some(io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused) => {
            not_running(socket_path)
        }
        Some(io::ErrorKind::TimedOut) => AppError::new(
            ErrorCode::Timeout,
            msg::LOCAL_TIMEOUT.with("path", socket_path),
        )
        .with_hint(msg::LOCAL_TIMEOUT_HINT),
        _ => AppError::new(
            ErrorCode::EngineNotRunning,
            msg::LOCAL_UNREACHABLE.with("path", socket_path),
        )
        .with_hint(msg::LOCAL_UNREACHABLE_HINT),
    };
    classified.with_cause(error)
}

fn not_running(socket_path: &str) -> AppError {
    let error = AppError::new(
        ErrorCode::EngineNotRunning,
        msg::LOCAL_NOT_RUNNING.with("path", socket_path),
    );
    match LocalEngine::of(socket_path) {
        LocalEngine::DockerEngine => error
            .with_hint(msg::LOCAL_NOT_RUNNING_DOCKER_ENGINE_HINT)
            .with_command("sudo systemctl enable --now docker"),
        LocalEngine::DockerRootless => error
            .with_hint(msg::LOCAL_NOT_RUNNING_DOCKER_ROOTLESS_HINT)
            .with_command("systemctl --user enable --now docker"),
        LocalEngine::Podman
            if cfg!(target_os = "linux") && socket_path.starts_with("/run/user/") =>
        {
            error
                .with_hint(msg::LOCAL_NOT_RUNNING_PODMAN_SOCKET_HINT)
                .with_command("systemctl --user enable --now podman.socket")
        }
        LocalEngine::Podman if cfg!(target_os = "linux") => error
            .with_hint(msg::LOCAL_NOT_RUNNING_PODMAN_SOCKET_HINT)
            .with_command("sudo systemctl enable --now podman.socket"),
        LocalEngine::Podman => error
            .with_hint(msg::LOCAL_NOT_RUNNING_PODMAN_MACHINE_HINT)
            .with_command("podman machine start"),
        LocalEngine::Colima => error
            .with_hint(msg::LOCAL_NOT_RUNNING_COLIMA_HINT)
            .with_command("colima start"),
        LocalEngine::OrbStack => error
            .with_hint(msg::LOCAL_NOT_RUNNING_ORBSTACK_HINT)
            .with_command("orb start"),
        LocalEngine::DockerDesktop => error.with_hint(msg::LOCAL_NOT_RUNNING_DOCKER_DESKTOP_HINT),
        LocalEngine::RancherDesktop => error.with_hint(msg::LOCAL_NOT_RUNNING_RANCHER_DESKTOP_HINT),
        LocalEngine::Unknown => error.with_hint(msg::LOCAL_NOT_RUNNING_UNKNOWN_HINT),
    }
}

fn not_installed(socket_path: &str) -> AppError {
    let error = AppError::new(
        ErrorCode::EngineNotInstalled,
        msg::LOCAL_NOT_INSTALLED.with("path", socket_path),
    );
    if cfg!(target_os = "linux") {
        error
            .with_hint(msg::LOCAL_NOT_INSTALLED_LINUX_HINT)
            .with_command(
                "curl -fsSL https://get.docker.com | sh\nsudo systemctl enable --now docker",
            )
            .with_command("sudo usermod -aG docker \"$USER\"")
    } else if cfg!(target_os = "macos") {
        error
            .with_hint(msg::LOCAL_NOT_INSTALLED_MACOS_HINT)
            .with_command("brew install colima docker docker-compose\ncolima start")
    } else {
        error
            .with_hint(msg::LOCAL_NOT_INSTALLED_WINDOWS_HINT)
            .with_command("winget install Docker.DockerDesktop")
    }
}

fn permission_denied(socket_path: &str) -> AppError {
    let error = AppError::new(
        ErrorCode::PermissionDenied,
        msg::LOCAL_PERMISSION_DENIED.with("path", socket_path),
    );
    match LocalEngine::of(socket_path) {
        LocalEngine::DockerEngine => error
            .with_hint(msg::LOCAL_PERMISSION_DENIED_DOCKER_GROUP_HINT)
            .with_command("sudo usermod -aG docker \"$USER\""),
        _ if cfg!(windows) => error
            .with_hint(msg::LOCAL_PERMISSION_DENIED_DOCKER_USERS_HINT)
            .with_command("net localgroup docker-users \"%USERNAME%\" /add"),
        _ => error
            .with_hint(msg::LOCAL_PERMISSION_DENIED_SOCKET_HINT)
            .with_command(format!("ls -l {socket_path}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sockets_name_the_product_that_owns_them() {
        for (path, engine) in [
            (
                "/home/me/.docker/desktop/docker.sock",
                LocalEngine::DockerDesktop,
            ),
            ("/Users/me/.colima/default/docker.sock", LocalEngine::Colima),
            ("/Users/me/.orbstack/run/docker.sock", LocalEngine::OrbStack),
            ("/Users/me/.rd/docker.sock", LocalEngine::RancherDesktop),
            ("/run/user/1000/podman/podman.sock", LocalEngine::Podman),
            ("/run/user/1000/docker.sock", LocalEngine::DockerRootless),
            ("//./pipe/docker_engine", LocalEngine::DockerDesktop),
            ("/opt/custom.sock", LocalEngine::Unknown),
        ] {
            assert_eq!(LocalEngine::of(path), engine, "{path}");
        }
    }

    #[test]
    fn a_stopped_colima_says_how_to_start_it() {
        let e = not_running("/Users/me/.colima/default/docker.sock");
        assert_eq!(e.code, ErrorCode::EngineNotRunning);
        assert_eq!(e.hint.unwrap().commands, ["colima start"]);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn a_refused_docker_socket_says_to_join_the_docker_group() {
        let e = local_socket(
            "/var/run/docker.sock",
            &io::Error::from(io::ErrorKind::PermissionDenied),
        );
        assert_eq!(e.code, ErrorCode::PermissionDenied);
        let hint = e.hint.unwrap();
        assert!(hint.text.contains("docker group"));
        assert_eq!(hint.commands, ["sudo usermod -aG docker \"$USER\""]);
        assert!(e.detail.unwrap().contains("ermission denied"));
    }
}
