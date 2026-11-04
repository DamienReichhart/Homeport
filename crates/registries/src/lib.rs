//! Registry credentials (see `GLOSSARY.md`): one login per registry address,
//! app-wide, so one Docker Hub login serves every Host.
//!
//! - [`address`] normalises registry addresses and finds the registry of an image reference.
//! - [`store`] keeps the logins, each secret sealed in SQLite (ADR 0002).
//! - [`login`] checks a login from this machine, as `docker login` does.

pub mod address;
mod http;
pub mod login;
pub mod store;

pub use self::address::{normalise, registry_of, DOCKER_HUB};
pub use self::store::{Credentials, Registry, RegistryDraft, RegistryStore};
