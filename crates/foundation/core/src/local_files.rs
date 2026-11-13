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
    fn a_file_that_is_longer_than_it_says_is_still_refused_at_the_limit() {
        let path = Path::new("/proc/kallsyms");
        // No size was asked: what is read is what counts.
        let endless = io::repeat(b'a');
        let e = read_bounded(endless, path).unwrap_err();
        assert_eq!(e.code, ErrorCode::FileTooLarge);
        // Up to the limit it is read whole, as before.
        let full = vec![b'a'; MAX_TEXT_FILE as usize];
        assert_eq!(read_bounded(&full[..], path).unwrap().len(), full.len());
        let not_text = read_bounded(&[0xff, 0xfe][..], path).unwrap_err();
        assert_eq!(not_text.code, ErrorCode::Validation);
        assert_eq!(not_text.message, "/proc/kallsyms isn't a text file.");
    }

    #[cfg(unix)]
    #[test]
    fn what_is_no_regular_file_is_refused_without_being_opened() {
        let dir = tempfile::tempdir().unwrap();
        let folder = read_text(dir.path()).unwrap_err();
        assert!(folder.message.ends_with("is a folder, not a file."));
        let device = read_text(Path::new("/dev/zero")).unwrap_err();
        assert_eq!(device.code, ErrorCode::Validation);
        assert_eq!(device.message, "/dev/zero isn't a regular file.");
        // A pipe nobody writes to would keep a reader waiting for ever.
        let pipe = dir.path().join("pipe");
        let made = std::process::Command::new("mkfifo").arg(&pipe).status();
        assert!(made.is_ok_and(|status| status.success()), "mkfifo");
        let (answer, answered) = std::sync::mpsc::channel();
        std::thread::spawn(move || answer.send(read_text(&pipe)));
        let refused = answered
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("the read came back")
            .unwrap_err();
        assert!(refused.message.ends_with("pipe isn't a regular file."));
        // A link to a regular file is read as the file.
        let file = dir.path().join("key.pem");
        std::fs::write(&file, "secret").unwrap();
        let link = dir.path().join("link.pem");
        std::os::unix::fs::symlink(&file, &link).unwrap();
        assert_eq!(read_text(&link).unwrap(), "secret");
    }

    #[cfg(unix)]
    #[test]
    fn a_replacement_is_private_until_it_is_given_a_mode() {
        use std::os::unix::fs::PermissionsExt;
        let mode = |file: &std::fs::File| file.metadata().unwrap().permissions().mode() & 0o777;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("logs.txt");
        let unfinished = Replacement::new(&path).unwrap();
        assert_eq!(mode(unfinished.file()), 0o600);
        // What any new file here gets, and nothing is left of asking.
        let plain = dir.path().join("plain.txt");
        std::fs::write(&plain, "").unwrap();
        unfinished.take_default_mode().unwrap();
        assert_eq!(
            mode(unfinished.file()),
            std::fs::metadata(&plain).unwrap().permissions().mode() & 0o777
        );
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
        std::fs::set_permissions(&plain, std::fs::Permissions::from_mode(0o640)).unwrap();
        unfinished
            .keep_mode_of(&std::fs::metadata(&plain).unwrap())
            .unwrap();
        assert_eq!(mode(unfinished.file()), 0o640);
    }

    #[test]
    fn a_replacement_that_cannot_be_put_in_place_is_handed_back() {
        let dir = tempfile::tempdir().unwrap();
        // A folder that holds something stands where the file should go.
        let path = dir.path().join("taken");
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join("inside"), "kept").unwrap();
        let unfinished = Replacement::new(&path).unwrap();
        std::io::Write::write_all(&mut unfinished.file(), b"new").unwrap();
        let Err((_, back)) = unfinished.try_commit() else {
            panic!("a file was put over a folder");
        };
        // Still there to be read, and gone once dropped.
        assert_eq!(back.file().metadata().unwrap().len(), 3);
        drop(back);
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
        let whole = Replacement::new(&path).unwrap();
        assert!(whole.commit().is_err());
    }

    #[test]
    fn text_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("logs.txt");
        write_text(&path, "hello").unwrap();
        assert_eq!(read_text(&path).unwrap(), "hello");
    }
}
