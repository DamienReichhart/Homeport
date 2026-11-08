//! What the registry knows about each Cluster's connection, as the frontend shows it.

use serde::Serialize;

use homeport_core::error::AppError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(
    tag = "state",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ClusterStatus {
    Disabled,
    Connecting,
    Connected {
        /// The API server's version, such as `v1.33.1`.
        server_version: String,
        /// Its OS and architecture, such as `linux/arm64`.
        platform: String,
    },
    /// Retrying on its own in `retry_in_secs`.
    Unreachable {
        error: AppError,
        retry_in_secs: u32,
    },
    /// The API server refused the credentials; waits for the user.
    Unauthorized {
        error: AppError,
    },
    CredentialsLost,
    KeychainUnavailable {
        error: AppError,
    },
    /// Something only the user can fix (a certificate, a missing plugin); waits for them.
    Invalid {
        error: AppError,
    },
}

impl ClusterStatus {
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

    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { .. })
    }
}
