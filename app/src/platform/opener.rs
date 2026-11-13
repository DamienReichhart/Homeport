//! What opens things outside the app: a link in the browser, a folder in
//! the file manager, a file shown where it lies. What
//! `tauri-plugin-opener` did.

use std::{
    ffi::{OsStr, OsString},
    io,
    path::Path,
    process::{Command, Stdio},
};

use homeport_core::{
    error::{AppError, AppResult, ErrorCode},
    messages as msg,
};

/// The kinds of link the app hands to the OS, as the Tauri app allowed:
/// anything else (a `file:` URL, a program's own scheme) is refused, so
/// what a container or a repository names can't start a program here.
const SCHEMES: &[&str] = &["http://", "https://", "mailto:", "tel:"];

/// What hands a link or a path to the OS: the OS ([`Native`]), or a
/// test's record of what was asked. Both may block for a moment, so they
/// are called off the window's thread.
pub trait Opener: Send + Sync {
    /// Opens a link or a path with what the OS has for it.
    fn open(&self, target: &OsStr) -> io::Result<()>;
    /// Shows `path` selected in its folder, in the file manager.
    fn reveal(&self, path: &Path) -> io::Result<()>;
}

/// `url` if it is a link the app opens.
pub fn link(url: &str) -> AppResult<&str> {
    let url = url.trim();
    let lower = url.to_ascii_lowercase();
    let known = SCHEMES
        .iter()
        .any(|scheme| lower.starts_with(scheme) && lower.len() > scheme.len());
    // A line break or a space would end the address for the OS's handler.
    if known && !url.chars().any(|c| c.is_control() || c == ' ') {
        Ok(url)
    } else {
        Err(link_not_opened(url).with_detail("only http, https, mailto and tel links are opened"))
    }
}

/// The OS had nothing to open `url` with, or it isn't a link the app opens.
pub fn link_not_opened(url: &str) -> AppError {
    AppError::new(ErrorCode::Io, msg::LINK_NOT_OPENED.with("url", url))
        .with_hint(msg::LINK_NOT_OPENED_HINT)
}

/// The system the app runs on, as far as showing a file goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Os {
    Linux,
    Macos,
    Windows,
}

impl Os {
    pub const fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::Macos
        } else if cfg!(windows) {
            Self::Windows
        } else {
            Self::Linux
        }
    }
}

/// `path` as a `file:` URI, every byte that isn't plainly safe escaped.
fn file_uri(path: &Path) -> String {
    let mut uri = String::from("file://");
    for &byte in path.as_os_str().as_encoded_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.' | b'~') {
            uri.push(char::from(byte));
        } else {
            uri.push_str(&format!("%{byte:02X}"));
        }
    }
    uri
}

/// The program that shows `path` selected in the file manager of `os`,
/// with its arguments: Finder, Explorer, or whichever file manager
/// answers to `org.freedesktop.FileManager1` on a Linux desktop.
pub fn reveal_command(os: Os, path: &Path) -> (&'static str, Vec<OsString>) {
    match os {
        Os::Macos => ("open", vec!["-R".into(), path.into()]),
        Os::Windows => ("explorer.exe", vec!["/select,".into(), path.into()]),
        Os::Linux => (
            "gdbus",
            [
                "call",
                "--session",
                "--dest",
                "org.freedesktop.FileManager1",
                "--object-path",
                "/org/freedesktop/FileManager1",
                "--method",
                "org.freedesktop.FileManager1.ShowItems",
                // The URI holds no quote and no comma: they are escaped.
                &format!("['{}']", file_uri(path)),
                "",
            ]
            .map(OsString::from)
            .to_vec(),
        ),
    }
}

/// The OS's own handlers.
#[derive(Debug, Clone, Copy, Default)]
pub struct Native;

impl Opener for Native {
    fn open(&self, target: &OsStr) -> io::Result<()> {
        // Detached: the browser or the file manager outlives the call,
        // and the app never waits for it.
        open::that_detached(target)
    }

    fn reveal(&self, path: &Path) -> io::Result<()> {
        let os = Os::current();
        let (program, arguments) = reveal_command(os, path);
        let mut command = Command::new(program);
        command
            .args(arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let shown = match os {
            // Explorer's exit code says nothing: it is 1 when it worked.
            Os::Windows => command.spawn().is_ok(),
            Os::Macos | Os::Linux => command.status().is_ok_and(|status| status.success()),
        };
        if shown {
            return Ok(());
        }
        // No file manager took it (no D-Bus, no `gdbus`): its folder at least.
        match path.parent() {
            Some(folder) if !folder.as_os_str().is_empty() => self.open(folder.as_os_str()),
            _ => self.open(path.as_os_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_mail_and_phone_links_are_opened_and_nothing_else() {
        for url in [
            "http://localhost:8080",
            "https://homeport.example/docs?q=a%20b#top",
            "HTTPS://EXAMPLE.COM",
            "mailto:ada@example.com",
            "tel:+33123456789",
            "  https://example.com  ",
        ] {
            assert_eq!(link(url).unwrap(), url.trim(), "{url}");
        }
        for url in [
            "",
            "https://",
            "file:///etc/passwd",
            "/usr/bin/xterm",
            "javascript:alert(1)",
            "vscode://open",
            "smb://server/share",
            "example.com",
            "https://example.com/a b",
            "https://example.com/\nfile:///etc/passwd",
            "-h",
        ] {
            let refused = link(url).unwrap_err();
            assert_eq!(refused.code, ErrorCode::Io, "{url}");
            assert_eq!(refused.message, format!("{} can't be opened.", url.trim()));
            assert!(refused.hint.is_some(), "{url}");
        }
    }

    #[test]
    fn each_system_is_asked_to_show_a_file_its_own_way() {
        let path = Path::new("/home/ada/My logs/it's, 100%.txt");
        let (program, arguments) = reveal_command(Os::Macos, path);
        assert_eq!(program, "open");
        assert_eq!(arguments, ["-R".into(), OsString::from(path)]);

        let (program, arguments) = reveal_command(Os::Windows, Path::new(r"C:\Users\ada\a b.txt"));
        assert_eq!(program, "explorer.exe");
        assert_eq!(
            arguments,
            ["/select,", r"C:\Users\ada\a b.txt"].map(OsString::from)
        );

        let (program, arguments) = reveal_command(Os::Linux, path);
        assert_eq!(program, "gdbus");
        // Nothing of the path can end the text it is quoted in.
        assert_eq!(
            arguments[8],
            OsString::from("['file:///home/ada/My%20logs/it%27s%2C%20100%25.txt']")
        );
        assert_eq!(arguments[9], OsString::new());
        assert_eq!(arguments.len(), 10);
    }

    #[test]
    fn a_path_in_any_script_is_a_valid_uri() {
        assert_eq!(
            file_uri(Path::new("/tmp/日本.txt")),
            "file:///tmp/%E6%97%A5%E6%9C%AC.txt"
        );
    }
}
