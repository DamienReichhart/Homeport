//! helm's chart repositories on this machine (`helm repo`) and the charts
//! in them (`helm search repo`). Homeport keeps no list of its own: these
//! are the repositories `helm repo list` shows in a terminal. Repositories
//! needing a login are added in a terminal, where helm keeps their
//! credentials (ADR 0009).

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::Helm;
use crate::{messages as msg, proxy::CliEvent};

/// A chart repository helm knows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ChartRepository {
    pub name: String,
    pub url: String,
}

/// A chart (or one version of it) found in the repositories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ChartVersion {
    /// `repository/chart`, as helm installs it.
    pub name: String,
    pub version: String,
    pub app_version: String,
    pub description: String,
}

#[derive(Deserialize)]
struct RawChart {
    name: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    app_version: String,
    #[serde(default)]
    description: String,
}

impl From<RawChart> for ChartVersion {
    fn from(raw: RawChart) -> Self {
        Self {
            name: raw.name,
            version: raw.version,
            app_version: raw.app_version,
            description: raw.description,
        }
    }
}

/// Checks a repository's name and URL before helm sees them.
fn validate(name: &str, url: &str) -> AppResult<()> {
    let name_ok = !name.is_empty()
        && name.len() <= 100
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
    if !name_ok {
        return Err(AppError::validation(
            msg::HELM_REPO_NAME_INVALID.with("name", name),
        ));
    }
    let lower = url.to_ascii_lowercase();
    if !(lower.starts_with("https://") || lower.starts_with("http://")) || url.contains(' ') {
        return Err(
            AppError::validation(msg::HELM_REPO_URL_INVALID.with("url", url))
                .with_hint(msg::HELM_REPO_URL_INVALID_HINT),
        );
    }
    if url.contains('@') {
        return Err(AppError::validation(msg::HELM_REPO_URL_CREDENTIALS)
            .with_hint(msg::HELM_REPO_URL_CREDENTIALS_HINT));
    }
    Ok(())
}

/// Whether helm refused because it has no repository at all, which
/// `helm repo list` reports as an error.
fn no_repositories(error: &AppError) -> bool {
    error.is(ErrorCode::NotFound)
        && error
            .detail
            .as_deref()
            .is_some_and(|d| d.contains("no repositories"))
}

impl Helm {
    /// The repositories helm knows, in its order.
    pub async fn repositories(&self) -> AppResult<Vec<ChartRepository>> {
        match self.json(vec!["repo".into(), "list".into()]).await {
            Err(e) if no_repositories(&e) => Ok(vec![]),
            other => other,
        }
    }

    /// Adds a public repository and downloads its index.
    pub async fn add_repository(&self, name: &str, url: &str) -> AppResult<()> {
        let (name, url) = (name.trim(), url.trim());
        validate(name, url)?;
        self.text(
            vec!["repo".into(), "add".into(), name.into(), url.into()],
            None,
        )
        .await
        .map(drop)
    }

    pub async fn remove_repository(&self, name: &str) -> AppResult<()> {
        self.text(vec!["repo".into(), "remove".into(), name.into()], None)
            .await
            .map(drop)
    }

    /// Downloads the indexes of `names`, or of every repository, streaming
    /// helm's output.
    pub async fn update_repositories(self, names: Vec<String>, sink: impl Fn(CliEvent) -> bool) {
        let mut args = vec!["repo".to_owned(), "update".to_owned()];
        args.extend(names);
        self.stream(args, None, sink).await;
    }

    /// Charts whose name or description matches `query` (every chart when
    /// empty), at their latest version, or every version with `versions`.
    pub async fn search_charts(&self, query: &str, versions: bool) -> AppResult<Vec<ChartVersion>> {
        let mut args = vec!["search".to_owned(), "repo".to_owned()];
        if !query.trim().is_empty() {
            args.push(query.trim().to_owned());
        }
        if versions {
            args.push("--versions".into());
        }
        let raw: Vec<RawChart> = match self.json(args).await {
            Err(e) if no_repositories(&e) => vec![],
            other => other?,
        };
        Ok(raw.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helm_repo_list_and_search_are_read() {
        let repos: Vec<ChartRepository> = serde_json::from_str(
            r#"[{"name":"bitnami","url":"https://charts.bitnami.com/bitnami"},{"name":"jetstack","url":"https://charts.jetstack.io"}]"#,
        )
        .unwrap();
        assert_eq!(repos[1].name, "jetstack");
        let raw: Vec<RawChart> = serde_json::from_str(
            r#"[{"name":"bitnami/postgresql","version":"16.2.1","app_version":"17.2.0","description":"PostgreSQL (Postgres) is an open source object-relational database"},{"name":"bitnami/postgresql-ha","version":"15.1.2","app_version":"17.2.0","description":"This PostgreSQL cluster solution"}]"#,
        )
        .unwrap();
        let charts: Vec<ChartVersion> = raw.into_iter().map(Into::into).collect();
        assert_eq!(charts[0].name, "bitnami/postgresql");
        assert_eq!(charts[0].app_version, "17.2.0");
    }

    #[test]
    fn repositories_are_checked_before_helm_sees_them() {
        assert!(validate("bitnami", "https://charts.bitnami.com/bitnami").is_ok());
        assert!(validate("my_repo.1", "http://127.0.0.1:8879").is_ok());
        for (name, url) in [
            ("", "https://x"),
            ("bit nami", "https://x"),
            ("a/b", "https://x"),
            ("ok", "oci://registry-1.docker.io/bitnamicharts"),
            ("ok", "charts.example.com"),
            ("ok", "https://user:token@charts.example.com"),
        ] {
            assert!(
                validate(name, url).unwrap_err().is(ErrorCode::Validation),
                "{name} {url}"
            );
        }
    }

    #[test]
    fn having_no_repository_isnt_an_error() {
        let e = super::super::failure(
            "repo",
            None,
            crate::proxy::Failure::Exit {
                code: Some(1),
                stderr: "Error: no repositories to show",
            },
        );
        assert!(no_repositories(&e));
    }
}
