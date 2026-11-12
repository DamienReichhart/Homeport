//! A Host's status, described (`describeStatus`, `statusError`).

use homeport_core::{error::AppError, host::HostConnection};
use homeport_hosts::registry::HostStatus;

use super::Summary;
use crate::{i18n::errors, t, ui::theme::tokens::Tone};

/// The classified cause of a status that needs the user, with its hint and
/// detail.
pub fn error(status: &HostStatus) -> Option<&AppError> {
    match status {
        HostStatus::Unreachable { error, .. }
        | HostStatus::Unauthorized { error }
        | HostStatus::KeychainUnavailable { error }
        | HostStatus::Invalid { error } => Some(error),
        _ => None,
    }
}

/// A short label, a colour and the reason, for showing a Host's status.
pub fn describe(status: &HostStatus) -> Summary {
    match status {
        HostStatus::Connected {
            engine_version,
            podman_version,
            containerd_version,
            incus_version,
            ..
        } => {
            let detail = if let Some(version) = podman_version {
                t!("core.host.podmanVersion", version = version.as_str())
            } else if let Some(version) = containerd_version {
                t!("core.host.containerdVersion", version = version.as_str())
            } else if let Some(version) = incus_version {
                t!("core.host.incusVersion", version = version.as_str())
            } else {
                t!("core.host.engineVersion", version = engine_version.as_str())
            };
            Summary::new(t!("core.status.connected"), Tone::Success).because(detail)
        }
        HostStatus::Connecting => Summary::new(t!("core.status.connecting"), Tone::Info),
        HostStatus::Disabled => Summary::new(t!("core.status.disabled"), Tone::Muted),
        HostStatus::Unreachable {
            error,
            retry_in_secs,
        } => Summary::new(t!("core.status.unreachable"), Tone::Danger).because(t!(
            "core.status.unreachableDetail",
            message = errors::message(error),
            secs = retry_in_secs.to_string()
        )),
        HostStatus::Unauthorized { error } => {
            Summary::new(t!("core.status.unauthorized"), Tone::Danger)
                .because(errors::message(error))
        }
        HostStatus::HostKeyUntrusted { fingerprint, .. } => {
            Summary::new(t!("core.status.hostKeyUntrusted"), Tone::Warning).because(t!(
                "core.status.hostKeyUntrustedDetail",
                fingerprint = fingerprint.as_str()
            ))
        }
        HostStatus::HostKeyMismatch {
            fingerprint,
            expected_fingerprint,
            ..
        } => Summary::new(t!("core.status.hostKeyMismatch"), Tone::Danger).because(t!(
            "core.status.hostKeyMismatchDetail",
            expected = expected_fingerprint.as_str(),
            actual = fingerprint.as_str()
        )),
        HostStatus::CredentialsLost => {
            Summary::new(t!("core.status.credentialsLost"), Tone::Danger)
                .because(t!("core.status.credentialsLostDetail"))
        }
        HostStatus::KeychainUnavailable { error } => {
            Summary::new(t!("core.status.keychainUnavailable"), Tone::Danger)
                .because(errors::message(error))
        }
        HostStatus::Invalid { error } => {
            Summary::new(t!("core.status.invalid"), Tone::Danger).because(errors::message(error))
        }
    }
}

pub fn is_connected(status: &HostStatus) -> bool {
    matches!(status, HostStatus::Connected { .. })
}

/// Where a Host's published ports are reachable, for links
/// (`hostAddress`): this machine, or the server without its port.
pub fn address(connection: &HostConnection) -> String {
    match connection {
        HostConnection::Local { .. } => "localhost".to_owned(),
        HostConnection::Tcp { address } | HostConnection::Tls { address } => {
            let digits = address.bytes().rev().take_while(u8::is_ascii_digit).count();
            let server = match address.len().checked_sub(digits + 1) {
                Some(colon) if digits > 0 && address.as_bytes()[colon] == b':' => &address[..colon],
                _ => address,
            };
            let server = server.strip_prefix('[').unwrap_or(server);
            server.strip_suffix(']').unwrap_or(server).to_owned()
        }
        HostConnection::Ssh { host, .. } => host.clone(),
    }
}
