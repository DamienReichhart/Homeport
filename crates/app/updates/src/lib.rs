//! How this copy of Homeport gets its updates, decided from how it was installed.
//!
//! The in-app updater replaces the installers it understands (Windows, macOS and
//! the AppImage). A `.deb` or `.rpm` belongs to the system's package manager,
//! which updates it from the Homeport repository, so Homeport only says how.
//! So do the installs Scoop (an NSIS installer under `scoop\apps`) and the
//! AUR (`homeport-bin` repackages the `.deb` on Arch) make.

use std::path::Path;

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
    Scoop,
    Aur,
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

/// The package that installs Homeport, in every repository but the AUR.
const PACKAGE: &str = "homeport";
/// The AUR package, which repackages the release's `.deb`.
const AUR_PACKAGE: &str = "homeport-bin";

/// `os_release` is the contents of `/etc/os-release`, used to tell openSUSE
/// (zypper) from Fedora and RHEL (dnf) for an `.rpm` install, and Arch (the
/// AUR) for a `.deb`. `exe` is the running binary, under `scoop\apps` when
/// Scoop installed it.
pub fn update_method(bundle: Bundle, os_release: Option<&str>, exe: Option<&Path>) -> UpdateMethod {
    let manager = match bundle {
        Bundle::Nsis if exe.is_some_and(is_scoop) => PackageManager::Scoop,
        Bundle::AppImage | Bundle::Msi | Bundle::Nsis | Bundle::App => return UpdateMethod::InApp,
        Bundle::Unknown => return UpdateMethod::Manual,
        Bundle::Deb if os_release.is_some_and(|o| has_id(o, "arch")) => PackageManager::Aur,
        Bundle::Deb => PackageManager::Apt,
        Bundle::Rpm if os_release.is_some_and(|o| has_id(o, "suse")) => PackageManager::Zypper,
        Bundle::Rpm => PackageManager::Dnf,
    };
    let command = match manager {
        PackageManager::Apt => {
            format!("sudo apt update && sudo apt install --only-upgrade {PACKAGE}")
        }
        PackageManager::Dnf => format!("sudo dnf upgrade --refresh {PACKAGE}"),
        PackageManager::Zypper => format!("sudo zypper refresh && sudo zypper update {PACKAGE}"),
        PackageManager::Scoop => format!("scoop update {PACKAGE}"),
        // yay is the most common AUR helper; paru takes the same arguments.
        PackageManager::Aur => format!("yay -Syu {AUR_PACKAGE}"),
    };
    UpdateMethod::PackageManager { manager, command }
}

/// Whether `ID` or `ID_LIKE` in an os-release file names `family` (`suse`, `arch`).
fn has_id(os_release: &str, family: &str) -> bool {
    os_release
        .lines()
        .filter_map(|line| line.split_once('='))
        .filter(|(key, _)| matches!(key.trim(), "ID" | "ID_LIKE"))
        .flat_map(|(_, value)| value.trim().trim_matches(['"', '\'']).split_whitespace())
        .any(|id| id.contains(family))
}

/// Whether the binary sits in a Scoop app folder (`…\scoop\apps\homeport\…`).
fn is_scoop(exe: &Path) -> bool {
    let parts: Vec<String> = exe
        .to_string_lossy()
        .split(['\\', '/'])
        .map(str::to_lowercase)
        .collect();
    parts.windows(2).any(|w| w[0] == "scoop" && w[1] == "apps")
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
            assert_eq!(update_method(bundle, None, None), UpdateMethod::InApp);
        }
    }

    #[test]
    fn development_builds_update_manually() {
        assert_eq!(
            update_method(Bundle::Unknown, Some(FEDORA), None),
            UpdateMethod::Manual
        );
    }

    #[test]
    fn deb_updates_with_apt() {
        assert_eq!(
            update_method(Bundle::Deb, None, None),
            UpdateMethod::PackageManager {
                manager: PackageManager::Apt,
                command: "sudo apt update && sudo apt install --only-upgrade homeport".into(),
            }
        );
    }

    #[test]
    fn rpm_updates_with_dnf_unless_on_suse() {
        let manager = |os_release| match update_method(Bundle::Rpm, os_release, None) {
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
        assert!(!has_id("NAME=\"Not suse\"\nID=fedora\n", "suse"));
        assert!(has_id("ID='opensuse-leap'\n", "suse"));
    }

    #[test]
    fn scoop_installs_update_with_scoop() {
        let exe = Path::new(r"C:\Users\me\scoop\apps\homeport\current\Homeport.exe");
        assert_eq!(
            update_method(Bundle::Nsis, None, Some(exe)),
            UpdateMethod::PackageManager {
                manager: PackageManager::Scoop,
                command: "scoop update homeport".into(),
            }
        );
        let installed = Path::new(r"C:\Users\me\AppData\Local\Homeport\Homeport.exe");
        assert_eq!(
            update_method(Bundle::Nsis, None, Some(installed)),
            UpdateMethod::InApp
        );
    }

    #[test]
    fn the_deb_on_arch_came_from_the_aur() {
        const ARCH: &str = "NAME=\"Arch Linux\"\nID=arch\n";
        const MANJARO: &str = "NAME=\"Manjaro Linux\"\nID=manjaro\nID_LIKE=arch\n";
        for os_release in [ARCH, MANJARO] {
            assert_eq!(
                update_method(Bundle::Deb, Some(os_release), None),
                UpdateMethod::PackageManager {
                    manager: PackageManager::Aur,
                    command: "yay -Syu homeport-bin".into(),
                }
            );
        }
        const UBUNTU: &str = "ID=ubuntu\nID_LIKE=debian\n";
        assert!(matches!(
            update_method(Bundle::Deb, Some(UBUNTU), None),
            UpdateMethod::PackageManager {
                manager: PackageManager::Apt,
                ..
            }
        ));
    }
}
