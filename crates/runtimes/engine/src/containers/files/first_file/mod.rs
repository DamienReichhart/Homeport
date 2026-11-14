//! The file of the archive the Engine sends for one, read out with the
//! `tar` crate as the archive comes, in one pass, on a thread that may
//! block: the archive's chunks come over a channel and are read as one
//! [`Read`].
//!
//! What the crate would take on trust is checked here:
//!
//! - Only a regular file is saved; a link, a folder, a device… is an
//!   error, never an empty file.
//! - Its size, once headers (a PAX `size` among them) said it, must be the
//!   one the Engine gave before sending it.
//! - The archive must hold all of it: an archive that ends early is an
//!   error, never a file cut short.
//! - The headers before the file (a long name, PAX records, which the crate
//!   reads whole into memory) may take [`HEADERS_MAX`] bytes, no more.

use std::{
    cell::Cell,
    io::{self, Read, Write},
    path::Path,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

use bytes::{Buf, Bytes};
use homeport_core::error::{local_file, AppError, AppResult, ErrorCode};
use tar::EntryType;

use crate::messages as msg;

/// How much of the file is read, then written, at once.
const CHUNK: usize = 64 * 1024;

/// The most the headers before the file may take: a long name, PAX
/// records. More is not an archive of one file.
const HEADERS_MAX: u64 = 1024 * 1024;

/// What reading the archive found so far, seen from outside it.
#[derive(Debug)]
struct Gauge {
    /// How many bytes of the archive may be read in all.
    limit: Cell<u64>,
    /// The chunks ended: what was missing then was never sent.
    ended: Cell<bool>,
}

/// The archive's chunks as they come, as one [`Read`].
struct Incoming<I> {
    chunks: I,
    current: Bytes,
    read: u64,
    gauge: Rc<Gauge>,
}

impl<I: Iterator<Item = Bytes>> Read for Incoming<I> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let room = self.gauge.limit.get().saturating_sub(self.read);
        if room == 0 {
            return Err(io::Error::other("the headers before the file are too long"));
        }
        while self.current.is_empty() {
            match self.chunks.next() {
                Some(chunk) => self.current = chunk,
                None => {
                    self.gauge.ended.set(true);
                    return Ok(0);
                }
            }
        }
        let length = buf
            .len()
            .min(self.current.len())
            .min(usize::try_from(room).unwrap_or(usize::MAX));
        buf[..length].copy_from_slice(&self.current[..length]);
        self.current.advance(length);
        self.read += length as u64;
        Ok(length)
    }
}

/// Why the archive couldn't be read: cut short if its chunks ended,
/// otherwise not an archive the file can be read out of.
fn unreadable(error: &io::Error, gauge: &Gauge) -> AppError {
    let message = if gauge.ended.get() {
        msg::FILES_CUT_ARCHIVE
    } else {
        msg::FILES_BAD_ARCHIVE
    };
    AppError::new(ErrorCode::EngineError, message).with_detail(error)
}

/// Writes the file of the archive `chunks` make (the container's `path`)
/// to `out`, saved at `destination`, a chunk at a time. `expected` is its
/// size if the Engine said it. Returns `false`, having written part of it
/// or nothing, once `stopped` says the download was dropped.
pub(super) fn copy_out(
    chunks: impl Iterator<Item = Bytes>,
    path: &str,
    expected: Option<u64>,
    out: &mut impl Write,
    destination: &Path,
    stopped: &AtomicBool,
) -> AppResult<bool> {
    let gauge = Rc::new(Gauge {
        limit: Cell::new(HEADERS_MAX),
        ended: Cell::new(false),
    });
    let mut archive = tar::Archive::new(Incoming {
        chunks,
        current: Bytes::new(),
        read: 0,
        gauge: gauge.clone(),
    });
    let mut entries = archive.entries().map_err(|e| unreadable(&e, &gauge))?;
    let mut entry = loop {
        match entries.next() {
            None => {
                return Err(AppError::new(
                    ErrorCode::EngineError,
                    msg::FILES_EMPTY_ARCHIVE,
                ))
            }
            Some(Err(e)) => return Err(unreadable(&e, &gauge)),
            // Records for every file that follows, not one itself.
            Some(Ok(entry)) if entry.header().entry_type() == EntryType::XGlobalHeader => {}
            Some(Ok(entry)) => break entry,
        }
    };
    if !matches!(
        entry.header().entry_type(),
        EntryType::Regular | EntryType::Continuous
    ) {
        return Err(AppError::validation(
            msg::FILES_NOT_REGULAR.with("path", path),
        ));
    }
    let size = entry.size();
    if expected.is_some_and(|expected| expected != size) {
        return Err(AppError::new(
            ErrorCode::EngineError,
            msg::FILES_SIZE_CHANGED.with("path", path),
        )
        .with_detail(format!("{} bytes, then {size}", expected.unwrap_or(0))));
    }
    // The file's own bytes are read a chunk at a time, however many.
    gauge.limit.set(u64::MAX);
    let mut buffer = vec![0; CHUNK];
    let mut copied = 0;
    while copied < size {
        if stopped.load(Ordering::SeqCst) {
            return Ok(false);
        }
        let length = entry
            .read(&mut buffer)
            .map_err(|e| unreadable(&e, &gauge))?;
        if length == 0 {
            return Err(AppError::new(
                ErrorCode::EngineError,
                msg::FILES_CUT_ARCHIVE,
            ));
        }
        out.write_all(&buffer[..length])
            .map_err(|e| local_file(destination, e))?;
        copied += length as u64;
    }
    Ok(true)
}

#[cfg(test)]
mod tests;
