//! Adds a running AppImage to the desktop's app menu. The .deb and .rpm
//! packages install a desktop entry, but an AppImage has no install step, so
//! it registers itself under `~/.local/share` each time it starts.

use std::{fs, io, path::Path};

/// Desktop entry file name. It differs from the packages' `Homeport.desktop`
/// so a user-level entry never shadows a system-wide install.
const ENTRY_NAME: &str = "homeport-appimage.desktop";
const ICON_NAME: &str = "homeport";
const ICON: &[u8] = include_bytes!("../../icons/128x128@2x.png");

/// Writes the desktop entry and icon into `data_dir` (usually `~/.local/share`)
/// when running from an AppImage. Does nothing otherwise.
pub fn integrate(data_dir: &Path) -> io::Result<()> {
    let Some(appimage) = std::env::var_os("APPIMAGE") else {
        return Ok(());
    };
    let Some(appimage) = appimage.to_str() else {
        return Err(io::Error::other("AppImage path is not valid UTF-8"));
    };
    write_if_changed(
        &data_dir
            .join("icons/hicolor/256x256/apps")
            .join(format!("{ICON_NAME}.png")),
        ICON,
    )?;
    write_if_changed(
        &data_dir.join("applications").join(ENTRY_NAME),
        desktop_entry(appimage).as_bytes(),
    )
}

/// Skips the write when the file is already up to date, so menus aren't
/// reloaded on every launch. The entry changes when the AppImage is moved.
fn write_if_changed(path: &Path, contents: &[u8]) -> io::Result<()> {
    if fs::read(path).is_ok_and(|current| current == contents) {
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, contents)
}

/// `TryExec` hides the entry once the AppImage is deleted.
fn desktop_entry(appimage: &str) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Homeport\n\
         Comment=Every Docker Engine, docked in one place.\n\
         Exec={exec}\n\
         TryExec={try_exec}\n\
         Icon={ICON_NAME}\n\
         Terminal=false\n\
         Categories=Development;\n\
         Keywords=docker;container;compose;\n\
         StartupWMClass=homeport\n",
        exec = escape_value(&quote_exec_arg(appimage)),
        try_exec = escape_value(appimage),
    )
}

/// Quotes a path as an `Exec` argument, per the Desktop Entry spec.
fn quote_exec_arg(arg: &str) -> String {
    let mut quoted = String::from("\"");
    for c in arg.chars() {
        match c {
            '"' | '`' | '$' | '\\' => quoted.extend(['\\', c]),
            '%' => quoted.push_str("%%"),
            _ => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}

/// Escapes a string value for a desktop entry file.
fn escape_value(value: &str) -> String {
    value.replace('\\', "\\\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_points_at_the_appimage() {
        let entry = desktop_entry("/home/me/Apps/Homeport.AppImage");
        assert!(entry.contains("\nExec=\"/home/me/Apps/Homeport.AppImage\"\n"));
        assert!(entry.contains("\nTryExec=/home/me/Apps/Homeport.AppImage\n"));
        assert!(entry.contains("\nCategories=Development;\n"));
    }

    #[test]
    fn exec_escapes_reserved_characters() {
        let entry = desktop_entry("/tmp/a b/$x\"100%\\.AppImage");
        assert!(entry.contains("\nExec=\"/tmp/a b/\\\\$x\\\\\"100%%\\\\\\\\.AppImage\"\n"));
        assert!(entry.contains("\nTryExec=/tmp/a b/$x\"100%\\\\.AppImage\n"));
    }

    #[test]
    fn rewrites_only_changed_files() {
        let dir = tempfile::tempdir().unwrap();
        let entry = dir.path().join("applications").join(ENTRY_NAME);
        write_if_changed(&entry, b"a").unwrap();
        let modified = fs::metadata(&entry).unwrap().modified().unwrap();
        write_if_changed(&entry, b"a").unwrap();
        assert_eq!(fs::metadata(&entry).unwrap().modified().unwrap(), modified);
        write_if_changed(&entry, b"b").unwrap();
        assert_eq!(fs::read(&entry).unwrap(), b"b");
    }
}
