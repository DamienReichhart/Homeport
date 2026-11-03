//! Classifies failures to reach an Engine: local sockets and named pipes,
//! then (for remote Hosts) the network, TLS and SSH layers.

use std::{error::Error as StdError, io, path::Path};

use super::{AppError, ErrorCode};

/// The first `io::Error` in `error`'s chain, looking through bollard's
/// transparent wrappers.
pub fn io_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a io::Error> {
    let mut current = Some(error);
    while let Some(e) = current {
        if let Some(io) = e.downcast_ref::<io::Error>() {
            return Some(io);
        }
        if let Some(bollard::errors::Error::IOError { err }) =
            e.downcast_ref::<bollard::errors::Error>()
        {
            return Some(err);
        }
        current = e.source();
    }
    None
}

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

    fn name(self) -> &'static str {
        match self {
            Self::DockerEngine | Self::DockerRootless | Self::Unknown => "The Docker Engine",
            Self::DockerDesktop => "Docker Desktop",
            Self::Podman => "Podman",
            Self::Colima => "Colima",
            Self::OrbStack => "OrbStack",
            Self::RancherDesktop => "Rancher Desktop",
        }
    }
}

/// Why the local socket at `socket_path` didn't answer.
pub fn local_socket(socket_path: &str, error: &(dyn StdError + 'static)) -> AppError {
    let kind = io_cause(error).map(io::Error::kind);
    let classified = match kind {
        Some(io::ErrorKind::PermissionDenied) => permission_denied(socket_path),
        Some(io::ErrorKind::NotFound) if !any_engine_cli_installed() => not_installed(socket_path),
        Some(io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused) => {
            not_running(socket_path)
        }
        Some(io::ErrorKind::TimedOut) => AppError::new(
            ErrorCode::Timeout,
            format!("The Docker Engine at {socket_path} didn't answer in time."),
        )
        .with_hint("It may be starting or overloaded. Homeport keeps retrying."),
        _ => AppError::new(
            ErrorCode::EngineNotRunning,
            format!("Homeport can't talk to the Docker Engine at {socket_path}."),
        )
        .with_hint("Check that the Docker Engine is running, then reconnect."),
    };
    classified.with_cause(error)
}

fn not_running(socket_path: &str) -> AppError {
    let engine = LocalEngine::of(socket_path);
    let error = AppError::new(
        ErrorCode::EngineNotRunning,
        format!("No Docker Engine is running at {socket_path}."),
    );
    match engine {
        LocalEngine::DockerEngine => error
            .with_hint("Start the Docker Engine, and have it start with the computer:")
            .with_command("sudo systemctl enable --now docker"),
        LocalEngine::DockerRootless => error
            .with_hint("Start your rootless Docker Engine, and have it start when you log in:")
            .with_command("systemctl --user enable --now docker"),
        LocalEngine::Podman if cfg!(target_os = "linux") && socket_path.starts_with("/run/user/") => error
            .with_hint("Start Podman's Docker-compatible socket:")
            .with_command("systemctl --user enable --now podman.socket"),
        LocalEngine::Podman if cfg!(target_os = "linux") => error
            .with_hint("Start Podman's Docker-compatible socket:")
            .with_command("sudo systemctl enable --now podman.socket"),
        LocalEngine::Podman => error
            .with_hint("Start the Podman machine:")
            .with_command("podman machine start"),
        LocalEngine::Colima => error
            .with_hint("Start Colima:")
            .with_command("colima start"),
        LocalEngine::OrbStack => error
            .with_hint("Open OrbStack, or start it from a terminal:")
            .with_command("orb start"),
        LocalEngine::DockerDesktop | LocalEngine::RancherDesktop => error.with_hint(format!(
            "Open {} and wait until its Engine is running.",
            engine.name()
        )),
        LocalEngine::Unknown => error.with_hint(
            "Start the Docker Engine that provides this socket, or check the socket path on the host page.",
        ),
    }
}

fn not_installed(socket_path: &str) -> AppError {
    let error = AppError::new(
        ErrorCode::EngineNotInstalled,
        format!("Docker doesn't seem to be installed: nothing exists at {socket_path}."),
    );
    if cfg!(target_os = "linux") {
        error
            .with_hint("Install Docker Engine (see docs.docker.com/engine/install), for example with Docker's install script, then let your user use it:")
            .with_command("curl -fsSL https://get.docker.com | sh\nsudo systemctl enable --now docker")
            .with_command("sudo usermod -aG docker \"$USER\"")
    } else if cfg!(target_os = "macos") {
        error
            .with_hint("Install Docker Desktop, OrbStack or Colima. With Homebrew, Colima is:")
            .with_command("brew install colima docker docker-compose\ncolima start")
    } else {
        error
            .with_hint("Install Docker Desktop (see docs.docker.com/desktop), then open it:")
            .with_command("winget install Docker.DockerDesktop")
    }
}

fn permission_denied(socket_path: &str) -> AppError {
    let error = AppError::new(
        ErrorCode::PermissionDenied,
        format!("Homeport isn't allowed to use the Docker Engine at {socket_path}."),
    );
    match LocalEngine::of(socket_path) {
        LocalEngine::DockerEngine => error
            .with_hint("Add your user to the docker group, then log out and back in (or restart) so it applies:")
            .with_command("sudo usermod -aG docker \"$USER\""),
        _ if cfg!(windows) => error
            .with_hint("Add your user to the docker-users group in an administrator terminal, then sign out and back in:")
            .with_command("net localgroup docker-users \"%USERNAME%\" /add"),
        _ => error
            .with_hint("Check who may use the socket:")
            .with_command(format!("ls -l {socket_path}")),
    }
}

/// Whether a Docker-compatible CLI is on the PATH, i.e. something is installed
/// even though its Engine isn't running.
fn any_engine_cli_installed() -> bool {
    ["docker", "podman", "colima", "orb"]
        .iter()
        .any(|program| on_path(program))
}

pub fn on_path(program: &str) -> bool {
    let Some(paths) = std::env::var_os("PATH") else {
        return false;
    };
    let names: Vec<String> = if cfg!(windows) {
        vec![format!("{program}.exe"), format!("{program}.cmd")]
    } else {
        vec![program.to_owned()]
    };
    std::env::split_paths(&paths).any(|dir| names.iter().any(|n| is_file(&dir.join(n))))
}

fn is_file(path: &Path) -> bool {
    path.metadata().is_ok_and(|m| m.is_file())
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

    #[test]
    fn io_causes_are_found_through_bollard() {
        let e = bollard::errors::Error::IOError {
            err: io::Error::from(io::ErrorKind::ConnectionRefused),
        };
        assert_eq!(
            io_cause(&e).map(io::Error::kind),
            Some(io::ErrorKind::ConnectionRefused)
        );
    }
}
