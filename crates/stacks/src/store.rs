//! Persistence for Managed Stacks; their environment is sealed.

use std::sync::Arc;

use homeport_storage::{Db, SecretBox};
use rusqlite::{OptionalExtension, Row};
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::definition;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ManagedStack {
    pub id: String,
    pub host_id: String,
    pub name: String,
    pub compose: String,
    pub has_env: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct StackStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

const COLUMNS: &str = "id, host_id, name, compose, env IS NOT NULL, created_at, updated_at";

impl StackStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    pub fn list(&self, host_id: &str) -> AppResult<Vec<ManagedStack>> {
        self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM stacks WHERE host_id = ?1 ORDER BY name"
            ))?
            .query_map([host_id], read_row)?
            .collect()
        })
    }

    pub fn get(&self, id: &str) -> AppResult<ManagedStack> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM stacks WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found("This stack no longer exists."))
    }

    pub fn create(
        &self,
        host_id: &str,
        name: &str,
        compose: &str,
        env: &str,
    ) -> AppResult<ManagedStack> {
        let name = name.trim();
        definition::validate_name(name).map_err(AppError::validation)?;
        let env = self.seal_env(env)?;
        let id = uuid::Uuid::new_v4().to_string();
        self.db
            .with(|c| {
                c.execute(
                    "INSERT INTO stacks (id, host_id, name, compose, env) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (&id, host_id, name, compose, &env),
                )
            })
            .map_err(|e| match e.code {
                ErrorCode::NameInUse => AppError {
                    message: format!("This host already has a stack named \"{name}\"."),
                    ..e
                },
                ErrorCode::NotFound => AppError {
                    message: "This host no longer exists.".into(),
                    ..e
                },
                _ => e,
            })?;
        self.get(&id)
    }

    /// Replaces the compose definition and environment.
    pub fn update(&self, id: &str, compose: &str, env: &str) -> AppResult<ManagedStack> {
        let env = self.seal_env(env)?;
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE stacks SET compose = ?2, env = ?3,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                (id, compose, &env),
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found("This stack no longer exists."));
        }
        self.get(id)
    }

    pub fn delete(&self, id: &str) -> AppResult<()> {
        self.db
            .with(|c| c.execute("DELETE FROM stacks WHERE id = ?1", [id]))?;
        Ok(())
    }

    /// The decrypted `.env` text, empty when there is none.
    pub fn env(&self, id: &str) -> AppResult<String> {
        let sealed: Option<Vec<u8>> = self
            .db
            .with(|c| {
                c.query_row("SELECT env FROM stacks WHERE id = ?1", [id], |r| r.get(0))
                    .optional()
            })?
            .ok_or_else(|| AppError::not_found("This stack no longer exists."))?;
        match sealed {
            Some(sealed) => Ok(String::from_utf8_lossy(&self.secrets.open(&sealed)?).into_owned()),
            None => Ok(String::new()),
        }
    }

    /// Empty environments need no keychain, so they are stored as NULL.
    fn seal_env(&self, env: &str) -> AppResult<Option<Vec<u8>>> {
        if env.trim().is_empty() {
            return Ok(None);
        }
        definition::parse_env(env).map_err(AppError::validation)?;
        Ok(Some(self.secrets.seal(env.as_bytes())?))
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<ManagedStack> {
    Ok(ManagedStack {
        id: row.get(0)?,
        host_id: row.get(1)?,
        name: row.get(2)?,
        compose: row.get(3)?,
        has_env: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use homeport_storage::KeyStore;

    use super::*;
    use homeport_hosts::{
        store::{HostDraft, HostStore},
        HostConnection,
    };

    const COMPOSE: &str = "services:\n  web:\n    image: nginx\n";

    fn setup(secrets: SecretBox) -> (StackStore, HostStore, String) {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let secrets = Arc::new(secrets);
        let hosts = HostStore::new(db.clone(), secrets.clone());
        let host = hosts
            .create(HostDraft {
                name: "local".into(),
                connection: HostConnection::Local {
                    socket_path: "/var/run/docker.sock".into(),
                },
                secrets: None,
            })
            .unwrap();
        (StackStore::new(db, secrets), hosts, host.id)
    }

    #[test]
    fn stacks_keep_their_definition_and_sealed_env() {
        let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
        // The secret must not be hex, or it could turn up inside the random id.
        let s = store
            .create(&host, "shop", COMPOSE, "TOKEN=hunter2\n")
            .unwrap();
        assert!(s.has_env);
        assert_eq!(store.env(&s.id).unwrap(), "TOKEN=hunter2\n");
        assert!(!serde_json::to_string(&s).unwrap().contains("hunter2"));

        let updated = store.update(&s.id, "services: {}\n", "").unwrap();
        assert!(!updated.has_env);
        assert_eq!(store.env(&s.id).unwrap(), "");
        assert_eq!(store.list(&host).unwrap(), [updated]);
    }

    #[test]
    fn names_are_unique_per_host_and_valid() {
        let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
        store.create(&host, "shop", COMPOSE, "").unwrap();
        assert!(matches!(
            store.create(&host, "shop", COMPOSE, ""),
            Err(ref e) if e.is(ErrorCode::NameInUse) && e.message.contains("\"shop\"")
        ));
        assert!(matches!(
            store.create(&host, "Shop!", COMPOSE, ""),
            Err(ref e) if e.is(ErrorCode::Validation)
        ));
        assert!(matches!(
            store.create("nope", "x", COMPOSE, ""),
            Err(ref e) if e.is(ErrorCode::NotFound)
        ));
    }

    #[test]
    fn deleting_a_host_deletes_its_stacks() {
        let (store, hosts, host) = setup(SecretBox::from_key(&[4; 32]));
        let s = store.create(&host, "shop", COMPOSE, "").unwrap();
        hosts.delete(&host).unwrap();
        assert!(matches!(store.get(&s.id), Err(ref e) if e.is(ErrorCode::NotFound)));
    }

    #[test]
    fn env_needs_the_keychain_but_empty_env_does_not() {
        struct NoKeychain;
        impl KeyStore for NoKeychain {
            fn load_or_create(&self) -> Result<homeport_storage::crypto::Key, String> {
                Err("none".into())
            }
        }
        let (store, _, host) = setup(SecretBox::from_store(&NoKeychain));
        assert!(store.create(&host, "plain", COMPOSE, "  \n").is_ok());
        assert!(matches!(
            store.create(&host, "secret", COMPOSE, "A=1"),
            Err(ref e) if e.is(ErrorCode::KeychainUnavailable)
        ));
    }
}
