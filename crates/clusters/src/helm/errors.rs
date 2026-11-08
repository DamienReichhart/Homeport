//! Explaining helm's failures: missing or outdated, or what it printed on
//! stderr when it exited with an error.

use homeport_core::error::{AppError, ErrorCode};

use crate::messages as msg;

/// No usable `helm`, and how to install it on this OS.
pub fn missing() -> AppError {
    let (hint, command) = install();
    AppError::new(ErrorCode::HelmMissing, msg::HELM_MISSING)
        .with_hint(hint)
        .with_command(command)
}

/// Helm 2, which Homeport doesn't drive.
pub(super) fn outdated(version: &str) -> AppError {
    let (hint, command) = install();
    AppError::new(
        ErrorCode::HelmOutdated,
        msg::HELM_OUTDATED.with("version", version),
    )
    .with_hint(hint)
    .with_command(command)
}

fn install() -> (homeport_core::error::Message, &'static str) {
    if cfg!(target_os = "macos") {
        (msg::HELM_INSTALL_MACOS_HINT, "brew install helm")
    } else if cfg!(windows) {
        (msg::HELM_INSTALL_WINDOWS_HINT, "winget install Helm.Helm")
    } else {
        (
            msg::HELM_INSTALL_LINUX_HINT,
            "sudo snap install helm --classic",
        )
    }
}
