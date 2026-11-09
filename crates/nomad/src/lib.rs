//! A typed client for a Nomad Cluster's HTTP API (see `GLOSSARY.md`).
//!
//! Homeport talks to the API an agent already serves, from this computer:
//! nothing is installed on the Cluster and no `nomad` CLI is needed here.
//!
//! - [`client`] holds the connection: the address, the region, the ACL
//!   token (sent in `X-Nomad-Token`, never in a URL) and the TLS material.
//! - [`error`] explains Nomad's answers and the failures to reach it.
//! - [`agent`] reads the Cluster's status: its leader, version and region.
//! - [`jobs`] lists, shows, stops and scales jobs; [`watch`] tells a page
//!   when what it shows changed, through Nomad's blocking queries.
//! - [`run`] parses, plans and runs a job specification, HCL2 or JSON.
//! - [`allocations`], [`logs`] and [`exec`] show an allocation, stream its
//!   tasks' logs and open terminals in them, through the servers.
//! - [`nodes`] lists client nodes, drains them and sets their eligibility.

pub mod address;
pub mod agent;
pub mod allocations;
pub mod client;
pub mod error;
pub mod exec;
pub mod jobs;
pub mod logs;
pub mod messages;
pub mod nodes;
pub mod run;
mod time;
mod tls;
pub mod watch;

pub use address::{normalize_address, DEFAULT_PORT};
pub use agent::NomadAgent;
pub use client::{Nomad, NomadConfig};
