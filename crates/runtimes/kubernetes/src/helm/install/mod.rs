//! Installing a chart: its default values (`helm show values`), a preview
//! rendered with the Cluster's help (`--dry-run=server`), then the install
//! with its output streamed. Values go on stdin, never in a file.

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use super::{repos::ChartVersion, Helm};
use crate::{messages as msg, proxy::CliEvent};

/// What to install, and how.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstallRequest {
    /// The release's name.
    pub name: String,
    pub namespace: String,
    /// `repository/chart`, or an `oci://` reference.
    pub chart: String,
    /// The chart version; the latest without.
    pub version: Option<String>,
    /// YAML; empty for the chart's defaults.
    pub values: String,
    /// Creates the Namespace when it doesn't exist.
    pub create_namespace: bool,
    /// Waits for the release's resources to be ready.
    pub wait: bool,
    /// How long `wait` may take, in seconds; helm's default (5 min) without.
    pub timeout_seconds: Option<u32>,
}

/// What an install or upgrade would apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ReleasePreview {
    /// The rendered manifest, hooks included.
    pub manifest: String,
    pub notes: String,
}

/// The longest release name helm takes.
const NAME_MAX: usize = 53;

/// A DNS-1123 subdomain: lowercase letters, digits, `-` and `.`, starting
/// and ending each part with a letter or digit.
fn is_dns_subdomain(name: &str) -> bool {
    !name.is_empty()
        && name.split('.').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                && !part.starts_with('-')
                && !part.ends_with('-')
        })
}

/// Checks a release name as helm does.
pub(super) fn validate_name(name: &str) -> AppResult<()> {
    if name.len() > NAME_MAX || !is_dns_subdomain(name) {
        return Err(AppError::validation(
            msg::HELM_RELEASE_NAME_INVALID.with("name", name),
        ));
    }
    Ok(())
}

/// Checks a Namespace name (a DNS-1123 label).
pub(super) fn validate_namespace(namespace: &str) -> AppResult<()> {
    if namespace.len() > 63 || namespace.contains('.') || !is_dns_subdomain(namespace) {
        return Err(AppError::validation(
            msg::HELM_NAMESPACE_INVALID.with("namespace", namespace),
        ));
    }
    Ok(())
}

/// Checks a chart reference: `repository/chart` or `oci://…`; local
/// folders, archives and URLs aren't installed from Homeport.
pub(super) fn validate_chart(chart: &str) -> AppResult<()> {
    let simple = |s: &str| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
            && !s.starts_with('.')
    };
    let ok = match chart.strip_prefix("oci://") {
        Some(rest) => rest.contains('/') && !rest.contains(char::is_whitespace),
        None => chart
            .split_once('/')
            .is_some_and(|(repo, name)| simple(repo) && simple(name)),
    };
    if ok {
        Ok(())
    } else {
        Err(
            AppError::validation(msg::HELM_CHART_INVALID.with("chart", chart))
                .with_hint(msg::HELM_CHART_INVALID_HINT),
        )
    }
}

/// Checks values are YAML helm can use: a mapping, or nothing.
pub(super) fn validate_values(values: &str) -> AppResult<()> {
    if values.trim().is_empty() {
        return Ok(());
    }
    match serde_yaml_ng::from_str::<serde_yaml_ng::Value>(values) {
        Ok(serde_yaml_ng::Value::Mapping(_) | serde_yaml_ng::Value::Null) => Ok(()),
        Ok(_) => Err(AppError::validation(msg::HELM_VALUES_NOT_MAP)),
        Err(e) => {
            let line = e.location().map_or(0, |l| l.line());
            Err(
                AppError::validation(msg::HELM_VALUES_YAML.with("line", line))
                    .with_detail(e.to_string()),
            )
        }
    }
}

fn validate(request: &InstallRequest) -> AppResult<()> {
    validate_name(&request.name)?;
    validate_namespace(&request.namespace)?;
    validate_chart(&request.chart)?;
    validate_values(&request.values)
}

/// `--version <v>` when one is given.
pub(super) fn version_args(version: Option<&str>) -> Vec<String> {
    match version.map(str::trim).filter(|v| !v.is_empty()) {
        Some(v) => vec!["--version".into(), v.to_owned()],
        None => vec![],
    }
}

/// `--wait --timeout <n>s` when waiting.
pub(super) fn wait_args(wait: bool, timeout_seconds: Option<u32>) -> Vec<String> {
    let mut args = vec![];
    if wait {
        args.push("--wait".into());
        if let Some(seconds) = timeout_seconds {
            args.extend(["--timeout".into(), format!("{seconds}s")]);
        }
    }
    args
}

/// The arguments of `helm install` for `request`, values read from stdin;
/// a server-side dry run printing JSON with `dry_run`.
pub(super) fn install_args(request: &InstallRequest, dry_run: bool) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "install".into(),
        request.name.clone(),
        request.chart.clone(),
        "--namespace".into(),
        request.namespace.clone(),
        "--values".into(),
        "-".into(),
    ];
    args.extend(version_args(request.version.as_deref()));
    if request.create_namespace {
        args.push("--create-namespace".into());
    }
    if dry_run {
        args.extend(["--dry-run=server".into(), "--output".into(), "json".into()]);
    } else {
        args.extend(wait_args(request.wait, request.timeout_seconds));
    }
    args
}

/// What `helm install|upgrade --dry-run --output json` prints, in part.
#[derive(Deserialize)]
pub(super) struct DryRun {
    #[serde(default)]
    manifest: String,
    #[serde(default)]
    hooks: Vec<Hook>,
    #[serde(default)]
    info: Info,
}

#[derive(Deserialize)]
struct Hook {
    #[serde(default)]
    path: String,
    #[serde(default)]
    manifest: String,
}

#[derive(Deserialize, Default)]
struct Info {
    #[serde(default)]
    notes: String,
}

impl From<DryRun> for ReleasePreview {
    fn from(run: DryRun) -> Self {
        let mut manifest = run.manifest;
        for hook in run.hooks {
            if !manifest.is_empty() && !manifest.ends_with('\n') {
                manifest.push('\n');
            }
            manifest.push_str(&format!("---\n# Hook: {}\n{}", hook.path, hook.manifest));
        }
        Self {
            manifest,
            notes: run.info.notes,
        }
    }
}

impl Helm {
    /// A chart's default values, as YAML (`helm show values`).
    pub async fn chart_values(&self, chart: &str, version: Option<&str>) -> AppResult<String> {
        validate_chart(chart)?;
        let mut args = vec!["show".to_owned(), "values".to_owned(), chart.to_owned()];
        args.extend(version_args(version));
        self.text(args, None).await
    }

    /// Every version of the repository chart `chart`, newest first; none
    /// for an OCI reference, whose versions helm can't list.
    pub async fn chart_versions(&self, chart: &str) -> AppResult<Vec<ChartVersion>> {
        validate_chart(chart)?;
        if chart.starts_with("oci://") {
            return Ok(vec![]);
        }
        Ok(self
            .search_charts(chart, true)
            .await?
            .into_iter()
            .filter(|c| c.name == chart)
            .collect())
    }

    /// What installing `request` would apply, rendered with the Cluster.
    pub async fn preview_install(&self, request: &InstallRequest) -> AppResult<ReleasePreview> {
        validate(request)?;
        self.dry_run(install_args(request, true), &request.values)
            .await
    }

    pub(super) async fn dry_run(
        &self,
        args: Vec<String>,
        values: &str,
    ) -> AppResult<ReleasePreview> {
        let text = self.text(args, Some(values.to_owned())).await?;
        let run: DryRun = serde_json::from_str(&text).map_err(|e| {
            AppError::new(
                homeport_core::error::ErrorCode::HelmFailed,
                msg::HELM_UNREADABLE,
            )
            .with_cause(&e)
        })?;
        Ok(run.into())
    }

    /// Installs `request`, streaming helm's output. A request that doesn't
    /// pass the checks fails before helm runs.
    pub async fn install(self, request: InstallRequest, sink: impl Fn(CliEvent) -> bool) {
        if let Err(error) = validate(&request) {
            sink(CliEvent::Error { error });
            return;
        }
        let args = install_args(&request, false);
        self.stream(args, Some(request.values), sink).await;
    }
}

#[cfg(test)]
mod tests;
