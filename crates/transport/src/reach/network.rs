//! A TCP or TLS Engine, or an SSH server, that the network didn't reach.

use std::{error::Error as StdError, io};

use homeport_core::error::{chain, AppError, ErrorCode};

use super::{connection_lost, io_cause};

/// Why `address` (a TCP or TLS Engine, or an SSH server) couldn't be reached
/// over the network.
pub fn network(address: &str, error: &(dyn StdError + 'static)) -> AppError {
    let text = chain(error);
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
    }
}
