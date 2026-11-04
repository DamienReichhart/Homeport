//! Where the key that seals every secret lives: the OS keychain.

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
