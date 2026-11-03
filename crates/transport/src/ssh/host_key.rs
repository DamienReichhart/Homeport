//! Server keys, trusted on first use and stored with the Host (ADR 0002).

use std::sync::{Arc, Mutex};

use russh::{
    client,
    keys::{HashAlg, PublicKey, PublicKeyOrCertificate},
};

use super::keys::fingerprint;
use crate::ConnectError;

/// How a server key compares to the one trusted on first use.
#[derive(Debug, PartialEq, Eq)]
pub enum HostKeyCheck {
    Trusted,
    Unknown,
    Mismatch,
}

pub fn check_host_key(trusted: Option<&str>, presented: &PublicKey) -> HostKeyCheck {
    match trusted.map(PublicKey::from_openssh) {
        None => HostKeyCheck::Unknown,
        Some(Ok(trusted)) if trusted.key_data() == presented.key_data() => HostKeyCheck::Trusted,
        Some(_) => HostKeyCheck::Mismatch,
    }
}

pub(super) fn host_key_error(trusted: Option<&str>, presented: &PublicKey) -> ConnectError {
    let key = presented.to_openssh().unwrap_or_default();
    let fingerprint = presented.fingerprint(HashAlg::Sha256).to_string();
    match trusted.and_then(fingerprint_of_trusted) {
        Some(expected_fingerprint) => ConnectError::HostKeyMismatch {
            key,
            fingerprint,
            expected_fingerprint,
        },
        None => ConnectError::HostKeyUntrusted { key, fingerprint },
    }
}

fn fingerprint_of_trusted(trusted: &str) -> Option<String> {
    fingerprint(trusted)
}

/// Accepts only the trusted server key, remembering what was presented so a
/// rejection can be reported as "untrusted" or "changed".
pub struct ServerKeyCheck {
    pub(super) trusted: Option<String>,
    pub(super) presented: Arc<Mutex<Option<PublicKey>>>,
}

impl client::Handler for ServerKeyCheck {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        let key = match server_public_key {
            PublicKeyOrCertificate::PublicKey { key, .. } => key.clone(),
            PublicKeyOrCertificate::Certificate(cert) => {
                PublicKey::new(cert.public_key().clone(), "")
            }
        };
        let trusted = check_host_key(self.trusted.as_deref(), &key) == HostKeyCheck::Trusted;
        if !trusted {
            *self.presented.lock().expect("lock poisoned") = Some(key);
        }
        Ok(trusted)
    }
}

#[cfg(test)]
mod tests {
    use super::super::keys::generate_key;
    use super::*;

    #[test]
    fn host_keys_are_trusted_only_when_they_match() {
        let a = generate_key("a").public_key;
        let b = generate_key("b").public_key;
        let presented = PublicKey::from_openssh(&a).unwrap();
        assert_eq!(check_host_key(None, &presented), HostKeyCheck::Unknown);
        assert_eq!(check_host_key(Some(&a), &presented), HostKeyCheck::Trusted);
        assert_eq!(check_host_key(Some(&b), &presented), HostKeyCheck::Mismatch);
    }

    #[test]
    fn fingerprints_use_sha256() {
        let fp = fingerprint(&generate_key("a").public_key).unwrap();
        assert!(fp.starts_with("SHA256:"));
        assert_eq!(fingerprint("garbage"), None);
    }
}
