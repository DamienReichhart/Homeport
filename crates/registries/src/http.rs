//! The HTTP client for registries and Docker Hub, called from this machine.

use std::time::Duration;

use homeport_core::error::{AppError, AppResult, ErrorCode};

/// A client on rustls with ring, the provider the rest of the app uses.
pub(crate) fn client() -> AppResult<reqwest::Client> {
    if rustls::crypto::CryptoProvider::get_default().is_none() {
        // Another thread may have installed it first; either way one is set.
        let _ = rustls::crypto::ring::default_provider().install_default();
    }
    reqwest::Client::builder()
        .user_agent(concat!("Homeport/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|e| {
            AppError::new(ErrorCode::Io, "Homeport couldn't prepare its HTTPS client.")
                .with_cause(&e)
        })
}

/// A request that never got an answer: DNS, refused, TLS or timed out.
pub(crate) fn unreachable(address: &str, error: &reqwest::Error) -> AppError {
    AppError::new(
        ErrorCode::RegistryUnreachable,
        format!("Homeport can't reach {address}."),
    )
    .with_hint("Check the address, and this computer's internet access, DNS and proxy settings.")
    .with_cause(error)
}

pub(crate) fn rate_limited(address: &str) -> AppError {
    AppError::new(
        ErrorCode::RegistryRateLimited,
        format!("{address} is limiting requests from this computer."),
    )
    .with_hint("Wait a few minutes, then try again.")
}

/// An answer Homeport didn't expect; the status goes to the detail.
pub(crate) fn unexpected(address: &str, status: reqwest::StatusCode) -> AppError {
    AppError::new(
        ErrorCode::RegistryUnreachable,
        format!("{address} gave an answer Homeport didn't expect."),
    )
    .with_hint("Check that the address is a container registry. Its exact answer is in Details.")
    .with_detail(format!("HTTP {status}"))
}
