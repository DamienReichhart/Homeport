//! A private local endpoint that forwards to a Host through its [`Dialer`],
//! so the compose CLI can reach SSH and TLS Hosts without key files on disk
//! (ADR 0001). It lives only as long as the [`Proxy`] value.

use std::{io, sync::Arc};

use tokio::task::JoinHandle;

use super::Dialer;

pub struct Proxy {
    /// Value for `DOCKER_HOST`.
    pub docker_host: String,
    accept: JoinHandle<()>,
    #[cfg(unix)]
    _dir: tempfile::TempDir,
}

impl Drop for Proxy {
    fn drop(&mut self) {
        self.accept.abort();
    }
}

async fn forward<S>(mut client: S, dialer: Arc<dyn Dialer>)
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    match dialer.dial().await {
        Ok(mut upstream) => {
            let _ = tokio::io::copy_bidirectional(&mut client, &mut upstream).await;
        }
        Err(e) => log::warn!("compose proxy could not reach the host: {e}"),
    }
}

/// Opens a Unix socket inside a fresh directory only this user can enter.
#[cfg(unix)]
pub fn open(dialer: Arc<dyn Dialer>) -> io::Result<Proxy> {
    use std::os::unix::fs::PermissionsExt;

    let base = std::env::var_os("XDG_RUNTIME_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let dir = tempfile::Builder::new()
        .prefix("homeport-")
        .tempdir_in(base)?;
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))?;
    let path = dir.path().join("docker.sock");
    let listener = tokio::net::UnixListener::bind(&path)?;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    let accept = tokio::spawn(async move {
        while let Ok((client, _)) = listener.accept().await {
            tokio::spawn(forward(client, dialer.clone()));
        }
    });
    Ok(Proxy {
        docker_host: format!("unix://{}", path.display()),
        accept,
        _dir: dir,
    })
}

/// Opens a named pipe that refuses remote clients.
#[cfg(windows)]
pub fn open(dialer: Arc<dyn Dialer>) -> io::Result<Proxy> {
    use tokio::net::windows::named_pipe::ServerOptions;

    let name = format!(r"\\.\pipe\homeport-{}", uuid::Uuid::new_v4().simple());
    let mut server = ServerOptions::new()
        .first_pipe_instance(true)
        .reject_remote_clients(true)
        .create(&name)?;
    let pipe = name.clone();
    let accept = tokio::spawn(async move {
        loop {
            if server.connect().await.is_err() {
                return;
            }
            let Ok(next) = ServerOptions::new()
                .reject_remote_clients(true)
                .create(&pipe)
            else {
                return;
            };
            let client = std::mem::replace(&mut server, next);
            tokio::spawn(forward(client, dialer.clone()));
        }
    });
    Ok(Proxy {
        docker_host: format!("npipe://{}", name.replace('\\', "/")),
        accept,
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::services::docker::testing::UnixDialer;

    #[tokio::test]
    #[ignore = "needs a local Docker Engine"]
    async fn the_proxy_carries_the_engine_api() {
        let proxy = open(Arc::new(UnixDialer)).unwrap();
        let path = proxy.docker_host.trim_start_matches("unix://").to_owned();
        let mode = std::os::unix::fs::PermissionsExt::mode(
            &std::fs::metadata(&path).unwrap().permissions(),
        );
        assert_eq!(mode & 0o777, 0o600);

        let docker =
            bollard::Docker::connect_with_unix(&path, 10, bollard::API_DEFAULT_VERSION).unwrap();
        docker.ping().await.unwrap();

        drop(proxy);
        assert!(!std::path::Path::new(&path).exists());
    }
}
