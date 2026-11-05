//! Persistence for Linked Stacks: where their compose files are, never the
//! files themselves, and nothing secret.

use std::sync::Arc;

use rusqlite::{OptionalExtension, Row};
use serde::Serialize;

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_storage::Db;

use crate::{definition, messages as msg, store::name_taken};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LinkedStack {
    pub id: String,
    pub host_id: String,
    pub name: String,
    /// The project folder on this machine, where compose runs.
    pub working_dir: String,
    /// The compose files, as absolute paths, in the order compose merges them.
    pub config_files: Vec<String>,
    /// Of the files as Homeport last read or wrote them, to tell when they
    /// changed outside Homeport.
    pub fingerprint: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct LinkedStore {
    db: Arc<Db>,
}

const COLUMNS: &str =
    "id, host_id, name, working_dir, config_files, fingerprint, created_at, updated_at";

impl LinkedStore {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn list(&self, host_id: &str) -> AppResult<Vec<LinkedStack>> {
        self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM linked_stacks WHERE host_id = ?1 ORDER BY name"
            ))?
            .query_map([host_id], read_row)?
            .collect()
        })
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
        host_id: &str,
        name: &str,
        working_dir: &str,
        config_files: &[String],
        fingerprint: &str,
    ) -> AppResult<LinkedStack> {
        let name = name.trim();
        definition::validate_name(name)?;
        let id = uuid::Uuid::new_v4().to_string();
        let files = serde_json::to_string(config_files)?;
        let created = self
            .db
            .with(|c| {
                let tx = c.transaction()?;
                if name_taken(&tx, host_id, name)? {
                    return Ok(false);
                }
                tx.execute(
                    "INSERT INTO linked_stacks (id, host_id, name, working_dir, config_files, fingerprint)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    (&id, host_id, name, working_dir, &files, fingerprint),
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

    /// Forgets the Linked Stack. Its folder and containers are left alone.
    pub fn delete(&self, id: &str) -> AppResult<()> {
        self.db
            .with(|c| c.execute("DELETE FROM linked_stacks WHERE id = ?1", [id]))?;
        Ok(())
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<LinkedStack> {
    let files: String = row.get(4)?;
    Ok(LinkedStack {
        id: row.get(0)?,
        host_id: row.get(1)?,
        name: row.get(2)?,
        working_dir: row.get(3)?,
        config_files: serde_json::from_str(&files).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, e.into())
        })?,
        fingerprint: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

#[cfg(test)]
mod tests {
    use homeport_hosts::{
        store::{HostDraft, HostStore},
        HostConnection,
    };
    use homeport_storage::SecretBox;

    use super::*;
    use crate::store::StackStore;

    fn setup() -> (LinkedStore, StackStore, HostStore, String) {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let secrets = Arc::new(SecretBox::from_key(&[4; 32]));
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
        (
            LinkedStore::new(db.clone()),
            StackStore::new(db, secrets),
            hosts,
            host.id,
        )
    }

    fn files() -> Vec<String> {
        vec![
            "/srv/shop/compose.yaml".into(),
            "/srv/shop/compose.override.yaml".into(),
        ]
    }

    #[test]
    fn linked_stacks_keep_their_folder_and_files_in_order() {
        let (store, _, _, host) = setup();
        let s = store
            .create(&host, "shop", "/srv/shop", &files(), "fp1")
            .unwrap();
        assert_eq!(s.config_files, files());
        assert_eq!(s.working_dir, "/srv/shop");
        let s = store.set_fingerprint(&s.id, "fp2").unwrap();
        assert_eq!(s.fingerprint, "fp2");
        assert_eq!(store.list(&host).unwrap(), std::slice::from_ref(&s));
        store.delete(&s.id).unwrap();
        assert!(matches!(store.get(&s.id), Err(ref e) if e.is(ErrorCode::NotFound)));
    }

    #[test]
    fn names_are_unique_per_host_across_kinds() {
        let (linked, managed, _, host) = setup();
        managed.create(&host, "shop", "services: {}\n", "").unwrap();
        assert!(matches!(
            linked.create(&host, "shop", "/srv/shop", &files(), "fp"),
            Err(ref e) if e.is(ErrorCode::NameInUse) && e.message.contains("\"shop\"")
        ));
        linked
            .create(&host, "blog", "/srv/blog", &files(), "fp")
            .unwrap();
        assert!(matches!(
            managed.create(&host, "blog", "services: {}\n", ""),
            Err(ref e) if e.is(ErrorCode::NameInUse)
        ));
        assert!(matches!(
            linked.create(&host, "blog", "/srv/blog", &files(), "fp"),
            Err(ref e) if e.is(ErrorCode::NameInUse)
        ));
    }

    #[test]
    fn deleting_a_host_forgets_its_linked_stacks() {
        let (store, _, hosts, host) = setup();
        let s = store
            .create(&host, "shop", "/srv/shop", &files(), "fp")
            .unwrap();
        hosts.delete(&host).unwrap();
        assert!(matches!(store.get(&s.id), Err(ref e) if e.is(ErrorCode::NotFound)));
    }
}
