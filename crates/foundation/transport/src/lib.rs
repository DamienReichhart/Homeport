//! How Homeport reaches a Host's Engine API.
//!
//! Every Host has a [`Dialer`], which opens a fresh byte stream to its
//! Engine socket, with secrets that never leave memory (ADR 0002). bollard's
//! client goes through it for TLS and SSH, and uses its own connectors for
//! local sockets and plain TCP. [`http`] sends any other request over it,
//! for the APIs bollard doesn't speak. The same dialer backs the compose CLI
//! proxy (ADR 0001).

mod connect;
mod dialer;
mod error;
pub mod http;
mod local;
pub mod messages;
pub mod proxy;
pub mod reach;
pub mod ssh;
#[cfg(any(test, feature = "test-support"))]
pub mod testing;
pub mod tls;

pub use self::{
    connect::{connect, dial_engine, Connected},
    dialer::{
        shell_quote, BoxFuture, BoxStream, CommandOutput, Dialer, Spawned, Stream, CONNECT_TIMEOUT,
    },
    error::ConnectError,
    local::{LocalDialer, TcpDialer},
};
