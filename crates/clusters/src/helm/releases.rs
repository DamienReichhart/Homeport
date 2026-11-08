//! Reading releases: the list (`helm list`), a release's history and what
//! one revision was installed with (`helm get`).

use serde::{Deserialize, Deserializer, Serialize};

use homeport_core::error::{AppResult, ErrorCode};

use super::Helm;

/// One release, as `helm list` gives it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseSummary {
    pub name: String,
    pub namespace: String,
    pub revision: u32,
    /// RFC 3339, when helm's date could be read.
    pub updated: Option<String>,
    /// `deployed`, `failed`, `pending-install`, `pending-upgrade`,
    /// `pending-rollback`, `uninstalling`, `superseded`, `uninstalled`…
    pub status: String,
    /// `nginx-15.1.0`, as helm names it.
    pub chart: String,
    pub chart_name: String,
    pub chart_version: String,
    pub app_version: String,
}

/// The releases of a Cluster.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseList {
    pub releases: Vec<ReleaseSummary>,
    /// Set when every Namespace was asked for but the credentials only
    /// allow this one, the Cluster's default, which was listed instead.
    pub only_namespace: Option<String>,
}

/// One revision of a release, as `helm history` gives it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseRevision {
    pub revision: u32,
    pub updated: Option<String>,
    pub status: String,
    pub chart: String,
    pub chart_name: String,
    pub chart_version: String,
    pub app_version: String,
    /// What helm wrote: `Install complete`, `Upgrade "x" failed: …`.
    pub description: String,
}

/// An object a release's manifest creates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseResource {
    pub api_version: String,
    pub kind: String,
    pub name: String,
    /// As the manifest says; namespaced objects without one are in the
    /// release's Namespace.
    pub namespace: Option<String>,
}

/// What one revision of a release was installed with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseDetails {
    /// The values given at install or upgrade, as YAML; empty for none.
    pub user_values: String,
    /// Every value the chart was rendered with, its defaults included.
    pub computed_values: String,
    pub notes: String,
    pub manifest: String,
    pub resources: Vec<ReleaseResource>,
}

/// `helm list` prints the revision as a string, `helm history` as a number.
fn number<'de, D: Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Either {
        N(u32),
        S(String),
    }
    match Either::deserialize(d)? {
        Either::N(n) => Ok(n),
        Either::S(s) => s.parse().map_err(serde::de::Error::custom),
    }
}

#[derive(Deserialize)]
struct RawRelease {
    name: String,
    namespace: String,
    #[serde(deserialize_with = "number")]
    revision: u32,
    #[serde(default)]
    updated: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    chart: String,
    #[serde(default)]
    app_version: String,
}

#[derive(Deserialize)]
struct RawRevision {
    #[serde(deserialize_with = "number")]
    revision: u32,
    #[serde(default)]
    updated: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    chart: String,
    #[serde(default)]
    app_version: String,
    #[serde(default)]
    description: String,
}

/// Splits helm's `name-version` chart label: the version starts after the
/// first `-` followed by a digit, or by `v` and a digit.
pub(super) fn split_chart(chart: &str) -> (String, String) {
    for (i, _) in chart.match_indices('-') {
        let rest = &chart[i + 1..];
        let digits = rest.strip_prefix('v').unwrap_or(rest);
        if digits.starts_with(|c: char| c.is_ascii_digit()) {
            return (chart[..i].to_owned(), rest.to_owned());
        }
    }
    (chart.to_owned(), String::new())
}

/// helm's dates as RFC 3339: `helm history` already gives one; `helm list`
/// gives Go's `2024-01-15 10:23:45.123456 +0100 CET`.
fn timestamp(text: &str) -> Option<String> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    if let Ok(t) = text.parse::<jiff::Timestamp>() {
        return Some(t.to_string());
    }
    let mut parts = text.split_whitespace();
    let (date, time, offset) = (parts.next()?, parts.next()?, parts.next()?);
    let offset = match offset.len() {
        5 => format!("{}:{}", &offset[..3], &offset[3..]),
        _ => offset.to_owned(),
    };
    format!("{date}T{time}{offset}")
        .parse::<jiff::Timestamp>()
        .ok()
        .map(|t| t.to_string())
}

impl From<RawRelease> for ReleaseSummary {
    fn from(raw: RawRelease) -> Self {
        let (chart_name, chart_version) = split_chart(&raw.chart);
        Self {
            name: raw.name,
            namespace: raw.namespace,
            revision: raw.revision,
            updated: timestamp(&raw.updated),
            status: raw.status,
            chart: raw.chart,
            chart_name,
            chart_version,
            app_version: raw.app_version,
        }
    }
}

impl From<RawRevision> for ReleaseRevision {
    fn from(raw: RawRevision) -> Self {
        let (chart_name, chart_version) = split_chart(&raw.chart);
        Self {
            revision: raw.revision,
            updated: timestamp(&raw.updated),
            status: raw.status,
            chart: raw.chart,
            chart_name,
            chart_version,
            app_version: raw.app_version,
            description: raw.description,
        }
    }
}

/// The objects a manifest creates, in its order.
pub(super) fn resources(manifest: &str) -> Vec<ReleaseResource> {
    let text = |value: &serde_yaml_ng::Value, key: &str| {
        value.get(key).and_then(|v| v.as_str()).map(str::to_owned)
    };
    serde_yaml_ng::Deserializer::from_str(manifest)
        .filter_map(|doc| serde_yaml_ng::Value::deserialize(doc).ok())
        .filter_map(|doc| {
            let metadata = doc.get("metadata")?;
            Some(ReleaseResource {
                api_version: text(&doc, "apiVersion")?,
                kind: text(&doc, "kind")?,
                name: text(metadata, "name")?,
                namespace: text(metadata, "namespace"),
            })
        })
        .collect()
}

/// YAML helm printed for values: `null` when there are none.
fn values_yaml(text: String) -> String {
    match text.trim() {
        "null" | "{}" => String::new(),
        _ => text,
    }
}

/// `helm get notes` starts its text with a `NOTES:` line.
fn notes_text(text: &str) -> String {
    text.strip_prefix("NOTES:")
        .unwrap_or(text)
        .trim_start_matches(['\r', '\n'])
        .to_owned()
}

fn args<const N: usize>(fixed: [&str; N]) -> Vec<String> {
    fixed.iter().map(|a| (*a).to_owned()).collect()
}

impl Helm {
    /// The releases in `namespace`, or in every Namespace with `None`, in
    /// every state (pending and failed ones too). When the credentials
    /// can't list every Namespace, lists `fallback` instead.
    pub async fn releases(
        &self,
        namespace: Option<&str>,
        fallback: Option<&str>,
    ) -> AppResult<ReleaseList> {
        let list = |namespace: Option<&str>| {
            let mut a = args(["list", "--all", "--max", "100000"]);
            match namespace {
                Some(ns) => a.extend(["--namespace".into(), ns.to_owned()]),
                None => a.push("--all-namespaces".into()),
            }
            a
        };
        let (raw, only_namespace) = match (namespace, fallback) {
            (Some(_), _) | (None, None) => {
                (self.json::<Vec<RawRelease>>(list(namespace)).await?, None)
            }
            (None, Some(fallback)) => match self.json::<Vec<RawRelease>>(list(None)).await {
                Err(e) if e.is(ErrorCode::Forbidden) => (
                    self.json(list(Some(fallback))).await?,
                    Some(fallback.to_owned()),
                ),
                other => (other?, None),
            },
        };
        let mut releases: Vec<ReleaseSummary> = raw.into_iter().map(Into::into).collect();
        releases.sort_by(|a, b| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)));
        Ok(ReleaseList {
            releases,
            only_namespace,
        })
    }

    /// Every revision of the release `name`, oldest first.
    pub async fn history(&self, namespace: &str, name: &str) -> AppResult<Vec<ReleaseRevision>> {
        let a = args(["history", name, "--namespace", namespace, "--max", "1000"]);
        let raw: Vec<RawRevision> = self.json(a).await?;
        Ok(raw.into_iter().map(Into::into).collect())
    }

    /// What revision `revision` of the release `name` was installed with,
    /// the current one with `None`.
    pub async fn details(
        &self,
        namespace: &str,
        name: &str,
        revision: Option<u32>,
    ) -> AppResult<ReleaseDetails> {
        let get = |what: &str, extra: &[&str]| {
            let mut a = args(["get", what, name, "--namespace", namespace]);
            if let Some(revision) = revision {
                a.extend(["--revision".into(), revision.to_string()]);
            }
            a.extend(extra.iter().map(|e| (*e).to_owned()));
            a
        };
        let (user, computed, notes, manifest) = tokio::join!(
            self.text(get("values", &["--output", "yaml"]), None),
            self.text(get("values", &["--all", "--output", "yaml"]), None),
            self.text(get("notes", &[]), None),
            self.text(get("manifest", &[]), None),
        );
        let manifest = manifest?;
        Ok(ReleaseDetails {
            user_values: values_yaml(user?),
            computed_values: values_yaml(computed?),
            // A chart without NOTES.txt makes helm fail with "no notes".
            notes: notes.map(|n| notes_text(&n)).unwrap_or_default(),
            resources: resources(&manifest),
            manifest,
        })
    }
}

#[cfg(test)]
mod tests;
