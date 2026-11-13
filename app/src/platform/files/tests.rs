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

/// The unfinished files of `dir`, by path.
fn parts(dir: &Path) -> Vec<PathBuf> {
    names(dir)
        .into_iter()
        .filter(|name| name.starts_with(".homeport-") && name.ends_with(".part"))
        .map(|name| dir.join(name))
        .collect()
}

#[test]
fn files_left_unfinished_never_stand_in_the_way_and_old_ones_are_taken_away() {
    let dir = tempfile::tempdir().unwrap();
    // An instance that was killed left a hundred of them, just now.
    for n in 0..100 {
        fs::write(dir.path().join(format!(".homeport-{n:06}.part")), "theirs").unwrap();
    }
    // And one a week ago, beside files of the user's that look alike.
    let week = std::time::SystemTime::now() - Duration::from_secs(7 * 24 * 60 * 60);
    for old in [".homeport-killed.part", "notes.part", ".homeport-notes.txt"] {
        let path = dir.path().join(old);
        fs::write(&path, "old").unwrap();
        let file = File::options().write(true).open(&path).unwrap();
        file.set_modified(week).unwrap();
    }
    let path = dir.path().join("logs.txt");
    replace(&path, |out| out.write_all(b"new")).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    let left = names(dir.path());
    assert_eq!(
        parts(dir.path()).len(),
        100,
        "what may still be written is left"
    );
    assert!(!left.contains(&".homeport-killed.part".to_owned()));
    assert!(left.contains(&"notes.part".to_owned()));
    assert!(left.contains(&".homeport-notes.txt".to_owned()));
    for part in parts(dir.path()) {
        assert_eq!(fs::read_to_string(part).unwrap(), "theirs");
    }
}

#[cfg(unix)]
#[test]
fn the_unfinished_file_is_never_more_readable_than_the_file_it_becomes() {
    use std::os::unix::fs::PermissionsExt;
    let mode = |path: &Path| fs::metadata(path).unwrap().permissions().mode() & 0o777;
    let dir = tempfile::tempdir().unwrap();
    // The one unfinished file of the folder, seen from inside the write.
    let unfinished = |dir: &Path| {
        let parts = parts(dir);
        assert_eq!(parts.len(), 1, "{parts:?}");
        mode(&parts[0])
    };

    // A private file is replaced by a file that was private all along.
    let key = dir.path().join("key.pem");
    fs::write(&key, "old").unwrap();
    fs::set_permissions(&key, fs::Permissions::from_mode(0o600)).unwrap();
    replace(&key, |out| {
        out.write_all(b"new secret")?;
        assert_eq!(unfinished(dir.path()), 0o600);
        Ok(())
    })
    .unwrap();
    assert_eq!(mode(&key), 0o600);

    // A new file is private while it is written, then what any new file is.
    let plain = dir.path().join("plain.txt");
    fs::write(&plain, "").unwrap();
    let fresh = dir.path().join("fresh.txt");
    replace(&fresh, |out| {
        out.write_all(b"logs")?;
        assert_eq!(unfinished(dir.path()), 0o600);
        Ok(())
    })
    .unwrap();
    assert_eq!(mode(&fresh), mode(&plain));

    // A file others may read keeps being one, and a read-only one is
    // replaced and stays read-only.
    fs::set_permissions(&plain, fs::Permissions::from_mode(0o444)).unwrap();
    replace(&plain, |out| {
        out.write_all(b"again")?;
        assert_eq!(unfinished(dir.path()), 0o444);
        Ok(())
    })
    .unwrap();
    assert_eq!(mode(&plain), 0o444);
    assert_eq!(fs::read_to_string(&plain).unwrap(), "again");
}

#[cfg(unix)]
#[test]
fn a_link_is_written_through_whatever_it_leads_to() {
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    let real = dir.path().join("real.txt");
    fs::write(&real, "old").unwrap();

    // A link to a link to the file, the second one relative.
    let near = dir.path().join("near");
    symlink("real.txt", &near).unwrap();
    let far = dir.path().join("far");
    symlink(&near, &far).unwrap();
    replace(&far, |out| out.write_all(b"through")).unwrap();
    assert!(
        far.is_symlink() && near.is_symlink(),
        "the links are still links"
    );
    assert_eq!(fs::read_to_string(&real).unwrap(), "through");

    // A link to a file that isn't there yet makes it, as a plain write does.
    let dangling = dir.path().join("dangling");
    symlink(dir.path().join("made.txt"), &dangling).unwrap();
    replace(&dangling, |out| out.write_all(b"made")).unwrap();
    assert!(dangling.is_symlink());
    assert_eq!(
        fs::read_to_string(dir.path().join("made.txt")).unwrap(),
        "made"
    );

    // Links that lead back to themselves are refused, and stay.
    let (a, b) = (dir.path().join("a"), dir.path().join("b"));
    symlink(&b, &a).unwrap();
    symlink(&a, &b).unwrap();
    replace(&a, |out| out.write_all(b"never")).unwrap_err();
    assert!(a.is_symlink() && b.is_symlink());
    assert!(parts(dir.path()).is_empty());
}

#[cfg(unix)]
#[test]
fn what_is_no_file_is_never_replaced_by_one() {
    let dir = tempfile::tempdir().unwrap();
    let pipe = dir.path().join("pipe");
    let made = std::process::Command::new("mkfifo").arg(&pipe).status();
    assert!(made.is_ok_and(|status| status.success()), "mkfifo");
    let refused = replace(&pipe, |out| out.write_all(b"never")).unwrap_err();
    assert_eq!(refused.key, "errors.file.notRegular");
    assert!(fs::symlink_metadata(&pipe).is_ok_and(|found| !found.is_file()));
    let device = replace(Path::new("/dev/null"), |out| out.write_all(b"never")).unwrap_err();
    assert_eq!(device.message, "/dev/null isn't a regular file.");
    assert!(parts(dir.path()).is_empty());
}

#[test]
fn a_file_that_cannot_be_renamed_over_is_written_into() {
    type Refused = Result<(), (io::Error, Replacement)>;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mounted.txt");
    fs::write(&path, "a much longer text than the new one").unwrap();
    // A file bound alone into a container answers a rename with "busy".
    let busy = |unfinished: Replacement| -> Refused {
        Err((io::ErrorKind::ResourceBusy.into(), unfinished))
    };
    replace_by(&path, &Stop::default(), |out| out.write_all(b"new"), busy).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    assert_eq!(names(dir.path()), ["mounted.txt"]);

    // Any other refusal is a failure that leaves the file alone, and a
    // file that isn't there yet is not made by other means.
    let denied = |unfinished: Replacement| -> Refused {
        Err((io::ErrorKind::PermissionDenied.into(), unfinished))
    };
    let failed = replace_by(
        &path,
        &Stop::default(),
        |out| out.write_all(b"never"),
        denied,
    )
    .unwrap_err();
    assert_eq!(failed.code, ErrorCode::PermissionDenied);
    let fresh = dir.path().join("fresh.txt");
    replace_by(
        &fresh,
        &Stop::default(),
        |out| out.write_all(b"never"),
        busy,
    )
    .unwrap_err();
    assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    assert_eq!(names(dir.path()), ["mounted.txt"]);
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
    assert_eq!(at(0, 0), 0);
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
    assert_eq!(saving.done.await.unwrap(), Saved::Whole);
    let steps = steps.await.unwrap();
    // The window is told once a percent at most, however many lines.
    assert!(!steps.is_empty() && steps.len() <= 101, "{}", steps.len());
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

/// A writer that writes a chunk at a time for as long as it is let to,
/// and tells when it started and how it ended.
fn endless(
    started: std::sync::mpsc::Sender<std::thread::ThreadId>,
    ended: std::sync::mpsc::Sender<io::Result<()>>,
) -> impl FnOnce(&mut dyn Write, &Meter) -> io::Result<()> + Send + 'static {
    move |out, _| {
        let _ = started.send(std::thread::current().id());
        let written: io::Result<()> = (|| loop {
            out.write_all(&[b'x'; 4096])?;
            std::thread::sleep(Duration::from_millis(1));
        })();
        let _ = ended.send(match &written {
            Ok(()) => Ok(()),
            Err(e) => Err(e.kind().into()),
        });
        written
    }
}

#[tokio::test]
async fn a_save_nobody_waits_for_any_more_stops_and_leaves_the_file_that_was_there() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    fs::write(&path, "kept").unwrap();
    let (started, starts) = std::sync::mpsc::channel();
    let (ended, ends) = std::sync::mpsc::channel();
    let saving = save(path.clone(), endless(started, ended));
    let waiting = tokio::spawn(saving.done);
    // The write is under way, on a thread that is not the one that waits.
    let writer = tokio::task::spawn_blocking(move || starts.recv_timeout(Duration::from_secs(10)))
        .await
        .unwrap()
        .expect("the write started");
    assert_ne!(
        writer,
        std::thread::current().id(),
        "the write is off the executor"
    );
    assert_eq!(parts(dir.path()).len(), 1);
    // The app closes: whoever waited is gone.
    waiting.abort();
    let ended = tokio::task::spawn_blocking(move || ends.recv_timeout(Duration::from_secs(10)))
        .await
        .unwrap()
        .expect("the write stopped by itself");
    assert!(ended.is_err(), "its next chunk was refused");
    // The thread ends just after: the unfinished file goes with it.
    let gone = std::time::Instant::now();
    while !parts(dir.path()).is_empty() {
        assert!(
            gone.elapsed() < Duration::from_secs(10),
            "the unfinished file stayed"
        );
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert_eq!(fs::read_to_string(&path).unwrap(), "kept");
    assert_eq!(names(dir.path()), ["logs.txt"]);
}

#[tokio::test]
async fn a_save_that_is_stopped_says_so_and_is_no_failure() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    let (started, starts) = std::sync::mpsc::channel();
    let (ended, _ends) = std::sync::mpsc::channel();
    let saving = save(path.clone(), endless(started, ended));
    let waiting = tokio::spawn(saving.done);
    tokio::task::spawn_blocking(move || starts.recv_timeout(Duration::from_secs(10)))
        .await
        .unwrap()
        .expect("the write started");
    saving.stop.now();
    let saved = tokio::time::timeout(Duration::from_secs(10), waiting)
        .await
        .expect("the write stopped")
        .unwrap();
    assert_eq!(saved, Ok(Saved::Stopped));
    assert!(
        names(dir.path()).is_empty(),
        "no file was made, and none is left"
    );
    // A writer that hides the refusal still writes nothing.
    let stop = Stop::default();
    stop.now();
    let hidden = replace_by(
        &path,
        &stop,
        |out| {
            let _ = out.write_all(b"never");
            Ok(())
        },
        Replacement::try_commit,
    );
    assert_eq!(hidden, Ok(Saved::Stopped));
    assert!(names(dir.path()).is_empty());
}

/// What replacing a file does not keep, said in
/// `docs/agents/iced-migration.md`: its other names.
#[test]
fn another_name_of_a_replaced_file_keeps_what_it_held() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("logs.txt");
    let other = dir.path().join("same-file.txt");
    fs::write(&path, "old").unwrap();
    fs::hard_link(&path, &other).unwrap();
    replace(&path, |out| out.write_all(b"new")).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "new");
    assert_eq!(fs::read_to_string(&other).unwrap(), "old");
}
