//! Docker Hub's tag lists, read from this machine through its HTTP API. The
//! Engine API can search Docker Hub but not list a repository's tags.

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult};

use crate::{http, Credentials, DOCKER_HUB};

const HUB: &str = "https://hub.docker.com";
const PAGE_SIZE: u32 = 50;

/// One tag of a Docker Hub repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HubTag {
    pub name: String,
    /// Compressed size of the image for one platform, in bytes.
    pub size: Option<i64>,
    /// ISO 8601, when the tag was last pushed.
    pub last_pushed: Option<String>,
}

/// A page of tags, newest first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HubTagPage {
    pub tags: Vec<HubTag>,
    /// The number of tags matching, across every page.
    pub count: u32,
    /// The page to ask for next, if any.
    pub next_page: Option<u32>,
}

#[derive(Deserialize)]
struct RawPage {
    #[serde(default)]
    count: u32,
    next: Option<String>,
    #[serde(default)]
    results: Vec<RawTag>,
}

#[derive(Deserialize)]
struct RawTag {
    name: String,
    full_size: Option<i64>,
    tag_last_pushed: Option<String>,
    last_updated: Option<String>,
}

/// `postgres` → `("library", "postgres")`, `bitnami/redis` → `("bitnami", "redis")`,
/// `docker.io/library/nginx:1.27` → `("library", "nginx")`. `None` for
/// references to other registries or that aren't a repository name.
pub fn repository(reference: &str) -> Option<(String, String)> {
    let reference = reference.trim().to_lowercase();
    let reference = reference.split('@').next().unwrap_or_default();
    if crate::registry_of(reference) != DOCKER_HUB {
        return None;
    }
    let reference = ["docker.io/", "index.docker.io/", "registry-1.docker.io/"]
        .iter()
        .find_map(|p| reference.strip_prefix(p))
        .unwrap_or(reference);
    // The tag follows the last `:`; no port can be left at this point.
    let name = reference.split(':').next().unwrap_or_default();
    let valid = |s: &str| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "._-".contains(c))
    };
    match name.split('/').collect::<Vec<_>>()[..] {
        [repo] if valid(repo) => Some(("library".into(), repo.into())),
        [ns, repo] if valid(ns) && valid(repo) => Some((ns.into(), repo.into())),
        _ => None,
    }
}

/// A page of the tags of `reference`'s Docker Hub repository, newest first,
/// whose names contain `filter`. A stored Docker Hub login raises the limits.
pub async fn tags(
    reference: &str,
    page: u32,
    filter: &str,
    login: Option<&Credentials>,
) -> AppResult<HubTagPage> {
    let (namespace, repo) = repository(reference).ok_or_else(|| {
        AppError::validation("Tags can only be listed for Docker Hub repositories.")
    })?;
    let client = http::client()?;
    let mut request = client
        .get(format!(
            "{HUB}/v2/namespaces/{namespace}/repositories/{repo}/tags"
        ))
        .query(&[
            ("page", page.max(1).to_string()),
            ("page_size", PAGE_SIZE.to_string()),
            ("ordering", "last_updated".to_owned()),
        ]);
    if !filter.trim().is_empty() {
        request = request.query(&[("name", filter.trim())]);
    }
    if let Some(token) = match login {
        Some(login) => token(&client, login).await,
        None => None,
    } {
        request = request.bearer_auth(token);
    }
    let response = request
        .send()
        .await
        .map_err(|e| http::unreachable("Docker Hub", &e))?;
    match response.status() {
        s if s.is_success() => {}
        StatusCode::NOT_FOUND => {
            let shown = if namespace == "library" {
                repo
            } else {
                format!("{namespace}/{repo}")
            };
            return Err(AppError::not_found(format!(
                "Docker Hub has no repository named {shown}."
            )));
        }
        StatusCode::TOO_MANY_REQUESTS => return Err(http::rate_limited("Docker Hub")),
        s => return Err(http::unexpected("Docker Hub", s)),
    }
    let raw: RawPage = response
        .json()
        .await
        .map_err(|e| http::unreachable("Docker Hub", &e))?;
    Ok(page_of(raw, page.max(1)))
}

fn page_of(raw: RawPage, page: u32) -> HubTagPage {
    HubTagPage {
        count: raw.count,
        next_page: raw.next.is_some().then_some(page + 1),
        tags: raw
            .results
            .into_iter()
            .map(|t| HubTag {
                name: t.name,
                size: t.full_size,
                last_pushed: t.tag_last_pushed.or(t.last_updated),
            })
            .collect(),
    }
}

/// A Docker Hub API token for the stored login; `None` (anonymous) when
/// Docker Hub refuses it, so listing tags still works.
async fn token(client: &reqwest::Client, login: &Credentials) -> Option<String> {
    #[derive(Deserialize)]
    struct Token {
        access_token: Option<String>,
        token: Option<String>,
    }
    let response = client
        .post(format!("{HUB}/v2/auth/token"))
        .json(&serde_json::json!({
            "identifier": login.username,
            "secret": login.secret,
        }))
        .send()
        .await
        .ok()?;
    if !response.status().is_success() {
        log::warn!(
            "Docker Hub refused the stored login for listing tags (HTTP {}); listing anonymously",
            response.status()
        );
        return None;
    }
    let token: Token = response.json().await.ok()?;
    token.access_token.or(token.token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_names_follow_docker_hub() {
        let r = |s| repository(s);
        assert_eq!(r("postgres"), Some(("library".into(), "postgres".into())));
        assert_eq!(
            r("postgres:17-alpine"),
            Some(("library".into(), "postgres".into()))
        );
        assert_eq!(r("Bitnami/Redis"), Some(("bitnami".into(), "redis".into())));
        assert_eq!(
            r("docker.io/library/nginx@sha256:abc"),
            Some(("library".into(), "nginx".into()))
        );
        assert_eq!(r("ghcr.io/org/app"), None);
        assert_eq!(r("localhost:5000/app"), None);
        assert_eq!(r("a/b/c"), None);
        assert_eq!(r(""), None);
        assert_eq!(r("bad name"), None);
    }

    #[test]
    fn hub_pages_parse_from_a_fixture() {
        let raw: RawPage = serde_json::from_str(
            r#"{
                "count": 1433,
                "next": "https://hub.docker.com/v2/namespaces/library/repositories/postgres/tags?page=3",
                "previous": null,
                "results": [
                    {"name": "17-alpine", "full_size": 109000000, "tag_last_pushed": "2026-11-01T10:08:04Z", "last_updated": "2026-11-01T10:08:05Z", "images": []},
                    {"name": "old", "full_size": null, "last_updated": "2020-01-01T00:00:00Z"}
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(
            page_of(raw, 2),
            HubTagPage {
                count: 1433,
                next_page: Some(3),
                tags: vec![
                    HubTag {
                        name: "17-alpine".into(),
                        size: Some(109_000_000),
                        last_pushed: Some("2026-11-01T10:08:04Z".into()),
                    },
                    HubTag {
                        name: "old".into(),
                        size: None,
                        last_pushed: Some("2020-01-01T00:00:00Z".into()),
                    },
                ],
            }
        );
        let last: RawPage = serde_json::from_str(r#"{"count": 0, "next": null}"#).unwrap();
        assert_eq!(page_of(last, 1).next_page, None);
    }
}
