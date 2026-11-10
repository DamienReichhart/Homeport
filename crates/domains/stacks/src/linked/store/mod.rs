//! Persistence for Linked Stacks: where their compose files are, never the
//! files themselves, and nothing secret.

use std::sync::Arc;

use rusqlite::{OptionalExtension, Row};
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_storage::Db;

use crate::{
    messages as msg,
    model::{definition, owner::StackOwner},
    store::name_taken,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LinkedStack {
    pub id: String,
    /// The Host or Swarm it's deployed to.
    pub owner: StackOwner,
    pub name: String,
    /// The project folder on this machine, where compose runs.
    pub working_dir: String,
    /// The compose files, as absolute paths, in the order compose merges them.
    pub config_files: Vec<String>,
    /// Of the files as Homeport last read or wrote them, to tell when they
    /// changed outside Homeport.
    pub fingerprint: String,
    /// The Compose profiles it deploys; none until the user turns one on.
    pub profiles: Vec<String>,
    /// On a Swarm: the docker CLI sends its own registry logins with a deploy.
    pub registry_auth: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct LinkedStore {
    db: Arc<Db>,
}

const COLUMNS: &str = "id, host_id, name, working_dir, config_files, fingerprint, created_at, \
     updated_at, profiles, swarm_id, registry_auth";

impl LinkedStore {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    /// The Linked Stacks of the Host `host_id`.
    pub fn list(&self, host_id: &str) -> AppResult<Vec<LinkedStack>> {
        self.list_of(&StackOwner::from(host_id))
    }

    /// The Linked Stacks of every Swarm.
    pub fn list_on_swarms(&self) -> AppResult<Vec<LinkedStack>> {
        self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM linked_stacks WHERE swarm_id IS NOT NULL ORDER BY swarm_id, name"
            ))?
            .query_map([], read_row)?
            .collect()
        })
    }

    /// The Linked Stacks of a Host or a Swarm.
    pub fn list_of(&self, owner: &StackOwner) -> AppResult<Vec<LinkedStack>> {
        self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM linked_stacks WHERE {} = ?1 ORDER BY name",
                owner.column()
            ))?
            .query_map([owner.id()], read_row)?
            .collect()
        })
    }

    /// Whether a swarm deploy sends the docker CLI's registry logins.
    pub fn set_registry_auth(&self, id: &str, on: bool) -> AppResult<LinkedStack> {
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE linked_stacks SET registry_auth = ?2 WHERE id = ?1",
                (id, on),
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    pub fn get(&self, id: &str) -> AppResult<LinkedStack> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM linked_stacks WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))
    }

    /// Records a Linked Stack. Its name must be free on the Host, among
    /// Managed and Linked Stacks alike.
    pub fn create(
        &self,
        owner: impl Into<StackOwner>,
        name: &str,
        working_dir: &str,
        config_files: &[String],
        fingerprint: &str,
    ) -> AppResult<LinkedStack> {
        let owner = owner.into();
        let name = name.trim();
        definition::validate_name(name)?;
        let id = uuid::Uuid::new_v4().to_string();
        let files = serde_json::to_string(config_files)?;
        let created = self
            .db
            .with(|c| {
                let tx = c.transaction()?;
                if name_taken(&tx, &owner, name)? {
                    return Ok(false);
                }
                tx.execute(
                    "INSERT INTO linked_stacks
                     (id, host_id, swarm_id, name, working_dir, config_files, fingerprint)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    (
                        &id,
                        owner.host_id(),
                        owner.swarm_id(),
                        name,
                        working_dir,
                        &files,
                        fingerprint,
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

    /// Notes the files as Homeport just read or wrote them.
    pub fn set_fingerprint(&self, id: &str, fingerprint: &str) -> AppResult<LinkedStack> {
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE linked_stacks SET fingerprint = ?2,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                (id, fingerprint),
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    /// Replaces the compose files the Linked Stack uses, in order, and notes
    /// them as just read.
    pub fn set_files(
        &self,
        id: &str,
        config_files: &[String],
        fingerprint: &str,
    ) -> AppResult<LinkedStack> {
        let files = serde_json::to_string(config_files)?;
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE linked_stacks SET config_files = ?2, fingerprint = ?3,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                (id, &files, fingerprint),
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    /// Sets the Compose profiles the Stack deploys.
    pub fn set_profiles(&self, id: &str, profiles: &[String]) -> AppResult<LinkedStack> {
        definition::validate_profiles(profiles)?;
        let json = serde_json::to_string(profiles)?;
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE linked_stacks SET profiles = ?2,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                (id, &json),
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    /// Forgets the Linked Stack. Its folder and containers are left alone.
    /// Its deploy history goes too.
    pub fn delete(&self, id: &str) -> AppResult<()> {
        self.db.with(|c| {
            let tx = c.transaction()?;
            tx.execute("DELETE FROM stack_deploys WHERE stack_id = ?1", [id])?;
            tx.execute("DELETE FROM linked_stacks WHERE id = ?1", [id])?;
            tx.commit()
        })?;
        Ok(())
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<LinkedStack> {
    let files: String = row.get(4)?;
    Ok(LinkedStack {
        id: row.get(0)?,
        owner: StackOwner::from_columns(row.get(1)?, row.get(9)?)?,
        name: row.get(2)?,
        working_dir: row.get(3)?,
        config_files: serde_json::from_str(&files).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, e.into())
        })?,
        fingerprint: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
        profiles: serde_json::from_str(&row.get::<_, String>(8)?).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, e.into())
        })?,
        registry_auth: row.get(10)?,
    })
}

#[cfg(test)]
mod tests;
