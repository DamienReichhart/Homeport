//! Homeport's app shell. The backend's logic lives in the crates under
//! `crates/`; this crate only exposes it to the webview: the Tauri app
//! ([`app`]), its IPC commands and the errors only Tauri produces.

mod app;
mod commands;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    app::run()
}
