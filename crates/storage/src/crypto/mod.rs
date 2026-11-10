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
mod tests {
    use super::*;
    use homeport_core::error::ErrorCode;

    struct NoSecretService;
    impl KeyStore for NoSecretService {
        fn load(&self) -> Result<Option<Key>, String> {
            Err("no secret service".into())
        }
        fn create(&self) -> Result<Key, String> {
            Err("no secret service".into())
        }
    }

    /// A keychain in memory, counting the keys it was asked to make.
    #[derive(Default)]
    struct Memory {
        key: std::sync::Mutex<Option<Key>>,
        created: std::sync::atomic::AtomicU32,
    }
    impl Memory {
        fn created(&self) -> u32 {
            self.created.load(std::sync::atomic::Ordering::SeqCst)
        }
    }
    impl KeyStore for Memory {
        fn load(&self) -> Result<Option<Key>, String> {
            Ok(*self.key.lock().unwrap())
        }
        fn create(&self) -> Result<Key, String> {
            self.created
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let key = random_key();
            *self.key.lock().unwrap() = Some(key);
            Ok(key)
        }
    }

    #[test]
    fn the_first_run_makes_a_key_and_later_runs_reuse_it() {
        let keychain = Memory::default();
        let first = SecretBox::from_store(&keychain, false);
        let sealed = first.seal(b"secret").unwrap();
        let again = SecretBox::from_store(&keychain, true);
        assert_eq!(again.open(&sealed).unwrap(), b"secret");
        assert_eq!(keychain.created(), 1);
    }

    #[test]
    fn a_missing_entry_never_gets_a_new_key_over_sealed_secrets() {
        // Another Secret Service answers this time and has no entry: the
        // database still holds secrets sealed under the key it lacks.
        let keychain = Memory::default();
        let secrets = SecretBox::from_store(&keychain, true);
        assert!(!secrets.is_available());
        assert!(secrets.entry_missing());
        assert_eq!(keychain.created(), 0);
        assert!(keychain.key.lock().unwrap().is_none(), "nothing written");
        let refused = secrets.seal(b"x").unwrap_err();
        assert!(refused.is(ErrorCode::KeychainUnavailable));
        assert_eq!(refused.key, "errors.secrets.keychainEntryMissing");

        // Starting over, as the user asks, makes one.
        secrets.start_over(&keychain).unwrap();
        assert!(secrets.is_available() && !secrets.entry_missing());
        assert_eq!(keychain.created(), 1);
        let sealed = secrets.seal(b"again").unwrap();
        assert_eq!(secrets.open(&sealed).unwrap(), b"again");
    }

    #[test]
    fn starting_over_uses_an_entry_that_came_back_and_never_replaces_a_working_key() {
        let keychain = Memory::default();
        let secrets = SecretBox::from_store(&keychain, true);
        let old = [9; 32];
        let sealed = SecretBox::from_key(&old).seal(b"kept").unwrap();
        *keychain.key.lock().unwrap() = Some(old);
        secrets.start_over(&keychain).unwrap();
        assert_eq!(secrets.open(&sealed).unwrap(), b"kept");
        assert_eq!(keychain.created(), 0);

        let refused = secrets.start_over(&keychain).unwrap_err();
        assert!(refused.is(ErrorCode::Validation));
        assert_eq!(*keychain.key.lock().unwrap(), Some(old));
        assert!(SecretBox::from_store(&NoSecretService, false)
            .start_over(&keychain)
            .is_err());
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
        assert!(matches!(sb.open(&sealed), Err(ref e) if e.is(ErrorCode::CredentialsLost)));
        assert!(matches!(sb.open(b""), Err(ref e) if e.is(ErrorCode::CredentialsLost)));
    }

    #[test]
    fn data_sealed_under_another_key_is_reported_as_lost_credentials() {
        let sealed = SecretBox::from_key(&[1; 32]).seal(b"secret").unwrap();
        let other = SecretBox::from_key(&[2; 32]);
        assert!(matches!(
            other.open(&sealed),
            Err(ref e) if e.is(ErrorCode::CredentialsLost)
        ));
    }

    #[test]
    fn without_a_keychain_secrets_are_unavailable() {
        let sb = SecretBox::from_store(&NoSecretService, false);
        assert!(!sb.is_available());
        assert!(matches!(
            sb.seal(b"x"),
            Err(ref e) if e.is(ErrorCode::KeychainUnavailable)
        ));
    }

    #[test]
    fn json_round_trips() {
        let sb = SecretBox::from_key(&random_key());
        let sealed = sb.seal_json(&vec!["a".to_string()]).unwrap();
        assert_eq!(sb.open_json::<Vec<String>>(&sealed).unwrap(), vec!["a"]);
    }
}
