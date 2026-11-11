//! What the app is on the desktop besides its window's content: the window
//! itself, the tray icon, notifications, starting at login and the AppImage's
//! integration.

#[cfg(target_os = "linux")]
pub(crate) mod appimage;
pub mod autostart;
pub(crate) mod notifications;
pub mod tray;
pub mod window;
