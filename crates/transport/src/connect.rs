//! Building a bollard client for a Host and checking that its Engine answers.

use std::{io, sync::Arc};

use bollard::{BollardRequest, Docker, API_DEFAULT_VERSION};
use hyper_util::rt::TokioIo;

use homeport_core::{
    error::{AppError, ErrorCode},
    host::{HostConnection, HostSecrets},
};

use crate::{reach, ssh, tls, ConnectError, Dialer, CONNECT_TIMEOUT};

/// Seconds bollard waits for a response before giving up.
const REQUEST_TIMEOUT_SECS: u64 = 120;

/// A live client for one Host.
#[derive(Clone)]
pub struct Connected {
    pub docker: Docker,
    /// Present for TLS and SSH Hosts; the compose proxy forwards through it.
    #[allow(dead_code)] // read by the compose proxy
    pub dialer: Option<Arc<dyn Dialer>>,
}

/// Whether bollard failed below HTTP, i.e. the network didn't carry the request.
fn io_failure(e: &bollard::errors::Error) -> bool {
    use bollard::errors::Error;
    matches!(
        e,
        Error::IOError { .. } | Error::HyperResponseError { .. } | Error::HttpClientError { .. }
    ) || reach::io_cause(e).is_some()
}

/// Builds a client for `connection` and checks the Engine answers. `name`
/// is the Host's name, for messages about remote Hosts.
pub async fn connect(
    name: &str,
    connection: &HostConnection,
    secrets: Option<HostSecrets>,
    trusted_host_key: Option<&str>,
) -> Result<Connected, ConnectError> {
    let label = |address: &str| format!("{name} ({address})");
    let (docker, dialer) = match connection {
        HostConnection::Local { socket_path } => (
            // A missing socket usually means the Engine isn't running.
            Docker::connect_with_socket(socket_path, REQUEST_TIMEOUT_SECS, API_DEFAULT_VERSION)
                // bollard checks that the socket exists before connecting.
                .map_err(|e| {
                    let missing = io::Error::new(io::ErrorKind::NotFound, e.to_string());
                    ConnectError::from(reach::local_socket(socket_path, &missing))
                })?,
            None,
        ),
        HostConnection::Tcp { address } => (
            Docker::connect_with_http(
                &format!("tcp://{address}"),
                REQUEST_TIMEOUT_SECS,
                API_DEFAULT_VERSION,
            )
            .map_err(|e| {
                ConnectError::Invalid(
                    AppError::validation("The TCP address isn't valid.").with_cause(&e),
                )
            })?,
            None,
        ),
        HostConnection::Tls { address } => {
            let Some(HostSecrets::Tls {
                ca_pem,
                cert_pem,
                key_pem,
            }) = secrets
            else {
                return Err(ConnectError::Invalid(AppError::new(ErrorCode::TlsMaterialInvalid, "The TLS certificates are missing.").with_hint("Add the CA certificate, client certificate and client key on the host page.")));
            };
            let dialer: Arc<dyn Dialer> = Arc::new(tls::TlsDialer::new(
                &label(address),
                address,
                &ca_pem,
                &cert_pem,
                &key_pem,
            )?);
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
                label: label(&format!("{user}@{host}:{port}")),
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
        .map_err(|_| {
            ConnectError::Unreachable(AppError::new(
                ErrorCode::Timeout,
                "The Docker Engine didn't answer in time.",
            ))
        })?
        .map_err(|e| match connection {
            HostConnection::Local { socket_path } => {
                ConnectError::from(reach::local_socket(socket_path, &e))
            }
            HostConnection::Tcp { address } if io_failure(&e) => {
                ConnectError::from(reach::network(&label(address), &e))
            }
            // TLS 1.3 servers reject a client certificate after the handshake,
            // so the refusal arrives with the first request.
            HostConnection::Tls { address }
                if io_failure(&e) && reach::classified_cause(&e).is_none() =>
            {
                ConnectError::from(reach::tls(&label(address), address, &e))
            }
            _ => ConnectError::from(AppError::from(e)),
        })?;
    Ok(Connected { docker, dialer })
}

/// A bollard client whose requests travel over streams from `dialer`.
/// Each request gets its own HTTP/1.1 connection, which also carries the
/// upgraded streams used by exec and attach.
pub(crate) fn docker_over(dialer: Arc<dyn Dialer>) -> Docker {
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

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::testing::UnixDialer;

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
            "local",
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

    async fn connect_local(socket_path: &str) -> ConnectError {
        connect(
            "local",
            &HostConnection::Local {
                socket_path: socket_path.into(),
            },
            None,
            None,
        )
        .await
        .err()
        .expect("nothing should answer")
    }

    #[tokio::test]
    async fn a_closed_tcp_port_refuses_the_connection() {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let address = format!("127.0.0.1:{port}");
        let err = connect(
            "prod",
            &HostConnection::Tcp {
                address: address.clone(),
            },
            None,
            None,
        )
        .await
        .err()
        .expect("the port is closed");
        let ConnectError::Unreachable(e) = err else {
            panic!("expected unreachable, got {err:?}");
        };
        assert_eq!(e.code, ErrorCode::ConnectionRefused, "{e:?}");
        assert!(e.message.contains(&format!("prod ({address})")), "{e:?}");
    }

    #[tokio::test]
    async fn a_missing_socket_means_no_engine_is_running_there() {
        let err = connect_local("/nonexistent/docker.sock").await;
        let ConnectError::Unreachable(e) = err else {
            panic!("expected unreachable, got {err:?}");
        };
        assert!(
            matches!(
                e.code,
                ErrorCode::EngineNotRunning | ErrorCode::EngineNotInstalled
            ),
            "{e:?}"
        );
        assert!(e.message.contains("/nonexistent/docker.sock"), "{e:?}");
        assert!(e.hint.is_some_and(|h| !h.text.is_empty()));
        assert!(e.detail.is_some());
    }

    #[tokio::test]
    async fn a_socket_nobody_listens_on_means_the_engine_is_stopped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("docker.sock");
        drop(std::os::unix::net::UnixListener::bind(&path).unwrap());
        let err = connect_local(path.to_str().unwrap()).await;
        let ConnectError::Unreachable(e) = err else {
            panic!("expected unreachable, got {err:?}");
        };
        assert_eq!(e.code, ErrorCode::EngineNotRunning, "{e:?}");
    }

    #[tokio::test]
    async fn a_socket_without_permission_refuses_access() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        if std::fs::metadata("/proc/self").is_ok_and(|m| m.uid() == 0) {
            return; // root ignores file permissions
        }
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("docker.sock");
        let _listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).unwrap();
        let err = connect_local(path.to_str().unwrap()).await;
        let ConnectError::Unauthorized(e) = err else {
            panic!("expected unauthorized, got {err:?}");
        };
        assert_eq!(e.code, ErrorCode::PermissionDenied, "{e:?}");
        assert!(e.message.contains(path.to_str().unwrap()), "{e:?}");
    }
}
