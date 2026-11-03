//! Hosts: the Docker Engines Homeport connects to (see GLOSSARY.md).

pub mod detect;
pub mod registry;
pub mod service;
pub mod store;

pub use homeport_core::host::*;
