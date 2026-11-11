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
    /// The Compose profiles the Stack deploys.
    pub profiles: Vec<String>,
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
        profiles: stack.profiles.clone(),
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
    // The editor showed it with U+FFFD for what isn't UTF-8; writing that
    // back would change bytes the user never touched.
    if std::str::from_utf8(&current).is_err() {
        return Err(AppError::validation(msg::FILE_NOT_UTF8.with("file", path))
            .with_hint(msg::FILE_NOT_UTF8_HINT));
    }
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
/// A symbolic link stays one: the file it points to is replaced, as an
/// editor would, instead of the link becoming a copy.
fn replace(target: &Path, bytes: &[u8]) -> io::Result<()> {
    let target = &std::fs::canonicalize(target)?;
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
mod tests;
