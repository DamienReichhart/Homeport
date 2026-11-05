//! What the registry knows about each Host's connection, as the frontend shows it.

use serde::Serialize;

use homeport_core::error::AppError;
use homeport_transport::ConnectError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(
    tag = "state",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum HostStatus {
    Disabled,
    Connecting,
    Connected {
        engine_version: String,
        api_version: String,
        os: String,
        arch: String,
    },
    Unreachable {
        error: AppError,
        retry_in_secs: u32,
    },
    Unauthorized {
        error: AppError,
    },
    /// `bastion` is the address of the bastion that presented the key,
    /// `None` for the Host's own SSH server.
    HostKeyUntrusted {
        key: String,
        fingerprint: String,
        bastion: Option<String>,
    },
    HostKeyMismatch {
        key: String,
        fingerprint: String,
        expected_fingerprint: String,
        bastion: Option<String>,
    },
    CredentialsLost,
    KeychainUnavailable {
        error: AppError,
    },
    Invalid {
        error: AppError,
    },
}

impl HostStatus {
    /// The classified cause of a status that needs the user, if it has one.
    pub fn error(&self) -> Option<&AppError> {
        match self {
            Self::Unreachable { error, .. }
            | Self::Unauthorized { error }
            | Self::KeychainUnavailable { error }
            | Self::Invalid { error } => Some(error),
            _ => None,
        }
    }
}

impl From<ConnectError> for HostStatus {
    fn from(e: ConnectError) -> Self {
        match e {
            ConnectError::Unreachable(error) => Self::Unreachable {
                error,
                retry_in_secs: 0,
            },
            ConnectError::Unauthorized(error) => Self::Unauthorized { error },
            ConnectError::HostKeyUntrusted {
                key,
                fingerprint,
                bastion,
            } => Self::HostKeyUntrusted {
                key,
                fingerprint,
                bastion,
            },
            ConnectError::HostKeyMismatch {
                key,
                fingerprint,
                expected_fingerprint,
                bastion,
            } => Self::HostKeyMismatch {
                key,
                fingerprint,
                expected_fingerprint,
                bastion,
            },
            ConnectError::Invalid(error) => Self::Invalid { error },
        }
    }
}
