//! The system tray: every Host with its live status, a way back into the
//! window, and **Quit**. Built from Rust only, so the frontend is granted no
//! tray permission; what the user picks reaches it as typed events.

mod menu;

use tauri::{
    menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Wry,
};
use tauri_specta::Event;

use super::{
    events::{TrayAddHostChosen, TrayHostChosen},
    window, AppState,
};
use menu::{Action, Item, TrayMenu};

/// The tray icon's id.
const ID: &str = "main";

/// Whether this desktop shows Homeport's tray icon. Some (GNOME without an
/// extension) accept it without showing it, which can't be told from here.
pub fn is_available(app: &AppHandle) -> bool {
    app.tray_by_id(ID).is_some()
}

/// Adds the tray icon, unless this desktop has no tray library.
pub fn create(app: &AppHandle) {
    #[cfg(target_os = "linux")]
    if !linux::has_appindicator() {
        log::warn!("no appindicator library found; Homeport runs without a tray icon");
        return;
    }
    let model = menu::menu(&hosts(app));
    let built = build_menu(app, &model).and_then(|menu| {
        let mut tray = TrayIconBuilder::with_id(ID)
            .tooltip(&model.tooltip)
            .menu(&menu)
            .on_menu_event(|app, event| act(app, event.id().as_ref()))
            .on_tray_icon_event(|tray, event| {
                if let TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } = event
                {
                    window::show(tray.app_handle());
                }
            });
        if let Some(icon) = app.default_window_icon() {
            tray = tray.icon(icon.clone());
        }
        tray.build(app)
    });
    if let Err(e) = built {
        log::warn!("couldn't add the tray icon: {e}");
    }
}

/// Rebuilds the menu and tooltip from the Hosts as they are now, e.g. after
/// a status change or a Host was renamed or removed.
pub fn refresh(app: &AppHandle) {
    let Some(tray) = app.tray_by_id(ID) else {
        return;
    };
    let model = menu::menu(&hosts(app));
    let handle = app.clone();
    // Menus are built on the main thread; don't hold the caller up.
    let queued = app.run_on_main_thread(move || {
        let updated = build_menu(&handle, &model)
            .and_then(|menu| tray.set_menu(Some(menu)))
            .and_then(|()| tray.set_tooltip(Some(&model.tooltip)));
        if let Err(e) = updated {
            log::warn!("couldn't update the tray menu: {e}");
        }
    });
    if let Err(e) = queued {
        log::warn!("couldn't update the tray menu: {e}");
    }
}

/// The stored Hosts with their statuses; none before the app's state exists.
fn hosts(app: &AppHandle) -> Vec<homeport_hosts::service::HostView> {
    let Some(state) = app.try_state::<AppState>() else {
        return Vec::new();
    };
    state.hosts.list().unwrap_or_else(|e| {
        log::warn!("couldn't list the Hosts for the tray: {e}");
        Vec::new()
    })
}

fn build_menu(app: &AppHandle, model: &TrayMenu) -> tauri::Result<Menu<Wry>> {
    let menu = Menu::new(app)?;
    let item = |action: Action, label: &str| {
        MenuItem::with_id(app, action.id(), label, true, None::<&str>)
    };
    for entry in &model.items {
        match entry {
            Item::Host {
                id,
                label,
                reconnect: false,
            } => menu.append(&item(Action::ChooseHost(id.clone()), label)?)?,
            Item::Host { id, label, .. } => {
                let open = item(Action::ChooseHost(id.clone()), "Open")?;
                let reconnect = item(Action::Reconnect(id.clone()), "Reconnect")?;
                let items: [&dyn IsMenuItem<Wry>; 2] = [&open, &reconnect];
                menu.append(&Submenu::with_items(app, label, true, &items)?)?;
            }
            Item::Separator => menu.append(&PredefinedMenuItem::separator(app)?)?,
            Item::Open => menu.append(&item(Action::Open, "Open Homeport")?)?,
            Item::AddHost => menu.append(&item(Action::AddHost, "Add host…")?)?,
            Item::Quit => menu.append(&item(Action::Quit, "Quit")?)?,
        }
    }
    Ok(menu)
}

/// Does what the menu item `id` stands for.
fn act(app: &AppHandle, id: &str) {
    let Some(action) = Action::parse(id) else {
        return;
    };
    let emitted = match action {
        Action::ChooseHost(host_id) => {
            window::show(app);
            TrayHostChosen { host_id }.emit(app)
        }
        Action::Reconnect(host_id) => {
            let app = app.clone();
            // The supervisor it restarts runs on the async runtime.
            tauri::async_runtime::spawn(async move {
                if let Some(state) = app.try_state::<AppState>() {
                    if let Err(e) = state.hosts.reconnect(&host_id) {
                        log::warn!("couldn't reconnect {host_id} from the tray: {e}");
                    }
                }
            });
            Ok(())
        }
        Action::Open => {
            window::show(app);
            Ok(())
        }
        Action::AddHost => {
            window::show(app);
            TrayAddHostChosen.emit(app)
        }
        // Always quits, whatever closing the window does.
        Action::Quit => {
            app.exit(0);
            Ok(())
        }
    };
    if let Err(e) = emitted {
        log::warn!("couldn't pass the tray's choice to the window: {e}");
    }
}

#[cfg(target_os = "linux")]
mod linux {
    //! The tray needs an appindicator library, which Homeport loads when it
    //! adds the icon and can't do without: look for it first.

    use std::path::{Path, PathBuf};

    const LIBRARIES: [&str; 4] = [
        "libayatana-appindicator3.so.1",
        "libappindicator3.so.1",
        "libayatana-appindicator3.so",
        "libappindicator3.so",
    ];

    /// Whether the dynamic loader can find one of [`LIBRARIES`]: in
    /// `LD_LIBRARY_PATH` (an AppImage's own libraries), the usual folders,
    /// or the linker's cache.
    pub(super) fn has_appindicator() -> bool {
        let arch = std::env::consts::ARCH;
        let mut dirs: Vec<PathBuf> = std::env::var_os("LD_LIBRARY_PATH")
            .map(|paths| std::env::split_paths(&paths).collect())
            .unwrap_or_default();
        dirs.extend(
            [
                format!("/usr/lib/{arch}-linux-gnu"),
                format!("/lib/{arch}-linux-gnu"),
                "/usr/lib64".into(),
                "/usr/lib".into(),
                "/lib64".into(),
                "/lib".into(),
                "/usr/local/lib".into(),
            ]
            .map(PathBuf::from),
        );
        if dirs.iter().any(|dir| in_dir(dir)) {
            return true;
        }
        ["/sbin/ldconfig", "/usr/sbin/ldconfig", "ldconfig"]
            .iter()
            .filter_map(|ldconfig| std::process::Command::new(ldconfig).arg("-p").output().ok())
            .any(|out| in_cache(&String::from_utf8_lossy(&out.stdout)))
    }

    fn in_dir(dir: &Path) -> bool {
        LIBRARIES.iter().any(|name| dir.join(name).exists())
    }

    /// Whether `ldconfig -p`'s output lists one of [`LIBRARIES`].
    fn in_cache(listing: &str) -> bool {
        listing.lines().any(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|name| LIBRARIES.contains(&name))
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn finds_the_library_in_the_linkers_cache() {
            let listing = "1024 libs found in cache `/etc/ld.so.cache'\n\
                \tlibayatana-appindicator3.so.1 (libc6,x86-64) => /lib/x86_64-linux-gnu/libayatana-appindicator3.so.1\n\
                \tlibz.so.1 (libc6,x86-64) => /lib/x86_64-linux-gnu/libz.so.1\n";
            assert!(in_cache(listing));
            assert!(!in_cache(
                "\tlibz.so.1 (libc6,x86-64) => /lib/x86_64-linux-gnu/libz.so.1\n"
            ));
        }

        #[test]
        fn finds_the_library_in_a_folder() {
            let dir = tempfile::tempdir().unwrap();
            assert!(!in_dir(dir.path()));
            std::fs::write(dir.path().join("libappindicator3.so.1"), b"").unwrap();
            assert!(in_dir(dir.path()));
        }
    }
}
