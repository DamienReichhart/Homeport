//! containerd's gRPC API (ADR 0020): the generated client code, the connection
//! over a Host's dialer, and what its failures mean.

pub mod api;
pub(crate) mod client;
pub(crate) mod error;
