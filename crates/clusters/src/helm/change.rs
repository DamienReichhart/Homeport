//! Changing a release: upgrading it (a chart version and values, previewed
//! first if asked), rolling it back to an earlier revision, uninstalling it.
//! Each streams helm's output; values go on stdin.

use serde::{Deserialize, Serialize};

use homeport_core::error::AppResult;

use super::{
    install::{
        validate_chart, validate_name, validate_namespace, validate_values, version_args,
        wait_args, ReleasePreview,
    },
    Helm,
};
use crate::proxy::CliEvent;

/// A new chart version and/or values for a release.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeRequest {
    pub name: String,
    pub namespace: String,
    /// `repository/chart` or an `oci://` reference: the release's chart,
    /// found in the user's repositories, or another.
    pub chart: String,
    /// The latest without.
    pub version: Option<String>,
    /// Every user-supplied value, as YAML: they replace the release's, and
    /// the chart's defaults fill the rest.
    pub values: String,
    pub wait: bool,
    pub timeout_seconds: Option<u32>,
}

/// Rolling a release back to one of its revisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RollbackRequest {
    pub name: String,
    pub namespace: String,
    pub revision: u32,
    pub wait: bool,
    pub timeout_seconds: Option<u32>,
}

/// Uninstalling a release, which deletes the objects it created.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UninstallRequest {
    pub name: String,
    pub namespace: String,
    /// Keeps the release's history, so it is listed as `uninstalled` and
    /// can be rolled back.
    pub keep_history: bool,
    pub wait: bool,
    pub timeout_seconds: Option<u32>,
}

fn validate_release(name: &str, namespace: &str) -> AppResult<()> {
    validate_name(name)?;
    validate_namespace(namespace)
}

fn validate_upgrade(request: &UpgradeRequest) -> AppResult<()> {
    validate_release(&request.name, &request.namespace)?;
    validate_chart(&request.chart)?;
    validate_values(&request.values)
}

/// The arguments of `helm upgrade` for `request`, values read from stdin;
/// a server-side dry run printing JSON with `dry_run`.
pub(super) fn upgrade_args(request: &UpgradeRequest, dry_run: bool) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "upgrade".into(),
        request.name.clone(),
        request.chart.clone(),
        "--namespace".into(),
        request.namespace.clone(),
        "--values".into(),
        "-".into(),
    ];
    args.extend(version_args(request.version.as_deref()));
    if dry_run {
        args.extend(["--dry-run=server".into(), "--output".into(), "json".into()]);
    } else {
        args.extend(wait_args(request.wait, request.timeout_seconds));
    }
    args
}

pub(super) fn rollback_args(request: &RollbackRequest) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "rollback".into(),
        request.name.clone(),
        request.revision.to_string(),
        "--namespace".into(),
        request.namespace.clone(),
    ];
    args.extend(wait_args(request.wait, request.timeout_seconds));
    args
}

pub(super) fn uninstall_args(request: &UninstallRequest) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "uninstall".into(),
        request.name.clone(),
        "--namespace".into(),
        request.namespace.clone(),
    ];
    if request.keep_history {
        args.push("--keep-history".into());
    }
    args.extend(wait_args(request.wait, request.timeout_seconds));
    args
}

/// The repository charts named `chart` (`nginx` matches `bitnami/nginx`),
/// those offering `version` first.
pub(super) fn candidates(found: &[super::ChartVersion], chart: &str, version: &str) -> Vec<String> {
    let mut names: Vec<(bool, String)> = vec![];
    for c in found {
        let Some((_, name)) = c.name.split_once('/') else {
            continue;
        };
        if name != chart {
            continue;
        }
        let has = c.version == version;
        match names.iter_mut().find(|(_, n)| *n == c.name) {
            Some(entry) => entry.0 |= has,
            None => names.push((has, c.name.clone())),
        }
    }
    names.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    names.into_iter().map(|(_, n)| n).collect()
}

impl Helm {
    /// The charts in the user's repositories a release of `chart` at
    /// `version` can be upgraded from, the likeliest first; empty when no
    /// repository has it.
    pub async fn chart_candidates(&self, chart: &str, version: &str) -> AppResult<Vec<String>> {
        let found = self.search_charts(chart, true).await?;
        Ok(candidates(&found, chart, version))
    }

    /// What upgrading as `request` asks would apply, rendered with the Cluster.
    pub async fn preview_upgrade(&self, request: &UpgradeRequest) -> AppResult<ReleasePreview> {
        validate_upgrade(request)?;
        self.dry_run(upgrade_args(request, true), &request.values)
            .await
    }

    pub async fn upgrade(self, request: UpgradeRequest, sink: impl Fn(CliEvent) -> bool) {
        if let Err(error) = validate_upgrade(&request) {
            sink(CliEvent::Error { error });
            return;
        }
        let args = upgrade_args(&request, false);
        self.stream(args, Some(request.values), sink).await;
    }

    pub async fn rollback(self, request: RollbackRequest, sink: impl Fn(CliEvent) -> bool) {
        if let Err(error) = validate_release(&request.name, &request.namespace) {
            sink(CliEvent::Error { error });
            return;
        }
        self.stream(rollback_args(&request), None, sink).await;
    }

    pub async fn uninstall(self, request: UninstallRequest, sink: impl Fn(CliEvent) -> bool) {
        if let Err(error) = validate_release(&request.name, &request.namespace) {
            sink(CliEvent::Error { error });
            return;
        }
        self.stream(uninstall_args(&request), None, sink).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helm::ChartVersion;

    fn upgrade() -> UpgradeRequest {
        UpgradeRequest {
            name: "web".into(),
            namespace: "shop".into(),
            chart: "bitnami/nginx".into(),
            version: Some("18.2.5".into()),
            values: "replicaCount: 3\n".into(),
            wait: true,
            timeout_seconds: None,
        }
    }

    #[test]
    fn an_upgrade_sends_its_values_on_stdin() {
        assert_eq!(
            upgrade_args(&upgrade(), false),
            [
                "upgrade",
                "web",
                "bitnami/nginx",
                "--namespace",
                "shop",
                "--values",
                "-",
                "--version",
                "18.2.5",
                "--wait"
            ]
        );
        assert_eq!(
            upgrade_args(&upgrade(), true)[7..],
            [
                "--version",
                "18.2.5",
                "--dry-run=server",
                "--output",
                "json"
            ]
        );
    }

    #[test]
    fn rollbacks_and_uninstalls_name_their_release() {
        assert_eq!(
            rollback_args(&RollbackRequest {
                name: "web".into(),
                namespace: "shop".into(),
                revision: 11,
                wait: true,
                timeout_seconds: Some(120),
            }),
            [
                "rollback",
                "web",
                "11",
                "--namespace",
                "shop",
                "--wait",
                "--timeout",
                "120s"
            ]
        );
        assert_eq!(
            uninstall_args(&UninstallRequest {
                name: "web".into(),
                namespace: "shop".into(),
                keep_history: true,
                wait: false,
                timeout_seconds: Some(120),
            }),
            ["uninstall", "web", "--namespace", "shop", "--keep-history"]
        );
    }

    #[test]
    fn the_release_chart_is_found_in_the_repositories() {
        let found: Vec<ChartVersion> = [
            ("bitnami/nginx", "18.2.4"),
            ("bitnami/nginx", "18.2.5"),
            ("mirror/nginx", "18.2.5"),
            ("other/nginx", "18.2.4"),
            ("bitnami/nginx-ingress-controller", "11.0.0"),
        ]
        .map(|(name, version)| ChartVersion {
            name: name.into(),
            version: version.into(),
            app_version: String::new(),
            description: String::new(),
        })
        .to_vec();
        assert_eq!(
            candidates(&found, "nginx", "18.2.4"),
            ["bitnami/nginx", "other/nginx", "mirror/nginx"]
        );
        assert!(candidates(&found, "redis", "1.0.0").is_empty());
    }
}
