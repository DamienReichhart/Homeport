//! Dialers for the Hosts bollard reaches with its own connectors: a local
//! socket or named pipe, and plain TCP. They sit beside bollard's client, so
//! the Docker path is unchanged, and give the APIs bollard doesn't speak
//! (Podman's libpod, and later Incus and containerd) a stream to the Engine.

use std::{io, time::Duration};

use tokio::net::TcpStream;

use crate::{reach, BoxFuture, BoxStream, CommandOutput, ConnectError, Dialer, CONNECT_TIMEOUT};

/// Dials the Engine socket (Unix) or named pipe (Windows) on this machine.
pub struct LocalDialer {
    socket_path: String,
}

impl LocalDialer {
    pub fn new(socket_path: &str) -> Self {
        Self {
            socket_path: socket_path.to_owned(),
        }
    }

    async fn open(&self) -> io::Result<BoxStream> {
        let open = open_local(&self.socket_path);
        tokio::time::timeout(CONNECT_TIMEOUT, open)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "connection timed out"))?
    }
}

#[cfg(unix)]
async fn open_local(socket_path: &str) -> io::Result<BoxStream> {
    let stream = tokio::net::UnixStream::connect(socket_path).await?;
    Ok(Box::new(stream))
}

/// Opens a named pipe given as `//./pipe/name` (Docker's spelling) or
/// `\\.\pipe\name`, waiting a little while every instance is busy.
#[cfg(windows)]
async fn open_local(socket_path: &str) -> io::Result<BoxStream> {
    use tokio::net::windows::named_pipe::ClientOptions;
    /// Windows' ERROR_PIPE_BUSY: every instance of the pipe is in use.
    const PIPE_BUSY: i32 = 231;
    let name = socket_path.replace('/', "\\");
    let mut attempts = 0;
    loop {
        match ClientOptions::new().open(&name) {
            Ok(pipe) => return Ok(Box::new(pipe)),
            Err(e) if e.raw_os_error() == Some(PIPE_BUSY) && attempts < 20 => {
                attempts += 1;
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

impl Dialer for LocalDialer {
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
        Box::pin(async move {
            match self.open().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ConnectError::from(reach::local_socket(
                    &self.socket_path,
                    &e,
                ))),
            }
        })
    }

    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
        Box::pin(async move {
            // Keep the classification for whoever reads the failed request.
            self.open()
                .await
                .map_err(|e| io::Error::new(e.kind(), reach::local_socket(&self.socket_path, &e)))
        })
    }

    /// On Linux a local socket's Engine runs on this computer; on macOS and
    /// Windows it runs in a VM (Podman machine, Docker Desktop), which this
    /// computer's commands don't reach.
    fn runs_commands(&self) -> bool {
        cfg!(target_os = "linux")
    }

    fn run(&self, command: String, timeout: Duration) -> BoxFuture<'_, io::Result<CommandOutput>> {
        Box::pin(async move {
            if !self.runs_commands() {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "this Engine runs in a virtual machine",
                ));
            }
            let child = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&command)
                .stdin(std::process::Stdio::null())
                .kill_on_drop(true)
                .output();
            let output = tokio::time::timeout(timeout, child)
                .await
                .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "the command timed out"))??;
            Ok(CommandOutput {
                status: output.status.code().and_then(|c| u32::try_from(c).ok()),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            })
        })
    }
}

/// Dials an Engine listening on plain TCP.
pub struct TcpDialer {
    /// The Host as messages name it, "prod (10.0.0.2:2375)".
    label: String,
    address: String,
}

impl TcpDialer {
    pub fn new(label: &str, address: &str) -> Self {
        Self {
            label: label.to_owned(),
            address: address.to_owned(),
        }
    }

    async fn open(&self) -> io::Result<BoxStream> {
        let tcp = tokio::time::timeout(CONNECT_TIMEOUT, TcpStream::connect(&self.address))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "connection timed out"))??;
        tcp.set_nodelay(true)?;
        Ok(Box::new(tcp))
    }
}

impl Dialer for TcpDialer {
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>> {
        Box::pin(async move {
            match self.open().await {
                Ok(_) => Ok(()),
                Err(e) => Err(ConnectError::from(reach::network(&self.label, &e))),
            }
        })
    }

    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
        Box::pin(async move {
            self.open()
                .await
                .map_err(|e| io::Error::new(e.kind(), reach::network(&self.label, &e)))
        })
    }
}

#[cfg(test)]
mod tests {
    use homeport_core::error::{reach::classified_cause, ErrorCode};

    use super::*;

    #[cfg(unix)]
    #[tokio::test]
    async fn a_local_socket_nobody_listens_on_is_classified() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("podman.sock");
        let dialer = LocalDialer::new(path.to_str().unwrap());
        let err = dialer.dial().await.err().unwrap();
        let classified = classified_cause(&err).expect("the dialer classifies its failures");
        assert!(
            matches!(
                classified.code,
                ErrorCode::EngineNotRunning | ErrorCode::EngineNotInstalled
            ),
            "{classified:?}"
        );
        assert!(matches!(
            dialer.connect().await,
            Err(ConnectError::Unreachable(_))
        ));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn a_listening_local_socket_is_dialed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("engine.sock");
        let _listener = tokio::net::UnixListener::bind(&path).unwrap();
        let dialer = LocalDialer::new(path.to_str().unwrap());
        dialer.connect().await.unwrap();
        dialer.dial().await.unwrap();
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn local_linux_sockets_run_commands_on_this_computer() {
        let dialer = LocalDialer::new("/run/podman/podman.sock");
        assert!(dialer.runs_commands());
        let out = dialer
            .run(
                "echo out; echo err >&2; exit 3".into(),
                Duration::from_secs(5),
            )
            .await
            .unwrap();
        assert_eq!(out.stdout, "out\n");
        assert_eq!(out.stderr, "err\n");
        assert_eq!(out.status, Some(3));
        assert!(!out.success());
        let slow = dialer
            .run("sleep 5".into(), Duration::from_millis(50))
            .await;
        assert_eq!(slow.unwrap_err().kind(), io::ErrorKind::TimedOut);
    }

    #[tokio::test]
    async fn tcp_hosts_run_no_commands() {
        let dialer = TcpDialer::new("lab", "127.0.0.1:1");
        assert!(!dialer.runs_commands());
        let err = dialer
            .run("true".into(), Duration::from_secs(1))
            .await
            .unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Unsupported);
    }

    #[tokio::test]
    async fn a_closed_tcp_port_is_refused_and_named() {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let address = format!("127.0.0.1:{port}");
        let dialer = TcpDialer::new(&format!("lab ({address})"), &address);
        let err = dialer.dial().await.err().unwrap();
        let classified = classified_cause(&err).unwrap();
        assert_eq!(classified.code, ErrorCode::ConnectionRefused);
        assert!(classified.message.contains("lab ("), "{classified:?}");
    }
}
