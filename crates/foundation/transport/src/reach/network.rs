//! A TCP or TLS Engine, or an SSH server, that the network didn't reach.

use std::{error::Error as StdError, io};

use homeport_core::error::{chain, AppError, ErrorCode};

use super::{connection_lost, io_cause};
use crate::messages as msg;

/// Why `address` (a TCP or TLS Engine, or an SSH server) couldn't be reached
/// over the network.
pub fn network(address: &str, error: &(dyn StdError + 'static)) -> AppError {
    let text = chain(error);
    let classified = if is_dns_failure(&text) {
        AppError::new(
            ErrorCode::DnsFailed,
            msg::NETWORK_DNS_FAILED.with("host", address),
        )
        .with_hint(msg::NETWORK_DNS_FAILED_HINT)
    } else {
        match io_cause(error).map(io::Error::kind) {
            Some(io::ErrorKind::ConnectionRefused) => AppError::new(
                ErrorCode::ConnectionRefused,
                msg::NETWORK_REFUSED.with("host", address),
            )
            .with_hint(msg::NETWORK_REFUSED_HINT),
            Some(io::ErrorKind::TimedOut) => AppError::new(
                ErrorCode::Timeout,
                msg::NETWORK_TIMEOUT.with("host", address),
            )
            .with_hint(msg::NETWORK_TIMEOUT_HINT),
            Some(io::ErrorKind::HostUnreachable | io::ErrorKind::NetworkUnreachable) => {
                AppError::new(
                    ErrorCode::NetworkUnreachable,
                    msg::NETWORK_UNREACHABLE.with("host", address),
                )
                .with_hint(msg::NETWORK_UNREACHABLE_HINT)
            }
            Some(
                io::ErrorKind::ConnectionReset
                | io::ErrorKind::ConnectionAborted
                | io::ErrorKind::BrokenPipe
                | io::ErrorKind::UnexpectedEof,
            ) => connection_lost(address),
            _ => AppError::new(
                ErrorCode::ConnectionRefused,
                msg::NETWORK_FAILED.with("host", address),
            )
            .with_hint(msg::NETWORK_FAILED_HINT),
        }
    };
    classified.with_cause(error)
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(e.key, "errors.transport.network.dnsFailed");
        assert_eq!(e.params["host"].to_string(), "prod.example:2375");
        assert_eq!(e.message, "The name of prod.example:2375 can't be found.");
        assert_eq!(
            e.hint.unwrap().key,
            "errors.transport.network.dnsFailed.hint"
        );
    }
}
