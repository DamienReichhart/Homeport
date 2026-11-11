//! Incus's REST API (ADR 0021): the client over a Host's dialer, its
//! websockets, the operations it answers with, and what its failures mean.

pub(crate) mod client;
pub(crate) mod error;
pub mod operations;
pub(crate) mod websocket;
