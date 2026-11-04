//! Failures of files on this computer.

use super::{AppError, ErrorCode};
use crate::messages as msg;

/// A file on this computer that the user picked (keys, compose files, downloads).
pub fn local_file(path: &std::path::Path, e: std::io::Error) -> AppError {
    let error = match e.kind() {
        std::io::ErrorKind::NotFound => {
            AppError::new(ErrorCode::Io, msg::FILE_MISSING.with("path", path))
        }
        std::io::ErrorKind::PermissionDenied => AppError::new(
            ErrorCode::PermissionDenied,
            msg::FILE_FORBIDDEN.with("path", path),
        )
        .with_hint(msg::FILE_FORBIDDEN_HINT),
        std::io::ErrorKind::StorageFull => {
            AppError::new(ErrorCode::DiskFull, msg::FILE_DISK_FULL.with("path", path))
                .with_hint(msg::FILE_DISK_FULL_HINT)
        }
        std::io::ErrorKind::IsADirectory => {
            AppError::validation(msg::FILE_IS_FOLDER.with("path", path))
        }
        std::io::ErrorKind::InvalidData => {
            AppError::validation(msg::FILE_NOT_TEXT.with("path", path))
        }
        _ => AppError::new(ErrorCode::Io, msg::FILE_INACCESSIBLE.with("path", path)),
    };
    error.with_cause(&e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_files_name_the_path_and_the_problem() {
        let path = std::path::Path::new("/home/me/key.pem");
        let e = local_file(
            path,
            std::io::Error::from(std::io::ErrorKind::PermissionDenied),
        );
        assert_eq!(e.code, ErrorCode::PermissionDenied);
        assert_eq!(
            e.message,
            "Homeport isn't allowed to access /home/me/key.pem."
        );
        let e = local_file(path, std::io::Error::from(std::io::ErrorKind::NotFound));
        assert_eq!(e.message, "/home/me/key.pem doesn't exist.");
    }
}
