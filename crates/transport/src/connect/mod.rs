//! Building a bollard client for a Host and checking that its Engine answers.

use std::{io, sync::Arc};

use bollard::{BollardRequest, Docker, API_DEFAULT_VERSION};

use homeport_core::{
    error::{AppError, ErrorCode},
    host::{EngineKind, HostConnection, HostSecrets},
};

use crate::{
    http, messages as msg, reach, ssh, tls, ConnectError, Dialer, LocalDialer, TcpDialer,
    CONNECT_TIMEOUT,
};

/// Seconds bollard waits for a response before giving up.
const REQUEST_TIMEOUT_SECS: u64 = 120;

/// A live client for one Host.
#[derive(Clone)]
pub struct Connected {
    pub docker: Docker,
    /// Opens streams to the Host's Engine socket, whatever the connection:
    /// bollard's client goes through it for TLS and SSH, [`http`] for every
    /// API bollard doesn't speak, and the compose proxy forwards through it.
    pub dialer: Arc<dyn Dialer>,
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
    // Local and TCP Hosts keep bollard's own connectors; their dialer sits
    // beside them, so nothing on the Docker path changes.
    let (docker, dialer, dialed): (Docker, Arc<dyn Dialer>, bool) = match connection {
        HostConnection::Local { socket_path } => (
            // A missing socket usually means the Engine isn't running.
            Docker::connect_with_socket(socket_path, REQUEST_TIMEOUT_SECS, API_DEFAULT_VERSION)
                // bollard checks that the socket exists before connecting.
                .map_err(|e| {
                    let missing = io::Error::new(io::ErrorKind::NotFound, e.to_string());
                    ConnectError::from(reach::local_socket(socket_path, &missing))
                })?,
            Arc::new(LocalDialer::new(socket_path)),
            false,
        ),
        HostConnection::Tcp { address } => (
            Docker::connect_with_http(
                &format!("tcp://{address}"),
                REQUEST_TIMEOUT_SECS,
                API_DEFAULT_VERSION,
            )
            .map_err(|e| {
                ConnectError::Invalid(
                    AppError::validation(msg::CONNECT_TCP_ADDRESS_INVALID).with_cause(&e),
                )
            })?,
            Arc::new(TcpDialer::new(&label(address), address)),
            false,
        ),
        HostConnection::Tls { address } => {
            let Some(HostSecrets::Tls {
                ca_pem,
                cert_pem,
                key_pem,
            }) = secrets
            else {
                return Err(ConnectError::Invalid(
                    AppError::new(
                        ErrorCode::TlsMaterialInvalid,
                        msg::CONNECT_TLS_MATERIAL_MISSING,
                    )
                    .with_hint(msg::CONNECT_TLS_MATERIAL_MISSING_HINT),
                ));
            };
            let dialer: Arc<dyn Dialer> = Arc::new(tls::TlsDialer::new(
                &label(address),
                address,
                &ca_pem,
                &cert_pem,
                &key_pem,
            )?);
            (docker_over(dialer.clone()), dialer, true)
        }
        HostConnection::Ssh { .. } => {
            let dialer = ssh_dialer(
                name,
                connection,
                secrets.as_ref(),
                trusted_host_key,
                EngineKind::Docker,
            )?;
            (docker_over(dialer.clone()), dialer, true)
        }
    };
    if dialed {
        dialer.connect().await?;
    }
    let docker = tokio::time::timeout(CONNECT_TIMEOUT, docker.negotiate_version())
        .await
        .map_err(|_| {
            ConnectError::Unreachable(AppError::new(
                ErrorCode::Timeout,
                msg::CONNECT_ENGINE_TIMEOUT,
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

/// The dialer of an SSH Host, which `connect` checks before using it.
fn ssh_dialer(
    name: &str,
    connection: &HostConnection,
    secrets: Option<&HostSecrets>,
    trusted_host_key: Option<&str>,
    engine: EngineKind,
) -> Result<Arc<dyn Dialer>, ConnectError> {
    let HostConnection::Ssh {
        host,
        port,
        user,
        socket_path,
        auth,
        bastions,
        ..
    } = connection
    else {
        unreachable!("only called for SSH Hosts");
    };
    let credentials = ssh::Credentials::from_secrets(*auth, secrets)?;
    let hops = bastions
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let address = b.address();
            Ok(ssh::SshHop {
                label: format!("{address} (bastion {} of {name})", i + 1),
                credentials: ssh::Credentials::for_bastion(
                    b.auth,
                    &address,
                    &credentials,
                    secrets,
                )?,
                host: b.host.clone(),
                port: b.port,
                user: b.user.clone(),
                trusted_host_key: b.host_key.clone(),
            })
        })
        .collect::<Result<Vec<_>, ConnectError>>()?;
    Ok(Arc::new(ssh::SshDialer::new(ssh::SshTarget {
        label: format!("{name} ({user}@{host}:{port})"),
        host: host.clone(),
        port: *port,
        user: user.clone(),
        socket_path: socket_path.clone(),
        credentials,
        trusted_host_key: trusted_host_key.map(str::to_owned),
        bastions: hops,
        engine,
    })))
}

/// A connected dialer to a Host whose Engine bollard doesn't speak, such as
/// containerd (ADR 0020) and Incus (ADR 0021): its local socket, or its
/// socket through SSH; and Incus over HTTPS, pinned to the server
/// certificate kept with the Host's TLS material. Plain TCP isn't offered
/// for these Engines.
pub async fn dial_engine(
    name: &str,
    connection: &HostConnection,
    secrets: Option<HostSecrets>,
    trusted_host_key: Option<&str>,
    engine: EngineKind,
) -> Result<Arc<dyn Dialer>, ConnectError> {
    let dialer: Arc<dyn Dialer> = match connection {
        HostConnection::Local { socket_path } => {
            Arc::new(LocalDialer::for_engine(socket_path, engine))
        }
        HostConnection::Ssh { .. } => {
            ssh_dialer(name, connection, secrets.as_ref(), trusted_host_key, engine)?
        }
        HostConnection::Tls { address } if engine == EngineKind::Incus => {
            let Some(HostSecrets::Tls {
                ca_pem,
                cert_pem,
                key_pem,
            }) = secrets
            else {
                return Err(ConnectError::Invalid(
                    AppError::new(
                        ErrorCode::TlsMaterialInvalid,
                        msg::CONNECT_TLS_MATERIAL_MISSING,
                    )
                    .with_hint(msg::CONNECT_INCUS_MATERIAL_MISSING_HINT),
                ));
            };
            Arc::new(tls::TlsDialer::pinned(
                &format!("{name} ({address})"),
                address,
                &ca_pem,
                &cert_pem,
                &key_pem,
            )?)
        }
        HostConnection::Tcp { .. } | HostConnection::Tls { .. } => {
            return Err(ConnectError::Invalid(AppError::validation(
                msg::CONNECT_ENGINE_NEEDS_SOCKET.with("engine", engine.label()),
            )))
        }
    };
    dialer.connect().await?;
    Ok(dialer)
}

/// A bollard client whose requests travel over streams from `dialer`.
/// Each request gets its own HTTP/1.1 connection, which also carries the
/// upgraded streams used by exec and attach.
pub(crate) fn docker_over(dialer: Arc<dyn Dialer>) -> Docker {
    let transport = move |mut req: BollardRequest| {
        let dialer = dialer.clone();
        async move {
            let mut sender = http::handshake(&*dialer).await?;
            http::origin_form(&mut req);
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
mod tests;
