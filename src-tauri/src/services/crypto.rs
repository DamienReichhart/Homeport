//! Field-level encryption for secrets stored in SQLite (see ADR 0002).
//!
//! Each secret is sealed separately with XChaCha20-Poly1305 under one random
//! 256-bit key held in the OS keychain. Sealed format: `version || nonce || ciphertext`.

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{AppError, AppResult};

const FORMAT_V1: u8 = 1;
const NONCE_LEN: usize = 24;

pub type Key = [u8; 32];

/// Where the encryption key lives. The app uses [`Keychain`]; tests pass a key directly.
pub trait KeyStore {
    /// Returns the key, creating and storing a fresh one if none exists yet.
    fn load_or_create(&self) -> Result<Key, String>;
}

/// The OS keychain: macOS Keychain, Windows Credential Manager or Secret Service.
pub struct Keychain {
    pub service: &'static str,
    pub account: &'static str,
}

impl KeyStore for Keychain {
    fn load_or_create(&self) -> Result<Key, String> {
        let entry = keyring::Entry::new(self.service, self.account).map_err(|e| e.to_string())?;
        match entry.get_secret() {
            Ok(bytes) => bytes
                .try_into()
                .map_err(|_| "the keychain entry has an unexpected length".to_string()),
            Err(keyring::Error::NoEntry) => {
                let key = random_key();
                entry.set_secret(&key).map_err(|e| e.to_string())?;
                Ok(key)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn random_key() -> Key {
    let mut key = [0u8; 32];
    getrandom::fill(&mut key).expect("OS random number generator failed");
    key
}

/// Seals and opens secrets. Without a key (no keychain), sealing and opening fail
/// with `keychain_unavailable` while everything else keeps working.
pub struct SecretBox {
    cipher: Result<XChaCha20Poly1305, String>,
}

impl SecretBox {
    pub fn from_store(store: &dyn KeyStore) -> Self {
        match store.load_or_create() {
            Ok(key) => Self::from_key(&key),
            Err(reason) => {
                log::warn!("OS keychain unavailable, secrets are disabled: {reason}");
                Self {
                    cipher: Err(reason),
                }
            }
        }
    }

    pub fn from_key(key: &Key) -> Self {
        Self {
            cipher: Ok(XChaCha20Poly1305::new(key.into())),
        }
    }

    pub fn is_available(&self) -> bool {
        self.cipher.is_ok()
    }

    fn cipher(&self) -> AppResult<&XChaCha20Poly1305> {
        self.cipher
            .as_ref()
            .map_err(|reason| AppError::KeychainUnavailable(reason.clone()))
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
                    .map_err(|_| AppError::CredentialsLost)
            }
            _ => Err(AppError::CredentialsLost),
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
mod tests {
    use super::*;

    struct Unavailable;
    impl KeyStore for Unavailable {
        fn load_or_create(&self) -> Result<Key, String> {
            Err("no secret service".into())
        }
    }

    #[test]
    fn sealed_data_opens_back() {
        let sb = SecretBox::from_key(&[7; 32]);
        let sealed = sb.seal(b"-----BEGIN OPENSSH PRIVATE KEY-----").unwrap();
        assert_eq!(
            sb.open(&sealed).unwrap(),
            b"-----BEGIN OPENSSH PRIVATE KEY-----"
        );
    }

    #[test]
    fn sealing_does_not_leak_plaintext_or_repeat() {
        let sb = SecretBox::from_key(&[7; 32]);
        let a = sb.seal(b"hunter2hunter2").unwrap();
        let b = sb.seal(b"hunter2hunter2").unwrap();
        assert_ne!(a, b);
        assert!(!a.windows(7).any(|w| w == b"hunter2"));
    }

    #[test]
    fn tampered_data_is_reported_as_lost_credentials() {
        let sb = SecretBox::from_key(&[7; 32]);
        let mut sealed = sb.seal(b"secret").unwrap();
        *sealed.last_mut().unwrap() ^= 1;
        assert!(matches!(sb.open(&sealed), Err(AppError::CredentialsLost)));
        assert!(matches!(sb.open(b""), Err(AppError::CredentialsLost)));
    }

    #[test]
    fn data_sealed_under_another_key_is_reported_as_lost_credentials() {
        let sealed = SecretBox::from_key(&[1; 32]).seal(b"secret").unwrap();
        let other = SecretBox::from_key(&[2; 32]);
        assert!(matches!(
            other.open(&sealed),
            Err(AppError::CredentialsLost)
        ));
    }

    #[test]
    fn without_a_keychain_secrets_are_unavailable() {
        let sb = SecretBox::from_store(&Unavailable);
        assert!(!sb.is_available());
        assert!(matches!(
            sb.seal(b"x"),
            Err(AppError::KeychainUnavailable(_))
        ));
    }

    #[test]
    fn json_round_trips() {
        let sb = SecretBox::from_key(&random_key());
        let sealed = sb.seal_json(&vec!["a".to_string()]).unwrap();
        assert_eq!(sb.open_json::<Vec<String>>(&sealed).unwrap(), vec!["a"]);
    }
}
