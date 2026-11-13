//! What a portal answers a file dialog with: `file:` URIs, as paths.

use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};

/// The path a `file:` URI names: `file:///home/ada/a%20b.txt`, with or
/// without the `localhost` it may carry. `None` for any other URI, which
/// names nothing on this computer.
pub(super) fn path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let rest = rest.strip_prefix("localhost").unwrap_or(rest);
    if !rest.starts_with('/') {
        return None;
    }
    let bytes = rest.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] == b'%' {
            let digits = rest.get(at + 1..at + 3)?;
            decoded.push(u8::from_str_radix(digits, 16).ok()?);
            at += 3;
        } else {
            decoded.push(bytes[at]);
            at += 1;
        }
    }
    Some(PathBuf::from(OsString::from_vec(decoded)))
}

#[cfg(test)]
mod tests {
    use std::{os::unix::ffi::OsStrExt, path::Path};

    use super::*;

    #[test]
    fn a_file_uri_is_the_path_it_names() {
        let cases = [
            ("file:///home/ada/logs.txt", "/home/ada/logs.txt"),
            ("file://localhost/srv/a", "/srv/a"),
            (
                "file:///home/ada/My%20logs/100%25.txt",
                "/home/ada/My logs/100%.txt",
            ),
            ("file:///tmp/%E6%97%A5%E6%9C%AC.txt", "/tmp/日本.txt"),
            ("file:///", "/"),
        ];
        for (uri, expected) in cases {
            assert_eq!(path(uri).as_deref(), Some(Path::new(expected)), "{uri}");
        }
        // A name that is no text is still the file's name.
        let raw = path("file:///tmp/%FF.bin").unwrap();
        assert_eq!(raw.as_os_str().as_bytes(), b"/tmp/\xff.bin");
    }

    #[test]
    fn what_names_no_local_file_is_no_path() {
        for uri in [
            "",
            "/home/ada/logs.txt",
            "https://example.com/logs.txt",
            "file://server/share/logs.txt",
            "file:relative.txt",
            "file:///tmp/100%",
            "file:///tmp/%zz.txt",
            "file:///tmp/%é.txt",
        ] {
            assert_eq!(path(uri), None, "{uri}");
        }
    }
}
