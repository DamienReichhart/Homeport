//! How Homeport reaches a Host's Engine API.
//!
//! Local sockets and plain TCP use bollard's own connectors. TLS and SSH go
//! through a [`Dialer`], which opens a fresh byte stream to the Engine API
//! with secrets that never leave memory (ADR 0002). The same dialer backs the
//! compose CLI proxy (ADR 0001).

mod connect;
mod dialer;
mod error;
pub mod proxy;
pub mod reach;
pub mod ssh;
#[cfg(any(test, feature = "test-support"))]
pub mod testing;
pub mod tls;

pub use self::{
    connect::{connect, Connected},
    dialer::{BoxFuture, BoxStream, Dialer, Stream, CONNECT_TIMEOUT},
    error::ConnectError,
};
