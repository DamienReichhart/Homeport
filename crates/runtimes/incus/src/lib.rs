//! Incus Hosts (ADR 0021): Incus's REST API over the Host's dialer, for its
//! projects, Instances with their console, exec and snapshots, images,
//! profiles, storage pools and networks; its websockets for events,
//! console and exec; and trust tokens for HTTPS.
//!
//! Every call about a project-scoped object names its project
//! ([`client::scoped`]); the frontend remembers which one each Host shows.

pub mod api;
pub mod events;
pub mod images;
pub mod instances;
pub mod messages;
pub mod networks;
pub mod profiles;
pub mod projects;
pub mod storage;
pub mod trust;

pub use api::client::{Incus, IncusServer, DEFAULT_PROJECT, REQUIRED_EXTENSIONS};
