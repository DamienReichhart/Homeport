//! An SSH session that failed before or while logging in.

use homeport_core::error::{AppError, ErrorCode};

use super::{connection_lost, network};
use crate::messages as msg;

/// Why an SSH session to `address` failed before or while logging in.
pub fn ssh(address: &str, error: &russh::Error) -> AppError {
    match error {
        russh::Error::IO(_) => network(address, error),
        russh::Error::ConnectionTimeout
        | russh::Error::KeepaliveTimeout
        | russh::Error::InactivityTimeout => AppError::new(
            ErrorCode::Timeout,
            msg::SSH_STOPPED_ANSWERING.with("host", address),
        )
        .with_hint(msg::SSH_STOPPED_ANSWERING_HINT)
        .with_cause(error),
        russh::Error::Disconnect | russh::Error::HUP => connection_lost(address).with_cause(error),
        russh::Error::NoCommonAlgo { .. } | russh::Error::KexInit | russh::Error::Kex => {
            AppError::new(
                ErrorCode::SshFailed,
                msg::SSH_NO_COMMON_ALGORITHM.with("host", address),
            )
            .with_hint(msg::SSH_NO_COMMON_ALGORITHM_HINT)
            .with_cause(error)
        }
        _ => AppError::new(ErrorCode::SshFailed, msg::SSH_FAILED.with("host", address))
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
