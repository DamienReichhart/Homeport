//! Checking a login from this machine, the way `docker login` does: ask the
//! registry's `/v2/` endpoint, and follow its `WWW-Authenticate` challenge
//! with the credentials. Bollard has no `/auth` call, and this needs nothing
//! on any Host.

use reqwest::{header::WWW_AUTHENTICATE, Client, StatusCode};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

use crate::{address, http, Credentials};

/// Succeeds when the registry accepts the credentials.
pub async fn check(credentials: &Credentials) -> AppResult<()> {
    let client = http::client()?;
    let address = credentials.address.as_str();
    let (base, response) = probe(&client, address).await?;
    let challenge = match response.status() {
        s if s.is_success() => return Ok(()),
        StatusCode::UNAUTHORIZED => response
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok())
            .map(Challenge::parse)
            .unwrap_or(Challenge::Basic),
        StatusCode::TOO_MANY_REQUESTS => return Err(http::rate_limited(address)),
        s => return Err(http::unexpected(address, s)),
    };
    let request = match &challenge {
        Challenge::Bearer { realm, service } => {
            let mut query = vec![("account", credentials.username.as_str())];
            if let Some(service) = service {
                query.push(("service", service));
            }
            client.get(realm).query(&query)
        }
        Challenge::Basic => client.get(format!("{base}/v2/")),
    };
    let status = request
        .basic_auth(&credentials.username, Some(&credentials.secret))
        .send()
        .await
        .map_err(|e| http::unreachable(address, &e))?
        .status();
    match status {
        s if s.is_success() => Ok(()),
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(login_failed(address)),
        StatusCode::TOO_MANY_REQUESTS => Err(http::rate_limited(address)),
        s => Err(http::unexpected(address, s)),
    }
}

/// `GET /v2/` over HTTPS, or plain HTTP for a registry on this machine
/// (Docker allows that for loopback addresses only).
async fn probe(client: &Client, address: &str) -> AppResult<(String, reqwest::Response)> {
    let host = address::api_host(address);
    let https = format!("https://{host}");
    match client.get(format!("{https}/v2/")).send().await {
        Ok(response) => Ok((https, response)),
        Err(e) if address::is_loopback(address) && e.is_connect() => {
            let plain = format!("http://{host}");
            let response = client
                .get(format!("{plain}/v2/"))
                .send()
                .await
                .map_err(|e| http::unreachable(address, &e))?;
            Ok((plain, response))
        }
        Err(e) => Err(http::unreachable(address, &e)),
    }
}

pub fn login_failed(address: &str) -> AppError {
    AppError::new(
        ErrorCode::RegistryLoginFailed,
        msg::LOGIN_FAILED.with("address", address),
    )
    .with_hint(msg::LOGIN_FAILED_HINT)
}

/// How a registry asks to be authenticated.
#[derive(Debug, PartialEq, Eq)]
enum Challenge {
    /// Get a token from `realm` with the credentials (Docker Hub, GHCR, most registries).
    Bearer {
        realm: String,
        service: Option<String>,
    },
    /// Send the credentials with every request (`registry:2` with htpasswd).
    Basic,
}

impl Challenge {
    /// Reads `Bearer realm="…",service="…",scope="…"`; anything else is Basic.
    fn parse(header: &str) -> Self {
        let Some(params) = header
            .trim()
            .strip_prefix("Bearer ")
            .or_else(|| header.trim().strip_prefix("bearer "))
        else {
            return Self::Basic;
        };
        let params = parse_params(params);
        let get = |key: &str| {
            params
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(key))
                .map(|(_, v)| v.clone())
        };
        match get("realm") {
            Some(realm) => Self::Bearer {
                realm,
                service: get("service"),
            },
            None => Self::Basic,
        }
    }
}

/// `key="value", key=value` pairs; quoted values may hold commas.
fn parse_params(input: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();
    let mut rest = input.trim();
    while !rest.is_empty() {
        let Some((key, after)) = rest.split_once('=') else {
            break;
        };
        let key = key.trim().trim_start_matches(',').trim().to_owned();
        let after = after.trim_start();
        let (value, remaining) = if let Some(quoted) = after.strip_prefix('"') {
            match quoted.find('"') {
                Some(end) => (quoted[..end].to_owned(), &quoted[end + 1..]),
                None => (quoted.to_owned(), ""),
            }
        } else {
            match after.find(',') {
                Some(end) => (after[..end].trim().to_owned(), &after[end..]),
                None => (after.trim().to_owned(), ""),
            }
        };
        params.push((key, value));
        rest = remaining.trim_start().trim_start_matches(',').trim_start();
    }
    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_challenges_name_their_token_service() {
        assert_eq!(
            Challenge::parse(
                r#"Bearer realm="https://auth.docker.io/token",service="registry.docker.io""#
            ),
            Challenge::Bearer {
                realm: "https://auth.docker.io/token".into(),
                service: Some("registry.docker.io".into()),
            }
        );
        assert_eq!(
            Challenge::parse(
                r#"Bearer realm="https://ghcr.io/token", service="ghcr.io", scope="repository:a/b:pull,push""#
            ),
            Challenge::Bearer {
                realm: "https://ghcr.io/token".into(),
                service: Some("ghcr.io".into()),
            }
        );
        assert_eq!(
            Challenge::parse("Bearer realm=https://r.example/token"),
            Challenge::Bearer {
                realm: "https://r.example/token".into(),
                service: None,
            }
        );
    }

    #[test]
    fn other_challenges_are_basic() {
        assert_eq!(
            Challenge::parse(r#"Basic realm="Registry Realm""#),
            Challenge::Basic
        );
        assert_eq!(Challenge::parse("Bearer service=x"), Challenge::Basic);
        assert_eq!(Challenge::parse(""), Challenge::Basic);
    }

    #[test]
    fn failed_logins_name_the_registry_and_keep_secrets_out() {
        let err = login_failed("ghcr.io");
        assert!(err.is(ErrorCode::RegistryLoginFailed));
        assert!(err.message.contains("ghcr.io"));
        assert!(err.detail.is_none());
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn logins_are_checked_against_a_real_registry() {
        use homeport_engine::testing;
        let docker = testing::docker();
        let registry = testing::registry(&docker, true).await;
        let login = |secret: &str| Credentials {
            address: registry.address.clone(),
            username: testing::REGISTRY_USER.into(),
            secret: secret.into(),
        };
        check(&login(testing::REGISTRY_PASSWORD)).await.unwrap();
        let err = check(&login("wrong")).await.unwrap_err();
        assert!(err.is(ErrorCode::RegistryLoginFailed), "{err:?}");
    }

    #[tokio::test]
    async fn unreachable_registries_are_reported_as_such() {
        let err = check(&Credentials {
            address: "127.0.0.1:1".into(),
            username: "a".into(),
            secret: "b".into(),
        })
        .await
        .unwrap_err();
        assert!(err.is(ErrorCode::RegistryUnreachable), "{err:?}");
        assert!(!format!("{err:?}").contains("\"b\""));
    }
}
