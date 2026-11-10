//! Files on this computer that the user picked in a dialog: keys,
//! certificates and compose files to read, logs to save.

use std::path::{Path, PathBuf};

use crate::error::{local_file, AppError, AppResult, ErrorCode};
use crate::messages as msg;

/// Largest file [`read_text`] accepts; certificates and keys are far smaller.
const MAX_TEXT_FILE: u64 = 1024 * 1024;

pub fn read_text(path: &Path) -> AppResult<String> {
    let size = std::fs::metadata(path)
        .map_err(|e| local_file(path, e))?
        .len();
    if size > MAX_TEXT_FILE {
        return Err(AppError::new(
            ErrorCode::FileTooLarge,
            msg::FILE_TOO_LARGE.with("path", path),
        )
        .with_hint(msg::FILE_TOO_LARGE_HINT));
    }
    std::fs::read_to_string(path).map_err(|e| local_file(path, e))
}

pub fn write_text(path: &Path, contents: &str) -> AppResult<()> {
    std::fs::write(path, contents).map_err(|e| local_file(path, e))
}

/// A file being written to take the place of `path` once complete: until
/// [`Replacement::commit`], whatever is at `path` (last week's backup,
/// picked again in the save dialog) stays as it was, and dropping it, on a
/// failure or a cancel, leaves nothing behind.
pub struct Replacement {
    temp: tempfile::NamedTempFile,
    path: PathBuf,
}

impl Replacement {
    /// Starts writing beside `path`, in the same folder, so the commit is a rename.
    pub fn new(path: &Path) -> AppResult<Self> {
        let dir = match path.parent() {
            Some(dir) if !dir.as_os_str().is_empty() => dir,
            _ => Path::new("."),
        };
        let temp = tempfile::Builder::new()
            .prefix(".homeport-")
            .suffix(".part")
            .tempfile_in(dir)
            .map_err(|e| local_file(path, e))?;
        Ok(Self {
            temp,
            path: path.to_owned(),
        })
    }

    /// The file to write to.
    pub fn file(&self) -> &std::fs::File {
        self.temp.as_file()
    }

    /// Flushes the file to disk and puts it at `path`.
    pub fn commit(self) -> AppResult<()> {
        let path = self.path;
        self.temp
            .as_file()
            .sync_all()
            .map_err(|e| local_file(&path, e))?;
        self.temp
            .persist(&path)
            .map_err(|e| local_file(&path, e.error))?;
        Ok(())
    }
}

/// Writes `contents` to `path` so that a failure part way leaves the file
/// that was there untouched (see [`Replacement`]).
pub fn write_replacing(path: &Path, contents: &[u8]) -> AppResult<()> {
    use std::io::Write as _;
    let replacement = Replacement::new(path)?;
    replacement
        .file()
        .write_all(contents)
        .map_err(|e| local_file(path, e))?;
    replacement.commit()
}

/// Creates `dir` if needed so it can be shown in the file manager.
pub fn ensure_folder(dir: &Path) -> AppResult<()> {
    std::fs::create_dir_all(dir).map_err(|e| local_file(dir, e))
}

/// The file manager couldn't show `dir`.
pub fn folder_not_opened(dir: &Path, cause: &(dyn std::error::Error + 'static)) -> AppError {
    AppError::new(ErrorCode::Io, msg::FOLDER_NOT_OPENED.with("path", dir))
        .with_hint(msg::FOLDER_NOT_OPENED_HINT)
        .with_cause(cause)
}

#[cfg(test)]
mod tests {
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
    fn text_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("logs.txt");
        write_text(&path, "hello").unwrap();
        assert_eq!(read_text(&path).unwrap(), "hello");
    }
}
