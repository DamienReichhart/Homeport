//! What an interactive terminal sends the frontend: its output, decoded as
//! UTF-8, then how it ended. A container's and a Pod's alike.

use serde::Serialize;

use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExecEvent {
    Output {
        data: String,
    },
    /// `error` explains an exit that means the command couldn't run.
    Exit {
        code: Option<i64>,
        error: Option<AppError>,
    },
    Error {
        error: AppError,
    },
}

/// Decodes a byte stream as UTF-8, holding back a character split across chunks.
#[derive(Default)]
pub struct Utf8Decoder {
    pending: Vec<u8>,
}

impl Utf8Decoder {
    pub fn push(&mut self, bytes: &[u8]) -> String {
        self.pending.extend_from_slice(bytes);
        let mut out = String::new();
        loop {
            match std::str::from_utf8(&self.pending) {
                Ok(s) => {
                    out.push_str(s);
                    self.pending.clear();
                    return out;
                }
                Err(e) => {
                    let valid = e.valid_up_to();
                    out.push_str(std::str::from_utf8(&self.pending[..valid]).expect("validated"));
                    match e.error_len() {
                        // An incomplete character at the end: wait for more bytes.
                        None => {
                            self.pending.drain(..valid);
                            return out;
                        }
                        Some(bad) => {
                            out.push(char::REPLACEMENT_CHARACTER);
                            self.pending.drain(..valid + bad);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_split_across_chunks_is_reassembled() {
        let mut d = Utf8Decoder::default();
        let bytes = "héllo ✓".as_bytes();
        let mut out = String::new();
        for b in bytes {
            out.push_str(&d.push(&[*b]));
        }
        assert_eq!(out, "héllo ✓");
        assert_eq!(d.push(&[b'a', 0xFF, b'b']), "a\u{FFFD}b");
    }
}
