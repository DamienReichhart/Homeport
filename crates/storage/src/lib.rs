//! What Homeport keeps on this computer: one SQLite database ([`Db`]),
//! migrated on open, and the [`SecretBox`] that seals each secret stored in it
//! under a key held in the OS keychain (ADR 0002).
//!
//! The stores of each domain (Hosts, Stacks, settings) live with their domain
//! and are built on these two.

pub mod crypto;
pub mod db;
pub mod messages;

pub use crypto::{KeyStore, Keychain, SecretBox};
pub use db::Db;
