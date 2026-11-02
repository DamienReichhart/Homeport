//! IPC surface. Commands are thin: validate input, call a service, map the result.
//! Register each new command in `lib.rs` (`generate_handler!`).

pub mod settings;
pub mod system;
