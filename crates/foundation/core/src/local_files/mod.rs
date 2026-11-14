//! Files on this computer that the user picked in a dialog: keys,
//! certificates and compose files to read, logs to save.

use std::{
    io::{self, Read as _},
    path::{Path, PathBuf},
};

use crate::error::{local_file, AppError, AppResult, ErrorCode};
use crate::messages as msg;

/// Largest file [`read_text`] accepts; certificates and keys are far smaller.
const MAX_TEXT_FILE: u64 = 1024 * 1024;

/// Reads a file the user picked as text. Only a regular file is read,
/// and never more than [`MAX_TEXT_FILE`] of it, whatever its size claims:
/// a device, a pipe or a file of `/proc` can't fill the memory or keep
/// the caller waiting.
pub fn read_text(path: &Path) -> AppResult<String> {
    let metadata = std::fs::metadata(path).map_err(|e| local_file(path, e))?;
    if metadata.is_dir() {
        return Err(local_file(path, io::ErrorKind::IsADirectory.into()));
    }
    if !metadata.is_file() {
        return Err(AppError::validation(
            msg::FILE_NOT_REGULAR.with("path", path),
        ));
    }
    if metadata.len() > MAX_TEXT_FILE {
        return Err(too_large(path));
    }
    let file = std::fs::File::open(path).map_err(|e| local_file(path, e))?;
    read_bounded(file, path)
}

fn too_large(path: &Path) -> AppError {
    AppError::new(
        ErrorCode::FileTooLarge,
        msg::FILE_TOO_LARGE.with("path", path),
    )
    .with_hint(msg::FILE_TOO_LARGE_HINT)
}

/// The text `file` holds, read up to the limit and no further: a size
/// asked beforehand says nothing of a file that grows, or that has none.
fn read_bounded(file: impl io::Read, path: &Path) -> AppResult<String> {
    let mut bytes = Vec::new();
    file.take(MAX_TEXT_FILE + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| local_file(path, e))?;
    if bytes.len() as u64 > MAX_TEXT_FILE {
        return Err(too_large(path));
    }
    String::from_utf8(bytes).map_err(|_| {
        // As `read_to_string` says it.
        let not_text = io::Error::new(
            io::ErrorKind::InvalidData,
            "stream did not contain valid UTF-8",
        );
        local_file(path, not_text)
    })
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
        self.try_commit().map_err(|(e, unfinished)| {
            let path = unfinished.path.clone();
            local_file(&path, e)
        })
    }

    /// [`Replacement::commit`] with the OS's own error and the file handed
    /// back, for a caller that has another way to put it in place.
    pub fn try_commit(self) -> Result<(), (io::Error, Self)> {
        if let Err(e) = self.temp.as_file().sync_all() {
            return Err((e, self));
        }
        let Self { temp, path } = self;
        temp.persist(&path)
            .map(|_| ())
            .map_err(|e| (e.error, Self { temp: e.file, path }))
    }

    /// Gives the file the permissions of the one it replaces.
    pub fn keep_mode_of(&self, replaced: &std::fs::Metadata) -> io::Result<()> {
        self.temp.as_file().set_permissions(replaced.permissions())
    }

    /// Gives the file the permissions any new file gets in this folder
    /// (the user's umask), where it is private while it is written. For a
    /// file that is not a secret: logs, an export of text.
    pub fn take_default_mode(&self) -> io::Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let dir = match self.path.parent() {
                Some(dir) if !dir.as_os_str().is_empty() => dir,
                _ => Path::new("."),
            };
            // A file asked for with every permission gets what the umask
            // leaves: that is read off one made for it, and gone at once.
            let probe = tempfile::Builder::new()
                .prefix(".homeport-")
                .suffix(".part")
                .permissions(std::fs::Permissions::from_mode(0o666))
                .tempfile_in(dir)?;
            let mode = probe.as_file().metadata()?.permissions();
            self.temp.as_file().set_permissions(mode)?;
        }
        Ok(())
    }
}

/// How old an unfinished [`Replacement`] must be for [`sweep_unfinished`]
/// to take it away: no write that still runs is that slow.
pub const UNFINISHED_AFTER: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);

/// Takes away the files a [`Replacement`] left unfinished in `folder` more
/// than [`UNFINISHED_AFTER`] ago: a write the app was killed in, or that a
/// panic aborted, whose file could not be taken away then. Only names the
/// Replacement makes (`.homeport-….part`) and regular files are touched;
/// whatever fails is left.
pub fn sweep_unfinished(folder: &Path) {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let ours = name
            .to_str()
            .is_some_and(|name| name.starts_with(".homeport-") && name.ends_with(".part"));
        let stale = entry.metadata().is_ok_and(|found| {
            found.is_file()
                && found
                    .modified()
                    .ok()
                    .and_then(|at| at.elapsed().ok())
                    .is_some_and(|age| age > UNFINISHED_AFTER)
        });
        if ours && stale {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

/// The folder a file at `path` is written in.
pub fn folder_of(path: &Path) -> &Path {
    match path.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => Path::new("."),
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
mod tests;
