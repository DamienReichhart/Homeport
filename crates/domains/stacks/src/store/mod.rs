//! Persistence for Managed Stacks; their environment is sealed.

use std::sync::Arc;

use rusqlite::{Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;
use homeport_storage::{Db, SecretBox};

use crate::model::{definition, owner::StackOwner};

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
    /// The Host or Swarm it's deployed to.
    pub owner: StackOwner,
    pub name: String,
    /// Its compose files, merged in this order; never empty.
    pub files: Vec<ComposeFile>,
    pub has_env: bool,
    /// The Compose profiles it deploys, besides its services without one.
    pub profiles: Vec<String>,
    /// On a Swarm: the docker CLI sends its own registry logins with a deploy.
    pub registry_auth: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct StackStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

const COLUMNS: &str =
    "id, host_id, name, env IS NOT NULL, created_at, updated_at, profiles, swarm_id, registry_auth";

impl StackStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    /// The Managed Stacks of the Host `host_id`.
    pub fn list(&self, host_id: &str) -> AppResult<Vec<ManagedStack>> {
        self.list_of(&StackOwner::from(host_id))
    }

    /// The Managed Stacks of every Swarm.
    pub fn list_on_swarms(&self) -> AppResult<Vec<ManagedStack>> {
        self.db.with(|c| {
            let stacks = c
                .prepare(&format!(
                    "SELECT {COLUMNS} FROM stacks WHERE swarm_id IS NOT NULL ORDER BY swarm_id, name"
                ))?
                .query_map([], read_row)?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            stacks.into_iter().map(|s| with_files(c, s)).collect()
        })
    }

    /// The Managed Stacks of a Host or a Swarm.
    pub fn list_of(&self, owner: &StackOwner) -> AppResult<Vec<ManagedStack>> {
        self.db.with(|c| {
            let stacks = c
                .prepare(&format!(
                    "SELECT {COLUMNS} FROM stacks WHERE {} = ?1 ORDER BY name",
                    owner.column()
                ))?
                .query_map([owner.id()], read_row)?
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

    /// Records a Managed Stack of a Host or a Swarm (`owner`).
    pub fn create(
        &self,
        owner: impl Into<StackOwner>,
        name: &str,
        files: &[ComposeFile],
        env: &str,
    ) -> AppResult<ManagedStack> {
        let owner = owner.into();
        let name = name.trim();
        definition::validate_name(name)?;
        definition::validate_files(files)?;
        let env = self.seal_env(env)?;
        let id = uuid::Uuid::new_v4().to_string();
        let created = self
            .db
            .with(|c| {
                let tx = c.transaction()?;
                if name_taken(&tx, &owner, name)? {
                    return Ok(false);
                }
                tx.execute(
                    "INSERT INTO stacks (id, host_id, swarm_id, name, env) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (&id, owner.host_id(), owner.swarm_id(), name, &env),
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

    /// Whether a swarm deploy sends the docker CLI's registry logins.
    pub fn set_registry_auth(&self, id: &str, on: bool) -> AppResult<ManagedStack> {
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE stacks SET registry_auth = ?2 WHERE id = ?1",
                (id, on),
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

/// Whether the Host or Swarm already has a Managed, Linked or Synced Stack named `name`.
pub(crate) fn name_taken(c: &Connection, owner: &StackOwner, name: &str) -> rusqlite::Result<bool> {
    let column = owner.column();
    c.query_row(
        &format!(
            "SELECT EXISTS (SELECT 1 FROM stacks WHERE {column} = ?1 AND name = ?2)
                 OR EXISTS (SELECT 1 FROM linked_stacks WHERE {column} = ?1 AND name = ?2)
                 OR EXISTS (SELECT 1 FROM synced_stacks WHERE {column} = ?1 AND name = ?2)"
        ),
        (owner.id(), name),
        |r| r.get(0),
    )
}

/// A Stack's row; its files are read separately.
fn read_row(row: &Row<'_>) -> rusqlite::Result<ManagedStack> {
    Ok(ManagedStack {
        id: row.get(0)?,
        owner: StackOwner::from_columns(row.get(1)?, row.get(7)?)?,
        name: row.get(2)?,
        files: vec![],
        has_env: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
        profiles: serde_json::from_str(&row.get::<_, String>(6)?).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, e.into())
        })?,
        registry_auth: row.get(8)?,
    })
}

pub mod backup;
pub mod history;
#[cfg(test)]
mod tests;
