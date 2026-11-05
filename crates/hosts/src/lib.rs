//! Hosts: the Engines Homeport connects to (see `GLOSSARY.md`).
//!
//! - [`store`] persists them, their secrets sealed (ADR 0002).
//! - [`registry`] keeps one supervised connection per Host and reports its
//!   status and Engine events through a [`registry::Notifier`].
//! - [`service`] holds the use cases the app exposes, keeping both in step.
//! - [`detect`] finds the Engines running on this computer.

pub mod detect;
pub mod messages;
pub mod registry;
pub mod service;
pub mod store;

pub use homeport_core::host::{Host, HostConnection, HostId, HostSecrets, SshAuth};
