//! Why a Host can't be reached, sorted by what the user can do about it.

use homeport_core::error::{AppError, ErrorCode};

use crate::messages as msg;

/// Why a Host can't be reached. Each case maps to a Host status the user can act on.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConnectError {
    /// Worth retrying: the Engine or the network may come back.
    #[error("{0}")]
    Unreachable(AppError),
    /// The Host answered but refused Homeport; retrying won't help.
    #[error("{0}")]
    Unauthorized(AppError),
    #[error("the SSH server's key isn't trusted yet ({fingerprint})")]
    HostKeyUntrusted { key: String, fingerprint: String },
    #[error("the SSH server's key changed: expected {expected_fingerprint}, got {fingerprint}")]
    HostKeyMismatch {
        key: String,
        fingerprint: String,
        expected_fingerprint: String,
    },
    /// The Host's settings can't work as entered.
    #[error("{0}")]
    Invalid(AppError),
}

/// Sorts a classified failure into the Host status it leads to.
impl From<AppError> for ConnectError {
    fn from(e: AppError) -> Self {
        use ErrorCode::*;
        match e.code {
            PermissionDenied
            | SshAuthFailed
            | SshAgentUnavailable
            | TlsClientRejected
            | TlsCertificateExpired
            | TlsCertificateNotYetValid
            | TlsUnknownCa
            | TlsNameMismatch
            | TlsHandshakeFailed => Self::Unauthorized(e),
            Validation | TlsMaterialInvalid | TlsRequired | TlsNotSupported | SshKeyInvalid => {
                Self::Invalid(e)
            }
            _ => Self::Unreachable(e),
        }
    }
}

impl From<ConnectError> for AppError {
    fn from(e: ConnectError) -> Self {
        match e {
            ConnectError::Unreachable(e)
            | ConnectError::Unauthorized(e)
            | ConnectError::Invalid(e) => e,
            ConnectError::HostKeyUntrusted { fingerprint, .. } => AppError::new(
                ErrorCode::HostKeyUntrusted,
                msg::SSH_HOST_KEY_UNTRUSTED.with("fingerprint", fingerprint),
            )
            .with_hint(msg::SSH_HOST_KEY_UNTRUSTED_HINT),
            ConnectError::HostKeyMismatch {
                fingerprint,
                expected_fingerprint,
                ..
            } => AppError::new(ErrorCode::HostKeyChanged, msg::SSH_HOST_KEY_CHANGED)
                .with_hint(msg::SSH_HOST_KEY_CHANGED_HINT)
                .with_detail(format!(
                    "expected {expected_fingerprint}, got {fingerprint}"
                )),
        }
    }
}
