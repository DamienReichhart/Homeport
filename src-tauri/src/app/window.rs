//! The main window: one per Homeport (a second launch focuses it), opened
//! where it was left, and brought back from the tray.
//!
//! Single-instance and window-state are driven from Rust only, so the
//! frontend is granted no permission for them (`capabilities/default.json`).

use tauri::{AppHandle, LogicalSize, Manager, Runtime, WebviewWindow};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

/// The window's label in `tauri.conf.json`.
pub const MAIN: &str = "main";

/// What is remembered between launches. Not visibility: a window started
/// hidden in the tray stays hidden.
pub const STATE: StateFlags = StateFlags::SIZE
    .union(StateFlags::POSITION)
    .union(StateFlags::MAXIMIZED);

/// The size the window opens at the first time, as in `tauri.conf.json`.
const DEFAULT_SIZE: LogicalSize<f64> = LogicalSize::new(1100.0, 720.0);

/// The window-state plugin, restoring [`STATE`] for the main window.
pub fn state_plugin<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri_plugin_window_state::Builder::new()
        .with_state_flags(STATE)
        .build()
}

/// Shows, unminimises and focuses the main window.
pub fn show<R: Runtime>(app: &AppHandle<R>) {
    let Some(window) = app.get_webview_window(MAIN) else {
        return;
    };
    for result in [window.show(), window.unminimize(), window.set_focus()] {
        if let Err(e) = result {
            log::warn!("couldn't bring the window back: {e}");
        }
    }
}

/// Hides the main window, saving where it is first.
pub fn hide<R: Runtime>(app: &AppHandle<R>) {
    if let Err(e) = app.save_window_state(STATE) {
        log::warn!("couldn't save the window's position: {e}");
    }
    if let Some(window) = app.get_webview_window(MAIN) {
        if let Err(e) = window.hide() {
            log::warn!("couldn't hide the window: {e}");
        }
    }
}

/// Moves the window back to the default size, centred, when its restored
/// place is off every monitor (a display was unplugged since).
pub fn keep_on_screen<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    let (Ok(position), Ok(size)) = (window.outer_position(), window.outer_size()) else {
        return Ok(());
    };
    let monitors: Vec<Rect> = window
        .available_monitors()?
        .iter()
        .map(|m| Rect {
            x: m.position().x,
            y: m.position().y,
            width: m.size().width,
            height: m.size().height,
        })
        .collect();
    let bounds = Rect {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
    };
    if !is_reachable(bounds, &monitors) {
        log::info!("the window was off every monitor; opening it centred");
        window.unmaximize()?;
        window.set_size(DEFAULT_SIZE)?;
        window.center()?;
    }
    Ok(())
}

/// A rectangle in physical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// How much of the window's top edge must show on one monitor for it to be
/// grabbed and moved: its title bar.
const GRAB_WIDTH: i64 = 100;
const GRAB_HEIGHT: i64 = 30;

/// Whether the window's top edge is on a monitor, at least [`GRAB_WIDTH`]
/// wide (or the whole window when narrower), so the user can reach it.
pub fn is_reachable(window: Rect, monitors: &[Rect]) -> bool {
    let (left, top) = (i64::from(window.x), i64::from(window.y));
    let right = left + i64::from(window.width);
    let needed = GRAB_WIDTH.min(i64::from(window.width)).max(1);
    monitors.iter().any(|m| {
        let (m_left, m_top) = (i64::from(m.x), i64::from(m.y));
        let (m_right, m_bottom) = (m_left + i64::from(m.width), m_top + i64::from(m.height));
        let shown = right.min(m_right) - left.max(m_left);
        top >= m_top
            && top + GRAB_HEIGHT.min(i64::from(window.height)) <= m_bottom
            && shown >= needed
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAPTOP: Rect = Rect {
        x: 0,
        y: 0,
        width: 1920,
        height: 1080,
    };
    /// A second display to the right of the laptop.
    const EXTERNAL: Rect = Rect {
        x: 1920,
        y: 0,
        width: 2560,
        height: 1440,
    };

    fn window(x: i32, y: i32) -> Rect {
        Rect {
            x,
            y,
            width: 1100,
            height: 720,
        }
    }

    #[test]
    fn a_window_on_a_monitor_stays_where_it_was() {
        assert!(is_reachable(window(100, 100), &[LAPTOP]));
        assert!(is_reachable(window(2500, 300), &[LAPTOP, EXTERNAL]));
    }

    #[test]
    fn a_window_on_an_unplugged_monitor_is_moved() {
        assert!(!is_reachable(window(2500, 300), &[LAPTOP]));
        assert!(!is_reachable(window(-3000, 0), &[LAPTOP]));
        assert!(!is_reachable(window(100, 100), &[]));
    }

    #[test]
    fn its_title_bar_must_show() {
        // Mostly off the left edge, but its right end is still grabbable.
        assert!(is_reachable(window(-1000, 100), &[LAPTOP]));
        // Only a sliver shows.
        assert!(!is_reachable(window(-1050, 100), &[LAPTOP]));
        // The title bar is above the top of every monitor.
        assert!(!is_reachable(window(100, -50), &[LAPTOP]));
        // The title bar is below the bottom.
        assert!(!is_reachable(window(100, 1070), &[LAPTOP]));
    }

    #[test]
    fn a_window_across_two_monitors_is_reachable() {
        assert!(is_reachable(window(1500, 200), &[LAPTOP, EXTERNAL]));
    }
}
