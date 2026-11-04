//! Failures of the secrets kept in the OS keychain (ADR 0002).

use std::fmt::Display;

use super::{AppError, ErrorCode};

impl AppError {
    pub fn credentials_lost() -> Self {
        Self::new(
            ErrorCode::CredentialsLost,
            "Stored credentials can't be decrypted.",
        )
        .with_hint("The key that protects them is gone from the OS keychain. Re-enter the credentials on the host page, or regenerate the key.")
    }

    pub fn keychain_unavailable(reason: impl Display) -> Self {
        let hint = if cfg!(target_os = "linux") {
            "Install and unlock a Secret Service keyring, such as GNOME Keyring or KWallet, then restart Homeport."
        } else if cfg!(target_os = "macos") {
            "Unlock the login keychain in Keychain Access, then restart Homeport."
        } else {
            "Make sure Windows Credential Manager is available, then restart Homeport."
        };
        Self::new(
            ErrorCode::KeychainUnavailable,
            "The OS keychain is unavailable, so secrets can't be stored.",
        )
        .with_hint(hint)
        .with_detail(reason)
    }
}
