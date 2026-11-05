//! A Linked Stack's compose files as they are on disk: reading them, writing
//! one back in place, and their fingerprints, so Homeport can tell when they
//! changed outside it (ADR 0008).

use std::{
    fmt::Write as _,
    io::{self, Write as _},
    path::Path,
};

use serde::Serialize;
use sha2::{Digest, Sha256};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use super::LinkedStack;
use crate::messages as msg;

/// One compose file of a Linked Stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LinkedFile {
    /// Its absolute path on this machine.
    pub path: String,
    pub content: String,
    /// Of `content`, to tell whether the file changed since.
    pub fingerprint: String,
}

/// A Linked Stack's compose files as they are now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LinkedFiles {
    pub working_dir: String,
    /// In the order compose merges them.
    pub files: Vec<LinkedFile>,
    /// Of every file together, as stored when they were last read or written.
    pub fingerprint: String,
    /// The files differ from what Homeport last read or wrote.
    pub changed_outside: bool,
}

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

/// Reads every compose file of `stack` from its folder.
pub fn read(stack: &LinkedStack) -> AppResult<LinkedFiles> {
    folder_present(stack)?;
    let files = stack
        .config_files
        .iter()
        .map(|path| {
            let bytes = std::fs::read(path).map_err(|e| file_error(path, &e))?;
            Ok(LinkedFile {
                path: path.clone(),
                fingerprint: content_fingerprint(&bytes),
                content: String::from_utf8_lossy(&bytes).into_owned(),
            })
        })
        .collect::<AppResult<Vec<_>>>()?;
    let fingerprint = files_fingerprint(&stack.config_files);
    Ok(LinkedFiles {
        working_dir: stack.working_dir.clone(),
        changed_outside: fingerprint != stack.fingerprint,
        files,
        fingerprint,
    })
}

/// Writes `content` to the compose file `path` of `stack`, in place and
/// atomically: a temporary file in the same folder, given the file's
/// permissions, then renamed over it. `expected` is the fingerprint of the
/// file the edit started from; when the file on disk no longer has it, the
/// write is refused. `None` overwrites whatever is there.
pub fn write(
    stack: &LinkedStack,
    path: &str,
    content: &str,
    expected: Option<&str>,
) -> AppResult<LinkedFile> {
    if !stack.config_files.iter().any(|f| f == path) {
        return Err(not_a_stack_file(path));
    }
    folder_present(stack)?;
    let target = Path::new(path);
    let current = std::fs::read(target).map_err(|e| file_error(path, &e))?;
    if let Some(expected) = expected {
        if content_fingerprint(&current) != expected {
            return Err(AppError::new(
                ErrorCode::StackFilesChanged,
                msg::FILES_CHANGED.with("file", path),
            )
            .with_hint(msg::FILES_CHANGED_HINT));
        }
    }
    replace(target, content.as_bytes()).map_err(|e| file_error(path, &e))?;
    Ok(LinkedFile {
        path: path.to_owned(),
        content: content.to_owned(),
        fingerprint: content_fingerprint(content.as_bytes()),
    })
}

/// Replaces `target` with `bytes` without a moment where it's half written.
fn replace(target: &Path, bytes: &[u8]) -> io::Result<()> {
    let dir = target
        .parent()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
    let permissions = std::fs::metadata(target)?.permissions();
    let mut temp = tempfile::NamedTempFile::new_in(dir)?;
    temp.write_all(bytes)?;
    temp.as_file().sync_all()?;
    std::fs::set_permissions(temp.path(), permissions)?;
    temp.persist(target).map_err(|e| e.error)?;
    Ok(())
}

fn folder_present(stack: &LinkedStack) -> AppResult<()> {
    if Path::new(&stack.working_dir).is_dir() {
        Ok(())
    } else {
        Err(folder_missing(&stack.working_dir))
    }
}

/// Checks the compose files picked for a Linked Stack: at least one, each a
/// file inside `working_dir`, none twice.
pub fn validate_choice(working_dir: &str, files: &[String]) -> AppResult<()> {
    if files.is_empty() {
        return Err(AppError::validation(msg::NO_FILES));
    }
    let folder = std::fs::canonicalize(working_dir).map_err(|_| folder_missing(working_dir))?;
    for (i, file) in files.iter().enumerate() {
        let path = std::fs::canonicalize(file).map_err(|e| file_error(file, &e))?;
        if !path.starts_with(&folder) || !path.is_file() {
            return Err(AppError::validation(
                msg::NOT_IN_FOLDER
                    .with("file", file)
                    .with("folder", working_dir),
            ));
        }
        if files[..i].contains(file) {
            return Err(AppError::validation(
                msg::FILE_NAME_TWICE.with("name", file),
            ));
        }
    }
    Ok(())
}

/// The error for a path that isn't one of a Linked Stack's compose files.
pub fn not_a_stack_file(path: &str) -> AppError {
    AppError::validation(msg::NOT_A_STACK_FILE.with("file", path))
}

/// The error for a Linked Stack whose folder is gone.
pub fn folder_missing(folder: &str) -> AppError {
    AppError::new(
        ErrorCode::StackFolderMissing,
        msg::FOLDER_MISSING.with("folder", folder),
    )
    .with_hint(msg::FOLDER_MISSING_HINT)
}

fn file_error(path: &str, e: &io::Error) -> AppError {
    match e.kind() {
        io::ErrorKind::NotFound => AppError::new(
            ErrorCode::StackFolderMissing,
            msg::FILE_MISSING.with("file", path),
        )
        .with_hint(msg::FILE_MISSING_HINT),
        _ => AppError::new(
            ErrorCode::StackFileUnreadable,
            msg::FILE_UNREADABLE.with("file", path),
        )
        .with_hint(msg::FILE_UNREADABLE_HINT),
    }
    .with_cause(e)
}

#[cfg(test)]
mod tests {
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
            host_id: "h".into(),
            name: "site".into(),
            working_dir: dir.to_string_lossy().into_owned(),
            fingerprint: files_fingerprint(&config_files),
            config_files,
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
        std::fs::set_permissions(&s.config_files[0], std::fs::Permissions::from_mode(0o000))
            .unwrap();
        // Root reads anything; the check only means something for other users.
        if std::fs::read(&s.config_files[0]).is_err() {
            assert!(read(&s).unwrap_err().is(ErrorCode::StackFileUnreadable));
        }
    }
}
