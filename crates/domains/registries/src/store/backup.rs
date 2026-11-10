//! Registry logins in a backup of Homeport's data, with their secrets
//! opened; restored with this computer's key.

use homeport_storage::db::sqlite_error;

use super::*;

/// A login as a backup holds it.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackedUpLogin {
    pub address: String,
    pub username: String,
    pub secret: String,
}

impl fmt::Debug for BackedUpLogin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackedUpLogin")
            .field("address", &self.address)
            .finish_non_exhaustive()
    }
}

impl RegistryStore {
    /// Every login with its secret; those whose secret can't be opened are
    /// left out and named in `lost`.
    pub fn backup(&self, lost: &mut Vec<String>) -> AppResult<Vec<BackedUpLogin>> {
        let rows: Vec<(String, String, Vec<u8>)> = self.db.with(|c| {
            c.prepare("SELECT address, username, secret FROM registries ORDER BY address")?
                .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
                .collect()
        })?;
        let mut logins = Vec::with_capacity(rows.len());
        for (address, username, sealed) in rows {
            match self
                .secrets
                .open(&sealed)
                .ok()
                .and_then(|s| String::from_utf8(s).ok())
            {
                Some(secret) => logins.push(BackedUpLogin {
                    address,
                    username,
                    secret,
                }),
                None => lost.push(address),
            }
        }
        Ok(logins)
    }
}

/// Writes a backup's logins in `tx`, into a database emptied of them.
pub fn restore(
    tx: &rusqlite::Transaction<'_>,
    secrets: &SecretBox,
    logins: &[BackedUpLogin],
) -> AppResult<()> {
    for login in logins {
        tx.execute(
            "INSERT INTO registries (id, address, username, secret) VALUES (?1, ?2, ?3, ?4)",
            (
                uuid::Uuid::new_v4().to_string(),
                &login.address,
                &login.username,
                secrets.seal(login.secret.as_bytes())?,
            ),
        )
        .map_err(sqlite_error)?;
    }
    Ok(())
}
