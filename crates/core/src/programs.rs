//! Finds the Docker-related programs installed on this machine.
//!
//! An app started from the Finder, a desktop launcher or the Start menu gets a
//! minimal PATH (on macOS, no `/usr/local/bin` or `/opt/homebrew/bin`), so
//! looking at PATH alone would report an installed Docker as missing. The
//! usual install locations are searched too, and handed to the programs
//! Homeport runs.

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

/// Where Docker, Podman, Colima, OrbStack and Rancher Desktop put their CLIs,
/// beyond what a GUI app's PATH usually holds.
fn install_dirs(home: Option<&Path>) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = if cfg!(target_os = "macos") {
        [
            "/usr/local/bin",
            "/opt/homebrew/bin",
            "/opt/local/bin",
            "/Applications/Docker.app/Contents/Resources/bin",
            "/Applications/OrbStack.app/Contents/MacOS/xbin",
            "/Applications/Rancher Desktop.app/Contents/Resources/resources/darwin/bin",
        ]
        .map(PathBuf::from)
        .to_vec()
    } else if cfg!(windows) {
        let program_files = std::env::var_os("ProgramFiles")
            .map_or_else(|| PathBuf::from(r"C:\Program Files"), PathBuf::from);
        vec![
            program_files.join(r"Docker\Docker\resources\bin"),
            program_files.join(r"RedHat\Podman"),
        ]
    } else {
        ["/usr/local/bin", "/usr/bin", "/bin", "/snap/bin"]
            .map(PathBuf::from)
            .to_vec()
    };
    if let Some(home) = home {
        for dir in [
            ".docker/bin",
            ".orbstack/bin",
            ".rd/bin",
            ".local/bin",
            "bin",
        ] {
            dirs.push(home.join(dir));
        }
    }
    dirs
}

/// Apps that provide an Engine even when their CLI isn't linked anywhere.
const APP_BUNDLES: &[&str] = &[
    "/Applications/Docker.app",
    "/Applications/OrbStack.app",
    "/Applications/Rancher Desktop.app",
    r"C:\Program Files\Docker\Docker",
];

fn home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

/// PATH followed by the usual install locations, without duplicates.
fn search_dirs(path: Option<OsString>, home: Option<&Path>) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = path
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    for dir in install_dirs(home) {
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    }
    dirs
}

fn find_in(program: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    let names: Vec<String> = if cfg!(windows) {
        vec![format!("{program}.exe"), format!("{program}.cmd")]
    } else {
        vec![program.to_owned()]
    };
    dirs.iter()
        .flat_map(|dir| names.iter().map(move |n| dir.join(n)))
        .find(|candidate| candidate.metadata().is_ok_and(|m| m.is_file()))
}

/// The full path of `program`, from PATH or a usual install location.
pub fn find(program: &str) -> Option<PathBuf> {
    find_in(
        program,
        &search_dirs(std::env::var_os("PATH"), home().as_deref()),
    )
}

/// A PATH for programs Homeport runs, so they find their own helpers (such as
/// Docker credential helpers) even when Homeport's PATH is minimal.
pub fn search_path() -> OsString {
    std::env::join_paths(search_dirs(std::env::var_os("PATH"), home().as_deref()))
        .unwrap_or_else(|_| std::env::var_os("PATH").unwrap_or_default())
}

/// Whether some Docker-compatible Engine is installed, even if it isn't running.
pub fn engine_installed() -> bool {
    ["docker", "dockerd", "podman", "colima", "orb"]
        .iter()
        .any(|program| find(program).is_some())
        || APP_BUNDLES.iter().any(|app| Path::new(app).exists())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn programs_outside_path_are_found_in_install_locations() {
        let home = tempfile::tempdir().unwrap();
        let bin = home.path().join(".docker/bin");
        std::fs::create_dir_all(&bin).unwrap();
        // A name no real install has, so the machine's own PATH can't answer.
        let program = "homeport-test-cli";
        let file = if cfg!(windows) {
            format!("{program}.exe")
        } else {
            program.to_owned()
        };
        std::fs::write(bin.join(&file), "").unwrap();

        // A GUI app's PATH, without the install location.
        let dirs = search_dirs(Some(OsString::from("/nonexistent")), Some(home.path()));
        assert_eq!(find_in(program, &dirs), Some(bin.join(file)));
        assert_eq!(find_in("homeport-missing-cli", &dirs), None);
    }

    #[test]
    fn path_comes_first_and_is_not_repeated() {
        let home = tempfile::tempdir().unwrap();
        let first = install_dirs(Some(home.path()))[0].clone();
        let path = std::env::join_paths([first.clone()]).unwrap();
        let dirs = search_dirs(Some(path), Some(home.path()));
        assert_eq!(dirs[0], first);
        assert_eq!(dirs.iter().filter(|d| **d == first).count(), 1);
    }
}
