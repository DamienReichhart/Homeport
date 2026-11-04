//! Failures of files on this computer.

use super::{AppError, ErrorCode};

/// A file on this computer that the user picked (keys, compose files, downloads).
pub fn local_file(path: &std::path::Path, e: std::io::Error) -> AppError {
    let path = path.display();
    let error = match e.kind() {
        std::io::ErrorKind::NotFound => {
            AppError::new(ErrorCode::Io, format!("{path} doesn't exist."))
        }
        std::io::ErrorKind::PermissionDenied => AppError::new(
            ErrorCode::PermissionDenied,
            format!("Homeport isn't allowed to access {path}."),
        )
        .with_hint("Check the file's permissions, or pick another location."),
        std::io::ErrorKind::StorageFull => AppError::new(
            ErrorCode::DiskFull,
            format!("There's no space left to write {path}."),
        )
        .with_hint("Free some disk space, or pick another drive."),
        std::io::ErrorKind::IsADirectory => {
            AppError::validation(format!("{path} is a folder, not a file."))
        }
        std::io::ErrorKind::InvalidData => {
            AppError::validation(format!("{path} isn't a text file."))
        }
        _ => AppError::new(ErrorCode::Io, format!("Homeport couldn't access {path}.")),
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
