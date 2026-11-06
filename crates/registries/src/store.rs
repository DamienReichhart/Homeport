//! Persistence for registry credentials. The password or token is sealed
//! before it reaches SQLite and never leaves this crate except as
//! [`Credentials`], which the shell hands straight to the Engine.

use std::{fmt, sync::Arc};

use rusqlite::{OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;
use homeport_storage::{Db, SecretBox};

use crate::address;

pub mod backup;

/// A stored login, as the frontend sees it: never the secret.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    pub id: String,
    /// Normalised, e.g. `docker.io`, `ghcr.io`, `registry.local:5000`.
    pub address: String,
    pub username: String,
    pub created_at: String,
    pub updated_at: String,
}

/// What the user typed to add or change a login. `secret: None` keeps the
/// stored one when editing.
#[derive(Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RegistryDraft {
    pub address: String,
    pub username: String,
    pub secret: Option<String>,
}

impl fmt::Debug for RegistryDraft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistryDraft")
            .field("address", &self.address)
            .field("username", &self.username)
            .field("secret", &self.secret.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

/// A login with its secret opened, for the Engine or a login check. Its
/// `Debug` hides the secret, so it can't reach a log by accident.
#[derive(Clone, PartialEq, Eq)]
pub struct Credentials {
    pub address: String,
    pub username: String,
    pub secret: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("address", &self.address)
            .field("username", &self.username)
            .field("secret", &"<redacted>")
            .finish()
    }
}

const COLUMNS: &str = "id, address, username, created_at, updated_at";

pub struct RegistryStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

impl RegistryStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    pub fn list(&self) -> AppResult<Vec<Registry>> {
        self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM registries ORDER BY address"
            ))?
            .query_map([], read_row)?
            .collect()
        })
    }

    pub fn get(&self, id: &str) -> AppResult<Registry> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM registries WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(gone)
    }

    /// Adds a login (`id: None`) or replaces one. One login per address.
    pub fn save(&self, id: Option<&str>, draft: RegistryDraft) -> AppResult<Registry> {
        let (address, username) = validated(&draft)?;
        let secret = draft.secret.filter(|s| !s.is_empty());
        let result = match id {
            None => {
                let secret = secret.ok_or_else(|| AppError::validation(msg::SECRET_EMPTY))?;
                let sealed = self.secrets.seal(secret.as_bytes())?;
                let id = uuid::Uuid::new_v4().to_string();
                self.db
                    .with(|c| {
                        c.execute(
                            "INSERT INTO registries (id, address, username, secret)
                             VALUES (?1, ?2, ?3, ?4)",
                            (&id, &address, &username, &sealed),
                        )
                    })
                    .map(|_| id)
            }
            Some(id) => {
                let sealed = match secret {
                    Some(secret) => Some(self.secrets.seal(secret.as_bytes())?),
                    None => None,
                };
                self.db
                    .with(|c| {
                        c.execute(
                            "UPDATE registries SET address = ?2, username = ?3,
                                 secret = COALESCE(?4, secret),
                                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                             WHERE id = ?1",
                            (id, &address, &username, &sealed),
                        )
                    })
                    .and_then(|n| {
                        if n == 0 {
                            Err(gone())
                        } else {
                            Ok(id.to_owned())
                        }
                    })
            }
        };
        let id = result.map_err(|e| match e.code {
            ErrorCode::NameInUse => e
                .with_message(msg::LOGIN_EXISTS.with("address", &address))
                .with_hint(msg::LOGIN_EXISTS_HINT),
            _ => e,
        })?;
        self.get(&id)
    }

    pub fn remove(&self, id: &str) -> AppResult<()> {
        let n = self
            .db
            .with(|c| c.execute("DELETE FROM registries WHERE id = ?1", [id]))?;
        if n == 0 {
            return Err(gone());
        }
        Ok(())
    }

    /// The credentials a draft stands for, for checking them before saving:
    /// without a new secret, the one stored under `id` is used.
    pub fn resolve(&self, id: Option<&str>, draft: RegistryDraft) -> AppResult<Credentials> {
        let (address, username) = validated(&draft)?;
        let secret = match (draft.secret.filter(|s| !s.is_empty()), id) {
            (Some(secret), _) => secret,
            (None, Some(id)) => {
                let sealed: Vec<u8> = self
                    .db
                    .with(|c| {
                        c.query_row("SELECT secret FROM registries WHERE id = ?1", [id], |r| {
                            r.get(0)
                        })
                        .optional()
                    })?
                    .ok_or_else(gone)?;
                self.open(address.clone(), username.clone(), &sealed)?
                    .secret
            }
            (None, None) => return Err(AppError::validation(msg::SECRET_EMPTY)),
        };
        Ok(Credentials {
            address,
            username,
            secret,
        })
    }

    /// The login stored for a registry address, secret opened.
    pub fn credentials(&self, address: &str) -> AppResult<Option<Credentials>> {
        let Some(address) = address::normalise(address) else {
            return Ok(None);
        };
        let row = self.db.with(|c| {
            c.query_row(
                "SELECT username, secret FROM registries WHERE address = ?1",
                [&address],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?)),
            )
            .optional()
        })?;
        row.map(|(username, sealed)| self.open(address, username, &sealed))
            .transpose()
    }

    /// The login for the registry an image reference is pulled from or pushed to.
    pub fn credentials_for(&self, reference: &str) -> AppResult<Option<Credentials>> {
        self.credentials(&address::registry_of(reference))
    }

    /// The stored login an image reference would use, without its secret.
    pub fn login_for(&self, reference: &str) -> AppResult<Option<Registry>> {
        let address = address::registry_of(reference);
        self.db.with(|c| {
            c.query_row(
                &format!("SELECT {COLUMNS} FROM registries WHERE address = ?1"),
                [&address],
                read_row,
            )
            .optional()
        })
    }

    /// Every login, secrets opened, for builds whose base images may come
    /// from any registry. Logins that can't be opened are left out and logged.
    pub fn all_credentials(&self) -> AppResult<Vec<Credentials>> {
        let rows = self.db.with(|c| {
            c.prepare("SELECT address, username, secret FROM registries ORDER BY address")?
                .query_map([], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, Vec<u8>>(2)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()
        })?;
        Ok(rows
            .into_iter()
            .filter_map(|(address, username, sealed)| {
                self.open(address.clone(), username, &sealed)
                    .map_err(|e| e.logged(&format!("registry login for {address}")))
                    .ok()
            })
            .collect())
    }

    fn open(&self, address: String, username: String, sealed: &[u8]) -> AppResult<Credentials> {
        let secret = String::from_utf8(self.secrets.open(sealed)?)
            .map_err(|_| AppError::credentials_lost())?;
        Ok(Credentials {
            address,
            username,
            secret,
        })
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<Registry> {
    Ok(Registry {
        id: row.get(0)?,
        address: row.get(1)?,
        username: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
    })
}

fn validated(draft: &RegistryDraft) -> AppResult<(String, String)> {
    let address = address::normalise(&draft.address)
        .ok_or_else(|| AppError::validation(msg::ADDRESS_INVALID))?;
    let username = draft.username.trim();
    if username.is_empty() {
        return Err(AppError::validation(msg::USERNAME_EMPTY));
    }
    Ok((address, username.to_owned()))
}

fn gone() -> AppError {
    AppError::not_found(msg::GONE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> RegistryStore {
        RegistryStore::new(
            Arc::new(Db::open_in_memory().unwrap()),
            Arc::new(SecretBox::from_key(&[3; 32])),
        )
    }

    fn draft(address: &str, username: &str, secret: Option<&str>) -> RegistryDraft {
        RegistryDraft {
            address: address.into(),
            username: username.into(),
            secret: secret.map(Into::into),
        }
    }

    #[test]
    fn logins_are_stored_with_their_secret_sealed() {
        let store = store();
        let saved = store
            .save(None, draft("https://GHCR.io/", "alice", Some("ghp_token")))
            .unwrap();
        assert_eq!(saved.address, "ghcr.io");
        assert_eq!(store.list().unwrap(), [saved]);

        let raw: Vec<u8> = store
            .db
            .with(|c| c.query_row("SELECT secret FROM registries", [], |r| r.get(0)))
            .unwrap();
        assert!(!raw.windows(9).any(|w| w == b"ghp_token"));

        let creds = store.credentials_for("ghcr.io/org/app:1").unwrap().unwrap();
        assert_eq!(creds.username, "alice");
        assert_eq!(creds.secret, "ghp_token");
        assert!(!format!("{creds:?}").contains("ghp_token"));
    }

    #[test]
    fn references_without_a_registry_use_the_docker_hub_login() {
        let store = store();
        store
            .save(None, draft("index.docker.io", "bob", Some("pw")))
            .unwrap();
        assert_eq!(
            store
                .credentials_for("nginx:1.27")
                .unwrap()
                .unwrap()
                .address,
            DOCKER_HUB
        );
        assert_eq!(
            store.login_for("bitnami/redis").unwrap().unwrap().username,
            "bob"
        );
        assert!(store.credentials_for("ghcr.io/x/y").unwrap().is_none());
    }

    #[test]
    fn editing_keeps_the_secret_unless_a_new_one_is_given() {
        let store = store();
        let saved = store
            .save(None, draft("ghcr.io", "alice", Some("one")))
            .unwrap();
        store
            .save(Some(&saved.id), draft("ghcr.io", "alice2", None))
            .unwrap();
        let creds = store.credentials("ghcr.io").unwrap().unwrap();
        assert_eq!(
            (creds.username.as_str(), creds.secret.as_str()),
            ("alice2", "one")
        );

        store
            .save(Some(&saved.id), draft("ghcr.io", "alice2", Some("two")))
            .unwrap();
        assert_eq!(store.credentials("ghcr.io").unwrap().unwrap().secret, "two");
    }

    #[test]
    fn drafts_are_checked_with_the_stored_secret_when_none_is_typed() {
        let store = store();
        let saved = store
            .save(None, draft("ghcr.io", "alice", Some("stored")))
            .unwrap();
        let creds = store
            .resolve(Some(&saved.id), draft("https://ghcr.io", "alice", None))
            .unwrap();
        assert_eq!(
            (creds.address.as_str(), creds.secret.as_str()),
            ("ghcr.io", "stored")
        );
        let typed = store
            .resolve(Some(&saved.id), draft("ghcr.io", "alice", Some("new")))
            .unwrap();
        assert_eq!(typed.secret, "new");
        assert!(store
            .resolve(None, draft("ghcr.io", "alice", None))
            .unwrap_err()
            .is(ErrorCode::Validation));
    }

    #[test]
    fn one_login_per_address() {
        let store = store();
        store
            .save(None, draft("docker.io", "a", Some("x")))
            .unwrap();
        let err = store
            .save(None, draft("https://index.docker.io/v1/", "b", Some("y")))
            .unwrap_err();
        assert!(err.is(ErrorCode::NameInUse));
        assert!(err.message.contains("docker.io"));
    }

    #[test]
    fn new_logins_need_every_field() {
        let store = store();
        for (d, field) in [
            (draft("", "a", Some("x")), "address"),
            (draft("ghcr.io", " ", Some("x")), "user name"),
            (draft("ghcr.io", "a", None), "password"),
            (draft("ghcr.io", "a", Some("")), "password"),
        ] {
            let err = store.save(None, d).unwrap_err();
            assert!(err.is(ErrorCode::Validation));
            assert!(err.message.contains(field), "{}", err.message);
        }
    }

    #[test]
    fn removed_logins_are_gone() {
        let store = store();
        let saved = store.save(None, draft("ghcr.io", "a", Some("x"))).unwrap();
        store.remove(&saved.id).unwrap();
        assert!(store.list().unwrap().is_empty());
        assert!(store.remove(&saved.id).unwrap_err().is(ErrorCode::NotFound));
        assert!(store
            .save(Some(&saved.id), draft("ghcr.io", "a", None))
            .unwrap_err()
            .is(ErrorCode::NotFound));
    }

    #[test]
    fn every_login_can_be_opened_for_builds() {
        let store = store();
        store.save(None, draft("ghcr.io", "a", Some("x"))).unwrap();
        store
            .save(None, draft("docker.io", "b", Some("y")))
            .unwrap();
        let all = store.all_credentials().unwrap();
        assert_eq!(
            all.iter().map(|c| c.address.as_str()).collect::<Vec<_>>(),
            ["docker.io", "ghcr.io"]
        );
    }

    #[test]
    fn without_a_keychain_logins_cant_be_stored() {
        struct NoKeychain;
        impl homeport_storage::KeyStore for NoKeychain {
            fn load_or_create(&self) -> Result<homeport_storage::crypto::Key, String> {
                Err("no secret service".into())
            }
        }
        let store = RegistryStore::new(
            Arc::new(Db::open_in_memory().unwrap()),
            Arc::new(SecretBox::from_store(&NoKeychain)),
        );
        let err = store
            .save(None, draft("ghcr.io", "a", Some("x")))
            .unwrap_err();
        assert!(err.is(ErrorCode::KeychainUnavailable));
    }

    use crate::DOCKER_HUB;
}
