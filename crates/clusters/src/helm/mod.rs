//! Helm on Clusters, through the user's `helm` CLI on this machine, pointed
//! at a Cluster through a private [`crate::proxy`] (ADR 0009). helm's own
//! configuration (chart repositories, registry logins, caches) stays the
//! user's.

mod errors;
mod install;
mod releases;
mod repos;
mod run;

use std::{path::PathBuf, process::Stdio};

use serde::Serialize;
use tokio::process::Command;

use homeport_core::{error::AppResult, programs};

pub use self::{
    errors::{failure, missing},
    install::{InstallRequest, ReleasePreview},
    releases::{ReleaseDetails, ReleaseList, ReleaseResource, ReleaseRevision, ReleaseSummary},
    repos::{ChartRepository, ChartVersion},
    run::Helm,
};

/// The `helm` program found on this computer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HelmCli {
    pub program: String,
    /// As `helm version --short` prints it, without build metadata: `v3.16.2`.
    pub version: String,
}

/// What `helm version --short` printed, read.
#[derive(Debug, PartialEq, Eq)]
enum Version {
    Supported(String),
    /// Helm 2 or older, with its version.
    Outdated(String),
    Unreadable,
}

fn parse_version(text: &str) -> Version {
    // Helm 2 prints `Client: v2.17.0+ga690bad`, Helm 3 `v3.16.2+g13654a5`.
    let text = text.trim().trim_start_matches("Client:").trim();
    let Some(version) = text.split_whitespace().next() else {
        return Version::Unreadable;
    };
    let version = version.split('+').next().unwrap_or(version);
    let major = version
        .strip_prefix('v')
        .and_then(|v| v.split('.').next())
        .and_then(|m| m.parse::<u32>().ok());
    match major {
        Some(major) if major >= 3 => Version::Supported(version.to_owned()),
        Some(_) => Version::Outdated(version.to_owned()),
        None => Version::Unreadable,
    }
}

/// The `helm` program, without checking its version: what every Helm
/// action runs, so a CLI installed since launch is found.
pub fn program() -> AppResult<PathBuf> {
    programs::find("helm").ok_or_else(missing)
}

/// Finds `helm` and checks it is Helm 3 or later; says how to install it
/// when it isn't there.
pub async fn detect() -> AppResult<HelmCli> {
    let program = program()?;
    let out = Command::new(&program)
        .args(["version", "--short"])
        .env("PATH", programs::search_path())
        .stdin(Stdio::null())
        .output()
        .await
        .map_err(|_| missing())?;
    // Helm 2 exits with an error when it can't reach Tiller, after
    // printing its own version.
    let printed = String::from_utf8_lossy(&out.stdout);
    match parse_version(&printed) {
        Version::Supported(version) if out.status.success() => Ok(HelmCli {
            program: program.to_string_lossy().into_owned(),
            version,
        }),
        Version::Outdated(version) => Err(errors::outdated(&version)),
        _ => Err(missing()),
    }
}

/// Variables of Homeport's environment that would point helm at another
/// cluster or Namespace than the proxy's kubeconfig says.
pub(crate) const FOREIGN_ENV: [&str; 9] = [
    "HELM_KUBECONTEXT",
    "HELM_KUBETOKEN",
    "HELM_KUBEAPISERVER",
    "HELM_KUBEASUSER",
    "HELM_KUBEASGROUPS",
    "HELM_KUBECAFILE",
    "HELM_KUBETLS_SERVER_NAME",
    "HELM_KUBEINSECURE_SKIP_TLS_VERIFY",
    "HELM_NAMESPACE",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helm_3_and_later_are_supported() {
        assert_eq!(
            parse_version("v3.16.2+g13654a5\n"),
            Version::Supported("v3.16.2".into())
        );
        assert_eq!(
            parse_version("v4.0.1+g12500dd"),
            Version::Supported("v4.0.1".into())
        );
    }

    #[test]
    fn helm_2_is_outdated() {
        assert_eq!(
            parse_version("Client: v2.17.0+ga690bad\n"),
            Version::Outdated("v2.17.0".into())
        );
    }

    #[test]
    fn anything_else_is_unreadable() {
        assert_eq!(parse_version(""), Version::Unreadable);
        assert_eq!(parse_version("command not found"), Version::Unreadable);
        assert_eq!(parse_version("vx.1"), Version::Unreadable);
    }
}
