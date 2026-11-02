//! How Homeport reaches a Host's Engine API.
//!
//! Local sockets and plain TCP use bollard's own connectors. TLS and SSH go
//! through a [`Dialer`], which opens a fresh byte stream to the Engine API
//! with secrets that never leave memory (ADR 0002). The same dialer backs the
//! compose CLI proxy (ADR 0001).

pub mod ssh;
pub mod tls;

use std::{future::Future, io, pin::Pin, sync::Arc, time::Duration};

use bollard::{BollardRequest, Docker, API_DEFAULT_VERSION};
use hyper_util::rt::TokioIo;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::services::hosts::{HostConnection, HostSecrets};

/// Seconds bollard waits for a response before giving up.
const REQUEST_TIMEOUT_SECS: u64 = 120;
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

pub trait Stream: AsyncRead + AsyncWrite + Send + Unpin {}
impl<T: AsyncRead + AsyncWrite + Send + Unpin> Stream for T {}

pub type BoxStream = Box<dyn Stream>;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Opens byte streams to a Host's Engine API.
pub trait Dialer: Send + Sync + 'static {
    /// Establishes (or re-establishes) whatever session the dialer needs,
    /// classifying failures for the Host's status.
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>>;
    /// Opens one stream speaking the Engine's HTTP API.
    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>>;
}

/// Why a Host can't be reached. Each case maps to a Host status the user can act on.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConnectError {
    #[error("{0}")]
    Unreachable(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("the SSH server's key isn't trusted yet ({fingerprint})")]
    HostKeyUntrusted { key: String, fingerprint: String },
    #[error("the SSH server's key changed: expected {expected_fingerprint}, got {fingerprint}")]
    HostKeyMismatch {
        key: String,
        fingerprint: String,
        expected_fingerprint: String,
    },
    #[error("stored credentials can't be decrypted")]
    CredentialsLost,
    #[error("the OS keychain is unavailable: {0}")]
    KeychainUnavailable(String),
    #[error("{0}")]
    Invalid(String),
}

/// A live client for one Host.
#[derive(Clone)]
pub struct Connected {
    pub docker: Docker,
    /// Present for TLS and SSH Hosts; the compose proxy forwards through it.
    pub dialer: Option<Arc<dyn Dialer>>,
}

/// Builds a client for `connection` and checks the Engine answers.
pub async fn connect(
    connection: &HostConnection,
    secrets: Option<HostSecrets>,
    trusted_host_key: Option<&str>,
) -> Result<Connected, ConnectError> {
    let (docker, dialer) = match connection {
        HostConnection::Local { socket_path } => (
            // A missing socket usually means the Engine isn't running.
            Docker::connect_with_socket(socket_path, REQUEST_TIMEOUT_SECS, API_DEFAULT_VERSION)
                .map_err(|e| ConnectError::Unreachable(e.to_string()))?,
            None,
        ),
        HostConnection::Tcp { address } => (
            Docker::connect_with_http(
                &format!("tcp://{address}"),
                REQUEST_TIMEOUT_SECS,
                API_DEFAULT_VERSION,
            )
            .map_err(|e| ConnectError::Invalid(e.to_string()))?,
            None,
        ),
        HostConnection::Tls { address } => {
            let Some(HostSecrets::Tls {
                ca_pem,
                cert_pem,
                key_pem,
            }) = secrets
            else {
                return Err(ConnectError::Invalid("TLS certificates are missing".into()));
            };
            let dialer: Arc<dyn Dialer> =
                Arc::new(tls::TlsDialer::new(address, &ca_pem, &cert_pem, &key_pem)?);
            (docker_over(dialer.clone()), Some(dialer))
        }
        HostConnection::Ssh {
            host,
            port,
            user,
            socket_path,
            auth,
            ..
        } => {
            let credentials = ssh::Credentials::from_secrets(*auth, secrets)?;
            let dialer: Arc<dyn Dialer> = Arc::new(ssh::SshDialer::new(ssh::SshTarget {
                host: host.clone(),
                port: *port,
                user: user.clone(),
                socket_path: socket_path.clone(),
                credentials,
                trusted_host_key: trusted_host_key.map(str::to_owned),
            }));
            (docker_over(dialer.clone()), Some(dialer))
        }
    };
    if let Some(dialer) = &dialer {
        dialer.connect().await?;
    }
    let docker = tokio::time::timeout(CONNECT_TIMEOUT, docker.negotiate_version())
        .await
        .map_err(|_| ConnectError::Unreachable("the Engine did not answer in time".into()))?
        .map_err(|e| ConnectError::Unreachable(describe_docker_error(&e)))?;
    Ok(Connected { docker, dialer })
}

/// A bollard client whose requests travel over streams from `dialer`.
/// Each request gets its own HTTP/1.1 connection, which also carries the
/// upgraded streams used by exec and attach.
fn docker_over(dialer: Arc<dyn Dialer>) -> Docker {
    let transport = move |mut req: BollardRequest| {
        let dialer = dialer.clone();
        async move {
            let stream = dialer.dial().await?;
            let (mut sender, conn) =
                hyper::client::conn::http1::handshake(TokioIo::new(stream)).await?;
            tokio::spawn(async move {
                if let Err(e) = conn.with_upgrades().await {
                    log::debug!("engine connection closed: {e}");
                }
            });
            // Send origin-form with an explicit Host header, as Go's server requires.
            let path = req
                .uri()
                .path_and_query()
                .map(|p| p.as_str().to_owned())
                .unwrap_or_else(|| "/".into());
            *req.uri_mut() = path.parse().expect("a path is a valid URI");
            req.headers_mut().insert(
                hyper::header::HOST,
                hyper::header::HeaderValue::from_static("docker"),
            );
            Ok(sender.send_request(req).await?)
        }
    };
    Docker::connect_with_custom_transport(
        transport,
        Some("http://docker"),
        REQUEST_TIMEOUT_SECS,
        API_DEFAULT_VERSION,
    )
    .expect("custom transports always construct")
}

/// A short, user-facing description of a bollard failure.
pub fn describe_docker_error(e: &bollard::errors::Error) -> String {
    use bollard::errors::Error;
    match e {
        Error::DockerResponseServerError { message, .. } => message.clone(),
        Error::IOError { err } => err.to_string(),
        Error::RequestTimeoutError => "the Engine did not answer in time".into(),
        other => other.to_string(),
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    /// Dials the local Engine socket, standing in for the TLS and SSH dialers.
    struct UnixDialer;

    impl Dialer for UnixDialer {
        fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
            Box::pin(async { Ok(()) })
        }
        fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
            Box::pin(async {
                let s = tokio::net::UnixStream::connect("/var/run/docker.sock").await?;
                Ok(Box::new(s) as BoxStream)
            })
        }
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn dialed_streams_carry_the_engine_api() {
        let docker = docker_over(Arc::new(UnixDialer));
        docker.ping().await.unwrap();
        docker.version().await.unwrap();
    }

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn local_socket_hosts_connect() {
        let connected = connect(
            &HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            None,
            None,
        )
        .await
        .unwrap();
        assert!(connected.dialer.is_none());
        connected.docker.ping().await.unwrap();
    }

    #[tokio::test]
    async fn unreachable_hosts_are_classified() {
        let err = connect(
            &HostConnection::Local {
                socket_path: "/nonexistent/docker.sock".into(),
            },
            None,
            None,
        )
        .await
        .err()
        .unwrap();
        assert!(matches!(err, ConnectError::Unreachable(_)), "{err:?}");
    }
}
