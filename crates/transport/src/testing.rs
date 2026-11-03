//! Test doubles for the crates that use a [`Dialer`].

#[cfg(unix)]
use std::{io, sync::Arc};

#[cfg(unix)]
use bollard::Docker;

#[cfg(unix)]
use crate::{BoxFuture, BoxStream, ConnectError, Dialer};

/// Dials the local Engine socket, standing in for the TLS and SSH dialers.
#[cfg(unix)]
pub struct UnixDialer;

#[cfg(unix)]
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

/// A client whose requests go through [`UnixDialer`], like TLS and SSH Hosts.
#[cfg(unix)]
pub fn dialed_docker() -> Docker {
    crate::docker_over(Arc::new(UnixDialer))
}
