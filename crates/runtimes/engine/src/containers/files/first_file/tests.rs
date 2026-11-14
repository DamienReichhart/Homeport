//! The file of an archive, read as it comes whatever the pieces it comes
//! in, from archives the `tar` crate writes as Docker does, and from
//! hostile ones: each gives the file's bytes or an error, never a panic, a
//! file cut short or an unbounded allocation.

use std::sync::atomic::AtomicBool;

use bytes::Bytes;
use homeport_core::error::{AppError, ErrorCode};

use super::copy_out;

/// A tar holding `name` with `bytes`, PAX records before it if given, then
/// another file.
fn archive(name: &str, bytes: &[u8], pax: Option<&[(&str, &[u8])]>) -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    if let Some(records) = pax {
        builder
            .append_pax_extensions(records.iter().copied())
            .unwrap();
    }
    let mut header = tar::Header::new_gnu();
    header.set_size(bytes.len() as u64);
    header.set_mode(0o644);
    builder.append_data(&mut header, name, bytes).unwrap();
    let mut other = tar::Header::new_gnu();
    other.set_size(5);
    builder
        .append_data(&mut other, "other", &b"other"[..])
        .unwrap();
    builder.into_inner().unwrap()
}

/// A header block of `kind` for `name`, `size` bytes long.
fn header(name: &str, kind: tar::EntryType, size: u64) -> Vec<u8> {
    let mut header = tar::Header::new_gnu();
    header.set_path(name).unwrap();
    header.set_entry_type(kind);
    header.set_size(size);
    header.set_mode(0o644);
    header.set_cksum();
    header.as_bytes().to_vec()
}

/// `bytes` padded to whole blocks.
fn padded(bytes: &[u8]) -> Vec<u8> {
    let mut bytes = bytes.to_vec();
    bytes.resize(bytes.len().div_ceil(512) * 512, 0);
    bytes
}

/// The end of an archive: two zero blocks.
fn end() -> Vec<u8> {
    vec![0; 1024]
}

/// What is read out of `archive` given in pieces of `piece` bytes, the
/// file expected `expected` bytes long.
fn read(archive: &[u8], piece: usize, expected: Option<u64>) -> Result<Vec<u8>, AppError> {
    let chunks = archive.chunks(piece).map(Bytes::copy_from_slice);
    let mut out = Vec::new();
    let done = copy_out(
        chunks,
        "/note.txt",
        expected,
        &mut out,
        std::path::Path::new("note.txt"),
        &AtomicBool::new(false),
    )?;
    assert!(done);
    Ok(out)
}

/// The message of what `read` failed with.
fn failure(archive: &[u8], expected: Option<u64>) -> String {
    let error = read(archive, 512, expected).expect_err("an error, not a file");
    assert!(
        matches!(error.code, ErrorCode::EngineError | ErrorCode::Validation),
        "{error:?}"
    );
    error.message
}

const CUT: &str = "The Docker Engine's archive of this file ended before the file did.";
const BAD: &str = "The Docker Engine's archive of this file can't be read.";
const LONG: &str =
    "The headers before this file in the Docker Engine's archive are too long to be read.";
const EMPTY: &str = "The Docker Engine sent an empty archive for this file.";
const NOT_REGULAR: &str = "/note.txt is a link or not a regular file, so it can't be downloaded.";

#[test]
fn a_file_is_read_out_whatever_its_pieces() {
    let bytes: Vec<u8> = (0..200_000_u32).map(|n| (n % 251) as u8).collect();
    let tar = archive("note.txt", &bytes, None);
    for piece in [1, 7, 511, 512, 513, 4096, 70_000, tar.len()] {
        let file = read(&tar, piece, Some(bytes.len() as u64)).unwrap();
        assert!(file == bytes, "pieces of {piece}");
    }
}

#[test]
fn a_long_name_and_pax_records_come_before_the_file() {
    let long = format!("{}/note.txt", "a-folder-with-a-long-name".repeat(8));
    assert_eq!(
        read(&archive(&long, b"hi\n", None), 100, None).unwrap(),
        b"hi\n"
    );
    let pax: &[(&str, &[u8])] = &[("path", b"pax-named.txt"), ("mtime", b"1700000000.5")];
    let tar = archive("note.txt", b"hi\n", Some(pax));
    assert_eq!(read(&tar, 3, Some(3)).unwrap(), b"hi\n");
    // A global header is records for every file: the file follows it.
    let mut tar = header("pax_global_header", tar::EntryType::XGlobalHeader, 0);
    tar.extend(archive("note.txt", b"hi\n", None));
    assert_eq!(read(&tar, 512, Some(3)).unwrap(), b"hi\n");
}

#[test]
fn a_pax_size_is_the_file_s_and_must_be_the_one_the_engine_gave() {
    // A file too large for the header's field says its size in PAX.
    let tar = archive("note.txt", b"0123456789", Some(&[("size", b"4")]));
    assert_eq!(read(&tar, 64, Some(4)).unwrap(), b"0123");
    assert_eq!(
        failure(&tar, Some(10)),
        "/note.txt changed while it was being downloaded. Download it again."
    );
    // Its detail is bare data: the two sizes.
    let error = read(&tar, 64, Some(10)).unwrap_err();
    assert_eq!(error.detail.as_deref(), Some("10 → 4"));
    // A `size=` inside another record's value is that value, not a size.
    let tar = archive(
        "note.txt",
        b"0123456789",
        Some(&[("comment", b"a\n7 size=4\n")]),
    );
    assert_eq!(read(&tar, 64, Some(10)).unwrap(), b"0123456789");
}

#[test]
fn a_link_or_anything_but_a_regular_file_is_refused_never_saved_empty() {
    use tar::EntryType::*;
    for kind in [
        Symlink,
        Link,
        Directory,
        Char,
        Block,
        Fifo,
        GNULongName,
        XHeader,
    ] {
        let mut tar = header("note.txt", kind, 0);
        tar.extend(end());
        // A long name or PAX records for nothing say so their own way.
        let message = failure(&tar, None);
        assert!(
            message == NOT_REGULAR || message == BAD || message == CUT,
            "{kind:?}: {message}"
        );
        if matches!(kind, Symlink | Link | Directory | Char | Block | Fifo) {
            assert_eq!(message, NOT_REGULAR, "{kind:?}");
        }
    }
    // GNU's sparse file: its holes are not in the archive.
    let mut tar = header("note.txt", tar::EntryType::GNUSparse, 0);
    tar.extend(end());
    assert!([NOT_REGULAR, BAD].contains(&failure(&tar, None).as_str()));
}

#[test]
fn a_huge_size_is_an_error_whatever_the_size_expected() {
    let mut block = header("note.txt", tar::EntryType::Regular, 0);
    // Base-256: the high bit, then every byte at its largest.
    block[124] = 0x80;
    block[125..136].fill(0xff);
    let mut fixed = tar::Header::new_old();
    fixed.as_mut_bytes().copy_from_slice(&block);
    fixed.set_cksum();
    let mut tar = fixed.as_bytes().to_vec();
    tar.extend(padded(b"hello"));
    tar.extend(end());
    for expected in [None, Some(5)] {
        let message = failure(&tar, expected);
        assert!(message != NOT_REGULAR, "{expected:?}: {message}");
    }
}

#[test]
fn global_headers_or_a_long_name_past_the_bound_are_refused() {
    // Twelve thousand empty global headers: six megabytes of nothing.
    let mut tar = header("g", tar::EntryType::XGlobalHeader, 0).repeat(12_000);
    tar.extend(archive("note.txt", b"hi\n", None));
    assert_eq!(failure(&tar, None), LONG);
    // Said by its message alone: no English of this crate's as detail.
    let error = read(&tar, 512, None).unwrap_err();
    assert_eq!(error.detail, None);
    // A long name of two megabytes, which the crate would read whole.
    let name = vec![b'a'; 2 * 1024 * 1024];
    let mut tar = header(
        "././@LongLink",
        tar::EntryType::GNULongName,
        name.len() as u64,
    );
    tar.extend(padded(&name));
    tar.extend(archive("note.txt", b"hi\n", None));
    assert_eq!(failure(&tar, None), LONG);
}

#[test]
fn an_archive_cut_short_or_holding_nothing_says_so() {
    let tar = archive("note.txt", &[7; 2000], None);
    // In the file's header, then in its bytes.
    assert_eq!(failure(&tar[..300], None), CUT);
    assert_eq!(failure(&tar[..1200], Some(2000)), CUT);
    assert_eq!(failure(&tar[..512], Some(2000)), CUT);
    let empty = tar::Builder::new(Vec::new()).into_inner().unwrap();
    assert_eq!(failure(&empty, None), EMPTY);
    assert_eq!(failure(&[], None), EMPTY);
    // An empty file is a file, though.
    let tar = archive("note.txt", b"", None);
    assert_eq!(read(&tar, 512, Some(0)).unwrap(), b"");
}

#[test]
fn a_header_whose_checksum_is_wrong_is_no_archive() {
    let mut tar = archive("note.txt", b"hello", None);
    tar[0] ^= 1;
    assert_eq!(failure(&tar, None), BAD);
}

#[test]
fn a_dropped_download_stops_between_chunks() {
    let tar = archive("note.txt", &[1; 300_000], None);
    let mut out = Vec::new();
    let done = copy_out(
        tar.chunks(4096).map(Bytes::copy_from_slice),
        "/note.txt",
        Some(300_000),
        &mut out,
        std::path::Path::new("note.txt"),
        &AtomicBool::new(true),
    )
    .unwrap();
    assert!(!done && out.is_empty());
}

#[test]
fn no_byte_of_the_headers_changed_makes_it_panic_or_cut_the_file() {
    let bytes = b"the file's bytes";
    let tar = archive("note.txt", bytes, Some(&[("mtime", b"1700000000")]));
    let headers = tar.len() - 3 * 512 - 1024;
    for at in 0..headers {
        for value in [0x00, 0x30, 0x37, 0x80, 0xff] {
            let mut changed = tar.clone();
            changed[at] = value;
            // Either the whole file, or an error.
            if let Ok(file) = read(&changed, 97, Some(bytes.len() as u64)) {
                assert_eq!(file.len(), bytes.len(), "byte {at} set to {value}");
            }
        }
    }
}
