//! A private local proxy to a Cluster's API, and a temporary kubeconfig
//! pointing at it, so a CLI such as `helm` reaches the Cluster without any
//! Cluster credential written to disk (ADR 0009).
//!
//! The proxy listens on `127.0.0.1` on a random port and only serves
//! requests carrying a random per-proxy bearer token, which is in the
//! kubeconfig and nowhere else. Each request goes on through the Cluster's
//! authenticated kube-rs client, asked for afresh so renewed exec-plugin
//! tokens are used. Streaming responses and upgraded connections (SPDY and
//! WebSocket) pass through. Dropping the [`ClusterProxy`] stops the listener
//! and deletes the kubeconfig's folder.

mod cli;
#[cfg(test)]
mod tests;

use std::{
    convert::Infallible,
    error::Error as StdError,
    io,
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::Arc,
};

use base64::Engine as _;
use bytes::Bytes;
use http::{header, HeaderMap, HeaderValue, Request, Response, StatusCode};
use http_body_util::{combinators::UnsyncBoxBody, BodyExt, Full};
use hyper::body::Incoming;
use hyper_util::rt::TokioIo;
use tokio::{net::TcpListener, task::JoinHandle};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::{messages as msg, model::Cluster};

pub use self::cli::{output, run_cli, CliCommand, CliEvent, ClusterAccess, Failure};

/// Where the proxy gets the Cluster's current client from: the registry's
/// supervisor in the app, a fixed client in tests.
pub type ClientSource = Arc<dyn Fn() -> AppResult<kube::Client> + Send + Sync>;

type ProxyBody = UnsyncBoxBody<Bytes, Box<dyn StdError + Send + Sync>>;

/// A running proxy and its kubeconfig. Dropping it stops the one and
/// deletes the other.
pub struct ClusterProxy {
    address: SocketAddr,
    kubeconfig: PathBuf,
    accept: JoinHandle<()>,
    // Deleted on drop, with the kubeconfig in it.
    _dir: tempfile::TempDir,
}

impl ClusterProxy {
    /// The kubeconfig to hand the CLI (`--kubeconfig`, `KUBECONFIG`).
    pub fn kubeconfig(&self) -> &Path {
        &self.kubeconfig
    }

    /// Where the proxy listens, on loopback.
    pub fn address(&self) -> SocketAddr {
        self.address
    }
}

impl Drop for ClusterProxy {
    fn drop(&mut self) {
        self.accept.abort();
    }
}

struct Shared {
    /// `Bearer <token>`, as the Authorization header must carry it.
    expected: Vec<u8>,
    clients: ClientSource,
}

/// Opens a proxy to `cluster`, whose requests go through the client
/// `clients` returns, and writes its kubeconfig. Must run inside a Tokio
/// runtime.
pub async fn open(cluster: &Cluster, clients: ClientSource) -> AppResult<ClusterProxy> {
    let failed = |e: &io::Error| {
        AppError::new(
            ErrorCode::ClusterProxyFailed,
            msg::PROXY_FAILED.with("name", &cluster.name),
        )
        .with_hint(msg::PROXY_FAILED_HINT)
        .with_cause(e)
    };
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .map_err(|e| failed(&e))?;
    let address = listener.local_addr().map_err(|e| failed(&e))?;
    let token = token();
    let dir = private_dir().map_err(|e| failed(&e))?;
    let kubeconfig = dir.path().join("kubeconfig");
    let namespace = cluster.connection.namespace.as_deref().unwrap_or("default");
    write_private(&kubeconfig, &kubeconfig_yaml(address, &token, namespace))
        .map_err(|e| failed(&e))?;

    let shared = Arc::new(Shared {
        expected: format!("Bearer {token}").into_bytes(),
        clients,
    });
    let accept = tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let shared = shared.clone();
            tokio::spawn(async move {
                let service = hyper::service::service_fn(move |req| {
                    let shared = shared.clone();
                    async move { Ok::<_, Infallible>(handle(req, &shared).await) }
                });
                if let Err(e) = hyper::server::conn::http1::Builder::new()
                    .serve_connection(TokioIo::new(stream), service)
                    .with_upgrades()
                    .await
                {
                    // Only the error is logged, never what the connection carried.
                    log::debug!("cluster proxy connection ended: {e}");
                }
            });
        }
    });
    Ok(ClusterProxy {
        address,
        kubeconfig,
        accept,
        _dir: dir,
    })
}

/// A random 256-bit token, URL-safe.
fn token() -> String {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).expect("OS random number generator failed");
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// The kubeconfig of a proxy at `address`: no Cluster credential, only the
/// proxy's own token.
fn kubeconfig_yaml(address: SocketAddr, token: &str, namespace: &str) -> String {
    let namespace = serde_json::to_string(namespace).expect("a string serializes");
    format!(
        "apiVersion: v1\nkind: Config\nclusters:\n- name: homeport\n  cluster:\n    server: http://{address}\ncontexts:\n- name: homeport\n  context:\n    cluster: homeport\n    user: homeport\n    namespace: {namespace}\ncurrent-context: homeport\nusers:\n- name: homeport\n  user:\n    token: {token}\n"
    )
}

/// A fresh folder only this user can enter: 0700 on Unix, under the
/// runtime folder when there's one; on Windows, in the user's own temporary
/// folder, which only they (and administrators) can open.
fn private_dir() -> io::Result<tempfile::TempDir> {
    let base = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .filter(|p| p.is_dir())
        .unwrap_or_else(std::env::temp_dir);
    let dir = tempfile::Builder::new()
        .prefix("homeport-kube-")
        .tempdir_in(base)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))?;
    }
    Ok(dir)
}

/// Writes `text` to a new file only this user can read (0600 on Unix).
fn write_private(path: &Path, text: &str) -> io::Result<()> {
    use std::io::Write as _;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)?.write_all(text.as_bytes())
}

/// Whether `given` is the expected Authorization header, compared in
/// constant time.
fn authorized(headers: &HeaderMap, expected: &[u8]) -> bool {
    let Some(given) = headers.get(header::AUTHORIZATION) else {
        return false;
    };
    let given = given.as_bytes();
    given.len() == expected.len()
        && given
            .iter()
            .zip(expected)
            .fold(0u8, |acc, (a, b)| acc | (a ^ b))
            == 0
}

/// Headers that belong to one connection, not to the request or response.
const HOP_BY_HOP: [&str; 8] = [
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "proxy-connection",
    "te",
    "trailer",
    "transfer-encoding",
];

fn is_upgrade(headers: &HeaderMap) -> bool {
    headers.contains_key(header::UPGRADE)
}

/// Copies `from` without the hop-by-hop headers, keeping `Connection` and
/// `Upgrade` for an upgrade.
fn forwarded_headers(from: &HeaderMap, upgrade: bool) -> HeaderMap {
    let mut headers = HeaderMap::with_capacity(from.len());
    for (name, value) in from {
        let hop = HOP_BY_HOP.contains(&name.as_str());
        if hop && !(upgrade && name == header::CONNECTION) {
            continue;
        }
        headers.append(name.clone(), value.clone());
    }
    headers
}

async fn handle(mut req: Request<Incoming>, shared: &Shared) -> Response<ProxyBody> {
    if !authorized(req.headers(), &shared.expected) {
        return status(
            StatusCode::UNAUTHORIZED,
            "Unauthorized",
            "missing or wrong proxy token",
        );
    }
    let client = match (shared.clients)() {
        Ok(client) => client,
        Err(e) => {
            return status(
                StatusCode::SERVICE_UNAVAILABLE,
                "ServiceUnavailable",
                &e.message,
            )
        }
    };
    let upgrade = is_upgrade(req.headers());
    let client_upgrade = upgrade.then(|| hyper::upgrade::on(&mut req));

    let (parts, body) = req.into_parts();
    let body = match body.collect().await {
        Ok(body) => body.to_bytes(),
        Err(e) => return status(StatusCode::BAD_REQUEST, "BadRequest", &e.to_string()),
    };
    let mut headers = forwarded_headers(&parts.headers, upgrade);
    headers.remove(header::AUTHORIZATION);
    headers.remove(header::HOST);
    headers.remove(header::CONTENT_LENGTH);
    let path = parts
        .uri
        .path_and_query()
        .map_or("/", |p| p.as_str())
        .to_owned();
    let mut upstream = Request::builder().method(parts.method).uri(path);
    if let Some(h) = upstream.headers_mut() {
        *h = headers;
    }
    let upstream = match upstream.body(kube::client::Body::from(body)) {
        Ok(request) => request,
        Err(e) => return status(StatusCode::BAD_REQUEST, "BadRequest", &e.to_string()),
    };

    let mut response = match client.send(upstream).await {
        Ok(response) => response,
        Err(e) => {
            log::debug!("cluster proxy request failed: {e}");
            return status(StatusCode::BAD_GATEWAY, "BadGateway", &e.to_string());
        }
    };

    if response.status() == StatusCode::SWITCHING_PROTOCOLS {
        if let Some(client_upgrade) = client_upgrade {
            let server_upgrade = hyper::upgrade::on(&mut response);
            tokio::spawn(async move {
                match tokio::try_join!(client_upgrade, server_upgrade) {
                    Ok((client, server)) => {
                        let (mut client, mut server) = (TokioIo::new(client), TokioIo::new(server));
                        if let Err(e) =
                            tokio::io::copy_bidirectional(&mut client, &mut server).await
                        {
                            log::debug!("cluster proxy upgraded connection ended: {e}");
                        }
                    }
                    Err(e) => log::debug!("cluster proxy couldn't upgrade a connection: {e}"),
                }
            });
        }
    }

    let (parts, body) = response.into_parts();
    let upgraded = parts.status == StatusCode::SWITCHING_PROTOCOLS;
    let mut out = Response::new(body.map_err(Into::into).boxed_unsync());
    *out.status_mut() = parts.status;
    *out.headers_mut() = forwarded_headers(&parts.headers, upgraded);
    out
}

/// A Kubernetes `Status` answer the proxy makes itself.
fn status(code: StatusCode, reason: &str, message: &str) -> Response<ProxyBody> {
    let body = serde_json::json!({
        "kind": "Status",
        "apiVersion": "v1",
        "status": "Failure",
        "message": format!("homeport proxy: {message}"),
        "reason": reason,
        "code": code.as_u16(),
    })
    .to_string();
    let mut response = Response::new(
        Full::new(Bytes::from(body))
            .map_err(|never| match never {})
            .boxed_unsync(),
    );
    *response.status_mut() = code;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    response
}
