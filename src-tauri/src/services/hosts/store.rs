//! Persistence for Hosts. Secrets are sealed before they reach SQLite.

use std::sync::Arc;

use homeport_storage::{Db, SecretBox};
use rusqlite::{OptionalExtension, Row};

use super::{Host, HostConnection, HostId, HostSecrets};
use crate::error::{AppError, AppResult, ErrorCode, Hint};

const COLUMNS: &str = "id, name, connection, enabled, host_key, created_at";

pub struct HostStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

/// Everything needed to define a Host.
pub struct HostDraft {
    pub name: String,
    pub connection: HostConnection,
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
            .ok_or_else(|| AppError::not_found("This host no longer exists."))?
    }

    pub fn create(&self, draft: HostDraft) -> AppResult<Host> {
        let HostDraft {
            name,
            connection,
            secrets,
        } = draft;
        let name = validated_name(&name)?;
        connection.validate()?;
        let sealed = self.seal(&connection, secrets.as_ref())?;
        let id: HostId = uuid::Uuid::new_v4().to_string();
        let connection_json = serde_json::to_string(&connection)?;
        self.db
            .with(|c| {
                c.execute(
                    "INSERT INTO hosts (id, name, connection, secrets) VALUES (?1, ?2, ?3, ?4)",
                    (&id, &name, &connection_json, &sealed),
                )
            })
            .map_err(|e| unique_name_error(e, &name))?;
        self.get(&id)
    }

    /// Replaces a Host's name and connection. `secrets: None` keeps the stored ones.
    /// Changing where an SSH Host points forgets its trusted server key.
    pub fn update(&self, id: &str, draft: HostDraft) -> AppResult<Host> {
        let current = self.get(id)?;
        let name = validated_name(&draft.name)?;
        draft.connection.validate()?;
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
                    "UPDATE hosts SET name = ?2, connection = ?3, secrets = ?4, host_key = ?5
                     WHERE id = ?1",
                    (id, &name, &connection_json, &sealed, &host_key),
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

    pub fn delete(&self, id: &str) -> AppResult<()> {
        let n = self
            .db
            .with(|c| c.execute("DELETE FROM hosts WHERE id = ?1", [id]))?;
        if n == 0 {
            return Err(AppError::not_found("This host no longer exists."));
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
            .ok_or_else(|| AppError::not_found("This host no longer exists."))
    }

    fn seal(
        &self,
        connection: &HostConnection,
        secrets: Option<&HostSecrets>,
    ) -> AppResult<Option<Vec<u8>>> {
        match (connection.needs_secrets(), secrets) {
            (true, Some(s)) => Ok(Some(self.secrets.seal_json(s)?)),
            (true, None) => Err(AppError::validation("This connection needs credentials.")),
            (false, _) => Ok(None),
        }
    }

    fn exec(&self, id: &str, sql: &str, value: impl rusqlite::ToSql) -> AppResult<Host> {
        let n = self.db.with(|c| c.execute(sql, (id, value)))?;
        if n == 0 {
            return Err(AppError::not_found("This host no longer exists."));
        }
        self.get(id)
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<AppResult<Host>> {
    let connection: String = row.get(2)?;
    Ok(Ok(Host {
        id: row.get(0)?,
        name: row.get(1)?,
        connection: match serde_json::from_str(&connection) {
            Ok(c) => c,
            Err(e) => return Ok(Err(e.into())),
        },
        enabled: row.get(3)?,
        host_key: row.get(4)?,
        created_at: row.get(5)?,
    }))
}

fn validated_name(name: &str) -> AppResult<String> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::validation("Give the host a name."));
    }
    Ok(name.to_string())
}

fn unique_name_error(e: AppError, name: &str) -> AppError {
    if e.is(ErrorCode::NameInUse) {
        AppError {
            message: format!("A host named \"{name}\" already exists."),
            hint: Some(Hint {
                text: "Pick another name.".into(),
                commands: vec![],
            }),
            ..e
        }
    } else {
        e
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
mod tests {
    use super::*;
    use crate::services::hosts::SshAuth;

    fn store() -> HostStore {
        HostStore::new(
            Arc::new(Db::open_in_memory().unwrap()),
            Arc::new(SecretBox::from_key(&[9; 32])),
        )
    }

    fn local(name: &str) -> HostDraft {
        HostDraft {
            name: name.into(),
            connection: HostConnection::Local {
                socket_path: "/var/run/docker.sock".into(),
            },
            secrets: None,
        }
    }

    fn ssh(name: &str, host: &str) -> HostDraft {
        HostDraft {
            name: name.into(),
            connection: HostConnection::Ssh {
                host: host.into(),
                port: 22,
                user: "deploy".into(),
                socket_path: "/var/run/docker.sock".into(),
                auth: SshAuth::Generated,
                public_key: Some("ssh-ed25519 AAAA homeport".into()),
            },
            secrets: Some(HostSecrets::Ssh {
                private_key: "PRIVATE".into(),
                passphrase: None,
            }),
        }
    }

    #[test]
    fn created_hosts_are_listed_by_name() {
        let s = store();
        s.create(local("zeta")).unwrap();
        let alpha = s.create(local("  Alpha ")).unwrap();
        assert_eq!(alpha.name, "Alpha");
        assert!(alpha.enabled);
        let names: Vec<_> = s.list().unwrap().into_iter().map(|h| h.name).collect();
        assert_eq!(names, ["Alpha", "zeta"]);
    }

    #[test]
    fn names_are_unique_ignoring_case() {
        let s = store();
        s.create(local("prod")).unwrap();
        let err = s.create(local("PROD")).unwrap_err();
        assert!(err.message.contains("already exists"));
    }

    #[test]
    fn secrets_are_kept_and_readable_only_through_the_store() {
        let s = store();
        let host = s.create(ssh("box", "10.0.0.5")).unwrap();
        assert_eq!(
            s.secrets(&host.id).unwrap(),
            Some(HostSecrets::Ssh {
                private_key: "PRIVATE".into(),
                passphrase: None
            })
        );
        assert!(!serde_json::to_string(&host).unwrap().contains("PRIVATE"));
    }

    #[test]
    fn connections_that_need_credentials_require_them() {
        let mut draft = ssh("box", "10.0.0.5");
        draft.secrets = None;
        assert!(matches!(
            store().create(draft),
            Err(ref e) if e.is(ErrorCode::Validation)
        ));
    }

    #[test]
    fn secrets_from_another_key_are_reported_lost() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let before = HostStore::new(db.clone(), Arc::new(SecretBox::from_key(&[1; 32])));
        let host = before.create(ssh("box", "10.0.0.5")).unwrap();
        let after = HostStore::new(db, Arc::new(SecretBox::from_key(&[2; 32])));
        assert!(matches!(
            after.secrets(&host.id),
            Err(ref e) if e.is(ErrorCode::CredentialsLost)
        ));
    }

    #[test]
    fn update_keeps_secrets_and_host_key_for_the_same_server() {
        let s = store();
        let host = s.create(ssh("box", "10.0.0.5")).unwrap();
        s.set_host_key(&host.id, Some("ssh-ed25519 SERVER"))
            .unwrap();

        let mut renamed = ssh("renamed", "10.0.0.5");
        renamed.secrets = None;
        let updated = s.update(&host.id, renamed).unwrap();
        assert_eq!(updated.name, "renamed");
        assert_eq!(updated.host_key.as_deref(), Some("ssh-ed25519 SERVER"));
        assert!(s.secrets(&host.id).unwrap().is_some());

        let mut moved = ssh("renamed", "10.0.0.6");
        moved.secrets = None;
        assert_eq!(s.update(&host.id, moved).unwrap().host_key, None);
    }

    #[test]
    fn hosts_can_be_disabled_and_deleted() {
        let s = store();
        let host = s.create(local("local")).unwrap();
        assert!(!s.set_enabled(&host.id, false).unwrap().enabled);
        s.delete(&host.id).unwrap();
        assert!(matches!(s.get(&host.id), Err(ref e) if e.is(ErrorCode::NotFound)));
        assert!(matches!(s.delete(&host.id), Err(ref e) if e.is(ErrorCode::NotFound)));
    }
}
