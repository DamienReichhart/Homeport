//! Business logic. Services are plain Rust (no Tauri command attributes), so they
//! are unit-testable without a running webview. Commands call into services.

pub mod crypto;
pub mod db;
#[allow(dead_code)] // wired to commands with the host registry
pub mod hosts;
pub mod settings;
