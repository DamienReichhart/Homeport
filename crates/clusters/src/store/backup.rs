//! Clusters in a backup of Homeport's data, with their credentials opened;
//! restored with this computer's key.

use std::fmt;

use serde::{Deserialize, Serialize};

use homeport_storage::db::sqlite_error;

use super::*;

/// A Cluster as a backup holds it. Its id is kept, so the remembered
/// selection still points at it after a restore. Its connection and
/// credentials are the JSON of its kind, as the store keeps them, so
/// backups made before Clusters had kinds still restore.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackedUpCluster {
    pub id: ClusterId,
    pub name: String,
    pub kind: ClusterKind,
    pub connection: serde_json::Value,
    pub enabled: bool,
    /// `None` when they couldn't be opened on the computer backed up.
    pub secrets: Option<serde_json::Value>,
}

impl fmt::Debug for BackedUpCluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackedUpCluster")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl ClusterStore {
    /// Every Cluster with its credentials; one whose credentials can't be
    /// opened is kept without them and named in `lost`.
    pub fn backup(&self, lost: &mut Vec<String>) -> AppResult<Vec<BackedUpCluster>> {
        let mut clusters = vec![];
        for cluster in self.list()? {
            let secrets = match self.secrets(&cluster.id) {
                Ok(secrets) => Some(secrets.to_value()?),
                Err(_) => {
                    lost.push(cluster.name.clone());
                    None
                }
            };
            clusters.push(BackedUpCluster {
                id: cluster.id,
                name: cluster.name,
                kind: cluster.kind,
                connection: serde_json::from_str(&cluster.connection.to_stored()?)?,
                enabled: cluster.enabled,
                secrets,
            });
        }
        Ok(clusters)
    }
}

/// Writes a backup's Clusters in `tx`, into a database emptied of them.
pub fn restore(
    tx: &rusqlite::Transaction<'_>,
    secrets: &SecretBox,
    clusters: &[BackedUpCluster],
) -> AppResult<()> {
    for cluster in clusters {
        // Read as the kind's, so a backup can't restore what the store can't open.
        let connection =
            ClusterConnection::from_stored(cluster.kind, &cluster.connection.to_string())?;
        let sealed = match &cluster.secrets {
            Some(s) => seal(
                secrets,
                &ClusterSecrets::from_value(cluster.kind, s.clone())?,
            )?,
            None => None,
        };
        tx.execute(
            "INSERT INTO clusters (id, name, kind, connection, secrets, enabled)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &cluster.id,
                &cluster.name,
                cluster.kind.as_str(),
                connection.to_stored()?,
                sealed,
                cluster.enabled,
            ),
        )
        .map_err(sqlite_error)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AuthMethod, KubernetesConnection, KubernetesSecrets};

    #[test]
    fn clusters_round_trip_through_a_backup_under_another_key() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let laptop = ClusterStore::new(db, Arc::new(SecretBox::from_key(&[1; 32])));
        let id = laptop
            .create(ClusterDraft {
                name: "prod".into(),
                connection: ClusterConnection::Kubernetes(KubernetesConnection {
                    server: "https://k8s.example.com".into(),
                    auth: AuthMethod::Token,
                    ..Default::default()
                }),
                secrets: ClusterSecrets::Kubernetes(KubernetesSecrets {
                    token: Some("t0ken".into()),
                    ..Default::default()
                }),
            })
            .unwrap()
            .id;
        let mut lost = vec![];
        let backed_up = laptop.backup(&mut lost).unwrap();
        assert!(lost.is_empty());
        assert!(
            !format!("{backed_up:?}").contains("t0ken"),
            "Debug hides them"
        );

        let db = Arc::new(Db::open_in_memory().unwrap());
        let key = Arc::new(SecretBox::from_key(&[2; 32]));
        db.transaction(|tx| restore(tx, &key, &backed_up)).unwrap();
        let desktop = ClusterStore::new(db, key);
        assert_eq!(desktop.get(&id).unwrap().name, "prod");
        let ClusterSecrets::Kubernetes(restored) = desktop.secrets(&id).unwrap() else {
            panic!("not Kubernetes credentials");
        };
        assert_eq!(restored.token.as_deref(), Some("t0ken"));
    }

    #[test]
    fn nomad_clusters_round_trip_with_their_token_and_key() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let laptop = ClusterStore::new(db, Arc::new(SecretBox::from_key(&[1; 32])));
        let id = laptop
            .create(ClusterDraft {
                name: "nomad-prod".into(),
                connection: ClusterConnection::Nomad(crate::model::NomadConnection {
                    address: "https://nomad.example.com:4646".into(),
                    acl_token: true,
                    ..Default::default()
                }),
                secrets: ClusterSecrets::Nomad(crate::model::NomadSecrets {
                    token: Some("n0mad-t0ken".into()),
                    client_key: Some("NOMAD KEY".into()),
                    ..Default::default()
                }),
            })
            .unwrap()
            .id;
        let backed_up = laptop.backup(&mut vec![]).unwrap();
        assert!(!format!("{backed_up:?}").contains("n0mad-t0ken"));

        let db = Arc::new(Db::open_in_memory().unwrap());
        let key = Arc::new(SecretBox::from_key(&[2; 32]));
        db.transaction(|tx| restore(tx, &key, &backed_up)).unwrap();
        let desktop = ClusterStore::new(db, key);
        assert_eq!(desktop.get(&id).unwrap().kind, ClusterKind::Nomad);
        let ClusterSecrets::Nomad(restored) = desktop.secrets(&id).unwrap() else {
            panic!("not Nomad credentials");
        };
        assert_eq!(restored.token.as_deref(), Some("n0mad-t0ken"));
        assert_eq!(restored.client_key.as_deref(), Some("NOMAD KEY"));
    }

    /// A backup's Cluster as Homeport wrote it before Clusters had kinds.
    #[test]
    fn backups_made_before_kinds_restore() {
        let backed_up: Vec<BackedUpCluster> = serde_json::from_value(serde_json::json!([{
            "id": "c1",
            "name": "prod",
            "kind": "kubernetes",
            "connection": { "server": "https://k8s.example.com", "auth": { "method": "token" } },
            "enabled": true,
            "secrets": { "token": "t0ken" },
        }]))
        .unwrap();
        let db = Arc::new(Db::open_in_memory().unwrap());
        let key = Arc::new(SecretBox::from_key(&[3; 32]));
        db.transaction(|tx| restore(tx, &key, &backed_up)).unwrap();
        let store = ClusterStore::new(db, key);
        assert_eq!(
            store.get("c1").unwrap().address(),
            "https://k8s.example.com"
        );
        assert!(!store.secrets("c1").unwrap().is_empty());
    }
}
