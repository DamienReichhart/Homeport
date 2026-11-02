//! Business logic. Services are plain Rust (no Tauri command attributes), so they
//! are unit-testable without a running webview. Commands call into services.

pub mod crypto;
pub mod db;
pub mod docker;
pub mod hosts;
pub mod settings;
pub mod stacks;
pub mod streams;
pub mod transport;
