//! How this copy of Homeport gets its updates, decided from how it was installed.
//!
//! The in-app updater replaces the installers it understands (Windows, macOS and
//! the AppImage). A `.deb` or `.rpm` belongs to the system's package manager,
//! which updates it from the Homeport repository, so Homeport only says how.

use serde::Serialize;

/// The bundle the running binary was installed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bundle {
    AppImage,
    Deb,
    Rpm,
    Msi,
    Nsis,
    App,
    /// Not a release bundle: `tauri dev`, `cargo run`, or a binary copied out of one.
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Apt,
    Dnf,
    Zypper,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum UpdateMethod {
    /// Homeport downloads the update, checks its signature, installs it and restarts.
    InApp,
    /// The package manager installs it; `command` is what the user runs.
    PackageManager {
        manager: PackageManager,
        command: String,
    },
    /// No installer to update (a development build): download it from the website.
    Manual,
}

/// The package that installs Homeport, in every repository.
const PACKAGE: &str = "homeport";

/// `os_release` is the contents of `/etc/os-release`, used to tell openSUSE
/// (zypper) from Fedora and RHEL (dnf) for an `.rpm` install.
pub fn update_method(bundle: Bundle, os_release: Option<&str>) -> UpdateMethod {
    let manager = match bundle {
        Bundle::AppImage | Bundle::Msi | Bundle::Nsis | Bundle::App => return UpdateMethod::InApp,
        Bundle::Unknown => return UpdateMethod::Manual,
        Bundle::Deb => PackageManager::Apt,
        Bundle::Rpm if os_release.is_some_and(is_suse) => PackageManager::Zypper,
        Bundle::Rpm => PackageManager::Dnf,
    };
    let command = match manager {
        PackageManager::Apt => {
            format!("sudo apt update && sudo apt install --only-upgrade {PACKAGE}")
        }
        PackageManager::Dnf => format!("sudo dnf upgrade --refresh {PACKAGE}"),
        PackageManager::Zypper => format!("sudo zypper refresh && sudo zypper update {PACKAGE}"),
    };
    UpdateMethod::PackageManager { manager, command }
}

/// Whether `ID` or `ID_LIKE` in an os-release file names SUSE.
fn is_suse(os_release: &str) -> bool {
    os_release
        .lines()
        .filter_map(|line| line.split_once('='))
        .filter(|(key, _)| matches!(key.trim(), "ID" | "ID_LIKE"))
        .flat_map(|(_, value)| value.trim().trim_matches(['"', '\'']).split_whitespace())
        .any(|id| id.contains("suse"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FEDORA: &str = "NAME=\"Fedora Linux\"\nID=fedora\nVERSION_ID=42\n";
    const TUMBLEWEED: &str =
        "NAME=\"openSUSE Tumbleweed\"\nID=\"opensuse-tumbleweed\"\nID_LIKE=\"opensuse suse\"\n";
    const SLES: &str = "NAME=\"SLES\"\nID=\"sles\"\nID_LIKE=\"suse\"\n";

    #[test]
    fn installers_update_in_app() {
        for bundle in [Bundle::AppImage, Bundle::Msi, Bundle::Nsis, Bundle::App] {
            assert_eq!(update_method(bundle, None), UpdateMethod::InApp);
        }
    }

    #[test]
    fn development_builds_update_manually() {
        assert_eq!(
            update_method(Bundle::Unknown, Some(FEDORA)),
            UpdateMethod::Manual
        );
    }

    #[test]
    fn deb_updates_with_apt() {
        assert_eq!(
            update_method(Bundle::Deb, None),
            UpdateMethod::PackageManager {
                manager: PackageManager::Apt,
                command: "sudo apt update && sudo apt install --only-upgrade homeport".into(),
            }
        );
    }

    #[test]
    fn rpm_updates_with_dnf_unless_on_suse() {
        let manager = |os_release| match update_method(Bundle::Rpm, os_release) {
            UpdateMethod::PackageManager { manager, .. } => manager,
            other => panic!("unexpected {other:?}"),
        };
        assert_eq!(manager(Some(FEDORA)), PackageManager::Dnf);
        assert_eq!(manager(None), PackageManager::Dnf);
        assert_eq!(manager(Some(TUMBLEWEED)), PackageManager::Zypper);
        assert_eq!(manager(Some(SLES)), PackageManager::Zypper);
    }

    #[test]
    fn suse_is_read_from_id_and_id_like_only() {
        assert!(!is_suse("NAME=\"Not suse\"\nID=fedora\n"));
        assert!(is_suse("ID='opensuse-leap'\n"));
    }
}
