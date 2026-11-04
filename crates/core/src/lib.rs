//! The shared kernel of Homeport's backend.
//!
//! Every other crate depends on this one and on nothing above it: it holds the
//! error model ([`error::AppError`]) that every failure reaches the frontend
//! with, what a Host is, and helpers for the computer Homeport runs on. It knows nothing about
//! Tauri, SQLite or how Hosts are reached.

pub mod error;
pub mod host;
pub mod local_files;
pub mod programs;
pub mod streams;
