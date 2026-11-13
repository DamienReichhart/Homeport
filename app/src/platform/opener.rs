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
            assert_eq!(refused.code, ErrorCode::Validation, "{url}");
            assert_eq!(refused.key, "errors.file.linkRefused", "{url}");
            assert_eq!(
                refused.message,
                format!(
                    "{} isn't a web, mail or phone link, so Homeport didn't open it.",
                    url.trim()
                )
            );
            // Nothing sends the user to open a `file:` address elsewhere,
            // and nothing in English is added to what the catalogue says.
            assert_eq!((refused.hint, refused.detail), (None, None), "{url}");
        }
        // A link the app does open and the OS could not is another matter.
        let failed = link_not_opened("https://example.com");
        assert_eq!(failed.key, "errors.file.linkNotOpened");
        assert!(failed.hint.is_some());
    }

    #[test]
    fn each_system_is_asked_to_show_a_file_its_own_way() {
        let path = Path::new("/home/ada/My logs/it's, 100%.txt");
        let finder = reveal_command(Os::Macos, path).unwrap();
        assert_eq!(finder.program, "open");
        assert_eq!(finder.arguments, ["-R".into(), OsString::from(path)]);
        assert_eq!(finder.raw, None);

        // Explorer: one argument, the path in quotes of its own, with the
        // comma and the spaces that split it otherwise.
        let explorer = reveal_command(Os::Windows, Path::new(r"C:\Users\ada\My logs\a, b.txt"));
        let explorer = explorer.unwrap();
        assert_eq!(
            explorer.line(),
            OsString::from(r#"explorer.exe /select,"C:\Users\ada\My logs\a, b.txt""#)
        );
        assert!(
            explorer.arguments.is_empty(),
            "nothing is left to the OS to quote"
        );

        let files = reveal_command(Os::Linux, path).unwrap();
        assert_eq!(files.program, "gdbus");
        // Nothing of the path can end the text it is quoted in.
        assert_eq!(
            files.arguments[8],
            OsString::from("['file:///home/ada/My%20logs/it%27s%2C%20100%25.txt']")
        );
        assert_eq!(files.arguments[9], OsString::new());
        assert_eq!(files.arguments.len(), 10);
        assert_eq!(files.raw, None);
    }

    #[test]
    fn a_path_is_a_uri_only_from_the_root() {
        assert_eq!(
            file_uri(Path::new("/tmp/日本.txt")).as_deref(),
            Some("file:///tmp/%E6%97%A5%E6%9C%AC.txt")
        );
        // Every byte that is not plainly safe, whatever it is.
        assert_eq!(
            file_uri(Path::new("/a b/#?&=+;'\"[]/c~d-e_f.g")).as_deref(),
            Some("file:///a%20b/%23%3F%26%3D%2B%3B%27%22%5B%5D/c~d-e_f.g")
        );
        for relative in ["logs.txt", "./logs.txt", "../logs.txt", ""] {
            assert_eq!(file_uri(Path::new(relative)), None, "{relative:?}");
            for os in [Os::Linux, Os::Macos] {
                let refused = reveal_command(os, Path::new(relative)).unwrap_err();
                assert_eq!(refused.kind(), io::ErrorKind::InvalidInput);
            }
        }
    }
}
