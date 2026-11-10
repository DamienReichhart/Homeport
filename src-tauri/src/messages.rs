//! The shell's own error messages (ADR 0011), and the English error
//! catalogue gathered from every crate.

#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use homeport_core::error::{arguments, Message};

homeport_core::messages! {
    /// The webview or a Tauri plugin failed.
    WINDOW_FAILED = "errors.shell.windowFailed" => "The app window failed.";
    UPDATE_DOWNLOAD = "errors.updates.download" => "Homeport couldn't download the update.";
    UPDATE_DOWNLOAD_HINT = "errors.updates.download.hint" => "Check your internet connection, then try again.";
    UPDATE_NONE_PUBLISHED = "errors.updates.nonePublished" => "No update is published for this system yet.";
    UPDATE_NONE_PUBLISHED_HINT = "errors.updates.nonePublished.hint" => "Try again later, or download Homeport from its website.";
    UPDATE_SIGNATURE = "errors.updates.signature" => "The update's signature doesn't match, so it wasn't installed.";
    UPDATE_SIGNATURE_HINT = "errors.updates.signature.hint" => "Download the new version from Homeport's website instead.";
    UPDATE_ADMIN_REFUSED = "errors.updates.adminRefused" => "Installing the update needs an administrator's approval, which was refused or cancelled.";
    UPDATE_ADMIN_REFUSED_HINT = "errors.updates.adminRefused.hint" => "Try again and enter an administrator password when asked.";
    UPDATE_NOT_ALLOWED = "errors.updates.notAllowed" => "Homeport isn't allowed to replace its own files.";
    UPDATE_NOT_ALLOWED_HINT = "errors.updates.notAllowed.hint" => "Move the AppImage to a folder you own, or install Homeport from its package repository.";
    UPDATE_WRONG_DISK = "errors.updates.wrongDisk" => "Homeport can't replace itself where it's installed.";
    UPDATE_WRONG_DISK_HINT = "errors.updates.wrongDisk.hint" => "Move Homeport to a folder on your main disk, or install it from its package repository.";
    UPDATE_FAILED = "errors.updates.failed" => "The update failed.";
    /// Installing from the app when a package manager owns the install.
    UPDATE_BY_PACKAGE_MANAGER = "errors.updates.byPackageManager" => "This copy of Homeport is updated by your package manager.";
    UPDATE_NOTHING_WAITING = "errors.updates.nothingWaiting" => "No update is waiting to be installed.";
    /// Turning start at login on or off failed.
    AUTOSTART_FAILED = "errors.desktop.autostartFailed" => "Homeport couldn't change whether it starts when you log in.";
    AUTOSTART_FAILED_HINT = "errors.desktop.autostartFailed.hint" => "Check that you may add login items on this computer, then try again.";
    /// An automatic deploy of a Synced Stack that ended without saying why.
    AUTO_DEPLOY_FAILED = "errors.stacks.autoDeployFailed" => "The automatic deploy didn't finish.";
    /// The tray menu (`native.`: drawn by the OS, not the window).
    TRAY_NO_HOSTS = "native.tray.noHosts" => "Homeport — no Hosts yet";
    TRAY_CONNECTED_ONE = "native.tray.connectedOfOne" => "Homeport — {connected} of 1 Host connected";
    TRAY_CONNECTED_MANY = "native.tray.connectedOfMany" => "Homeport — {connected} of {total} Hosts connected";
    TRAY_HOST = "native.tray.host" => "{host} — {status}";
    TRAY_CONNECTED = "native.tray.status.connected" => "Connected";
    TRAY_CONNECTING = "native.tray.status.connecting" => "Connecting";
    TRAY_UNREACHABLE = "native.tray.status.unreachable" => "Unreachable";
    TRAY_DISABLED = "native.tray.status.disabled" => "Disabled";
    TRAY_NEEDS_ATTENTION = "native.tray.status.needsAttention" => "Needs attention";
    TRAY_OPEN_HOST = "native.tray.openHost" => "Open";
    TRAY_RECONNECT = "native.tray.reconnect" => "Reconnect";
    TRAY_OPEN = "native.tray.open" => "Open Homeport";
    TRAY_ADD_HOST = "native.tray.addHost" => "Add host…";
    TRAY_QUIT = "native.tray.quit" => "Quit";
}

/// Where `bun run bindings` writes the English error catalogue.
#[cfg(test)]
const CATALOGUE_PATH: &str = "../src/shared/i18n/en/errors.json";
/// …and the English text the OS draws (tray, notifications).
#[cfg(test)]
const NATIVE_CATALOGUE_PATH: &str = "../src/shared/i18n/en/native.json";

/// Every crate's messages, by key; fails on a key declared twice with
/// different text, a key outside `errors.` and `native.`, or an argument
/// that isn't a plain name (Rust renders `{name}` arguments only).
#[cfg(test)]
fn catalogue() -> Result<BTreeMap<&'static str, &'static str>, String> {
    let all: [&[Message]; 14] = [
        homeport_core::messages::ALL,
        homeport_storage::messages::ALL,
        homeport_transport::messages::ALL,
        homeport_engine::messages::ALL,
        homeport_containerd::messages::ALL,
        homeport_incus::messages::ALL,
        homeport_hosts::messages::ALL,
        homeport_clusters::messages::ALL,
        homeport_nomad::messages::ALL,
        homeport_stacks::messages::ALL,
        homeport_registries::messages::ALL,
        homeport_git::messages::ALL,
        homeport_backup::messages::ALL,
        ALL,
    ];
    let mut catalogue = BTreeMap::new();
    for m in all.into_iter().flatten() {
        if !m.key.starts_with("errors.") && !m.key.starts_with("native.") {
            return Err(format!("{} doesn't start with errors. or native.", m.key));
        }
        for arg in arguments(m.en) {
            if arg.is_empty() || !arg.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(format!("{}: {{{arg}}} isn't a plain argument", m.key));
            }
        }
        if let Some(previous) = catalogue.insert(m.key, m.en) {
            if previous != m.en {
                return Err(format!("{} is declared twice with different text", m.key));
            }
        }
    }
    Ok(catalogue)
}

/// The messages under `prefix` as the JSON file the frontend imports.
#[cfg(test)]
fn catalogue_json(prefix: &str) -> String {
    let catalogue: BTreeMap<_, _> = catalogue()
        .unwrap_or_else(|e| panic!("{e}"))
        .into_iter()
        .filter(|(key, _)| key.starts_with(prefix))
        .collect();
    format!(
        "{}\n",
        serde_json::to_string_pretty(&catalogue).expect("strings serialise")
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    fn check(path: &str, prefix: &str) {
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let fresh = super::catalogue_json(prefix);
        if std::env::var_os("UPDATE_BINDINGS").is_some() {
            std::fs::write(&committed, fresh).unwrap();
            return;
        }
        assert_eq!(
            std::fs::read_to_string(&committed).unwrap_or_default(),
            fresh,
            "{path} is stale: run `bun run bindings`"
        );
    }

    #[test]
    fn bindings_are_up_to_date_for_errors() {
        check(super::CATALOGUE_PATH, "errors.");
    }

    #[test]
    fn bindings_are_up_to_date_for_native_text() {
        check(super::NATIVE_CATALOGUE_PATH, "native.");
    }
}
