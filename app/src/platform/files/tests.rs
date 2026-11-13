use iced::futures::StreamExt;

use super::*;

/// The files of `dir`, by name.
fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[tokio::test]
async fn text_is_written_and_read_back() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    write_text(path.clone(), "one\ntwo\n".to_owned())
        .await
        .unwrap();
    assert_eq!(read_text(path.clone()).await.unwrap(), "one\ntwo\n");
    // Picked again: the new text takes the place of the old.
    write_text(path.clone(), "three\n".to_owned())
        .await
        .unwrap();
    assert_eq!(read_text(path).await.unwrap(), "three\n");
    assert_eq!(names(dir.path()), ["logs.txt"]);
}

#[test]
fn a_write_that_fails_part_way_leaves_the_file_as_it_was() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    fs::write(&path, "last week's").unwrap();
    let failed = replace(&path, |out| {
        // More than the buffer holds, so some of it reached the disk.
        out.write_all(&vec![b'x'; 1 << 20])?;
        Err(io::ErrorKind::StorageFull.into())
    })
    .unwrap_err();
    assert_eq!(failed.code, ErrorCode::DiskFull);
    assert!(failed.message.contains("logs.txt"), "{}", failed.message);
    assert_eq!(fs::read_to_string(&path).unwrap(), "last week's");
    assert_eq!(names(dir.path()), ["logs.txt"], "nothing is left beside it");
}

#[test]
fn a_write_that_fails_where_there_was_no_file_leaves_none() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("new.txt");
    let failed = replace(&path, |_| Err(io::ErrorKind::PermissionDenied.into())).unwrap_err();
    assert_eq!(failed.code, ErrorCode::PermissionDenied);
    assert!(names(dir.path()).is_empty());
}

#[test]
fn a_file_that_cannot_be_put_in_place_leaves_what_was_there() {
    let dir = tempfile::tempdir().unwrap();
    // A folder stands where the file should go.
    let path = dir.path().join("taken");
    fs::create_dir(&path).unwrap();
    fs::write(path.join("inside.txt"), "kept").unwrap();
    replace(&path, |out| out.write_all(b"new")).unwrap_err();
    assert_eq!(fs::read_to_string(path.join("inside.txt")).unwrap(), "kept");
    assert_eq!(names(dir.path()), ["taken"]);

    // A folder that isn't there: nothing is made on the way.
    let missing = dir.path().join("no/such/folder/logs.txt");
    let failed = replace(&missing, |out| out.write_all(b"new")).unwrap_err();
    assert_eq!(failed.code, ErrorCode::Io);
    assert!(failed.message.contains("logs.txt"));
    assert_eq!(names(dir.path()), ["taken"]);
}

#[test]
fn unfinished_files_of_two_writers_never_collide() {
    /// More names than the tests beside this one take meanwhile, and
    /// fewer than a writer tries.
    const HELD: u64 = 48;
    let dir = tempfile::tempdir().unwrap();
    // Files left by an instance that was killed hold the next names.
    let next = PARTS.load(Ordering::Relaxed);
    let pid = std::process::id();
    let held = |n: u64| dir.path().join(format!(".homeport-{pid}-{n}.part"));
    for n in next..next + HELD {
        fs::write(held(n), "theirs").unwrap();
    }
    let path = dir.path().join("logs.txt");
    let first = Part::create(&path).unwrap();
    let second = Part::create(&path).unwrap();
    assert_ne!(first.temp, second.temp);
    for part in [&first, &second] {
        assert!((next..next + HELD).all(|n| held(n) != part.temp));
    }
    drop((first, second));
    // None was taken over, written to or removed.
    let left = names(dir.path());
    assert_eq!(left.len(), 48, "{left:?}");
    for n in next..next + HELD {
        assert_eq!(fs::read_to_string(held(n)).unwrap(), "theirs");
    }
}

#[cfg(unix)]
#[test]
fn a_replaced_file_keeps_its_permissions_and_a_link_its_target() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("key.pem");
    fs::write(&path, "old").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    replace(&path, |out| out.write_all(b"new")).unwrap();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );

    // A new file gets what any new file gets, not a private mode.
    let fresh = dir.path().join("fresh.txt");
    let plain = dir.path().join("plain.txt");
    replace(&fresh, |out| out.write_all(b"new")).unwrap();
    fs::write(&plain, "new").unwrap();
    assert_eq!(
        fs::metadata(&fresh).unwrap().permissions().mode(),
        fs::metadata(&plain).unwrap().permissions().mode()
    );

    let link = dir.path().join("link.pem");
    std::os::unix::fs::symlink(&path, &link).unwrap();
    replace(&link, |out| out.write_all(b"through")).unwrap();
    assert!(link.is_symlink(), "the link is still a link");
    assert_eq!(fs::read_to_string(&path).unwrap(), "through");
}

#[test]
fn files_that_are_not_text_or_too_large_are_refused_by_name() {
    let dir = tempfile::tempdir().unwrap();
    let missing = local_files::read_text(&dir.path().join("gone.pem")).unwrap_err();
    assert_eq!(
        missing.message,
        format!("{} doesn't exist.", dir.path().join("gone.pem").display())
    );
    let binary = dir.path().join("image.png");
    fs::write(&binary, [0xff, 0xfe, 0x00, 0xc3]).unwrap();
    let refused = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(read_text(binary))
        .unwrap_err();
    assert_eq!(refused.code, ErrorCode::Validation);
    assert!(refused.message.ends_with("image.png isn't a text file."));
}

#[test]
fn progress_is_a_percent_that_never_overshoots() {
    let at = |done, total| Progress { done, total }.percent();
    assert_eq!(at(0, 0), 100);
    assert_eq!(at(0, 10), 0);
    assert_eq!(at(5, 10), 50);
    assert_eq!(at(10, 10), 100);
    assert_eq!(at(11, 10), 100);
    assert_eq!(at(1, 1_000_000), 0);
    assert_eq!(at(u64::MAX, u64::MAX), 100);
}

#[tokio::test]
async fn a_large_save_tells_how_far_it_is_and_lands_whole() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    const LINES: u64 = 200_000;
    let saving = save(path.clone(), |out, meter| {
        for n in 0..LINES {
            writeln!(out, "line {n}")?;
            meter.at(n + 1, LINES);
        }
        Ok(())
    });
    let steps = tokio::spawn(saving.progress.collect::<Vec<_>>());
    saving.done.await.unwrap();
    let steps = steps.await.unwrap();
    // The window is told a hundred times at most, however many lines.
    assert!(!steps.is_empty() && steps.len() <= 100, "{}", steps.len());
    assert!(steps.windows(2).all(|pair| pair[0].done < pair[1].done));
    assert_eq!(steps.last().unwrap().percent(), 100);
    let written = fs::read_to_string(&path).unwrap();
    assert_eq!(written.lines().count(), 200_000);
    assert!(written.ends_with("line 199999\n"));
    assert_eq!(names(dir.path()), ["logs.txt"]);
}

#[tokio::test]
async fn a_save_that_fails_says_why_and_its_progress_ends() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    fs::write(&path, "kept").unwrap();
    let saving = save(path.clone(), |out, meter| {
        out.write_all(b"half")?;
        meter.at(1, 2);
        Err(io::ErrorKind::PermissionDenied.into())
    });
    let failed = saving.done.await.unwrap_err();
    assert_eq!(failed.code, ErrorCode::PermissionDenied);
    assert!(failed.hint.is_some());
    // The stream ends: a page waiting on it is not left waiting.
    let steps: Vec<Progress> = saving.progress.collect().await;
    assert!(steps.len() <= 1);
    assert_eq!(fs::read_to_string(&path).unwrap(), "kept");
}
