//! Persistence for Synced Stacks: their repository, ref and compose file
//! path, the commits fetched and deployed, and their sealed environment.

use std::sync::Arc;

use rusqlite::{OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_git::RefKind;
use homeport_storage::{Db, SecretBox};

use crate::{definition, messages as msg, store::name_taken};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SyncedStack {
    pub id: String,
    pub host_id: String,
    pub name: String,
    pub repository_id: String,
    pub ref_kind: RefKind,
    pub ref_name: String,
    /// The compose file, relative to the repository's root.
    pub compose_path: String,
    pub has_env: bool,
    pub profiles: Vec<String>,
    /// The commit of the last successful deploy.
    pub deployed_commit: Option<String>,
    /// The ref's commit as last fetched.
    pub fetched_commit: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// What a Synced Stack is created from.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SyncedDraft {
    pub name: String,
    pub repository_id: String,
    pub ref_kind: RefKind,
    pub ref_name: String,
    pub compose_path: String,
}

pub struct SyncedStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

const COLUMNS: &str = "id, host_id, name, repository_id, ref_kind, ref_name, compose_path, \
     env IS NOT NULL, profiles, deployed_commit, fetched_commit, created_at, updated_at";

impl SyncedStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    pub fn list(&self, host_id: &str) -> AppResult<Vec<SyncedStack>> {
        self.query("WHERE host_id = ?1 ORDER BY name", [host_id])
    }

    /// Every Synced Stack, on every Host.
    pub fn list_all(&self) -> AppResult<Vec<SyncedStack>> {
        self.query("ORDER BY host_id, name", [])
    }

    /// The Synced Stacks made from repository `repository_id`.
    pub fn using(&self, repository_id: &str) -> AppResult<Vec<SyncedStack>> {
        self.query("WHERE repository_id = ?1 ORDER BY name", [repository_id])
    }

    fn query(&self, clause: &str, params: impl rusqlite::Params) -> AppResult<Vec<SyncedStack>> {
        self.db.with(|c| {
            c.prepare(&format!("SELECT {COLUMNS} FROM synced_stacks {clause}"))?
                .query_map(params, read_row)?
                .collect()
        })
    }

    pub fn get(&self, id: &str) -> AppResult<SyncedStack> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM synced_stacks WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))
    }

    /// Records a Synced Stack whose ref was just fetched at `fetched_commit`.
    /// Its name must be free on the Host, among every kind of Stack.
    pub fn create(
        &self,
        host_id: &str,
        draft: &SyncedDraft,
        env: &str,
        fetched_commit: &str,
    ) -> AppResult<SyncedStack> {
        let name = draft.name.trim();
        definition::validate_name(name)?;
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
                    "INSERT INTO synced_stacks
                     (id, host_id, name, repository_id, ref_kind, ref_name, compose_path, env, fetched_commit)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    (
                        &id,
                        host_id,
                        name,
                        &draft.repository_id,
                        draft.ref_kind.text(),
                        draft.ref_name.trim(),
                        &draft.compose_path,
                        &env,
                        fetched_commit,
                    ),
                )?;
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

    /// Replaces the environment.
    pub fn set_env(&self, id: &str, env: &str) -> AppResult<SyncedStack> {
        let env = self.seal_env(env)?;
        self.update(id, "env = ?2", (id, &env))
    }

    /// Sets the Compose profiles the Stack deploys.
    pub fn set_profiles(&self, id: &str, profiles: &[String]) -> AppResult<SyncedStack> {
        definition::validate_profiles(profiles)?;
        let json = serde_json::to_string(profiles)?;
        self.update(id, "profiles = ?2", (id, &json))
    }

    /// Notes the ref's commit as just fetched.
    pub fn set_fetched(&self, id: &str, commit: &str) -> AppResult<SyncedStack> {
        self.update(id, "fetched_commit = ?2", (id, commit))
    }

    /// Notes the commit a deploy just succeeded with.
    pub fn set_deployed(&self, id: &str, commit: &str) -> AppResult<SyncedStack> {
        self.update(id, "deployed_commit = ?2", (id, commit))
    }

    fn update(&self, id: &str, set: &str, params: impl rusqlite::Params) -> AppResult<SyncedStack> {
        let n = self.db.with(|c| {
            c.execute(
                &format!(
                    "UPDATE synced_stacks SET {set},
                     updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1"
                ),
                params,
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    /// Forgets the Synced Stack and its deploy history.
    pub fn delete(&self, id: &str) -> AppResult<()> {
        self.db.with(|c| {
            let tx = c.transaction()?;
            tx.execute("DELETE FROM stack_deploys WHERE stack_id = ?1", [id])?;
            tx.execute("DELETE FROM synced_stacks WHERE id = ?1", [id])?;
            tx.commit()
        })?;
        Ok(())
    }

    /// The decrypted `.env` text, empty when there is none.
    pub fn env(&self, id: &str) -> AppResult<String> {
        let sealed: Option<Vec<u8>> = self
            .db
            .with(|c| {
                c.query_row("SELECT env FROM synced_stacks WHERE id = ?1", [id], |r| {
                    r.get(0)
                })
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

fn read_row(row: &Row<'_>) -> rusqlite::Result<SyncedStack> {
    Ok(SyncedStack {
        id: row.get(0)?,
        host_id: row.get(1)?,
        name: row.get(2)?,
        repository_id: row.get(3)?,
        ref_kind: RefKind::from_text(&row.get::<_, String>(4)?),
        ref_name: row.get(5)?,
        compose_path: row.get(6)?,
        has_env: row.get(7)?,
        profiles: serde_json::from_str(&row.get::<_, String>(8)?).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, e.into())
        })?,
        deployed_commit: row.get(9)?,
        fetched_commit: row.get(10)?,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}
