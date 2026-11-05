//! Persistence for Managed Stacks; their environment is sealed.

use std::sync::Arc;

use rusqlite::{Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;
use homeport_storage::{Db, SecretBox};

use crate::definition;

/// One compose file of a Managed Stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ComposeFile {
    /// A file name such as `compose.prod.yaml`, unique in the Stack.
    pub name: String,
    pub content: String,
}

impl ComposeFile {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ManagedStack {
    pub id: String,
    pub host_id: String,
    pub name: String,
    /// Its compose files, merged in this order; never empty.
    pub files: Vec<ComposeFile>,
    pub has_env: bool,
    /// The Compose profiles it deploys, besides its services without one.
    pub profiles: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct StackStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

const COLUMNS: &str = "id, host_id, name, env IS NOT NULL, created_at, updated_at, profiles";

impl StackStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    pub fn list(&self, host_id: &str) -> AppResult<Vec<ManagedStack>> {
        self.db.with(|c| {
            let stacks = c
                .prepare(&format!(
                    "SELECT {COLUMNS} FROM stacks WHERE host_id = ?1 ORDER BY name"
                ))?
                .query_map([host_id], read_row)?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            stacks.into_iter().map(|s| with_files(c, s)).collect()
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
                .optional()?
                .map(|s| with_files(c, s))
                .transpose()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))
    }

    pub fn create(
        &self,
        host_id: &str,
        name: &str,
        files: &[ComposeFile],
        env: &str,
    ) -> AppResult<ManagedStack> {
        let name = name.trim();
        definition::validate_name(name)?;
        definition::validate_files(files)?;
        let env = self.seal_env(env)?;
        let id = uuid::Uuid::new_v4().to_string();
        let created = self
            .db
            .with(|c| {
                let tx = c.transaction()?;
                if name_taken(&tx, host_id, name)? {
                    return Ok(false);
                }
                tx.execute(
                    "INSERT INTO stacks (id, host_id, name, env) VALUES (?1, ?2, ?3, ?4)",
                    (&id, host_id, name, &env),
                )?;
                write_files(&tx, &id, files)?;
                tx.commit()?;
                Ok(true)
            })
            .map_err(|e| match e.code {
                ErrorCode::NameInUse => e.with_message(msg::NAME_IN_USE.with("name", name)),
                ErrorCode::NotFound => e.with_message(msg::HOST_GONE),
                _ => e,
            })?;
        if !created {
            return Err(AppError::new(
                ErrorCode::NameInUse,
                msg::NAME_IN_USE.with("name", name),
            ));
        }
        self.get(&id)
    }

    /// Replaces the compose files and environment.
    pub fn update(&self, id: &str, files: &[ComposeFile], env: &str) -> AppResult<ManagedStack> {
        definition::validate_files(files)?;
        let env = self.seal_env(env)?;
        let n = self.db.with(|c| {
            let tx = c.transaction()?;
            let n = tx.execute(
                "UPDATE stacks SET env = ?2,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                (id, &env),
            )?;
            if n > 0 {
                tx.execute("DELETE FROM stack_files WHERE stack_id = ?1", [id])?;
                write_files(&tx, id, files)?;
            }
            tx.commit()?;
            Ok(n)
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    /// Sets the Compose profiles the Stack deploys.
    pub fn set_profiles(&self, id: &str, profiles: &[String]) -> AppResult<ManagedStack> {
        definition::validate_profiles(profiles)?;
        let json = serde_json::to_string(profiles)?;
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE stacks SET profiles = ?2,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                (id, &json),
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    /// Forgets the Stack's definition and its deploy history.
    pub fn delete(&self, id: &str) -> AppResult<()> {
        self.db.with(|c| {
            let tx = c.transaction()?;
            tx.execute("DELETE FROM stack_deploys WHERE stack_id = ?1", [id])?;
            tx.execute("DELETE FROM stacks WHERE id = ?1", [id])?;
            tx.commit()
        })?;
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
            .ok_or_else(|| AppError::not_found(msg::GONE))?;
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
        definition::parse_env(env)?;
        Ok(Some(self.secrets.seal(env.as_bytes())?))
    }
}

fn write_files(c: &Connection, id: &str, files: &[ComposeFile]) -> rusqlite::Result<()> {
    for (position, file) in files.iter().enumerate() {
        c.execute(
            "INSERT INTO stack_files (stack_id, position, name, content) VALUES (?1, ?2, ?3, ?4)",
            (id, position as i64, &file.name, &file.content),
        )?;
    }
    Ok(())
}

fn with_files(c: &Connection, mut stack: ManagedStack) -> rusqlite::Result<ManagedStack> {
    stack.files = c
        .prepare("SELECT name, content FROM stack_files WHERE stack_id = ?1 ORDER BY position")?
        .query_map([&stack.id], |r| {
            Ok(ComposeFile {
                name: r.get(0)?,
                content: r.get(1)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(stack)
}

/// Whether the Host already has a Managed, Linked or Synced Stack named `name`.
pub(crate) fn name_taken(c: &Connection, host_id: &str, name: &str) -> rusqlite::Result<bool> {
    c.query_row(
        "SELECT EXISTS (SELECT 1 FROM stacks WHERE host_id = ?1 AND name = ?2)
             OR EXISTS (SELECT 1 FROM linked_stacks WHERE host_id = ?1 AND name = ?2)
             OR EXISTS (SELECT 1 FROM synced_stacks WHERE host_id = ?1 AND name = ?2)",
        (host_id, name),
        |r| r.get(0),
    )
}

/// A Stack's row; its files are read separately.
fn read_row(row: &Row<'_>) -> rusqlite::Result<ManagedStack> {
    Ok(ManagedStack {
        id: row.get(0)?,
        host_id: row.get(1)?,
        name: row.get(2)?,
        files: vec![],
        has_env: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
        profiles: serde_json::from_str(&row.get::<_, String>(6)?).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, e.into())
        })?,
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

    fn one(content: &str) -> Vec<ComposeFile> {
        vec![ComposeFile::new("compose.yaml", content)]
    }

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
            .create(&host, "shop", &one(COMPOSE), "TOKEN=hunter2\n")
            .unwrap();
        assert!(s.has_env);
        assert_eq!(store.env(&s.id).unwrap(), "TOKEN=hunter2\n");
        assert!(!serde_json::to_string(&s).unwrap().contains("hunter2"));

        let updated = store.update(&s.id, &one("services: {}\n"), "").unwrap();
        assert!(!updated.has_env);
        assert_eq!(store.env(&s.id).unwrap(), "");
        assert_eq!(store.list(&host).unwrap(), [updated]);
    }

    #[test]
    fn names_are_unique_per_host_and_valid() {
        let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
        store.create(&host, "shop", &one(COMPOSE), "").unwrap();
        assert!(matches!(
            store.create(&host, "shop", &one(COMPOSE), ""),
            Err(ref e) if e.is(ErrorCode::NameInUse) && e.message.contains("\"shop\"")
        ));
        assert!(matches!(
            store.create(&host, "Shop!", &one(COMPOSE), ""),
            Err(ref e) if e.is(ErrorCode::Validation)
        ));
        assert!(matches!(
            store.create("nope", "x", &one(COMPOSE), ""),
            Err(ref e) if e.is(ErrorCode::NotFound)
        ));
    }

    #[test]
    fn stacks_keep_several_files_in_order() {
        let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
        let files = vec![
            ComposeFile::new("compose.yaml", COMPOSE),
            ComposeFile::new(
                "compose.prod.yaml",
                "services:\n  web:\n    ports: [\"80:80\"]\n",
            ),
        ];
        let s = store.create(&host, "shop", &files, "").unwrap();
        assert_eq!(s.files, files);
        let reversed: Vec<_> = files.iter().rev().cloned().collect();
        assert_eq!(store.update(&s.id, &reversed, "").unwrap().files, reversed);
        assert!(matches!(
            store.update(&s.id, &[], ""),
            Err(ref e) if e.is(ErrorCode::Validation)
        ));
        assert_eq!(
            store.get(&s.id).unwrap().files,
            reversed,
            "a refused update changes nothing"
        );
    }

    #[test]
    fn stacks_keep_their_profiles() {
        let (store, _, host) = setup(SecretBox::from_key(&[4; 32]));
        let s = store.create(&host, "shop", &one(COMPOSE), "").unwrap();
        assert!(s.profiles.is_empty());
        let s = store.set_profiles(&s.id, &["debug".into()]).unwrap();
        assert_eq!(s.profiles, ["debug"]);
        assert!(store.set_profiles(&s.id, &["no good".into()]).is_err());
    }

    #[test]
    fn deleting_a_host_deletes_its_stacks() {
        let (store, hosts, host) = setup(SecretBox::from_key(&[4; 32]));
        let s = store.create(&host, "shop", &one(COMPOSE), "").unwrap();
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
        assert!(store.create(&host, "plain", &one(COMPOSE), "  \n").is_ok());
        assert!(matches!(
            store.create(&host, "secret", &one(COMPOSE), "A=1"),
            Err(ref e) if e.is(ErrorCode::KeychainUnavailable)
        ));
    }
}
