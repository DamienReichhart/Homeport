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
mod tests;
