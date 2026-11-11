//! Where the key that seals every secret lives: the OS keychain.

pub type Key = [u8; 32];

/// Where the encryption key lives. The app uses [`Keychain`]; tests pass a key directly.
pub trait KeyStore {
    /// The stored key; `None` when the store answers but has no entry.
    /// Only a definite "no entry" is `None`: anything else is an error.
    fn load(&self) -> Result<Option<Key>, String>;

    /// Makes a fresh key and stores it. Called only after [`KeyStore::load`]
    /// found no entry and nothing is sealed under a key that may come back.
    fn create(&self) -> Result<Key, String>;
}

/// The OS keychain: macOS Keychain, Windows Credential Manager or Secret Service.
pub struct Keychain {
    pub service: &'static str,
    pub account: &'static str,
}

impl Keychain {
    fn entry(&self) -> Result<keyring::Entry, String> {
        keyring::Entry::new(self.service, self.account).map_err(|e| e.to_string())
    }
}

impl KeyStore for Keychain {
    fn load(&self) -> Result<Option<Key>, String> {
        match self.entry()?.get_secret() {
            Ok(bytes) => bytes
                .try_into()
                .map(Some)
                .map_err(|_| "the keychain entry has an unexpected length".to_string()),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    fn create(&self) -> Result<Key, String> {
        let key = random_key();
        self.entry()?.set_secret(&key).map_err(|e| e.to_string())?;
        Ok(key)
    }
}

pub fn random_key() -> Key {
    let mut key = [0u8; 32];
    getrandom::fill(&mut key).expect("OS random number generator failed");
    key
}
