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

/// What hands a link or a folder to the OS: the OS ([`Native`]), or a
/// test's record of what was asked. Both may block for a moment, so they
/// are called off the window's thread.
pub trait Opener: Send + Sync {
    /// Opens a link or a folder with what the OS has for it. The OS runs
    /// a file that is a program: [`Platform`](super::Platform) only ever
    /// gives this a link it checked or a folder.
    fn open(&self, target: &OsStr) -> io::Result<()>;
    /// Shows `path` selected in its folder, in the file manager.
    fn reveal(&self, path: &Path) -> io::Result<()>;
}

/// `url` if it is a link the app opens. One that is not is refused as
/// what it is: there is nothing to try elsewhere with a `file:` or a
/// `javascript:` address, so no hint sends the user to their browser.
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
        Err(AppError::validation(msg::LINK_REFUSED.with("url", url)))
    }
}

/// The OS had nothing to open `url` with: a link the app does open.
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

/// `path` as a `file:` URI, every byte that isn't plainly safe escaped
/// (RFC 3986's unreserved characters and `/` are). `None` for a path
/// that doesn't start at the root: a URI can't say "from here".
fn file_uri(path: &Path) -> Option<String> {
    if !path.is_absolute() {
        return None;
    }
    let mut uri = String::from("file://");
    for &byte in path.as_os_str().as_encoded_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.' | b'~') {
            uri.push(char::from(byte));
        } else {
            uri.push_str(&format!("%{byte:02X}"));
        }
    }
    Some(uri)
}

/// A program to start, and what it is given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launch {
    pub program: &'static str,
    /// Arguments the OS quotes by its own rules.
    pub arguments: Vec<OsString>,
    /// On Windows, text put after them on the command line as it is: for
    /// a program that reads its command line by rules of its own.
    pub raw: Option<OsString>,
}

impl Launch {
    /// The command line as the program reads it, where quoting matters:
    /// what the tests hold Explorer's to.
    pub fn line(&self) -> OsString {
        let mut line = OsString::from(self.program);
        for argument in self.arguments.iter().chain(&self.raw) {
            line.push(" ");
            line.push(argument);
        }
        line
    }
}

/// What shows `path` selected in the file manager of `os`: Finder,
/// Explorer, or whichever file manager answers to
/// `org.freedesktop.FileManager1` on a Linux desktop. `path` is whole,
/// from the root: there is nothing to show of a relative one.
pub fn reveal_command(os: Os, path: &Path) -> io::Result<Launch> {
    if !path.is_absolute() && os != Os::Windows {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    Ok(match os {
        Os::Macos => Launch {
            program: "open",
            arguments: vec!["-R".into(), path.into()],
            raw: None,
        },
        // Explorer reads `/select,` and the path as one argument, in
        // quotes of its own: quoted the usual way (the whole argument,
        // or the path as a second one), a comma or a space in the path
        // opens the Documents folder instead. A path holds no quote.
        Os::Windows => {
            let mut select = OsString::from("/select,\"");
            select.push(path);
            select.push("\"");
            Launch {
                program: "explorer.exe",
                arguments: vec![],
                raw: Some(select),
            }
        }
        Os::Linux => {
            let uri = file_uri(path).ok_or(io::ErrorKind::InvalidInput)?;
            Launch {
                program: "gdbus",
                arguments: [
                    "call",
                    "--session",
                    "--dest",
                    "org.freedesktop.FileManager1",
                    "--object-path",
                    "/org/freedesktop/FileManager1",
                    "--method",
                    "org.freedesktop.FileManager1.ShowItems",
                    // The URI holds no quote and no comma: they are escaped.
                    &format!("['{uri}']"),
                    "",
                ]
                .map(OsString::from)
                .to_vec(),
                raw: None,
            }
        }
    })
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
        let launch = reveal_command(os, path)?;
        let mut command = Command::new(launch.program);
        command
            .args(&launch.arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        #[cfg(windows)]
        if let Some(raw) = &launch.raw {
            use std::os::windows::process::CommandExt as _;
            command.raw_arg(raw);
        }
        let shown = match os {
            // Explorer's exit code says nothing (it is 1 when it worked)
            // and it shows a folder of its own choosing for a path it
            // can't find: all that is known is whether it started. The
            // path was checked to exist before it was handed here.
            Os::Windows => command.spawn().is_ok(),
            // Without `gdbus` (it comes with GLib, so with GNOME, and
            // with most desktops' file managers) or without a file
            // manager that answers, the folder is opened below.
            Os::Macos | Os::Linux => command.status().is_ok_and(|status| status.success()),
        };
        if shown {
            return Ok(());
        }
        // No file manager took it (no D-Bus, no `gdbus`): its folder at
        // least. Never the file itself, which the OS would run.
        match path.parent() {
            Some(folder) if folder.is_dir() => self.open(folder.as_os_str()),
            _ => Err(io::ErrorKind::NotFound.into()),
        }
    }
}

#[cfg(test)]
mod tests;
