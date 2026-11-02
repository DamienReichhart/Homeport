//! Business logic. Services are plain Rust (no Tauri command attributes), so they
//! are unit-testable without a running webview. Commands call into services.

pub mod settings;
