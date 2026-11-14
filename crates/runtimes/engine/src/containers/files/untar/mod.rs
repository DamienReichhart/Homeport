//! The first file of a tar archive, read out as the archive comes, in
//! one pass: what the Engine sends for one file of a container (its
//! headers, then its bytes), so the file can be written as it comes.
//!
//! The headers before the file's own are read and left: a PAX extended
//! header (whose `size` is the file's when it is too large for the
//! header's field), a global one, and GNU's long name and long link.
//! A size is octal, or base-256 when its first byte has its high bit.

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

/// A tar block.
const BLOCK: usize = 512;

/// The most the headers before the file may take: a long name, a PAX
/// header. More is not an archive of one file.
const HEADERS_MAX: usize = 1024 * 1024;

#[derive(Debug, Default)]
enum Phase {
    /// Reading headers; what came of them is kept until a header is whole.
    #[default]
    Headers,
    /// So many bytes of the file are still to come.
    Data(u64),
    /// The file is whole; what follows (padding, the end) is left.
    Done,
}

/// The first file of an archive, as its bytes come.
#[derive(Debug, Default)]
pub(super) struct Untar {
    phase: Phase,
    pending: Vec<u8>,
    /// The size a PAX header gave the next file.
    pax_size: Option<u64>,
    /// The archive ended (two zero blocks) before any file.
    empty: bool,
}

fn malformed(why: &str) -> AppError {
    AppError::new(ErrorCode::EngineError, msg::FILES_CUT_ARCHIVE).with_detail(why)
}

/// A header's size field: octal digits, or base-256.
fn size_of(field: &[u8]) -> AppResult<u64> {
    if field[0] & 0x80 != 0 {
        let mut value = u64::from(field[0] & 0x7f);
        for byte in &field[1..] {
            value = value
                .checked_mul(256)
                .and_then(|value| value.checked_add(u64::from(*byte)))
                .ok_or_else(|| malformed("a size too large"))?;
        }
        return Ok(value);
    }
    let text: String = field
        .iter()
        .take_while(|byte| **byte != 0)
        .map(|byte| char::from(*byte))
        .collect();
    let text = text.trim();
    if text.is_empty() {
        return Ok(0);
    }
    u64::from_str_radix(text, 8).map_err(|_| malformed("a size that is no number"))
}

/// The `size` of PAX records (`"%d size=%d\n"`…), if they give one.
fn pax_size(records: &[u8]) -> Option<u64> {
    let text = String::from_utf8_lossy(records);
    text.lines()
        .filter_map(|record| record.split_once(' ').map(|(_, rest)| rest))
        .filter_map(|record| record.strip_prefix("size="))
        .find_map(|size| size.trim().parse().ok())
}

/// Bytes rounded up to whole blocks.
fn padded(size: u64) -> u64 {
    size.div_ceil(BLOCK as u64) * BLOCK as u64
}

impl Untar {
    /// The file's bytes in `chunk`, the next of the archive.
    pub(super) fn feed<'c>(&mut self, chunk: &'c [u8]) -> AppResult<std::borrow::Cow<'c, [u8]>> {
        use std::borrow::Cow;
        match self.phase {
            Phase::Done => Ok(Cow::Borrowed(&[])),
            Phase::Data(left) => Ok(Cow::Borrowed(self.take(chunk, left))),
            Phase::Headers => {
                self.pending.extend_from_slice(chunk);
                if self.pending.len() > HEADERS_MAX + BLOCK {
                    return Err(malformed("headers too long"));
                }
                let Some(start) = self.headers()? else {
                    return Ok(Cow::Borrowed(&[]));
                };
                let rest = self.pending.split_off(start);
                self.pending.clear();
                let Phase::Data(left) = self.phase else {
                    return Ok(Cow::Borrowed(&[]));
                };
                Ok(Cow::Owned(self.take(&rest, left).to_vec()))
            }
        }
    }

    /// What of `bytes` is the file's, `left` of it still to come.
    fn take<'b>(&mut self, bytes: &'b [u8], left: u64) -> &'b [u8] {
        let length = usize::try_from(left).map_or(bytes.len(), |left| left.min(bytes.len()));
        let left = left - length as u64;
        self.phase = if left == 0 {
            Phase::Done
        } else {
            Phase::Data(left)
        };
        &bytes[..length]
    }

    /// Reads the headers kept so far; where the file's bytes start once
    /// its own header was read, `None` while more is needed.
    fn headers(&mut self) -> AppResult<Option<usize>> {
        let mut at = 0;
        while self.pending.len() >= at + BLOCK {
            let header = &self.pending[at..at + BLOCK];
            if header.iter().all(|byte| *byte == 0) {
                self.empty = true;
                self.phase = Phase::Done;
                return Ok(Some(at + BLOCK));
            }
            let size = size_of(&header[124..136])?;
            match header[156] {
                // PAX (`x`, `g`) and GNU's long name and link (`L`, `K`):
                // their content, then the next header.
                kind @ (b'x' | b'g' | b'L' | b'K') => {
                    let length =
                        usize::try_from(padded(size)).map_err(|_| malformed("headers too long"))?;
                    if self.pending.len() < at + BLOCK + length {
                        return Ok(None);
                    }
                    if kind == b'x' {
                        let records = &self.pending[at + BLOCK..at + BLOCK + size as usize];
                        self.pax_size = pax_size(records).or(self.pax_size);
                    }
                    at += BLOCK + length;
                }
                // The file (or what stands for one: a link has no bytes).
                _ => {
                    let size = self.pax_size.take().unwrap_or(size);
                    self.phase = if size == 0 {
                        Phase::Done
                    } else {
                        Phase::Data(size)
                    };
                    return Ok(Some(at + BLOCK));
                }
            }
        }
        // Keep only what is not read yet.
        self.pending.drain(..at);
        Ok(None)
    }

    /// The file is whole.
    pub(super) fn is_done(&self) -> bool {
        matches!(self.phase, Phase::Done)
    }

    /// The archive held no file.
    pub(super) fn is_empty(&self) -> bool {
        self.empty
    }
}

#[cfg(test)]
mod tests;
