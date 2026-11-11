//! A Cluster's status, described (`describeClusterStatus`).

use homeport_clusters::{model::ClusterKind, registry::ClusterStatus};

use super::Summary;
use crate::{i18n::errors, t, ui::theme::tokens::Tone};

/// The name of a kind of Cluster, as the Cluster pages show it.
pub fn kind_label(kind: ClusterKind) -> String {
    match kind {
        ClusterKind::Kubernetes => t!("core.cluster.kind.kubernetes"),
        ClusterKind::Nomad => t!("core.cluster.kind.nomad"),
    }
}

/// What a connected Cluster of `kind` runs: its version, and a Nomad
/// Cluster's region.
fn connected(version: &str, region: Option<&str>, kind: ClusterKind) -> String {
    if kind == ClusterKind::Kubernetes {
        return t!("core.cluster.serverVersion", version = version);
    }
    let region = region.unwrap_or("global");
    if version.is_empty() {
        t!("core.cluster.nomadRegion", region = region)
    } else {
        t!(
            "core.cluster.nomadVersion",
            version = version,
            region = region
        )
    }
}

/// A short label, a colour and the reason, for showing the status of a
/// Cluster of `kind`.
pub fn describe(status: &ClusterStatus, kind: ClusterKind) -> Summary {
    match status {
        ClusterStatus::Connected {
            server_version,
            region,
            ..
        } => Summary::new(t!("core.status.connected"), Tone::Success).because(connected(
            server_version,
            region.as_deref(),
            kind,
        )),
        ClusterStatus::Connecting => Summary::new(t!("core.status.connecting"), Tone::Info),
        ClusterStatus::Disabled => Summary::new(t!("core.status.disabled"), Tone::Muted),
        ClusterStatus::Unreachable {
            error,
            retry_in_secs,
        } => Summary::new(t!("core.status.unreachable"), Tone::Danger).because(t!(
            "core.status.unreachableDetail",
            message = errors::message(error),
            secs = retry_in_secs.to_string()
        )),
        ClusterStatus::Unauthorized { error } => {
            Summary::new(t!("core.status.unauthorized"), Tone::Danger)
                .because(errors::message(error))
        }
        ClusterStatus::CredentialsLost => {
            Summary::new(t!("core.status.credentialsLost"), Tone::Danger)
                .because(t!("core.cluster.credentialsLostDetail"))
        }
        ClusterStatus::KeychainUnavailable { error } => {
            Summary::new(t!("core.status.keychainUnavailable"), Tone::Danger)
                .because(errors::message(error))
        }
        ClusterStatus::Invalid { error } => {
            Summary::new(t!("core.status.invalid"), Tone::Danger).because(errors::message(error))
        }
    }
}
