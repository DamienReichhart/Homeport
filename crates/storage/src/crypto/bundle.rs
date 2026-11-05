//! Passphrase-protected files, for moving Homeport's data between computers
//! (ADR 0013). Unlike [`SecretBox`](super::SecretBox), the key comes from a
//! passphrase the user types, not from the OS keychain.
//!
//! Format: `HOMEPORT-BUNDLE`, a format version byte, the Argon2id cost (memory
//! in KiB, iterations, lanes; little-endian u32s), a 16-byte salt and a 24-byte
//! nonce, then the payload sealed with XChaCha20-Poly1305 under the derived
//! key, with everything before it as associated data. The whole payload is
//! encrypted, so the file reveals nothing but its size without the passphrase.

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

pub const MAGIC: &[u8] = b"HOMEPORT-BUNDLE";
const FORMAT_V1: u8 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;
const HEADER_LEN: usize = MAGIC.len() + 1 + 3 * 4 + SALT_LEN + NONCE_LEN;

/// The shortest passphrase Homeport accepts for a bundle.
pub const MIN_PASSPHRASE_CHARS: usize = 12;

/// How hard deriving the key is. Stored in the file, so it can grow later.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cost {
    pub memory_kib: u32,
    pub iterations: u32,
    pub lanes: u32,
}

impl Cost {
    /// What new bundles use: 64 MiB, 3 passes, one lane.
    pub const DEFAULT: Self = Self {
        memory_kib: 64 * 1024,
        iterations: 3,
        lanes: 1,
    };

    /// Beyond this, a file is refused rather than tying the computer up.
    const MAX: Self = Self {
        memory_kib: 1024 * 1024,
        iterations: 16,
        lanes: 16,
    };

    fn key(self, passphrase: &str, salt: &[u8]) -> AppResult<[u8; 32]> {
        let params = Params::new(self.memory_kib, self.iterations, self.lanes, Some(32))
            .map_err(|_| damaged())?;
        let mut key = [0u8; 32];
        Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
            .hash_password_into(passphrase.as_bytes(), salt, &mut key)
            .map_err(|_| damaged())?;
        Ok(key)
    }
}

/// Seals `payload` under `passphrase` with the default cost.
pub fn seal(passphrase: &str, payload: &[u8]) -> AppResult<Vec<u8>> {
    seal_with(Cost::DEFAULT, passphrase, payload)
}

pub fn seal_with(cost: Cost, passphrase: &str, payload: &[u8]) -> AppResult<Vec<u8>> {
    if passphrase.chars().count() < MIN_PASSPHRASE_CHARS {
        return Err(AppError::validation(
            msg::BUNDLE_PASSPHRASE_SHORT.with("min", MIN_PASSPHRASE_CHARS),
        ));
    }
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::fill(&mut salt).expect("OS random number generator failed");
    getrandom::fill(&mut nonce).expect("OS random number generator failed");

    let mut out = Vec::with_capacity(HEADER_LEN + payload.len() + 16);
    out.extend_from_slice(MAGIC);
    out.push(FORMAT_V1);
    for n in [cost.memory_kib, cost.iterations, cost.lanes] {
        out.extend_from_slice(&n.to_le_bytes());
    }
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce);

    let key = cost.key(passphrase, &salt)?;
    let ciphertext = XChaCha20Poly1305::new((&key).into())
        .encrypt(
            XNonce::from_slice(&nonce),
            Payload {
                msg: payload,
                aad: &out,
            },
        )
        .expect("XChaCha20-Poly1305 encryption cannot fail for in-memory buffers");
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Opens a bundle. A wrong passphrase and a damaged file can't be told apart.
pub fn open(passphrase: &str, bytes: &[u8]) -> AppResult<Vec<u8>> {
    let Some(rest) = bytes.strip_prefix(MAGIC) else {
        return Err(AppError::validation(msg::BUNDLE_NOT_A_BUNDLE));
    };
    match rest.first() {
        Some(&FORMAT_V1) => {}
        Some(_) => return Err(too_new()),
        None => return Err(damaged()),
    }
    if bytes.len() < HEADER_LEN {
        return Err(damaged());
    }
    let (header, ciphertext) = bytes.split_at(HEADER_LEN);
    let mut at = MAGIC.len() + 1;
    let mut u32_at = || {
        let n = u32::from_le_bytes(header[at..at + 4].try_into().expect("four bytes"));
        at += 4;
        n
    };
    let cost = Cost {
        memory_kib: u32_at(),
        iterations: u32_at(),
        lanes: u32_at(),
    };
    if cost.memory_kib > Cost::MAX.memory_kib
        || cost.iterations > Cost::MAX.iterations
        || cost.lanes > Cost::MAX.lanes
    {
        return Err(damaged());
    }
    let salt = &header[at..at + SALT_LEN];
    let nonce = &header[at + SALT_LEN..];
    let key = cost.key(passphrase, salt)?;
    XChaCha20Poly1305::new((&key).into())
        .decrypt(
            XNonce::from_slice(nonce),
            Payload {
                msg: ciphertext,
                aad: header,
            },
        )
        .map_err(|_| damaged())
}

/// The payload inside is of a version this Homeport doesn't know.
pub fn too_new() -> AppError {
    AppError::new(ErrorCode::BundleTooNew, msg::BUNDLE_TOO_NEW).with_hint(msg::BUNDLE_TOO_NEW_HINT)
}

fn damaged() -> AppError {
    AppError::new(ErrorCode::BundleUnreadable, msg::BUNDLE_UNREADABLE)
        .with_hint(msg::BUNDLE_UNREADABLE_HINT)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cheap enough for debug builds; the format is the same.
    const TEST: Cost = Cost {
        memory_kib: 64,
        iterations: 1,
        lanes: 1,
    };
    const PASS: &str = "correct horse battery";

    #[test]
    fn a_sealed_bundle_opens_with_its_passphrase() {
        let sealed = seal_with(TEST, PASS, b"{\"hosts\":[]}").unwrap();
        assert!(sealed.starts_with(MAGIC));
        assert_eq!(open(PASS, &sealed).unwrap(), b"{\"hosts\":[]}");
    }

    #[test]
    fn nothing_of_the_payload_shows_in_the_file() {
        let sealed = seal_with(TEST, PASS, b"prod.example.com deploy").unwrap();
        assert!(!sealed.windows(4).any(|w| w == b"prod"));
        assert_ne!(
            sealed,
            seal_with(TEST, PASS, b"prod.example.com deploy").unwrap()
        );
    }

    #[test]
    fn a_wrong_passphrase_is_refused() {
        let sealed = seal_with(TEST, PASS, b"x").unwrap();
        let err = open("not the passphrase", &sealed).unwrap_err();
        assert_eq!(err.code, ErrorCode::BundleUnreadable);
        assert!(err.hint.is_some());
    }

    #[test]
    fn tampering_anywhere_is_refused() {
        let sealed = seal_with(TEST, PASS, b"payload").unwrap();
        // The cost, the salt, the nonce and the ciphertext are all checked.
        for at in [
            MAGIC.len() + 1,
            MAGIC.len() + 14,
            HEADER_LEN - 1,
            sealed.len() - 1,
        ] {
            let mut bad = sealed.clone();
            bad[at] ^= 1;
            let err = open(PASS, &bad).unwrap_err();
            assert_eq!(err.code, ErrorCode::BundleUnreadable, "byte {at}");
        }
        assert_eq!(
            open(PASS, &sealed[..HEADER_LEN - 3]).unwrap_err().code,
            ErrorCode::BundleUnreadable
        );
    }

    #[test]
    fn files_from_a_newer_format_and_other_files_say_so() {
        let mut newer = seal_with(TEST, PASS, b"x").unwrap();
        newer[MAGIC.len()] = 2;
        assert_eq!(
            open(PASS, &newer).unwrap_err().code,
            ErrorCode::BundleTooNew
        );
        assert_eq!(
            open(PASS, b"{\"not\": \"a bundle\"}").unwrap_err().code,
            ErrorCode::Validation
        );
    }

    #[test]
    fn an_absurd_cost_is_refused_before_deriving() {
        let mut sealed = seal_with(TEST, PASS, b"x").unwrap();
        let at = MAGIC.len() + 1;
        sealed[at..at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(
            open(PASS, &sealed).unwrap_err().code,
            ErrorCode::BundleUnreadable
        );
    }

    #[test]
    fn passphrases_need_twelve_characters() {
        assert!(matches!(
            seal_with(TEST, "short", b"x"),
            Err(ref e) if e.is(ErrorCode::Validation)
        ));
        assert!(seal_with(TEST, "twelve chars", b"x").is_ok());
    }

    #[test]
    fn the_default_cost_round_trips() {
        let sealed = seal(PASS, b"x").unwrap();
        assert_eq!(open(PASS, &sealed).unwrap(), b"x");
    }
}
