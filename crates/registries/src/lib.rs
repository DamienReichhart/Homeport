//! Registry credentials (see `GLOSSARY.md`): one login per registry address,
//! app-wide, so one Docker Hub login serves every Host.
//!
//! - [`address`] normalises registry addresses and finds the registry of an image reference.
//! - [`store`] keeps the logins, each secret sealed in SQLite (ADR 0002).
//! - [`login`] checks a login from this machine, as `docker login` does.
//! - [`hub`] lists Docker Hub tags.
//! - [`hints`] points refused pulls and pushes to **Settings → Registries**.

pub mod address;
pub mod hints;
mod http;
pub mod hub;
pub mod login;
pub mod messages;
pub mod store;

pub use self::address::{normalise, registry_of, DOCKER_HUB};
pub use self::store::{Credentials, Registry, RegistryDraft, RegistryStore};
