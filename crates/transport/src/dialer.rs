//! The seam between Homeport and a Host's Engine API: something that opens
//! byte streams speaking it.

use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    sync::oneshot,
};

use crate::ConnectError;

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

pub trait Stream: AsyncRead + AsyncWrite + Send + Unpin {}
impl<T: AsyncRead + AsyncWrite + Send + Unpin> Stream for T {}

pub type BoxStream = Box<dyn Stream>;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// What a command run where the Engine runs printed, and how it exited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    /// Its exit status; `None` when it was killed or didn't say.
    pub status: Option<u32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn success(&self) -> bool {
        self.status == Some(0)
    }
}

/// A command started with [`Dialer::spawn`], still running.
pub struct Spawned {
    /// Reads what it prints on stdout and writes to its stdin. Dropping it
    /// ends the command.
    pub io: BoxStream,
    /// Resolves once it has exited, with its status and stderr; its
    /// `stdout` is empty, since that went to `io`.
    pub exit: BoxFuture<'static, CommandOutput>,
}

/// A stream that tells whoever holds the other end of `guard` when it's
/// dropped, so the command behind it can be ended.
pub(crate) struct Guarded<S> {
    pub(crate) inner: S,
    pub(crate) _guard: oneshot::Sender<()>,
}

impl<S: AsyncRead + Unpin> AsyncRead for Guarded<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for Guarded<S> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

/// Opens byte streams to a Host's Engine API.
pub trait Dialer: Send + Sync + 'static {
    /// Establishes (or re-establishes) whatever session the dialer needs,
    /// classifying failures for the Host's status.
    fn connect(&self) -> BoxFuture<'_, Result<(), ConnectError>>;
    /// Opens one stream speaking the Engine's HTTP API.
    fn dial(&self) -> BoxFuture<'_, io::Result<BoxStream>>;

    /// Whether [`Self::run`] can run commands on the machine the Engine runs
    /// on: over SSH, or on this computer for a local Linux socket (ADR 0019).
    fn runs_commands(&self) -> bool {
        false
    }

    /// Runs one POSIX shell command on the machine the Engine runs on, with
    /// its user's rights, for the few actions the Engine's API lacks
    /// (ADR 0019). Quote every value with [`shell_quote`]. Fails with
    /// `Unsupported` where [`Self::runs_commands`] is false.
    fn run(&self, command: String, timeout: Duration) -> BoxFuture<'_, io::Result<CommandOutput>> {
        let _ = (command, timeout);
        Box::pin(async {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "this connection can't run commands",
            ))
        })
    }

    /// Starts one POSIX shell command where [`Self::run`] would, and hands
    /// back its stdin and stdout as a stream while it runs, for following a
    /// file or carrying a terminal (ADR 0020). Fails with `Unsupported`
    /// where [`Self::runs_commands`] is false.
    fn spawn(&self, command: String) -> BoxFuture<'_, io::Result<Spawned>> {
        let _ = command;
        Box::pin(async {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "this connection can't run commands",
            ))
        })
    }
}

/// `value` as one word for a POSIX shell.
pub fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_are_quoted_as_one_shell_word() {
        assert_eq!(shell_quote("web.service"), "'web.service'");
        assert_eq!(shell_quote("it's; rm -rf /"), r"'it'\''s; rm -rf /'");
    }
}
