//! Classifies failures to reach an Engine: local sockets and named pipes,
//! then (for remote Hosts) the network, TLS and SSH layers.

use std::{error::Error as StdError, io, path::Path};

use super::{AppError, ErrorCode};

/// `error` as an `io::Error`, looking through the transparent wrappers of
/// bollard and russh (whose `source()` skips the io::Error itself).
fn as_io<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a io::Error> {
    if let Some(io) = error.downcast_ref::<io::Error>() {
        return Some(io);
    }
    if let Some(bollard::errors::Error::IOError { err }) =
        error.downcast_ref::<bollard::errors::Error>()
    {
        return Some(err);
    }
    if let Some(russh::Error::IO(err)) = error.downcast_ref::<russh::Error>() {
        return Some(err);
    }
    None
}

/// Every `io::Error` in `error`'s chain, outermost first.
fn io_errors<'a>(error: &'a (dyn StdError + 'static)) -> impl Iterator<Item = &'a io::Error> {
    std::iter::successors(Some(error), |&e| e.source()).filter_map(as_io)
}

/// The first `io::Error` in `error`'s chain.
pub fn io_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a io::Error> {
    io_errors(error).next()
}

/// An error a dialer already classified, carried inside an `io::Error`
/// through hyper and bollard.
pub fn classified_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a AppError> {
    io_errors(error).find_map(|io| io.get_ref()?.downcast_ref::<AppError>())
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
        format!("The Docker Engine doesn't seem to be installed: nothing exists at {socket_path}."),
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

/// Why `address` (a TCP or TLS Engine, or an SSH server) couldn't be reached
/// over the network.
pub fn network(address: &str, error: &(dyn StdError + 'static)) -> AppError {
    let text = super::chain(error);
    let classified = if is_dns_failure(&text) {
        AppError::new(
            ErrorCode::DnsFailed,
            format!("The name of {address} can't be found."),
        )
        .with_hint("Check the address for typos, and that this computer's network and DNS work.")
    } else {
        match io_cause(error).map(io::Error::kind) {
            Some(io::ErrorKind::ConnectionRefused) => AppError::new(
                ErrorCode::ConnectionRefused,
                format!("{address} refused the connection."),
            )
            .with_hint("Check the port, that the service listens on it, and that no firewall blocks it."),
            Some(io::ErrorKind::TimedOut) => AppError::new(
                ErrorCode::Timeout,
                format!("{address} didn't answer in time."),
            )
            .with_hint("Check the address, that the machine is on, and that no firewall drops the connection."),
            Some(io::ErrorKind::HostUnreachable | io::ErrorKind::NetworkUnreachable) => {
                AppError::new(
                    ErrorCode::NetworkUnreachable,
                    format!("{address} can't be reached from this network."),
                )
                .with_hint("Check your network connection, or connect the VPN it's behind.")
            }
            Some(
                io::ErrorKind::ConnectionReset
                | io::ErrorKind::ConnectionAborted
                | io::ErrorKind::BrokenPipe
                | io::ErrorKind::UnexpectedEof,
            ) => connection_lost(address),
            _ => AppError::new(
                ErrorCode::ConnectionRefused,
                format!("Homeport can't reach {address}."),
            )
            .with_hint("Check the address and your network, then reconnect."),
        }
    };
    classified.with_cause(error)
}

pub fn connection_lost(address: &str) -> AppError {
    AppError::new(
        ErrorCode::ConnectionLost,
        format!("The connection to {address} was cut."),
    )
    .with_hint("The machine or the network may have restarted. Homeport keeps retrying.")
}

fn is_dns_failure(text: &str) -> bool {
    [
        "failed to lookup address",
        "dns error",
        "Name or service not known",
        "nodename nor servname",
        "No such host is known",
        "Temporary failure in name resolution",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

/// The first rustls error in `error`'s chain; rustls reports through `io::Error`.
fn rustls_cause<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a rustls::Error> {
    std::iter::successors(Some(error), |&e| e.source()).find_map(|e| {
        e.downcast_ref::<rustls::Error>()
            .or_else(|| as_io(e)?.get_ref()?.downcast_ref::<rustls::Error>())
    })
}

/// Why the TLS connection to `address` failed; network failures fall back to
/// [`network`]. `label` names the Host in messages.
pub fn tls(label: &str, address: &str, error: &(dyn StdError + 'static)) -> AppError {
    use rustls::{AlertDescription as Alert, CertificateError as Cert, Error as Tls};
    let Some(tls) = rustls_cause(error) else {
        return network(label, error);
    };
    let host = address
        .rsplit_once(':')
        .map_or(address, |(host, _)| host)
        .trim_start_matches('[')
        .trim_end_matches(']');
    let client_rejected = || {
        AppError::new(
            ErrorCode::TlsClientRejected,
            format!("{label} rejected Homeport's client certificate."),
        )
        .with_hint("Use a client certificate (cert.pem and key.pem) signed by the CA the Engine trusts, its --tlscacert.")
    };
    let classified = match tls {
        Tls::InvalidCertificate(Cert::Expired | Cert::ExpiredContext { .. }) => AppError::new(
            ErrorCode::TlsCertificateExpired,
            format!("The certificate of {label} has expired."),
        )
        .with_hint("Renew the Docker Engine's TLS certificate, then reconnect."),
        Tls::InvalidCertificate(Cert::NotValidYet | Cert::NotValidYetContext { .. }) => {
            AppError::new(
                ErrorCode::TlsCertificateNotYetValid,
                format!("The certificate of {label} isn't valid yet."),
            )
            .with_hint("Check that this computer's date and time are right.")
        }
        Tls::InvalidCertificate(Cert::UnknownIssuer) => AppError::new(
            ErrorCode::TlsUnknownCa,
            format!("The certificate of {label} isn't signed by the CA certificate on the host page."),
        )
        .with_hint("Paste the ca.pem that signed the Engine's certificate."),
        Tls::InvalidCertificate(Cert::NotValidForName | Cert::NotValidForNameContext { .. }) => {
            AppError::new(
                ErrorCode::TlsNameMismatch,
                format!("The certificate of {label} isn't made for {host}."),
            )
            .with_hint("Connect with a name or IP address listed in the certificate, or reissue it to include this one.")
        }
        Tls::AlertReceived(Alert::CertificateExpired) => AppError::new(
            ErrorCode::TlsCertificateExpired,
            "Homeport's client certificate has expired.",
        )
        .with_hint("Make a new client certificate signed by the Engine's CA and paste it on the host page."),
        Tls::AlertReceived(
            Alert::BadCertificate
            | Alert::UnknownCA
            | Alert::CertificateRequired
            | Alert::CertificateUnknown
            | Alert::AccessDenied
            | Alert::DecryptError,
        ) => client_rejected(),
        Tls::InvalidMessage(_) | Tls::InappropriateMessage { .. } => AppError::new(
            ErrorCode::TlsNotSupported,
            format!("{label} doesn't speak TLS."),
        )
        .with_hint("Use a TCP connection for this address, or the Engine's TLS port (usually 2376)."),
        _ => AppError::new(
            ErrorCode::TlsHandshakeFailed,
            format!("The secure connection to {label} failed."),
        )
        .with_hint("Check the certificates on the host page against the ones the Engine uses."),
    };
    classified.with_cause(error)
}

/// Why an SSH session to `address` failed before or while logging in.
pub fn ssh(address: &str, error: &russh::Error) -> AppError {
    match error {
        russh::Error::IO(_) => network(address, error),
        russh::Error::ConnectionTimeout
        | russh::Error::KeepaliveTimeout
        | russh::Error::InactivityTimeout => AppError::new(
            ErrorCode::Timeout,
            format!("The SSH server at {address} stopped answering."),
        )
        .with_hint("Check that the machine is on and reachable. Homeport keeps retrying.")
        .with_cause(error),
        russh::Error::Disconnect | russh::Error::HUP => {
            connection_lost(address).with_cause(error)
        }
        russh::Error::NoCommonAlgo { .. } | russh::Error::KexInit | russh::Error::Kex => {
            AppError::new(
                ErrorCode::SshFailed,
                format!("Homeport and the SSH server at {address} share no encryption method."),
            )
            .with_hint("Update OpenSSH on that machine, or allow modern algorithms such as ed25519 and curve25519 in its sshd_config.")
            .with_cause(error)
        }
        _ => AppError::new(
            ErrorCode::SshFailed,
            format!("The SSH connection to {address} failed."),
        )
        .with_cause(error),
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
    fn network_failures_are_told_apart() {
        for (kind, code) in [
            (
                io::ErrorKind::ConnectionRefused,
                ErrorCode::ConnectionRefused,
            ),
            (io::ErrorKind::TimedOut, ErrorCode::Timeout),
            (
                io::ErrorKind::NetworkUnreachable,
                ErrorCode::NetworkUnreachable,
            ),
            (
                io::ErrorKind::HostUnreachable,
                ErrorCode::NetworkUnreachable,
            ),
            (io::ErrorKind::ConnectionReset, ErrorCode::ConnectionLost),
        ] {
            let e = network("prod.example:2375", &io::Error::from(kind));
            assert_eq!(e.code, code, "{kind:?}");
            assert!(e.message.contains("prod.example:2375"), "{e:?}");
            assert!(e.hint.is_some(), "{kind:?}");
        }
    }

    #[test]
    fn unknown_server_names_are_dns_failures() {
        let e = network(
            "prod.example:2375",
            &io::Error::other("failed to lookup address information: Name or service not known"),
        );
        assert_eq!(e.code, ErrorCode::DnsFailed);
    }

    fn tls_failure(error: rustls::Error) -> AppError {
        tls(
            "prod (prod.example:2376)",
            "prod.example:2376",
            &io::Error::new(io::ErrorKind::InvalidData, error),
        )
    }

    #[test]
    fn certificate_problems_are_told_apart() {
        use rustls::{AlertDescription as Alert, CertificateError as Cert, Error as Tls};
        for (error, code) in [
            (
                Tls::InvalidCertificate(Cert::Expired),
                ErrorCode::TlsCertificateExpired,
            ),
            (
                Tls::InvalidCertificate(Cert::NotValidYet),
                ErrorCode::TlsCertificateNotYetValid,
            ),
            (
                Tls::InvalidCertificate(Cert::UnknownIssuer),
                ErrorCode::TlsUnknownCa,
            ),
            (
                Tls::InvalidCertificate(Cert::NotValidForName),
                ErrorCode::TlsNameMismatch,
            ),
            (
                Tls::AlertReceived(Alert::BadCertificate),
                ErrorCode::TlsClientRejected,
            ),
            (
                Tls::AlertReceived(Alert::CertificateExpired),
                ErrorCode::TlsCertificateExpired,
            ),
            (
                Tls::InvalidMessage(rustls::InvalidMessage::InvalidContentType),
                ErrorCode::TlsNotSupported,
            ),
        ] {
            let text = error.to_string();
            let e = tls_failure(error);
            assert_eq!(e.code, code, "{text}");
            assert!(e.hint.is_some(), "{text}");
        }
    }

    #[test]
    fn name_mismatches_name_the_host() {
        let e = tls_failure(rustls::Error::InvalidCertificate(
            rustls::CertificateError::NotValidForName,
        ));
        assert!(e.message.contains("isn't made for prod.example."), "{e:?}");
    }

    #[test]
    fn a_client_certificate_refused_after_the_handshake_is_still_a_rejection() {
        // TLS 1.3 servers send this alert on the first read, inside bollard's request.
        let alert = rustls::Error::AlertReceived(rustls::AlertDescription::CertificateRequired);
        let e = bollard::errors::Error::IOError {
            err: io::Error::new(io::ErrorKind::InvalidData, alert),
        };
        let e = tls("prod (prod.example:2376)", "prod.example:2376", &e);
        assert_eq!(e.code, ErrorCode::TlsClientRejected);
    }

    #[test]
    fn tls_falls_back_to_network_failures() {
        let e = tls(
            "prod (prod.example:2376)",
            "prod.example:2376",
            &io::Error::from(io::ErrorKind::ConnectionRefused),
        );
        assert_eq!(e.code, ErrorCode::ConnectionRefused);
    }

    #[test]
    fn ssh_network_failures_are_network_failures() {
        let e = ssh(
            "prod.example:22",
            &russh::Error::IO(io::Error::from(io::ErrorKind::TimedOut)),
        );
        assert_eq!(e.code, ErrorCode::Timeout);
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
