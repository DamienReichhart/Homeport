//! The main window: one per Homeport (a second launch focuses it), opened
//! where it was left, and brought back from the tray.
//!
//! Single-instance and window-state are driven from Rust only, so the
//! frontend is granted no permission for them (`capabilities/default.json`).

use std::{collections::HashMap, fs};

use tauri::{AppHandle, LogicalSize, Manager, Monitor, PhysicalPosition, Runtime, WebviewWindow};
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

/// Opens the window at the default size, centred, when the place it was
/// saved at is off every monitor (a display was unplugged since). Reads the
/// saved place, since a window that isn't shown yet has no size on Linux.
pub fn keep_on_screen<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    let Some(saved) = saved_place(window.app_handle()) else {
        return Ok(());
    };
    let monitors = window.available_monitors()?;
    let rects: Vec<Rect> = monitors.iter().map(rect).collect();
    if is_reachable(saved, &rects) {
        return Ok(());
    }
    let Some(monitor) = window
        .primary_monitor()?
        .or_else(|| monitors.into_iter().next())
    else {
        return Ok(());
    };
    log::info!("the window was saved off every monitor; opening it centred");
    let size = DEFAULT_SIZE.to_physical::<u32>(monitor.scale_factor());
    let (x, y) = centred(rect(&monitor), size.width, size.height);
    window.unmaximize()?;
    window.set_size(size)?;
    window.set_position(PhysicalPosition { x, y })
}

/// Where the window-state plugin saved the main window, before maximising.
fn saved_place<R: Runtime>(app: &AppHandle<R>) -> Option<Rect> {
    #[derive(serde::Deserialize)]
    struct Saved {
        width: u32,
        height: u32,
        x: i32,
        y: i32,
        #[serde(default)]
        prev_x: i32,
        #[serde(default)]
        prev_y: i32,
        #[serde(default)]
        maximized: bool,
    }
    let path = app.path().app_config_dir().ok()?.join(app.filename());
    let mut saved: HashMap<String, Saved> = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
    let main = saved.remove(MAIN)?;
    // The plugin's placeholder for a window it never saw move.
    if main.width == 0 || main.height == 0 {
        return None;
    }
    let (x, y) = if main.maximized {
        (main.prev_x, main.prev_y)
    } else {
        (main.x, main.y)
    };
    Some(Rect {
        x,
        y,
        width: main.width,
        height: main.height,
    })
}

fn rect(monitor: &Monitor) -> Rect {
    Rect {
        x: monitor.position().x,
        y: monitor.position().y,
        width: monitor.size().width,
        height: monitor.size().height,
    }
}

/// The top-left corner of a `width` × `height` window centred on `monitor`,
/// kept on it when the window is bigger.
fn centred(monitor: Rect, width: u32, height: u32) -> (i32, i32) {
    let offset = |space: u32, size: u32| i32::try_from(space.saturating_sub(size) / 2).unwrap_or(0);
    (
        monitor.x + offset(monitor.width, width),
        monitor.y + offset(monitor.height, height),
    )
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
    fn the_default_window_is_centred_on_the_monitor_left() {
        assert_eq!(centred(LAPTOP, 1100, 720), (410, 180));
        assert_eq!(centred(EXTERNAL, 1100, 720), (1920 + 730, 360));
        // Bigger than the monitor: its corner stays on it.
        assert_eq!(centred(LAPTOP, 2200, 1440), (0, 0));
    }

    #[test]
    fn a_window_across_two_monitors_is_reachable() {
        assert!(is_reachable(window(1500, 200), &[LAPTOP, EXTERNAL]));
    }
}
