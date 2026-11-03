//! The seam between Homeport and a Host's Engine API: something that opens
//! byte streams speaking it.

use std::{future::Future, io, pin::Pin, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::ConnectError;

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
