//! Homeport's desktop shell, on iced.

// Prevents an extra console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Nothing depends on the app, so `pub` inside it only means "visible to the
// rest of the app"; the library crates keep the lint.
#![allow(unreachable_pub)]
// Pages are ported one at a time; what none reads yet is still to come.
#![allow(dead_code)]

mod app;
mod ui;

fn main() -> iced::Result {
    ui::run()
}
