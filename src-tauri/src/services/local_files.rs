//! Files on this computer that the user picked in a dialog: keys,
//! certificates and compose files to read, logs to save.

use std::path::Path;

use crate::error::{local_file, AppError, AppResult, ErrorCode};

/// Largest file [`read_text`] accepts; certificates and keys are far smaller.
const MAX_TEXT_FILE: u64 = 1024 * 1024;

pub fn read_text(path: &Path) -> AppResult<String> {
    let size = std::fs::metadata(path)
        .map_err(|e| local_file(path, e))?
        .len();
    if size > MAX_TEXT_FILE {
        return Err(AppError::new(
            ErrorCode::FileTooLarge,
            format!(
                "{} is too large to be a key, certificate or compose file.",
                path.display()
            ),
        )
        .with_hint("Pick a file under 1 MB."));
    }
    std::fs::read_to_string(path).map_err(|e| local_file(path, e))
}

pub fn write_text(path: &Path, contents: &str) -> AppResult<()> {
    std::fs::write(path, contents).map_err(|e| local_file(path, e))
}

/// Creates `dir` if needed so it can be shown in the file manager.
pub fn ensure_folder(dir: &Path) -> AppResult<()> {
    std::fs::create_dir_all(dir).map_err(|e| local_file(dir, e))
}

/// The file manager couldn't show `dir`.
pub fn folder_not_opened(dir: &Path, cause: &(dyn std::error::Error + 'static)) -> AppError {
    AppError::new(ErrorCode::Io, format!("{} can't be opened.", dir.display()))
        .with_hint("Open it in your file manager instead.")
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
    fn text_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("logs.txt");
        write_text(&path, "hello").unwrap();
        assert_eq!(read_text(&path).unwrap(), "hello");
    }
}
