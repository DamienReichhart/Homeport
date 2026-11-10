//! Nomad's HTTP API: the client and its TLS, the address it is reached at,
//! blocking queries, and what its failures mean.

pub mod address;
pub mod client;
pub mod error;
pub(crate) mod time;
pub(crate) mod tls;
pub mod watch;
