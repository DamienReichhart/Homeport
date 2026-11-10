//! Field-level encryption for secrets stored in SQLite (see ADR 0002).
//!
//! Each secret is sealed separately with XChaCha20-Poly1305 under one random
//! 256-bit key held in the OS keychain. Sealed format: `version || nonce || ciphertext`.

use std::sync::RwLock;

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use serde::{de::DeserializeOwned, Serialize};

use homeport_core::error::{AppError, AppResult};

pub mod bundle;
mod keystore;

pub use self::keystore::{random_key, Key, KeyStore, Keychain};

const FORMAT_V1: u8 = 1;

fn cipher_of(key: &Key) -> XChaCha20Poly1305 {
    XChaCha20Poly1305::new(key.into())
}

fn unavailable(why: &Unavailable) -> AppError {
    match why {
        Unavailable::Keychain(reason) => AppError::keychain_unavailable(reason),
        Unavailable::EntryMissing => AppError::keychain_entry_missing(),
    }
}
const NONCE_LEN: usize = 24;

/// Seals and opens secrets. Without a key (no keychain, or no entry for the
/// key the database's secrets were sealed with), sealing and opening fail
/// with `keychain_unavailable` while everything else keeps working.
pub struct SecretBox {
    cipher: RwLock<Result<XChaCha20Poly1305, Unavailable>>,
}

/// Why there's no key.
#[derive(Debug, Clone)]
enum Unavailable {
    /// The keychain can't be used: its reason.
    Keychain(String),
    /// The keychain answers but has no entry, and the database holds
    /// secrets sealed under a key: a new one would orphan them for good if
    /// the entry comes back (another Secret Service answering this time).
    EntryMissing,
}

impl SecretBox {
    /// The key of `store`, or a new one when it has none and nothing is
    /// sealed yet (`holds_sealed` false): the first run. When secrets are
    /// sealed and the entry is missing, no key is made over them; the box
    /// stays unavailable until [`SecretBox::start_over`].
    pub fn from_store(store: &dyn KeyStore, holds_sealed: bool) -> Self {
        let cipher = match store.load() {
            Ok(Some(key)) => Ok(cipher_of(&key)),
            Ok(None) if holds_sealed => {
                log::warn!(
                    "the OS keychain has no entry for the key Homeport's secrets were sealed with; \
                     not making a new one over them"
                );
                Err(Unavailable::EntryMissing)
            }
            Ok(None) => store.create().map(|key| cipher_of(&key)).map_err(|reason| {
                log::warn!("couldn't store a new key in the OS keychain: {reason}");
                Unavailable::Keychain(reason)
            }),
            Err(reason) => {
                log::warn!("OS keychain unavailable, secrets are disabled: {reason}");
                Err(Unavailable::Keychain(reason))
            }
        };
        Self {
            cipher: RwLock::new(cipher),
        }
    }

    pub fn from_key(key: &Key) -> Self {
        Self {
            cipher: RwLock::new(Ok(cipher_of(key))),
        }
    }

    pub fn is_available(&self) -> bool {
        self.read().is_ok()
    }

    /// The keychain has no entry for the key the stored secrets were sealed with.
    pub fn entry_missing(&self) -> bool {
        matches!(*self.read(), Err(Unavailable::EntryMissing))
    }

    /// Gives up on the missing key, as the user asked: uses the keychain's
    /// entry if it is back, or stores a new key. Secrets sealed under the
    /// old one then read as `credentials_lost`, to re-enter or restore.
    /// Refused unless the entry is what's missing, so a working key is
    /// never replaced.
    pub fn start_over(&self, store: &dyn KeyStore) -> AppResult<()> {
        if !self.entry_missing() {
            return Err(match &*self.read() {
                Ok(_) => AppError::validation(crate::messages::KEY_IN_USE),
                Err(e) => unavailable(e),
            });
        }
        let key = match store.load() {
            Ok(Some(key)) => key,
            Ok(None) => store.create().map_err(AppError::keychain_unavailable)?,
            Err(reason) => return Err(AppError::keychain_unavailable(reason)),
        };
        *self.cipher.write().expect("secret box poisoned") = Ok(cipher_of(&key));
        Ok(())
    }

    fn read(&self) -> std::sync::RwLockReadGuard<'_, Result<XChaCha20Poly1305, Unavailable>> {
        self.cipher.read().expect("secret box poisoned")
    }

    fn cipher(&self) -> AppResult<XChaCha20Poly1305> {
        self.read().clone().map_err(|e| unavailable(&e))
    }

    pub fn seal(&self, plaintext: &[u8]) -> AppResult<Vec<u8>> {
        let cipher = self.cipher()?;
        let mut nonce = [0u8; NONCE_LEN];
        getrandom::fill(&mut nonce).expect("OS random number generator failed");
        let ciphertext = cipher
            .encrypt(XNonce::from_slice(&nonce), plaintext)
            .expect("XChaCha20-Poly1305 encryption cannot fail for in-memory buffers");
        let mut sealed = Vec::with_capacity(1 + NONCE_LEN + ciphertext.len());
        sealed.push(FORMAT_V1);
        sealed.extend_from_slice(&nonce);
        sealed.extend_from_slice(&ciphertext);
        Ok(sealed)
    }

    /// Fails with `credentials_lost` when the data was sealed under another key
    /// (e.g. the keychain entry was deleted) or has been tampered with.
    pub fn open(&self, sealed: &[u8]) -> AppResult<Vec<u8>> {
        let cipher = self.cipher()?;
        match sealed {
            [FORMAT_V1, rest @ ..] if rest.len() > NONCE_LEN => {
                let (nonce, ciphertext) = rest.split_at(NONCE_LEN);
                cipher
                    .decrypt(XNonce::from_slice(nonce), ciphertext)
                    .map_err(|_| AppError::credentials_lost())
            }
            _ => Err(AppError::credentials_lost()),
        }
    }

    pub fn seal_json<T: Serialize>(&self, value: &T) -> AppResult<Vec<u8>> {
        self.seal(&serde_json::to_vec(value)?)
    }

    pub fn open_json<T: DeserializeOwned>(&self, sealed: &[u8]) -> AppResult<T> {
        Ok(serde_json::from_slice(&self.open(sealed)?)?)
    }
}

#[cfg(test)]
mod tests;
