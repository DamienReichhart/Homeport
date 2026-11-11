//! The use cases: one function per former IPC command, taking the
//! [`AppState`](crate::app::AppState) and returning an
//! [`AppResult`](homeport_core::error::AppResult). They are thin: parse the
//! arguments, call one crate, map the result (ADR 0005). Nothing here knows
//! iced, so `cargo test -p homeport-app` runs them without a window.
//!
//! Where a command took a Tauri `Channel<T>`, the function takes
//! `impl Fn(T) -> bool + Send + Sync + 'static`: it is called for each item
//! and returns `false` once nobody listens, as the crates expect. The data
//! layer turns those into iced subscriptions.

pub mod app;
pub mod clusters;
pub mod containerd;
pub mod engine;
pub mod hosts;
pub mod incus;
pub mod stacks;
