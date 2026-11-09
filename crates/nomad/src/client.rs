//! The connection to one Nomad Cluster's HTTP API, made in memory from its
//! address, region, ACL token and TLS material: nothing is written to disk.
//! The token travels in the `X-Nomad-Token` header, never in a URL.

use std::{sync::Arc, time::Duration};

use reqwest::{Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{
    error::{answer, unreachable, unreadable},
    messages as msg,
    tls::{self, TlsMaterial},
};

/// How long connecting to the agent may take.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// How long a request that isn't a blocking query or a stream may take.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
/// The header Nomad reads the ACL token from.
pub const TOKEN_HEADER: &str = "X-Nomad-Token";
/// The header Nomad answers a blocking query's index in.
const INDEX_HEADER: &str = "X-Nomad-Index";

/// Everything needed to reach a Nomad Cluster. `name` names it in messages.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct NomadConfig {
    pub name: String,
    /// A normalised address, such as `https://nomad.example.com:4646`.
    pub address: String,
    /// The region to ask, when not the agent's own.
    pub region: Option<String>,
    /// The name to check the agent's certificate against, when not the address's.
    pub tls_server_name: Option<String>,
    pub token: Option<String>,
    pub certificate_authority: Option<String>,
    pub client_certificate: Option<String>,
    pub client_key: Option<String>,
}

impl std::fmt::Debug for NomadConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NomadConfig")
            .field("name", &self.name)
            .field("address", &self.address)
            .field("region", &self.region)
            .finish_non_exhaustive()
    }
}

/// A client for one Nomad Cluster; cheap to clone.
#[derive(Clone)]
pub struct Nomad(Arc<Inner>);

struct Inner {
    http: reqwest::Client,
    name: String,
    address: String,
    region: Option<String>,
    token: Option<String>,
}

/// An answer to a blocking query, with the index to wait from next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indexed<T> {
    pub value: T,
    pub index: u64,
}

/// Empty text counts as none.
fn given(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

/// `segment` escaped for a URL path, so a job ID such as
/// `backup/periodic-1700000000` stays one segment.
pub fn segment(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for b in segment.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

impl Nomad {
    /// A client for `config`; fails when its TLS material can't be used.
    pub fn new(config: &NomadConfig) -> AppResult<Self> {
        let tls = tls::config(&TlsMaterial {
            name: &config.name,
            certificate_authority: config.certificate_authority.as_deref(),
            client_certificate: config.client_certificate.as_deref(),
            client_key: config.client_key.as_deref(),
            server_name: config.tls_server_name.as_deref(),
        })?;
        let http = reqwest::Client::builder()
            .user_agent(concat!("Homeport/", env!("CARGO_PKG_VERSION")))
            .connect_timeout(CONNECT_TIMEOUT)
            .tls_backend_preconfigured(tls)
            .build()
            .map_err(|e| AppError::new(ErrorCode::Io, msg::HTTPS_CLIENT).with_cause(&e))?;
        Ok(Self(Arc::new(Inner {
            http,
            name: config.name.clone(),
            address: config.address.trim_end_matches('/').to_owned(),
            region: given(&config.region),
            token: given(&config.token),
        })))
    }

    /// The Cluster's name, for messages.
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// The agent's address, such as `http://127.0.0.1:4646`.
    pub fn address(&self) -> &str {
        &self.0.address
    }

    pub(crate) fn region(&self) -> Option<&str> {
        self.0.region.as_deref()
    }

    pub(crate) fn token(&self) -> Option<&str> {
        self.0.token.as_deref()
    }

    /// A request to `path` with the token and the region, timed out after
    /// [`REQUEST_TIMEOUT`].
    pub(crate) fn request(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
    ) -> RequestBuilder {
        let mut request = self
            .0
            .http
            .request(method, format!("{}{path}", self.0.address))
            .timeout(REQUEST_TIMEOUT)
            .query(query);
        if let Some(region) = self.region() {
            request = request.query(&[("region", region)]);
        }
        if let Some(token) = self.token() {
            request = request.header(TOKEN_HEADER, token);
        }
        request
    }

    /// Sends `request`; an error for no answer or an error status.
    pub(crate) async fn send(&self, request: RequestBuilder) -> AppResult<Response> {
        let response = request
            .send()
            .await
            .map_err(|e| unreachable(self.name(), self.address(), &e))?;
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let text = response.text().await.unwrap_or_default();
        Err(answer(self.name(), status.as_u16(), &text))
    }

    /// The JSON body of `response`.
    pub(crate) async fn read<T: DeserializeOwned>(&self, response: Response) -> AppResult<T> {
        let bytes = response
            .bytes()
            .await
            .map_err(|e| unreachable(self.name(), self.address(), &e))?;
        serde_json::from_slice(&bytes).map_err(|e| unreadable(self.name(), &e))
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> AppResult<T> {
        let response = self.send(self.request(Method::GET, path, query)).await?;
        self.read(response).await
    }

    /// A blocking query: answers once Nomad's index for `path` passes
    /// `index`, or after `wait` with the same index. `index` 0 answers at once.
    pub async fn get_indexed<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
        index: u64,
        wait: Duration,
    ) -> AppResult<Indexed<T>> {
        let mut request = self.request(Method::GET, path, query);
        if index > 0 {
            request = request
                .query(&[
                    ("index", index.to_string()),
                    ("wait", format!("{}s", wait.as_secs())),
                ])
                // Nomad adds up to a sixteenth of `wait` as jitter.
                .timeout(wait + wait / 8 + REQUEST_TIMEOUT);
        }
        let response = self.send(request).await?;
        let index = response
            .headers()
            .get(INDEX_HEADER)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(index);
        Ok(Indexed {
            value: self.read(response).await?,
            index,
        })
    }

    pub(crate) async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
        body: &impl Serialize,
    ) -> AppResult<T> {
        let response = self
            .send(self.request(Method::POST, path, query).json(body))
            .await?;
        self.read(response).await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> AppResult<T> {
        let response = self.send(self.request(Method::DELETE, path, query)).await?;
        self.read(response).await
    }
}

#[cfg(test)]
pub(crate) mod testing {
    //! A fake Nomad agent answering canned responses by path, recording
    //! what it was sent.

    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    use super::*;

    /// A request the fake agent received.
    #[derive(Debug, Clone)]
    pub(crate) struct Received {
        pub method: String,
        /// The path and query, as sent.
        pub target: String,
        pub headers: HashMap<String, String>,
        pub body: String,
    }

    /// A canned answer: `METHOD /path`, status, extra headers and body.
    pub(crate) type Route = (&'static str, u16, Vec<(&'static str, String)>, String);

    pub(crate) struct Agent {
        pub address: String,
        pub received: Arc<Mutex<Vec<Received>>>,
    }

    impl Agent {
        pub(crate) fn received(&self) -> Vec<Received> {
            self.received.lock().unwrap().clone()
        }

        pub(crate) fn client(&self, token: Option<&str>) -> Nomad {
            Nomad::new(&NomadConfig {
                name: "dev".into(),
                address: self.address.clone(),
                token: token.map(str::to_owned),
                ..Default::default()
            })
            .unwrap()
        }
    }

    /// An agent answering `METHOD /path` (without the query) with a status,
    /// headers and a body; anything else gets 404.
    pub(crate) async fn agent(routes: Vec<Route>) -> Agent {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let received = Arc::new(Mutex::new(vec![]));
        let routes = Arc::new(routes);
        let log = received.clone();
        tokio::spawn(async move {
            loop {
                let Ok((mut socket, _)) = listener.accept().await else {
                    return;
                };
                let routes = routes.clone();
                let log = log.clone();
                tokio::spawn(async move {
                    let mut buf = Vec::new();
                    loop {
                        let mut chunk = [0; 8192];
                        let Ok(n) = socket.read(&mut chunk).await else {
                            return;
                        };
                        if n == 0 {
                            return;
                        }
                        buf.extend_from_slice(&chunk[..n]);
                        let Some(end) = buf.windows(4).position(|w| w == b"\r\n\r\n") else {
                            continue;
                        };
                        let head = String::from_utf8_lossy(&buf[..end]).into_owned();
                        let mut lines = head.lines();
                        let mut first = lines.next().unwrap_or_default().split(' ');
                        let method = first.next().unwrap_or_default().to_owned();
                        let target = first.next().unwrap_or_default().to_owned();
                        let headers: HashMap<String, String> = lines
                            .filter_map(|l| l.split_once(':'))
                            .map(|(k, v)| (k.trim().to_lowercase(), v.trim().to_owned()))
                            .collect();
                        let length: usize = headers
                            .get("content-length")
                            .and_then(|l| l.parse().ok())
                            .unwrap_or(0);
                        while buf.len() < end + 4 + length {
                            let mut chunk = [0; 8192];
                            match socket.read(&mut chunk).await {
                                Ok(0) | Err(_) => return,
                                Ok(n) => buf.extend_from_slice(&chunk[..n]),
                            }
                        }
                        let body =
                            String::from_utf8_lossy(&buf[end + 4..end + 4 + length]).into_owned();
                        buf.drain(..end + 4 + length);
                        let path = target.split('?').next().unwrap_or_default();
                        let key = format!("{method} {path}");
                        log.lock().unwrap().push(Received {
                            method,
                            target: target.clone(),
                            headers,
                            body,
                        });
                        let (status, extra, body) = routes
                            .iter()
                            .find(|(route, ..)| *route == key)
                            .map(|(_, s, h, b)| (*s, h.clone(), b.clone()))
                            .unwrap_or((404, vec![], "no route".into()));
                        let mut reply =
                            format!("HTTP/1.1 {status} X\r\ncontent-length: {}\r\n", body.len());
                        let count = log.lock().unwrap().len();
                        for (k, v) in extra {
                            // `$COUNT` stands for how many requests came so far.
                            let v = v.replace("$COUNT", &count.to_string());
                            reply.push_str(&format!("{k}: {v}\r\n"));
                        }
                        reply.push_str("\r\n");
                        reply.push_str(&body);
                        if socket.write_all(reply.as_bytes()).await.is_err() {
                            return;
                        }
                    }
                });
            }
        });
        Agent { address, received }
    }

    /// A route answering 200 with `body`.
    pub(crate) fn ok(route: &'static str, body: impl Into<String>) -> Route {
        (route, 200, vec![], body.into())
    }
}

#[cfg(test)]
mod tests {
    use super::testing::{agent, ok};
    use super::*;

    #[test]
    fn path_segments_are_escaped() {
        assert_eq!(segment("web"), "web");
        assert_eq!(
            segment("backup/periodic-1700000000"),
            "backup%2Fperiodic-1700000000"
        );
        assert_eq!(segment("a b?"), "a%20b%3F");
    }

    #[test]
    fn the_config_never_prints_its_secrets() {
        let config = NomadConfig {
            name: "prod".into(),
            token: Some("secret-token".into()),
            client_key: Some("PRIVATE KEY".into()),
            ..Default::default()
        };
        let printed = format!("{config:?}");
        assert!(!printed.contains("secret-token") && !printed.contains("PRIVATE"));
    }

    #[tokio::test]
    async fn the_token_goes_in_a_header_and_the_region_in_the_query() {
        let fake = agent(vec![ok("GET /v1/regions", r#"["eu"]"#)]).await;
        let nomad = Nomad::new(&NomadConfig {
            name: "prod".into(),
            address: fake.address.clone(),
            region: Some("eu".into()),
            token: Some("secret-token".into()),
            ..Default::default()
        })
        .unwrap();
        let regions: Vec<String> = nomad.get("/v1/regions", &[]).await.unwrap();
        assert_eq!(regions, ["eu"]);
        let sent = &fake.received()[0];
        assert_eq!((sent.method.as_str(), sent.body.as_str()), ("GET", ""));
        assert_eq!(sent.headers["x-nomad-token"], "secret-token");
        assert!(sent.target.contains("region=eu"), "{}", sent.target);
        assert!(!sent.target.contains("secret-token"));
    }

    #[tokio::test]
    async fn no_token_sends_no_header() {
        let fake = agent(vec![ok("GET /v1/regions", "[]")]).await;
        let _: Vec<String> = fake
            .client(Some("  "))
            .get("/v1/regions", &[])
            .await
            .unwrap();
        assert!(!fake.received()[0].headers.contains_key("x-nomad-token"));
    }

    #[tokio::test]
    async fn error_answers_are_classified() {
        let fake = agent(vec![(
            "GET /v1/jobs",
            403,
            vec![],
            "ACL token not found".into(),
        )])
        .await;
        let e = fake
            .client(Some("t"))
            .get::<serde_json::Value>("/v1/jobs", &[])
            .await
            .unwrap_err();
        assert!(e.is(ErrorCode::NomadTokenInvalid));
    }

    #[tokio::test]
    async fn an_answer_that_isnt_json_is_unreadable() {
        let fake = agent(vec![ok("GET /v1/jobs", "<html>")]).await;
        let e = fake
            .client(None)
            .get::<serde_json::Value>("/v1/jobs", &[])
            .await
            .unwrap_err();
        assert_eq!(e.key, "errors.nomad.unreadable");
    }

    #[tokio::test]
    async fn a_blocking_query_sends_its_index_and_reads_the_next() {
        let fake = agent(vec![(
            "GET /v1/jobs",
            200,
            vec![("X-Nomad-Index", "42".into())],
            "[]".into(),
        )])
        .await;
        let nomad = fake.client(None);
        let first: Indexed<Vec<serde_json::Value>> = nomad
            .get_indexed("/v1/jobs", &[], 0, Duration::from_secs(30))
            .await
            .unwrap();
        assert_eq!(first.index, 42);
        nomad
            .get_indexed::<Vec<serde_json::Value>>("/v1/jobs", &[], 42, Duration::from_secs(30))
            .await
            .unwrap();
        let sent = fake.received();
        assert!(!sent[0].target.contains("index="));
        assert!(sent[1].target.contains("index=42") && sent[1].target.contains("wait=30s"));
    }
}
