//! The first file of an archive, read as it comes, whatever the pieces
//! it comes in, from archives the `tar` crate writes as Docker does.

use super::{size_of, Untar};

/// A tar holding `name` with `bytes`, then another file.
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

/// What `untar` reads of `archive` given in pieces of `piece` bytes.
fn read(archive: &[u8], piece: usize) -> (Vec<u8>, Untar) {
    let mut untar = Untar::default();
    let mut file = vec![];
    for chunk in archive.chunks(piece) {
        file.extend_from_slice(&untar.feed(chunk).unwrap());
        if untar.is_done() {
            break;
        }
    }
    (file, untar)
}

#[test]
fn a_file_is_read_out_whatever_its_pieces() {
    let bytes: Vec<u8> = (0..5000_u32).map(|n| (n % 251) as u8).collect();
    let tar = archive("note.txt", &bytes, None);
    for piece in [1, 7, 511, 512, 513, 4096, tar.len()] {
        let (file, untar) = read(&tar, piece);
        assert_eq!(file, bytes, "pieces of {piece}");
        assert!(untar.is_done() && !untar.is_empty());
    }
}

#[test]
fn a_long_name_and_a_pax_header_come_before_the_file() {
    let long = format!("{}/note.txt", "a-folder-with-a-long-name".repeat(8));
    let (file, _) = read(&archive(&long, b"hello\n", None), 100);
    assert_eq!(file, b"hello\n");
    let pax: &[(&str, &[u8])] = &[("path", b"pax-named.txt"), ("mtime", b"1700000000.5")];
    let (file, _) = read(&archive("note.txt", b"hello\n", Some(pax)), 3);
    assert_eq!(file, b"hello\n");
}

#[test]
fn a_pax_size_is_the_file_s() {
    // A file too large for the header's field says its size in PAX: the
    // header's own is then not the one to read.
    let bytes = b"0123456789";
    let mut tar = archive("note.txt", bytes, Some(&[("size", b"4")]));
    let (file, untar) = read(&tar, 64);
    assert_eq!(file, b"0123");
    assert!(untar.is_done());
    tar.clear();
}

#[test]
fn an_archive_that_ends_early_or_holds_nothing_says_so() {
    let tar = archive("note.txt", &[7; 2000], None);
    let (file, untar) = read(&tar[..1200], 100);
    assert_eq!(file.len(), 1200 - 512);
    assert!(!untar.is_done());
    let empty = tar::Builder::new(Vec::new()).into_inner().unwrap();
    let (file, untar) = read(&empty, 512);
    assert!(file.is_empty() && untar.is_empty());
}

#[test]
fn a_size_is_octal_or_base_256() {
    assert_eq!(size_of(b"00000000012\0").unwrap(), 10);
    assert_eq!(size_of(b"           \0").unwrap(), 0);
    let mut large = [0_u8; 12];
    large[0] = 0x80;
    large[11] = 2;
    large[10] = 1;
    assert_eq!(size_of(&large).unwrap(), 258);
    assert!(size_of(b"not a size!\0").is_err());
}
