//! Hosts: the Engines Homeport connects to (see `GLOSSARY.md`).
//!
//! - [`store`] persists them, their secrets sealed (ADR 0002).
//! - [`registry`] keeps one supervised connection per Host and reports its
//!   status and Engine events through a [`registry::Notifier`].
//! - [`service`] holds the use cases the app exposes, keeping both in step.
//! - [`bundle`] writes Hosts to a passphrase-protected file and reads them back.
//! - [`detect`] finds the Engines running on this computer.
//! - [`import`] reads the Hosts `~/.ssh/config` and Docker contexts describe.

pub mod bundle;
pub mod detect;
pub mod import;
pub mod messages;
pub mod registry;
pub mod service;
pub mod store;

pub use homeport_core::host::{
    Bastion, BastionAuth, Host, HostConnection, HostId, HostSecrets, SshAuth, SshKeySecret,
};
