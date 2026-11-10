//! Persistence for Hosts. Secrets are sealed before they reach SQLite.

use std::sync::Arc;

use rusqlite::{OptionalExtension, Row};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;
use homeport_storage::{Db, SecretBox};

use crate::{EngineKind, Host, HostConnection, HostId, HostSecrets};

const COLUMNS: &str = "id, name, connection, enabled, host_key, created_at, engine";

pub struct HostStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

/// Everything needed to define a Host.
pub struct HostDraft {
    pub name: String,
    pub connection: HostConnection,
    /// The kind of Engine it points to; only Docker-compatible ones for now.
    pub engine: EngineKind,
    pub secrets: Option<HostSecrets>,
}

impl HostStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    pub fn list(&self) -> AppResult<Vec<Host>> {
        let rows = self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM hosts ORDER BY name COLLATE NOCASE"
            ))?
            .query_map([], read_row)?
            .collect::<Result<Vec<_>, _>>()
        })?;
        rows.into_iter().collect()
    }

    pub fn get(&self, id: &str) -> AppResult<Host> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM hosts WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))?
    }

    pub fn create(&self, draft: HostDraft) -> AppResult<Host> {
        self.insert(draft, true)
    }

    /// Creates a Host from an export, which may have lost its secrets: it is
    /// then created without them and shows them missing until re-entered.
    pub fn create_imported(&self, draft: HostDraft) -> AppResult<Host> {
        self.insert(draft, false)
    }

    fn insert(&self, draft: HostDraft, require_secrets: bool) -> AppResult<Host> {
        let HostDraft {
            name,
            connection,
            engine,
            secrets,
        } = draft;
        let name = validated_name(&name)?;
        connection.validate()?;
        engine.validate(&connection)?;
        let sealed = match secrets {
            None if !require_secrets => None,
            secrets => self.seal(&connection, secrets.as_ref())?,
        };
        let id: HostId = uuid::Uuid::new_v4().to_string();
        let connection_json = serde_json::to_string(&connection)?;
        self.db
            .with(|c| {
                c.execute(
                    "INSERT INTO hosts (id, name, connection, secrets, engine)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    (&id, &name, &connection_json, &sealed, engine.as_str()),
                )
            })
            .map_err(|e| unique_name_error(e, &name))?;
        self.get(&id)
    }

    /// Replaces a Host's name and connection. `secrets: None` keeps the stored ones.
    /// Changing where an SSH Host points forgets its trusted server key; each
    /// bastion keeps the key trusted for its address, whatever the draft says.
    pub fn update(&self, id: &str, mut draft: HostDraft) -> AppResult<Host> {
        let current = self.get(id)?;
        let name = validated_name(&draft.name)?;
        draft.connection.validate()?;
        draft.engine.validate(&draft.connection)?;
        keep_bastion_keys(&current.connection, &mut draft.connection);
        let connection_json = serde_json::to_string(&draft.connection)?;
        let sealed = match &draft.secrets {
            Some(secrets) => self.seal(&draft.connection, Some(secrets))?,
            None => self.sealed_secrets(id)?,
        };
        let host_key = if same_ssh_server(&current.connection, &draft.connection) {
            current.host_key
        } else {
            None
        };
        self.db
            .with(|c| {
                c.execute(
                    "UPDATE hosts SET name = ?2, connection = ?3, secrets = ?4, host_key = ?5,
                         engine = ?6
                     WHERE id = ?1",
                    (
                        id,
                        &name,
                        &connection_json,
                        &sealed,
                        &host_key,
                        draft.engine.as_str(),
                    ),
                )
            })
            .map_err(|e| unique_name_error(e, &name))?;
        self.get(id)
    }

    pub fn set_enabled(&self, id: &str, enabled: bool) -> AppResult<Host> {
        self.exec(id, "UPDATE hosts SET enabled = ?2 WHERE id = ?1", enabled)
    }

    /// Records the SSH server key the user trusted, or forgets it with `None`.
    pub fn set_host_key(&self, id: &str, host_key: Option<&str>) -> AppResult<Host> {
        self.exec(id, "UPDATE hosts SET host_key = ?2 WHERE id = ?1", host_key)
    }

    /// Records the server key the user trusted for the bastion at `address`.
    pub fn set_bastion_key(&self, id: &str, address: &str, host_key: &str) -> AppResult<Host> {
        let mut host = self.get(id)?;
        let HostConnection::Ssh { bastions, .. } = &mut host.connection else {
            return Err(AppError::validation(
                msg::NO_SUCH_BASTION.with("address", address),
            ));
        };
        let Some(bastion) = bastions.iter_mut().find(|b| b.address() == address) else {
            return Err(AppError::validation(
                msg::NO_SUCH_BASTION.with("address", address),
            ));
        };
        bastion.host_key = Some(host_key.to_owned());
        let json = serde_json::to_string(&host.connection)?;
        self.exec(id, "UPDATE hosts SET connection = ?2 WHERE id = ?1", json)
    }

    pub fn delete(&self, id: &str) -> AppResult<()> {
        let n = self
            .db
            .with(|c| c.execute("DELETE FROM hosts WHERE id = ?1", [id]))?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        Ok(())
    }

    /// Decrypts a Host's secrets; `credentials_lost` when they can't be opened.
    pub fn secrets(&self, id: &str) -> AppResult<Option<HostSecrets>> {
        self.sealed_secrets(id)?
            .map(|sealed| self.secrets.open_json(&sealed))
            .transpose()
    }

    fn sealed_secrets(&self, id: &str) -> AppResult<Option<Vec<u8>>> {
        self.db
            .with(|c| {
                c.query_row("SELECT secrets FROM hosts WHERE id = ?1", [id], |r| {
                    r.get(0)
                })
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))
    }

    fn seal(
        &self,
        connection: &HostConnection,
        secrets: Option<&HostSecrets>,
    ) -> AppResult<Option<Vec<u8>>> {
        match (connection.needs_secrets(), secrets) {
            (true, Some(s)) => Ok(Some(self.secrets.seal_json(s)?)),
            (true, None) => Err(AppError::validation(msg::NEEDS_CREDENTIALS)),
            (false, _) => Ok(None),
        }
    }

    fn exec(&self, id: &str, sql: &str, value: impl rusqlite::ToSql) -> AppResult<Host> {
        let n = self.db.with(|c| c.execute(sql, (id, value)))?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<AppResult<Host>> {
    let connection: String = row.get(2)?;
    let engine: String = row.get(6)?;
    // The table's CHECK only admits known kinds; a newer database's would
    // still open, as a Docker-compatible Engine.
    let engine = EngineKind::parse(&engine).unwrap_or_default();
    Ok(Ok(Host {
        id: row.get(0)?,
        name: row.get(1)?,
        connection: match serde_json::from_str(&connection) {
            Ok(c) => c,
            Err(e) => return Ok(Err(e.into())),
        },
        engine,
        enabled: row.get(3)?,
        host_key: row.get(4)?,
        created_at: row.get(5)?,
    }))
}

fn validated_name(name: &str) -> AppResult<String> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::validation(msg::NAME_EMPTY));
    }
    Ok(name.to_string())
}

fn unique_name_error(e: AppError, name: &str) -> AppError {
    if e.is(ErrorCode::NameInUse) {
        e.with_message(msg::NAME_IN_USE.with("name", name))
            .with_hint(msg::NAME_IN_USE_HINT)
    } else {
        e
    }
}

/// Gives each bastion of `next` the server key trusted for its address in
/// `current`, and none to a bastion at a new address.
fn keep_bastion_keys(current: &HostConnection, next: &mut HostConnection) {
    let HostConnection::Ssh { bastions, .. } = next else {
        return;
    };
    let trusted = |address: &str| match current {
        HostConnection::Ssh { bastions, .. } => bastions
            .iter()
            .find(|b| b.address() == address)
            .and_then(|b| b.host_key.clone()),
        _ => None,
    };
    for bastion in bastions {
        bastion.host_key = trusted(&bastion.address());
    }
}

fn same_ssh_server(a: &HostConnection, b: &HostConnection) -> bool {
    match (a, b) {
        (
            HostConnection::Ssh {
                host: h1, port: p1, ..
            },
            HostConnection::Ssh {
                host: h2, port: p2, ..
            },
        ) => h1 == h2 && p1 == p2,
        _ => false,
    }
}

#[cfg(test)]
mod tests;
