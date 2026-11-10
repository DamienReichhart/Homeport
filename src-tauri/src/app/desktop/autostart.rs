//! Starting Homeport when the user logs in. The OS's login entry is the
//! truth (a user may remove it by hand); the only setting is whether a
//! start at login stays hidden in the tray.
//!
//! Driven from Rust only: the frontend is granted no autostart permission.

use tauri::{plugin::TauriPlugin, AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;

use crate::error::{self, AppResult};

/// The argument the login entry starts Homeport with. It's always there, so
/// turning **Start hidden** on or off never rewrites the OS's entry.
pub const LOGIN_ARG: &str = "--login";

/// The autostart plugin: a Launch Agent on macOS, the `Run` key on Windows,
/// an XDG autostart entry on Linux (pointing at `$APPIMAGE` for an AppImage).
pub fn plugin<R: Runtime>() -> TauriPlugin<R> {
    let builder = tauri_plugin_autostart::Builder::new().arg(LOGIN_ARG);
    #[cfg(target_os = "macos")]
    let builder = builder.macos_launcher(tauri_plugin_autostart::MacosLauncher::LaunchAgent);
    builder.build()
}

/// Whether Homeport starts at login, as the OS has it now.
pub fn is_enabled(app: &AppHandle) -> AppResult<bool> {
    app.autolaunch().is_enabled().map_err(error::autostart)
}

/// Adds or removes the login entry, and says whether it's there now.
pub fn set_enabled(app: &AppHandle, enabled: bool) -> AppResult<bool> {
    let autolaunch = app.autolaunch();
    if enabled {
        autolaunch.enable()
    } else {
        autolaunch.disable()
    }
    .map_err(error::autostart)?;
    is_enabled(app)
}

/// Whether to keep the window hidden at launch: started at login, asked to
/// start hidden, and with a tray icon to open it from.
pub fn starts_hidden(
    mut args: impl Iterator<Item = String>,
    start_hidden: bool,
    tray: bool,
) -> bool {
    start_hidden && tray && args.any(|arg| arg == LOGIN_ARG)
}

/// Writes the login entry again when it's on, so an AppImage that was moved
/// since is started from where it is now.
#[cfg(target_os = "linux")]
pub fn follow_appimage(app: &AppHandle) {
    if std::env::var_os("APPIMAGE").is_none() {
        return;
    }
    if is_enabled(app).unwrap_or(false) {
        if let Err(e) = app.autolaunch().enable() {
            log::warn!("couldn't update the login entry for the AppImage: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> impl Iterator<Item = String> {
        list.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[test]
    fn a_login_start_stays_hidden_only_when_asked_and_with_a_tray() {
        let login = ["/usr/bin/homeport", LOGIN_ARG];
        assert!(starts_hidden(args(&login), true, true));
        assert!(!starts_hidden(args(&login), false, true));
        // Without a tray there'd be no way back to the window.
        assert!(!starts_hidden(args(&login), true, false));
    }

    #[test]
    fn opening_homeport_yourself_always_shows_the_window() {
        assert!(!starts_hidden(args(&["/usr/bin/homeport"]), true, true));
    }
}
