//! Business logic. Services are plain Rust (no Tauri command attributes), so they
//! are unit-testable without a running webview. Commands call into services.

// Used by the host and stack stores.
#[allow(dead_code)]
pub mod crypto;
pub mod db;
pub mod settings;
