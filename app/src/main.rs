//! Homeport's desktop shell, on iced.

// Prevents an extra console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ui;

fn main() -> iced::Result {
    ui::run()
}
