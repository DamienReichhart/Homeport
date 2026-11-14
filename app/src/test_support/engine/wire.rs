//! What the fake Engine reads and writes on the wire: a query's values,
//! and the tar archives a container's files come and go in.

/// The value of `name` in a query, percent-decoded.
pub(super) fn query_value(query: &str, name: &str) -> Option<String> {
    let value = query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .find(|(key, _)| *key == name)?
        .1;
    let mut bytes = vec![];
    let mut rest = value.as_bytes();
    while let Some((first, tail)) = rest.split_first() {
        match (first, tail) {
            (b'%', [high, low, tail @ ..]) => {
                let hex = [*high, *low];
                let hex = std::str::from_utf8(&hex).ok()?;
                bytes.push(u8::from_str_radix(hex, 16).ok()?);
                rest = tail;
            }
            (b'+', _) => {
                bytes.push(b' ');
                rest = tail;
            }
            _ => {
                bytes.push(*first);
                rest = tail;
            }
        }
    }
    String::from_utf8(bytes).ok()
}

/// The names of the entries at the top of a tar archive.
pub(super) fn entry_names(archive: &[u8]) -> Vec<String> {
    let mut archive = tar::Archive::new(archive);
    let Ok(entries) = archive.entries() else {
        return vec![];
    };
    entries
        .filter_map(Result::ok)
        .filter_map(|entry| Some(entry.path().ok()?.display().to_string()))
        .collect()
}

/// What a path of the container holds.
#[derive(Debug, Clone)]
pub(super) enum Held {
    File(Vec<u8>),
    Folder,
    /// A symbolic link, to this path.
    Link(String),
}

/// What the Engine says of a path (`X-Docker-Container-Path-Stat`), with
/// Go's mode bits for a folder and a link.
pub(super) fn stat_of(name: &str, held: &Held) -> String {
    let (size, mode, target) = match held {
        Held::File(bytes) => (bytes.len(), 0o644_u32, ""),
        Held::Folder => (4096, (1 << 31) | 0o755, ""),
        Held::Link(target) => (target.len(), (1 << 27) | 0o777, target.as_str()),
    };
    format!(
        r#"{{"name":"{name}","size":{size},"mode":{mode},"mtime":"2026-05-01T10:00:00Z","linkTarget":"{target}"}}"#
    )
}

/// What the Engine sends for a path: a tar of the file, of a folder that
/// holds one file, or of the link itself.
pub(super) fn archive_of(name: &str, held: &Held) -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    let mut header = tar::Header::new_gnu();
    header.set_mode(0o644);
    let _ = match held {
        Held::File(bytes) => {
            header.set_size(bytes.len() as u64);
            builder.append_data(&mut header, name, &bytes[..])
        }
        Held::Folder => {
            header.set_size(7);
            builder.append_data(&mut header, format!("{name}/inside.txt"), &b"inside\n"[..])
        }
        Held::Link(target) => {
            header.set_entry_type(tar::EntryType::Symlink);
            header.set_size(0);
            builder.append_link(&mut header, name, target)
        }
    };
    builder.into_inner().unwrap_or_default()
}

/// Where a download's answer waits, once `Engine::hold_transfers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stall {
    /// Every byte of the file but its last has gone.
    BeforeLastByte,
    /// The file's bytes have gone, not the archive's end.
    AfterData,
}
