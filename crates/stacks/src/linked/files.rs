//! A Linked Stack's compose files as they are on disk: their fingerprints,
//! so Homeport can tell when they changed outside it.

use std::{fmt::Write as _, path::Path};

use sha2::{Digest, Sha256};

/// The SHA-256 of a file's content, in hex.
pub fn content_fingerprint(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// One fingerprint for all of a Stack's files, in order: it changes when any
/// of them is edited, added, removed or reordered. A missing or unreadable
/// file counts as its own state, so it changes the fingerprint too.
pub fn files_fingerprint(files: &[String]) -> String {
    let mut hasher = Sha256::new();
    for file in files {
        hasher.update(file.as_bytes());
        hasher.update([0]);
        match std::fs::read(Path::new(file)) {
            Ok(bytes) => hasher.update(content_fingerprint(&bytes)),
            Err(_) => hasher.update("missing"),
        }
        hasher.update([0]);
    }
    hex(&hasher.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprints_follow_the_files_content_and_order() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("compose.yaml");
        let b = dir.path().join("compose.override.yaml");
        std::fs::write(&a, "services: {}\n").unwrap();
        std::fs::write(&b, "services: {}\n").unwrap();
        let a = a.to_string_lossy().into_owned();
        let b = b.to_string_lossy().into_owned();

        let first = files_fingerprint(&[a.clone(), b.clone()]);
        assert_eq!(first, files_fingerprint(&[a.clone(), b.clone()]));
        assert_ne!(first, files_fingerprint(&[b.clone(), a.clone()]));

        std::fs::write(&b, "services:\n  web:\n    image: nginx\n").unwrap();
        let edited = files_fingerprint(&[a.clone(), b.clone()]);
        assert_ne!(first, edited);

        std::fs::remove_file(&b).unwrap();
        assert_ne!(edited, files_fingerprint(&[a, b]));
        assert_eq!(
            content_fingerprint(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
