//! Dialers for the Hosts bollard reaches with its own connectors: a local
//! socket or named pipe, and plain TCP. They sit beside bollard's client, so
//! the Docker path is unchanged, and give the APIs bollard doesn't speak
//! (Podman's libpod, and later Incus and containerd) a stream to the Engine.

use std::{io, process::Stdio, time::Duration};

use tokio::{io::AsyncReadExt, net::TcpStream, sync::oneshot};

use homeport_core::{error::AppError, host::EngineKind};

use crate::{
    dialer::Guarded, reach, BoxFuture, BoxStream, CommandOutput, ConnectError, Dialer, Spawned,
    CONNECT_TIMEOUT,
};

/// Dials the Engine socket (Unix) or named pipe (Windows) on this machine.
pub struct LocalDialer {
    socket_path: String,
    /// The kind of Engine behind the socket, for the hints of a failure.
    engine: EngineKind,
}

impl LocalDialer {
    pub fn new(socket_path: &str) -> Self {
        Self::for_engine(socket_path, EngineKind::Docker)
    }

    /// A dialer for an Engine of another kind than Docker's, whose failures
    /// say how to start or reach that Engine.
    pub fn for_engine(socket_path: &str, engine: EngineKind) -> Self {
        Self {
            socket_path: socket_path.to_owned(),
            engine,
        }
    }

    fn classify(&self, e: &io::Error) -> AppError {
        match self.engine {
            EngineKind::Containerd => reach::local_containerd(&self.socket_path, e),
            EngineKind::Incus => reach::local_incus(&self.socket_path, e),
            _ => reach::local_socket(&self.socket_path, e),
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
                Err(e) => Err(ConnectError::from(self.classify(&e))),
            }
        })
    }

    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>> {
        Box::pin(async move {
            // Keep the classification for whoever reads the failed request.
            self.open()
                .await
                .map_err(|e| io::Error::new(e.kind(), self.classify(&e)))
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

    fn spawn(&self, command: String) -> BoxFuture<'_, io::Result<Spawned>> {
        Box::pin(async move {
            if !self.runs_commands() {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "this Engine runs in a virtual machine",
                ));
            }
            spawn_here(&command)
        })
    }
}

/// Starts `command` with `sh` on this computer, its stdin and stdout as the
/// stream; dropping the stream kills it.
fn spawn_here(command: &str) -> io::Result<Spawned> {
    let mut child = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    let (stdin, stdout, mut stderr) = (
        child.stdin.take().expect("stdin is piped"),
        child.stdout.take().expect("stdout is piped"),
        child.stderr.take().expect("stderr is piped"),
    );
    let (guard, mut dropped) = oneshot::channel::<()>();
    let (done, exited) = oneshot::channel();
    tokio::spawn(async move {
        let mut err = Vec::new();
        let read_err = stderr.read_to_end(&mut err);
        let status = tokio::select! {
            (_, status) = async { tokio::join!(read_err, child.wait()) } => status.ok(),
            _ = &mut dropped => {
                child.start_kill().ok();
                child.wait().await.ok()
            }
        };
        done.send(CommandOutput {
            status: status
                .and_then(|s| s.code())
                .and_then(|c| u32::try_from(c).ok()),
            stdout: String::new(),
            stderr: String::from_utf8_lossy(&err).into_owned(),
        })
        .ok();
    });
    Ok(Spawned {
        io: Box::new(Guarded {
            inner: tokio::io::join(stdout, ClosingStdin(Some(stdin))),
            _guard: guard,
        }),
        exit: Box::pin(async move {
            exited.await.unwrap_or(CommandOutput {
                status: None,
                stdout: String::new(),
                stderr: String::new(),
            })
        }),
    })
}

/// A child's stdin that closes the pipe on shutdown, so the child reads
/// the end of its input; tokio's only flushes it.
struct ClosingStdin(Option<tokio::process::ChildStdin>);

impl tokio::io::AsyncWrite for ClosingStdin {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        match self.0.as_mut() {
            Some(stdin) => std::pin::Pin::new(stdin).poll_write(cx, buf),
            None => std::task::Poll::Ready(Err(io::ErrorKind::BrokenPipe.into())),
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        match self.0.as_mut() {
            Some(stdin) => std::pin::Pin::new(stdin).poll_flush(cx),
            None => std::task::Poll::Ready(Ok(())),
        }
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        if let Some(stdin) = self.0.as_mut() {
            std::task::ready!(std::pin::Pin::new(stdin).poll_flush(cx))?;
        }
        self.0 = None;
        std::task::Poll::Ready(Ok(()))
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
mod tests;
