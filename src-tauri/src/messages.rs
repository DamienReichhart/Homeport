//! The shell's own error messages (ADR 0011), and the English error
//! catalogue gathered from every crate.

#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use homeport_core::error::{arguments, Message};

homeport_core::messages! {
    /// Restoring Homeport's data from an export of some Hosts.
    BACKUP_IS_AN_EXPORT = "errors.backup.isAnExport" => "{path} is an export of Hosts, not a backup of Homeport's data.";
    BACKUP_IS_AN_EXPORT_HINT = "errors.backup.isAnExport.hint" => "Import its Hosts from the Hosts page instead, which adds them to the ones you have.";
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
    /// An automatic deploy of a Synced Stack that ended without saying why.
    AUTO_DEPLOY_FAILED = "errors.stacks.autoDeployFailed" => "The automatic deploy didn't finish.";
}

/// Where `bun run bindings` writes the English error catalogue.
#[cfg(test)]
const CATALOGUE_PATH: &str = "../src/shared/i18n/en/errors.json";

/// Every crate's messages, by key; fails on a key declared twice with
/// different text, a key outside `errors.`, or an argument that isn't a
/// plain name (Rust renders `{name}` arguments only).
#[cfg(test)]
fn catalogue() -> Result<BTreeMap<&'static str, &'static str>, String> {
    let all: [&[Message]; 9] = [
        homeport_core::messages::ALL,
        homeport_storage::messages::ALL,
        homeport_transport::messages::ALL,
        homeport_engine::messages::ALL,
        homeport_hosts::messages::ALL,
        homeport_stacks::messages::ALL,
        homeport_registries::messages::ALL,
        homeport_git::messages::ALL,
        ALL,
    ];
    let mut catalogue = BTreeMap::new();
    for m in all.into_iter().flatten() {
        if !m.key.starts_with("errors.") {
            return Err(format!("{} doesn't start with errors.", m.key));
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

/// The catalogue as the JSON file the frontend imports.
#[cfg(test)]
fn catalogue_json() -> String {
    let catalogue = catalogue().unwrap_or_else(|e| panic!("{e}"));
    format!(
        "{}\n",
        serde_json::to_string_pretty(&catalogue).expect("strings serialise")
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn bindings_are_up_to_date_for_errors() {
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join(super::CATALOGUE_PATH);
        let fresh = super::catalogue_json();
        if std::env::var_os("UPDATE_BINDINGS").is_some() {
            std::fs::write(&committed, fresh).unwrap();
            return;
        }
        assert_eq!(
            std::fs::read_to_string(&committed).unwrap_or_default(),
            fresh,
            "the error catalogue is stale: run `bun run bindings`"
        );
    }
}
