//! Failures of the secrets kept in the OS keychain (ADR 0002).

use std::fmt::Display;

use super::{AppError, ErrorCode};
use crate::messages as msg;

impl AppError {
    pub fn credentials_lost() -> Self {
        Self::new(ErrorCode::CredentialsLost, msg::CREDENTIALS_LOST)
            .with_hint(msg::CREDENTIALS_LOST_HINT)
    }

    pub fn keychain_unavailable(reason: impl Display) -> Self {
        let hint = if cfg!(target_os = "linux") {
            msg::KEYCHAIN_UNAVAILABLE_HINT_LINUX
        } else if cfg!(target_os = "macos") {
            msg::KEYCHAIN_UNAVAILABLE_HINT_MACOS
        } else {
            msg::KEYCHAIN_UNAVAILABLE_HINT_WINDOWS
        };
        Self::new(ErrorCode::KeychainUnavailable, msg::KEYCHAIN_UNAVAILABLE)
            .with_hint(hint)
            .with_detail(reason)
    }
}
