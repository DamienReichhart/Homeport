//! Homeport's own SSH keys: generating them, reading imported ones, and what
//! the user runs on the server to authorise them.

use russh::keys::{
    decode_secret_key,
    ssh_key::{
        private::{Ed25519Keypair, KeypairData},
        LineEnding,
    },
    HashAlg, PrivateKey, PublicKey,
};

use homeport_core::error::{AppError, ErrorCode};

use crate::{messages as msg, ConnectError};

/// A freshly generated Ed25519 keypair in OpenSSH formats.
pub struct GeneratedKey {
    pub private_key: String,
    pub public_key: String,
}

/// Generates the per-Host keypair. The comment is reduced to characters that
/// are safe inside the single-quoted `authorized_keys` command.
pub fn generate_key(comment: &str) -> GeneratedKey {
    let mut seed = [0u8; 32];
    getrandom::fill(&mut seed).expect("OS random number generator failed");
    let comment: String = comment
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || "@._-".contains(c) {
                c
            } else {
                '-'
            }
        })
        .collect();
    let key = PrivateKey::new(
        KeypairData::Ed25519(Ed25519Keypair::from_seed(&seed)),
        comment,
    )
    .expect("an Ed25519 keypair is always valid");
    GeneratedKey {
        private_key: key
            .to_openssh(LineEnding::LF)
            .expect("Ed25519 keys always encode")
            .to_string(),
        public_key: key
            .public_key()
            .to_openssh()
            .expect("Ed25519 keys always encode"),
    }
}

/// Whether a private key can only be read with its passphrase.
pub fn needs_passphrase(private_key: &str) -> bool {
    let text = private_key.trim_start();
    matches!(
        decode_secret_key(private_key, None),
        Err(russh::keys::Error::KeyIsEncrypted)
    ) || text.starts_with("-----BEGIN ENCRYPTED PRIVATE KEY-----")
        || (text.starts_with("PuTTY-User-Key-File-") && !text.contains("Encryption: none"))
}

/// Reads a user-supplied private key and returns its public half in OpenSSH format.
pub fn public_key_of(private_key: &str, passphrase: Option<&str>) -> Result<String, ConnectError> {
    let key = decode_secret_key(private_key, passphrase).map_err(key_unreadable)?;
    key.public_key().to_openssh().map_err(|e| {
        ConnectError::Invalid(
            AppError::new(ErrorCode::SshKeyInvalid, msg::SSH_PUBLIC_KEY_UNENCODABLE).with_cause(&e),
        )
    })
}

/// The one-liner a user pastes on the server to authorize `public_key`.
pub fn authorized_keys_command(public_key: &str) -> String {
    format!(
        "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo '{}' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys",
        public_key.trim()
    )
}

/// `SHA256:…` fingerprint of an OpenSSH public key, as `ssh-keygen -l` shows it.
pub fn fingerprint(openssh_public_key: &str) -> Option<String> {
    PublicKey::from_openssh(openssh_public_key)
        .ok()
        .map(|k| k.fingerprint(HashAlg::Sha256).to_string())
}

pub(super) fn key_unreadable(e: russh::keys::Error) -> ConnectError {
    ConnectError::Invalid(
        AppError::new(ErrorCode::SshKeyInvalid, msg::SSH_KEY_UNREADABLE)
            .with_hint(msg::SSH_KEY_UNREADABLE_HINT)
            .with_cause(&e),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_keys_are_ed25519_with_a_safe_comment() {
        let key = generate_key("homeport@my laptop-'prod'");
        assert!(key.public_key.starts_with("ssh-ed25519 "));
        assert!(key.public_key.ends_with(" homeport@my-laptop--prod-"));
        assert!(key.private_key.contains("BEGIN OPENSSH PRIVATE KEY"));
        assert_eq!(
            public_key_of(&key.private_key, None).unwrap(),
            key.public_key
        );
    }

    #[test]
    fn encrypted_keys_need_their_passphrase() {
        // A throwaway key made for this test; its passphrase is "secret".
        let encrypted = include_str!("testdata/encrypted_ed25519");
        assert!(needs_passphrase(encrypted));
        assert!(public_key_of(encrypted, Some("secret")).is_ok());
        assert!(!needs_passphrase(&generate_key("plain").private_key));
        assert!(!needs_passphrase("garbage"));
    }

    #[test]
    fn each_generated_key_is_unique() {
        assert_ne!(generate_key("a").public_key, generate_key("a").public_key);
    }

    #[test]
    fn authorized_keys_command_appends_the_key_with_safe_permissions() {
        assert_eq!(
            authorized_keys_command("ssh-ed25519 AAAAC3Nza homeport@laptop\n"),
            "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo 'ssh-ed25519 AAAAC3Nza homeport@laptop' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys"
        );
    }

    #[test]
    fn unreadable_private_keys_are_rejected() {
        assert!(matches!(
            public_key_of("not a key", None),
            Err(ConnectError::Invalid(_))
        ));
    }
}
