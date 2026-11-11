//! The use cases: one function per former IPC command, taking the
//! [`AppState`](crate::app::AppState) and returning an
//! [`AppResult`](homeport_core::error::AppResult). They are thin: parse the
//! arguments, call one crate, map the result (ADR 0005). Nothing here knows
//! iced, so `cargo test -p homeport-app` runs them without a window.
//!
//! Where a command took a Tauri `Channel<T>`, the function takes
//! `impl Fn(T) -> bool + Send + Sync + 'static`: it is called for each item
//! and its answer is whether to go on.
//!
//! `false` ends the stream, and for a job that is an abort: a deploy, a
//! build, a push or a backup stops being followed and its outcome is never
//! recorded. The webview's channel never said `false` while the app ran, so
//! a job only ended through `stream_cancel`. Whoever turns these into iced
//! subscriptions must keep that: answer `false` for a view that went away
//! only where the stream is a view (logs, stats, watches), and keep
//! answering `true` for a job the user left the page of.

pub mod app;
pub mod clusters;
pub mod containerd;
pub mod engine;
pub mod hosts;
pub mod incus;
pub mod stacks;
