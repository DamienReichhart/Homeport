use super::*;

const BASE: &str = "services:\n  web:\n    image: nginx\n";

fn stack(dir: &Path, names: &[&str]) -> LinkedStack {
    let config_files: Vec<String> = names
        .iter()
        .map(|n| {
            let path = dir.join(n);
            std::fs::write(&path, BASE).unwrap();
            path.to_string_lossy().into_owned()
        })
        .collect();
    LinkedStack {
        id: "l1".into(),
        owner: crate::StackOwner::from("h"),
        registry_auth: false,
        name: "site".into(),
        working_dir: dir.to_string_lossy().into_owned(),
        fingerprint: files_fingerprint(&config_files),
        config_files,
        profiles: vec![],
        created_at: String::new(),
        updated_at: String::new(),
    }
}

#[test]
fn fingerprints_follow_the_files_content_and_order() {
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml", "compose.override.yaml"]);
    let (a, b) = (s.config_files[0].clone(), s.config_files[1].clone());

    let first = files_fingerprint(&[a.clone(), b.clone()]);
    assert_eq!(first, files_fingerprint(&[a.clone(), b.clone()]));
    assert_ne!(first, files_fingerprint(&[b.clone(), a.clone()]));

    std::fs::write(&b, "services: {}\n").unwrap();
    let edited = files_fingerprint(&[a.clone(), b.clone()]);
    assert_ne!(first, edited);

    std::fs::remove_file(&b).unwrap();
    assert_ne!(edited, files_fingerprint(&[a, b]));
    assert_eq!(
        content_fingerprint(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn files_are_read_in_order_and_outside_changes_are_noticed() {
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml", "compose.prod.yaml"]);
    let read_back = read(&s).unwrap();
    assert_eq!(read_back.files.len(), 2);
    assert_eq!(read_back.files[1].path, s.config_files[1]);
    assert_eq!(read_back.files[0].content, BASE);
    assert!(!read_back.changed_outside);

    std::fs::write(&s.config_files[1], "services: {}\n").unwrap();
    assert!(read(&s).unwrap().changed_outside);
}

#[test]
fn writes_land_in_place_and_keep_the_permissions() {
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml"]);
    let path = &s.config_files[0];
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o640)).unwrap();
    }
    let base = content_fingerprint(BASE.as_bytes());
    let edited = "services:\n  web:\n    image: nginx:1.27\n";
    let written = write(&s, path, edited, Some(&base)).unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), edited);
    assert_eq!(written.fingerprint, content_fingerprint(edited.as_bytes()));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o640);
    }
    // Only the file, no temporary left behind.
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn writing_through_a_symbolic_link_keeps_the_link_and_edits_its_file() {
    let dir = tempfile::tempdir().unwrap();
    let dotfiles = tempfile::tempdir().unwrap();
    let real = dotfiles.path().join("compose.yaml");
    std::fs::write(&real, BASE).unwrap();
    let link = dir.path().join("compose.yaml");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    let mut s = stack(dir.path(), &[]);
    s.config_files = vec![link.to_string_lossy().into_owned()];

    let edited = "services:\n  web:\n    image: nginx:1.27\n";
    let base = content_fingerprint(BASE.as_bytes());
    write(&s, &s.config_files[0], edited, Some(&base)).unwrap();
    assert!(
        std::fs::symlink_metadata(&link)
            .unwrap()
            .file_type()
            .is_symlink(),
        "the user's link is still a link"
    );
    assert_eq!(std::fs::read_to_string(&real).unwrap(), edited);
    assert_eq!(std::fs::read_dir(dotfiles.path()).unwrap().count(), 1);
}

#[test]
fn a_file_that_isnt_utf8_is_never_written_back_with_replacement_characters() {
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml"]);
    let path = &s.config_files[0];
    // A Latin-1 "é" in a comment: the editor shows it as U+FFFD.
    let latin1 = b"# caf\xe9\nservices:\n  web:\n    image: nginx\n".to_vec();
    std::fs::write(path, &latin1).unwrap();
    let opened = read(&s).unwrap();
    assert!(opened.files[0].content.contains('\u{fffd}'));

    let edited = opened.files[0].content.replace("nginx", "nginx:1.27");
    for expected in [Some(opened.files[0].fingerprint.as_str()), None] {
        let refused = write(&s, path, &edited, expected).unwrap_err();
        assert!(refused.is(ErrorCode::Validation), "{refused:?}");
        assert_eq!(refused.key, "errors.stacks.fileNotUtf8");
        assert!(refused.hint.is_some());
        assert_eq!(
            std::fs::read(path).unwrap(),
            latin1,
            "the bytes stay as they were"
        );
    }
}

#[test]
fn a_file_changed_elsewhere_is_only_overwritten_when_asked() {
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml"]);
    let path = &s.config_files[0];
    let opened = content_fingerprint(BASE.as_bytes());
    std::fs::write(path, "services: {} # edited elsewhere\n").unwrap();

    let refused = write(&s, path, "mine", Some(&opened)).unwrap_err();
    assert!(refused.is(ErrorCode::StackFilesChanged));
    assert!(refused.hint.is_some());
    assert_eq!(
        std::fs::read_to_string(path).unwrap(),
        "services: {} # edited elsewhere\n"
    );

    write(&s, path, "mine", None).unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), "mine");
}

#[test]
fn missing_folders_and_foreign_paths_are_explained() {
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml"]);
    assert!(write(&s, "/etc/passwd", "x", None)
        .unwrap_err()
        .is(ErrorCode::Validation));

    std::fs::remove_file(&s.config_files[0]).unwrap();
    assert!(read(&s).unwrap_err().is(ErrorCode::StackFolderMissing));

    drop(dir);
    let e = read(&s).unwrap_err();
    assert!(e.is(ErrorCode::StackFolderMissing));
    assert!(e.message.contains(&s.working_dir));
}

#[test]
fn chosen_files_must_be_in_the_folder() {
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml", "compose.prod.yaml"]);
    let folder = &s.working_dir;
    assert!(validate_choice(folder, &s.config_files).is_ok());
    let reversed: Vec<_> = s.config_files.iter().rev().cloned().collect();
    assert!(validate_choice(folder, &reversed).is_ok());
    assert!(validate_choice(folder, &[]).is_err());
    let twice = vec![s.config_files[0].clone(), s.config_files[0].clone()];
    assert!(validate_choice(folder, &twice).is_err());

    let elsewhere = tempfile::tempdir().unwrap();
    let outside = elsewhere.path().join("compose.yaml");
    std::fs::write(&outside, BASE).unwrap();
    let e = validate_choice(folder, &[outside.to_string_lossy().into_owned()]).unwrap_err();
    assert!(e.is(ErrorCode::Validation), "{e:?}");
    let sneaky = format!(
        "{folder}/../{}",
        outside.file_name().unwrap().to_string_lossy()
    );
    assert!(validate_choice(folder, &[sneaky]).is_err());
}

#[cfg(unix)]
#[test]
fn unreadable_files_say_so() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let s = stack(dir.path(), &["compose.yaml"]);
    std::fs::set_permissions(&s.config_files[0], std::fs::Permissions::from_mode(0o000)).unwrap();
    // Root reads anything; the check only means something for other users.
    if std::fs::read(&s.config_files[0]).is_err() {
        assert!(read(&s).unwrap_err().is(ErrorCode::StackFileUnreadable));
    }
}
