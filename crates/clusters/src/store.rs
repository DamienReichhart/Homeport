//! Persistence for Clusters. Credentials are sealed before they reach SQLite.

use std::sync::Arc;

use rusqlite::{OptionalExtension, Row};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_storage::{Db, SecretBox};

use crate::messages as msg;
use crate::model::{Cluster, ClusterConnection, ClusterId, ClusterKind, ClusterSecrets};

const COLUMNS: &str = "id, name, kind, connection, enabled, created_at";

pub struct ClusterStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

/// Everything needed to define a Cluster.
#[derive(Debug, Clone)]
pub struct ClusterDraft {
    pub name: String,
    pub kind: ClusterKind,
    pub connection: ClusterConnection,
    pub secrets: ClusterSecrets,
}

impl ClusterStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    pub fn list(&self) -> AppResult<Vec<Cluster>> {
        let rows = self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM clusters ORDER BY name COLLATE NOCASE"
            ))?
            .query_map([], read_row)?
            .collect::<Result<Vec<_>, _>>()
        })?;
        rows.into_iter().collect()
    }

    pub fn get(&self, id: &str) -> AppResult<Cluster> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM clusters WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))?
    }

    pub fn create(&self, draft: ClusterDraft) -> AppResult<Cluster> {
        let name = validated_name(&draft.name)?;
        validate(&draft.connection)?;
        let sealed = self.seal(&draft.secrets)?;
        let id: ClusterId = uuid::Uuid::new_v4().to_string();
        let connection = serde_json::to_string(&draft.connection)?;
        self.db
            .with(|c| {
                c.execute(
                    "INSERT INTO clusters (id, name, kind, connection, secrets)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    (&id, &name, draft.kind.as_str(), &connection, &sealed),
                )
            })
            .map_err(|e| unique_name_error(e, &name))?;
        self.get(&id)
    }

    /// Renames a Cluster.
    pub fn rename(&self, id: &str, name: &str) -> AppResult<Cluster> {
        let name = validated_name(name)?;
        self.exec(id, "UPDATE clusters SET name = ?2 WHERE id = ?1", &name)
            .map_err(|e| unique_name_error(e, &name))
    }

    /// Replaces how a Cluster is reached and its credentials, keeping its
    /// name, as when it is refreshed from its kubeconfig.
    pub fn replace_connection(
        &self,
        id: &str,
        connection: &ClusterConnection,
        secrets: &ClusterSecrets,
    ) -> AppResult<Cluster> {
        validate(connection)?;
        let sealed = self.seal(secrets)?;
        let json = serde_json::to_string(connection)?;
        let n = self.db.with(|c| {
            c.execute(
                "UPDATE clusters SET connection = ?2, secrets = ?3 WHERE id = ?1",
                (id, &json, &sealed),
            )
        })?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }

    pub fn set_enabled(&self, id: &str, enabled: bool) -> AppResult<Cluster> {
        self.exec(
            id,
            "UPDATE clusters SET enabled = ?2 WHERE id = ?1",
            enabled,
        )
    }

    pub fn delete(&self, id: &str) -> AppResult<()> {
        let n = self
            .db
            .with(|c| c.execute("DELETE FROM clusters WHERE id = ?1", [id]))?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        Ok(())
    }

    /// Decrypts a Cluster's credentials; `credentials_lost` when they can't be opened.
    pub fn secrets(&self, id: &str) -> AppResult<ClusterSecrets> {
        let sealed: Option<Vec<u8>> = self
            .db
            .with(|c| {
                c.query_row("SELECT secrets FROM clusters WHERE id = ?1", [id], |r| {
                    r.get(0)
                })
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))?;
        match sealed {
            Some(sealed) => self.secrets.open_json(&sealed),
            None => Ok(ClusterSecrets::default()),
        }
    }

    fn seal(&self, secrets: &ClusterSecrets) -> AppResult<Option<Vec<u8>>> {
        if secrets.is_empty() {
            return Ok(None);
        }
        self.secrets.seal_json(secrets).map(Some)
    }

    fn exec(&self, id: &str, sql: &str, value: impl rusqlite::ToSql) -> AppResult<Cluster> {
        let n = self.db.with(|c| c.execute(sql, (id, value)))?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<AppResult<Cluster>> {
    let kind: String = row.get(2)?;
    let connection: String = row.get(3)?;
    let Some(kind) = ClusterKind::parse(&kind) else {
        return Ok(Err(AppError::new(
            ErrorCode::Json,
            msg::UNKNOWN_KIND.with("kind", kind),
        )));
    };
    Ok(Ok(Cluster {
        id: row.get(0)?,
        name: row.get(1)?,
        kind,
        connection: match serde_json::from_str(&connection) {
            Ok(c) => c,
            Err(e) => return Ok(Err(e.into())),
        },
        enabled: row.get(4)?,
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

fn validate(connection: &ClusterConnection) -> AppResult<()> {
    let server = connection.server.trim();
    if !(server.starts_with("https://") || server.starts_with("http://"))
        || server.parse::<http::Uri>().is_err()
    {
        return Err(AppError::validation(
            msg::SERVER_INVALID.with("server", server),
        ));
    }
    Ok(())
}

fn unique_name_error(e: AppError, name: &str) -> AppError {
    if e.is(ErrorCode::NameInUse) {
        e.with_message(msg::NAME_IN_USE.with("name", name))
            .with_hint(msg::NAME_IN_USE_HINT)
    } else {
        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AuthMethod, ExecConfig};

    fn store() -> (Arc<Db>, ClusterStore) {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let store = ClusterStore::new(db.clone(), Arc::new(SecretBox::from_key(&[7; 32])));
        (db, store)
    }

    fn draft(name: &str) -> ClusterDraft {
        ClusterDraft {
            name: name.into(),
            kind: ClusterKind::Kubernetes,
            connection: ClusterConnection {
                server: "https://127.0.0.1:6443".into(),
                namespace: Some("shop".into()),
                auth: AuthMethod::Token,
                ..Default::default()
            },
            secrets: ClusterSecrets {
                token: Some("very-secret-token".into()),
                client_key: Some("-----BEGIN EC PRIVATE KEY-----".into()),
                exec: Some(ExecConfig {
                    command: "aws".into(),
                    env: vec![("AWS_PROFILE".into(), "prod-profile".into())],
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }

    #[test]
    fn clusters_round_trip_with_their_secrets_sealed() {
        let (db, store) = store();
        let created = store.create(draft("kind-dev")).unwrap();
        assert_eq!(store.get(&created.id).unwrap(), created);
        assert_eq!(store.secrets(&created.id).unwrap(), draft("x").secrets);

        let (connection, sealed): (String, Vec<u8>) = db
            .with(|c| {
                c.query_row("SELECT connection, secrets FROM clusters", [], |r| {
                    Ok((r.get(0)?, r.get(1)?))
                })
            })
            .unwrap();
        for secret in ["very-secret-token", "PRIVATE KEY", "prod-profile"] {
            assert!(!connection.contains(secret), "{secret} in the connection");
            let raw = String::from_utf8_lossy(&sealed);
            assert!(!raw.contains(secret), "{secret} in clear in the secrets");
        }
    }

    #[test]
    fn a_cluster_without_credentials_stores_none() {
        let (_, store) = store();
        let mut d = draft("anonymous");
        d.secrets = ClusterSecrets::default();
        let c = store.create(d).unwrap();
        assert_eq!(store.secrets(&c.id).unwrap(), ClusterSecrets::default());
    }

    #[test]
    fn secrets_sealed_with_another_key_are_lost() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let before = ClusterStore::new(db.clone(), Arc::new(SecretBox::from_key(&[1; 32])));
        let id = before.create(draft("prod")).unwrap().id;
        let after = ClusterStore::new(db, Arc::new(SecretBox::from_key(&[2; 32])));
        assert!(after
            .secrets(&id)
            .unwrap_err()
            .is(ErrorCode::CredentialsLost));
    }

    #[test]
    fn names_are_unique_and_required() {
        let (_, store) = store();
        let c = store.create(draft("prod")).unwrap();
        let e = store.create(draft("PROD")).unwrap_err();
        assert!(e.is(ErrorCode::NameInUse));
        assert!(e.message.contains("PROD"));
        assert!(store
            .rename(&c.id, "  ")
            .unwrap_err()
            .is(ErrorCode::Validation));
        assert_eq!(store.rename(&c.id, " staging ").unwrap().name, "staging");
    }

    #[test]
    fn a_server_must_be_a_url() {
        let (_, store) = store();
        let mut d = draft("bad");
        d.connection.server = "127.0.0.1:6443".into();
        assert!(store.create(d).unwrap_err().is(ErrorCode::Validation));
    }

    #[test]
    fn refreshing_replaces_the_connection_and_keeps_the_name() {
        let (_, store) = store();
        let c = store.create(draft("kind-dev")).unwrap();
        let connection = ClusterConnection {
            server: "https://127.0.0.1:7443".into(),
            ..c.connection.clone()
        };
        let secrets = ClusterSecrets {
            token: Some("new".into()),
            ..Default::default()
        };
        let next = store
            .replace_connection(&c.id, &connection, &secrets)
            .unwrap();
        assert_eq!(next.name, "kind-dev");
        assert_eq!(next.connection.server, "https://127.0.0.1:7443");
        assert_eq!(store.secrets(&c.id).unwrap(), secrets);
    }

    #[test]
    fn enabling_and_deleting() {
        let (_, store) = store();
        let c = store.create(draft("kind-dev")).unwrap();
        assert!(!store.set_enabled(&c.id, false).unwrap().enabled);
        store.delete(&c.id).unwrap();
        assert!(store.get(&c.id).unwrap_err().is(ErrorCode::NotFound));
        assert!(store.delete(&c.id).unwrap_err().is(ErrorCode::NotFound));
    }
}
