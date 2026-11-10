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
