//! Persistence for Clusters. Credentials are sealed before they reach SQLite.

pub mod backup;

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

/// Everything needed to define a Cluster; its kind is its connection's.
#[derive(Debug, Clone)]
pub struct ClusterDraft {
    pub name: String,
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
        validate(&draft.connection, &draft.secrets)?;
        let sealed = self.seal(&draft.secrets)?;
        let id: ClusterId = uuid::Uuid::new_v4().to_string();
        let connection = draft.connection.to_stored()?;
        self.db
            .with(|c| {
                c.execute(
                    "INSERT INTO clusters (id, name, kind, connection, secrets)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    (
                        &id,
                        &name,
                        draft.connection.kind().as_str(),
                        &connection,
                        &sealed,
                    ),
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
    /// name, as when it is refreshed from its kubeconfig. Its kind stays.
    pub fn replace_connection(
        &self,
        id: &str,
        connection: &ClusterConnection,
        secrets: &ClusterSecrets,
    ) -> AppResult<Cluster> {
        validate(connection, secrets)?;
        if self.get(id)?.kind != connection.kind() {
            return Err(AppError::validation(msg::KIND_CHANGED));
        }
        let sealed = self.seal(secrets)?;
        let json = connection.to_stored()?;
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
        let (kind, sealed): (String, Option<Vec<u8>>) = self
            .db
            .with(|c| {
                c.query_row(
                    "SELECT kind, secrets FROM clusters WHERE id = ?1",
                    [id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))?;
        let kind = parse_kind(kind)?;
        match sealed {
            Some(sealed) => Ok(ClusterSecrets::from_value(
                kind,
                self.secrets.open_json(&sealed)?,
            )?),
            None => Ok(ClusterSecrets::none(kind)),
        }
    }

    fn seal(&self, secrets: &ClusterSecrets) -> AppResult<Option<Vec<u8>>> {
        seal(&self.secrets, secrets)
    }

    fn exec(&self, id: &str, sql: &str, value: impl rusqlite::ToSql) -> AppResult<Cluster> {
        let n = self.db.with(|c| c.execute(sql, (id, value)))?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        self.get(id)
    }
}

/// `secrets` sealed as the variant's own JSON; `None` when there are none.
fn seal(key: &SecretBox, secrets: &ClusterSecrets) -> AppResult<Option<Vec<u8>>> {
    if secrets.is_empty() {
        return Ok(None);
    }
    key.seal_json(&secrets.to_value()?).map(Some)
}

/// A stored kind; an error for one this version doesn't know.
fn parse_kind(kind: String) -> AppResult<ClusterKind> {
    ClusterKind::parse(&kind)
        .ok_or_else(|| AppError::new(ErrorCode::Json, msg::UNKNOWN_KIND.with("kind", kind)))
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<AppResult<Cluster>> {
    let kind = match parse_kind(row.get(2)?) {
        Ok(kind) => kind,
        Err(e) => return Ok(Err(e)),
    };
    let connection: String = row.get(3)?;
    Ok(Ok(Cluster {
        id: row.get(0)?,
        name: row.get(1)?,
        kind,
        connection: match ClusterConnection::from_stored(kind, &connection) {
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

/// Refuses a connection that can't be used, and credentials of another kind.
fn validate(connection: &ClusterConnection, secrets: &ClusterSecrets) -> AppResult<()> {
    if connection.kind() != secrets.kind() {
        return Err(AppError::validation(msg::KIND_CHANGED));
    }
    let server = match connection {
        ClusterConnection::Kubernetes(c) => c.server.trim(),
        ClusterConnection::Nomad(c) => c.address.trim(),
    };
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
    use crate::model::{AuthMethod, ExecConfig, KubernetesConnection, KubernetesSecrets};

    fn store() -> (Arc<Db>, ClusterStore) {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let store = ClusterStore::new(db.clone(), Arc::new(SecretBox::from_key(&[7; 32])));
        (db, store)
    }

    fn draft(name: &str) -> ClusterDraft {
        ClusterDraft {
            name: name.into(),
            connection: ClusterConnection::Kubernetes(KubernetesConnection {
                server: "https://127.0.0.1:6443".into(),
                namespace: Some("shop".into()),
                auth: AuthMethod::Token,
                ..Default::default()
            }),
            secrets: ClusterSecrets::Kubernetes(KubernetesSecrets {
                token: Some("very-secret-token".into()),
                client_key: Some("-----BEGIN EC PRIVATE KEY-----".into()),
                exec: Some(ExecConfig {
                    command: "aws".into(),
                    env: vec![("AWS_PROFILE".into(), "prod-profile".into())],
                    ..Default::default()
                }),
                ..Default::default()
            }),
        }
    }

    #[test]
    fn clusters_round_trip_with_their_secrets_sealed() {
        let (db, store) = store();
        let created = store.create(draft("kind-dev")).unwrap();
        assert_eq!(store.get(&created.id).unwrap(), created);
        assert_eq!(store.secrets(&created.id).unwrap(), draft("x").secrets);
        assert_eq!(created.kind, ClusterKind::Kubernetes);

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
        d.secrets = ClusterSecrets::none(ClusterKind::Kubernetes);
        let c = store.create(d).unwrap();
        assert_eq!(
            store.secrets(&c.id).unwrap(),
            ClusterSecrets::none(ClusterKind::Kubernetes)
        );
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
        if let ClusterConnection::Kubernetes(c) = &mut d.connection {
            c.server = "127.0.0.1:6443".into();
        }
        assert!(store.create(d).unwrap_err().is(ErrorCode::Validation));
    }

    #[test]
    fn refreshing_replaces_the_connection_and_keeps_the_name() {
        let (_, store) = store();
        let c = store.create(draft("kind-dev")).unwrap();
        let connection = ClusterConnection::Kubernetes(KubernetesConnection {
            server: "https://127.0.0.1:7443".into(),
            ..c.kubernetes().unwrap().clone()
        });
        let secrets = ClusterSecrets::Kubernetes(KubernetesSecrets {
            token: Some("new".into()),
            ..Default::default()
        });
        let next = store
            .replace_connection(&c.id, &connection, &secrets)
            .unwrap();
        assert_eq!(next.name, "kind-dev");
        assert_eq!(next.address(), "https://127.0.0.1:7443");
        assert_eq!(store.secrets(&c.id).unwrap(), secrets);
    }

    #[test]
    fn a_cluster_keeps_its_kind() {
        let (_, store) = store();
        let c = store.create(draft("kind-dev")).unwrap();
        assert_eq!(c.kind, ClusterKind::Kubernetes);
        let nomad = ClusterConnection::Nomad(crate::model::NomadConnection {
            address: "http://127.0.0.1:4646".into(),
        });
        let secrets = ClusterSecrets::none(ClusterKind::Nomad);
        assert!(store
            .replace_connection(&c.id, &nomad, &secrets)
            .unwrap_err()
            .is(ErrorCode::Validation));
        let mut mixed = draft("mixed");
        mixed.secrets = secrets;
        assert!(store.create(mixed).unwrap_err().is(ErrorCode::Validation));
    }

    /// Rows written before Clusters had kinds other than Kubernetes, in the
    /// shape migration 0016 left them: the connection's JSON untagged and
    /// the credentials sealed as they were.
    #[test]
    fn clusters_stored_before_kinds_keep_every_field_and_secret() {
        let (db, store) = store();
        let key = SecretBox::from_key(&[7; 32]);
        let old_secrets = serde_json::json!({ "token": "old-token", "clientKey": "KEY" });
        let sealed = key.seal_json(&old_secrets).unwrap();
        db.with(|c| {
            c.execute(
                "INSERT INTO clusters (id, name, connection, secrets) VALUES ('old', 'old', ?1, ?2)",
                (
                    r#"{"server":"https://10.0.0.1:6443","namespace":"shop","auth":{"method":"token"},"source":{"context":"prod","pasted":false}}"#,
                    &sealed,
                ),
            )
        })
        .unwrap();
        let old = store.get("old").unwrap();
        assert_eq!(old.kind, ClusterKind::Kubernetes);
        let k = old.kubernetes().unwrap();
        assert_eq!(k.server, "https://10.0.0.1:6443");
        assert_eq!(k.namespace.as_deref(), Some("shop"));
        assert_eq!(k.source.as_ref().unwrap().context, "prod");
        let ClusterSecrets::Kubernetes(secrets) = store.secrets("old").unwrap() else {
            panic!("not Kubernetes credentials");
        };
        assert_eq!(secrets.token.as_deref(), Some("old-token"));
        assert_eq!(secrets.client_key.as_deref(), Some("KEY"));
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
