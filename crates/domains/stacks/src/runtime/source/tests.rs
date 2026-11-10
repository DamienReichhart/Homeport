use super::*;
use crate::store::ComposeFile;

#[test]
fn managed_definitions_go_in_on_stdin_from_an_empty_folder() {
    let compose = "services:\n  web:\n    image: nginx\n";
    let source = StackSource::Managed {
        files: vec![ComposeFile::new("compose.yaml", compose)],
        env: "A=1".into(),
    };
    let prepared = source.prepare(ComposeAction::Up).unwrap();
    assert_eq!(prepared.file_args, ["--file", "-"]);
    assert_eq!(prepared.stdin.as_deref(), Some(compose));
    assert_eq!(prepared.vars, [("A".to_owned(), "1".to_owned())]);
    assert!(matches!(prepared.workdir, WorkDir::Temp(_)));
    assert_eq!(
        std::fs::read_dir(prepared.workdir.path()).unwrap().count(),
        0
    );
}

#[test]
fn a_blocked_definition_can_still_be_taken_down() {
    let source = StackSource::Managed {
        files: vec![ComposeFile::new(
            "compose.yaml",
            "services:\n  app:\n    build: .\n",
        )],
        env: String::new(),
    };
    assert!(source.prepare(ComposeAction::Up).is_err());
    assert!(source.prepare(ComposeAction::Down).is_ok());
}

#[test]
fn several_managed_files_are_written_to_the_private_folder_in_order() {
    let source = StackSource::Managed {
        files: vec![
            ComposeFile::new("compose.yaml", "services:\n  web:\n    image: nginx\n"),
            ComposeFile::new(
                "compose.prod.yaml",
                "services:\n  web:\n    ports: [\"80:80\"]\n",
            ),
        ],
        env: "SECRET=hunter2".into(),
    };
    let prepared = source.prepare(ComposeAction::Up).unwrap();
    assert_eq!(
        prepared.file_args,
        ["--file", "compose.yaml", "--file", "compose.prod.yaml"]
    );
    assert!(prepared.stdin.is_none());
    let dir = prepared.workdir.path().to_owned();
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    assert_eq!(names, ["compose.prod.yaml", "compose.yaml"]);
    // The environment is never written next to them.
    for name in &names {
        assert!(!std::fs::read_to_string(dir.join(name))
            .unwrap()
            .contains("hunter2"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&dir).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o700);
    }
    drop(prepared);
    assert!(!dir.exists(), "removed after the run");
}

fn linked(dir: &Path, files: &[(&str, &str)]) -> StackSource {
    let config_files = files
        .iter()
        .map(|(name, text)| {
            let path = dir.join(name);
            std::fs::write(&path, text).unwrap();
            path.to_string_lossy().into_owned()
        })
        .collect();
    StackSource::Linked {
        working_dir: dir.to_owned(),
        config_files,
    }
}

#[test]
fn linked_stacks_run_in_their_folder_with_each_file() {
    let dir = tempfile::tempdir().unwrap();
    let source = linked(
        dir.path(),
        &[
            ("compose.yaml", "services:\n  app:\n    build: .\n"),
            (
                "compose.override.yaml",
                "services:\n  app:\n    env_file: .env\n",
            ),
        ],
    );
    let prepared = source.prepare(ComposeAction::Up).unwrap();
    let folder = dir.path().to_string_lossy().into_owned();
    let file = |n: &str| dir.path().join(n).to_string_lossy().into_owned();
    assert_eq!(
        prepared.file_args,
        [
            "--project-directory".to_owned(),
            folder,
            "--file".into(),
            file("compose.yaml"),
            "--file".into(),
            file("compose.override.yaml"),
        ]
    );
    assert_eq!(prepared.workdir.path(), dir.path());
    assert!(prepared.stdin.is_none());
    assert!(prepared.vars.is_empty());
    assert_eq!(prepared.definition.len(), 2);
}

#[test]
fn linked_stacks_need_their_folder() {
    let dir = tempfile::tempdir().unwrap();
    let source = linked(dir.path(), &[("compose.yaml", "services: {}\n")]);
    drop(dir);
    for action in [ComposeAction::Up, ComposeAction::Down] {
        assert!(matches!(
            source.prepare(action),
            Err(ref e) if e.is(ErrorCode::StackFolderMissing)
        ));
    }
}

#[test]
fn synced_stacks_run_in_their_folder_with_their_environment() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("compose.yaml");
    std::fs::write(&path, "services:\n  app:\n    build: .\n").unwrap();
    let source = StackSource::Synced {
        working_dir: dir.path().to_owned(),
        config_files: vec![path.to_string_lossy().into_owned()],
        env: "TOKEN=hunter2\n".into(),
    };
    let prepared = source.prepare(ComposeAction::Up).unwrap();
    assert_eq!(prepared.file_args[0], "--project-directory");
    assert_eq!(prepared.vars, [("TOKEN".to_owned(), "hunter2".to_owned())]);
    assert_eq!(prepared.workdir.path(), dir.path());
    assert!(!std::fs::read_to_string(&path).unwrap().contains("hunter2"));
}
