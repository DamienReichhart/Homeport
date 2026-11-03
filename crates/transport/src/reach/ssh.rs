//! An SSH session that failed before or while logging in.

use homeport_core::error::{AppError, ErrorCode};

use super::{connection_lost, network};

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

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn ssh_network_failures_are_network_failures() {
        let e = ssh(
            "prod.example:22",
            &russh::Error::IO(io::Error::from(io::ErrorKind::TimedOut)),
        );
        assert_eq!(e.code, ErrorCode::Timeout);
    }
}
