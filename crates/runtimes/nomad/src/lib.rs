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
//! - [`namespaces`] lists the namespaces the Jobs page picks from.

pub mod agent;
pub mod allocations;
pub mod api;
pub mod jobs;
pub mod messages;
pub mod namespaces;
pub mod nodes;

pub use agent::NomadAgent;
pub use api::address::{normalize_address, DEFAULT_PORT};
pub use api::client::{Nomad, NomadConfig};
