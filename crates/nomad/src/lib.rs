//! A typed client for a Nomad Cluster's HTTP API (see `GLOSSARY.md`).
//!
//! Homeport talks to the API an agent already serves, from this computer:
//! nothing is installed on the Cluster and no `nomad` CLI is needed here.
//!
//! - [`client`] holds the connection: the address, the region, the ACL
//!   token (sent in `X-Nomad-Token`, never in a URL) and the TLS material.
//! - [`error`] explains Nomad's answers and the failures to reach it.
//! - [`agent`] reads the Cluster's status: its leader, version and region.

pub mod address;
pub mod agent;
pub mod client;
pub mod error;
pub mod messages;
mod tls;

pub use address::{normalize_address, DEFAULT_PORT};
pub use agent::NomadAgent;
pub use client::{Nomad, NomadConfig};
