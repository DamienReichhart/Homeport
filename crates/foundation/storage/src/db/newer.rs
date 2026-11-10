//! Refusing a database that a newer Homeport has migrated, before anything
//! is written to it.
//!
//! Migrations only go forward, so an older Homeport can't open such a file.
//! It must not try either: opening it for writing switches the journal mode
//! and leaves `-wal` and `-shm` files behind. The schema version is read
//! from the file's header instead, and the version to install from
//! `database_info` through a connection that takes no lock and writes nothing.

use std::{fs::File, io::Read, path::Path};

use rusqlite::{Connection, OpenFlags, OptionalExtension};

use homeport_core::error::AppResult;

use super::error::from_newer;

/// The first bytes of every SQLite database file.
const MAGIC: &[u8; 16] = b"SQLite format 3\0";
/// Where `PRAGMA user_version` lives in the header, big-endian.
const USER_VERSION_AT: usize = 60;

/// The `database_info` key naming the oldest Homeport able to open the file.
pub(super) const REQUIRES: &str = "requires";

/// Fails when the database at `path` is past migration `known`, having
/// changed nothing: not the file, and no file beside it.
pub(super) fn refuse_newer(path: &Path, known: usize) -> AppResult<()> {
    let found = if has_pending_wal(path) {
        // The newest header may only be in the write-ahead log, which a plain
        // read-only connection reads; its side files are there already.
        read_only(path).and_then(|conn| user_version(&conn).map(|v| (v, requires(&conn))))
    } else {
        header_version(path).map(|v| (v, None))
    };
    let Some((version, mut required)) = found else {
        // Missing, empty or not a database: opening it says what's wrong.
        return Ok(());
    };
    if version <= known {
        return Ok(());
    }
    if required.is_none() {
        required = immutable(path).and_then(|conn| requires(&conn));
    }
    Err(from_newer(required.as_deref(), version, known))
}

/// Whether a write-ahead log with content sits beside the database: its
/// last writer didn't close it, so the file alone may be out of date.
fn has_pending_wal(path: &Path) -> bool {
    let mut wal = path.as_os_str().to_owned();
    wal.push("-wal");
    std::fs::metadata(wal).is_ok_and(|m| m.len() > 0)
}

/// `user_version` as the file's header has it, or `None` when the file
/// isn't there or isn't a SQLite database.
fn header_version(path: &Path) -> Option<usize> {
    let mut header = [0u8; 100];
    File::open(path).ok()?.read_exact(&mut header).ok()?;
    if &header[..MAGIC.len()] != MAGIC {
        return None;
    }
    let bytes: [u8; 4] = header[USER_VERSION_AT..USER_VERSION_AT + 4]
        .try_into()
        .ok()?;
    usize::try_from(u32::from_be_bytes(bytes)).ok()
}

fn user_version(conn: &Connection) -> Option<usize> {
    conn.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
        .ok()
        .and_then(|v| usize::try_from(v).ok())
}

/// The version `database_info` asks for; databases from before 1.0 have no
/// such table.
fn requires(conn: &Connection) -> Option<String> {
    conn.query_row(
        "SELECT value FROM database_info WHERE key = ?1",
        [REQUIRES],
        |r| r.get(0),
    )
    .optional()
    .ok()
    .flatten()
}

fn read_only(path: &Path) -> Option<Connection> {
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).ok()
}

/// A connection that treats the file as read-only media: no lock, no
/// journal, no `-wal` or `-shm` created.
fn immutable(path: &Path) -> Option<Connection> {
    let uri = format!("{}?immutable=1", file_uri(path)?);
    Connection::open_with_flags(
        uri,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
    )
    .ok()
}

/// `path` as a `file:` URI SQLite accepts, or `None` when it isn't UTF-8.
fn file_uri(path: &Path) -> Option<String> {
    let text = path.to_str()?.replace('\\', "/");
    let mut uri = String::from("file://");
    // `C:/Users/…` becomes `file:///C:/Users/…`.
    if !text.starts_with('/') {
        uri.push('/');
    }
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' | b':' => {
                uri.push(char::from(byte));
            }
            _ => uri.push_str(&format!("%{byte:02X}")),
        }
    }
    Some(uri)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_become_uris_sqlite_can_open() {
        assert_eq!(
            file_uri(Path::new("/home/me/My Data?/homeport.db")).unwrap(),
            "file:///home/me/My%20Data%3F/homeport.db"
        );
        assert_eq!(
            file_uri(Path::new(r"C:\Users\Zoé\homeport.db")).unwrap(),
            "file:///C:/Users/Zo%C3%A9/homeport.db"
        );
    }

    #[test]
    fn files_that_are_not_databases_are_left_to_sqlite() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("homeport.db");
        assert!(refuse_newer(&path, 3).is_ok(), "missing");
        std::fs::write(&path, b"").unwrap();
        assert!(refuse_newer(&path, 3).is_ok(), "empty");
        std::fs::write(&path, vec![b'x'; 4096]).unwrap();
        assert!(refuse_newer(&path, 3).is_ok(), "not a database");
    }
}
