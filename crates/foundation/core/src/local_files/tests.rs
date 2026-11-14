use super::*;

#[test]
fn oversized_files_are_refused_by_name() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("huge.pem");
    std::fs::write(&path, vec![b'a'; MAX_TEXT_FILE as usize + 1]).unwrap();
    let e = read_text(&path).unwrap_err();
    assert_eq!(e.code, ErrorCode::FileTooLarge);
    assert!(e.message.contains("huge.pem"));
}

#[test]
fn a_replacement_lands_whole_or_not_at_all() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("homeport.backup");
    std::fs::write(&path, "last week's").unwrap();
    let unfinished = Replacement::new(&path).unwrap();
    std::io::Write::write_all(&mut unfinished.file(), b"half of this").unwrap();
    drop(unfinished);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "last week's");
    write_replacing(&path, b"this week's").unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "this week's");
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[test]
fn a_file_that_is_longer_than_it_says_is_still_refused_at_the_limit() {
    let path = Path::new("/proc/kallsyms");
    // No size was asked: what is read is what counts.
    let endless = io::repeat(b'a');
    let e = read_bounded(endless, path).unwrap_err();
    assert_eq!(e.code, ErrorCode::FileTooLarge);
    // Up to the limit it is read whole, as before.
    let full = vec![b'a'; MAX_TEXT_FILE as usize];
    assert_eq!(read_bounded(&full[..], path).unwrap().len(), full.len());
    let not_text = read_bounded(&[0xff, 0xfe][..], path).unwrap_err();
    assert_eq!(not_text.code, ErrorCode::Validation);
    assert_eq!(not_text.message, "/proc/kallsyms isn't a text file.");
}

#[cfg(unix)]
#[test]
fn what_is_no_regular_file_is_refused_without_being_opened() {
    let dir = tempfile::tempdir().unwrap();
    let folder = read_text(dir.path()).unwrap_err();
    assert!(folder.message.ends_with("is a folder, not a file."));
    let device = read_text(Path::new("/dev/zero")).unwrap_err();
    assert_eq!(device.code, ErrorCode::Validation);
    assert_eq!(device.message, "/dev/zero isn't a regular file.");
    // A pipe nobody writes to would keep a reader waiting for ever.
    let pipe = dir.path().join("pipe");
    let made = std::process::Command::new("mkfifo").arg(&pipe).status();
    assert!(made.is_ok_and(|status| status.success()), "mkfifo");
    let (answer, answered) = std::sync::mpsc::channel();
    std::thread::spawn(move || answer.send(read_text(&pipe)));
    let refused = answered
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("the read came back")
        .unwrap_err();
    assert!(refused.message.ends_with("pipe isn't a regular file."));
    // A link to a regular file is read as the file.
    let file = dir.path().join("key.pem");
    std::fs::write(&file, "secret").unwrap();
    let link = dir.path().join("link.pem");
    std::os::unix::fs::symlink(&file, &link).unwrap();
    assert_eq!(read_text(&link).unwrap(), "secret");
}

#[cfg(unix)]
#[test]
fn a_replacement_is_private_until_it_is_given_a_mode() {
    use std::os::unix::fs::PermissionsExt;
    let mode = |file: &std::fs::File| file.metadata().unwrap().permissions().mode() & 0o777;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    let unfinished = Replacement::new(&path).unwrap();
    assert_eq!(mode(unfinished.file()), 0o600);
    // What any new file here gets, and nothing is left of asking.
    let plain = dir.path().join("plain.txt");
    std::fs::write(&plain, "").unwrap();
    unfinished.take_default_mode().unwrap();
    assert_eq!(
        mode(unfinished.file()),
        std::fs::metadata(&plain).unwrap().permissions().mode() & 0o777
    );
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
    std::fs::set_permissions(&plain, std::fs::Permissions::from_mode(0o640)).unwrap();
    unfinished
        .keep_mode_of(&std::fs::metadata(&plain).unwrap())
        .unwrap();
    assert_eq!(mode(unfinished.file()), 0o640);
}

#[test]
fn a_replacement_that_cannot_be_put_in_place_is_handed_back() {
    let dir = tempfile::tempdir().unwrap();
    // A folder that holds something stands where the file should go.
    let path = dir.path().join("taken");
    std::fs::create_dir(&path).unwrap();
    std::fs::write(path.join("inside"), "kept").unwrap();
    let unfinished = Replacement::new(&path).unwrap();
    std::io::Write::write_all(&mut unfinished.file(), b"new").unwrap();
    let Err((_, back)) = unfinished.try_commit() else {
        panic!("a file was put over a folder");
    };
    // Still there to be read, and gone once dropped.
    assert_eq!(back.file().metadata().unwrap().len(), 3);
    drop(back);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
    let whole = Replacement::new(&path).unwrap();
    assert!(whole.commit().is_err());
}

#[test]
fn text_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    write_text(&path, "hello").unwrap();
    assert_eq!(read_text(&path).unwrap(), "hello");
}

#[test]
fn only_the_names_a_replacement_makes_are_its_own() {
    let dir = tempfile::tempdir().unwrap();
    let unfinished = Replacement::new(&dir.path().join("logs.txt")).unwrap();
    let made = std::fs::read_dir(dir.path())
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    assert!(is_unfinished(made.file_name().to_str().unwrap()));
    drop(unfinished);
    // Files of the user's that look alike: none is a Replacement's.
    for theirs in [
        ".homeport-notes.part",
        ".homeport-backup.part",
        ".homeport-0123456789ab.txt",
        ".homeport-0123456789a.part",
        ".homeport-0123456789abc.part",
        ".homeport-0123456789-b.part",
        "homeport-0123456789ab.part",
    ] {
        assert!(!is_unfinished(theirs), "{theirs}");
    }
    assert!(is_unfinished(".homeport-0123456789ab.part"));
}

#[test]
fn a_synced_replacement_is_put_in_place_by_a_rename() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("note.txt");
    let saved = Replacement::new(&path).unwrap();
    std::io::Write::write_all(&mut saved.file(), b"hello").unwrap();
    saved.file().sync_all().unwrap();
    saved.commit_synced().unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"hello");
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}
