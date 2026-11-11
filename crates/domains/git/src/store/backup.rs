//! Repositories in a backup of Homeport's data, with their deploy keys and
//! tokens opened; restored with this computer's key, under the same ids so
//! their Synced Stacks find them.

use std::fmt;

use homeport_storage::db::sqlite_error;

use super::*;

/// A repository as a backup holds it.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackedUpRepository {
    pub id: String,
    pub url: String,
    pub auth: RepoAuth,
    /// The deploy key's private half, in OpenSSH format.
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    pub username: Option<String>,
    pub token: Option<String>,
    /// The SSH server's trusted `known_hosts` line.
    pub host_key: Option<String>,
}

impl fmt::Debug for BackedUpRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackedUpRepository")
            .field("url", &self.url)
            .finish_non_exhaustive()
    }
}

type Row = (
    String,
    String,
    String,
    Option<Vec<u8>>,
    Option<String>,
    Option<String>,
    Option<Vec<u8>>,
    Option<String>,
);

impl RepositoryStore {
    /// Every repository with its key or token; one whose secret can't be
    /// opened is kept without it, and its URL named in `lost`.
    pub fn backup(&self, lost: &mut Vec<String>) -> AppResult<Vec<BackedUpRepository>> {
        let rows: Vec<Row> = self.db.with(|c| {
            c.prepare(
                "SELECT id, url, auth, private_key, public_key, username, token, host_key
                 FROM git_repositories ORDER BY url",
            )?
            .query_map([], |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                    r.get(7)?,
                ))
            })?
            .collect()
        })?;
        let open = |sealed: Option<Vec<u8>>, missing: &mut bool| {
            sealed.and_then(|s| {
                let opened = self
                    .secrets
                    .open(&s)
                    .ok()
                    .and_then(|s| String::from_utf8(s).ok());
                *missing |= opened.is_none();
                opened
            })
        };
        let mut repositories = Vec::with_capacity(rows.len());
        for (id, url, auth, private_key, public_key, username, token, host_key) in rows {
            let mut missing = false;
            let private_key = open(private_key, &mut missing);
            let token = open(token, &mut missing);
            if missing {
                lost.push(url.clone());
            }
            repositories.push(BackedUpRepository {
                id,
                url,
                auth: RepoAuth::from_text(&auth),
                private_key,
                public_key,
                username,
                token,
                host_key,
            });
        }
        Ok(repositories)
    }
}

/// Writes a backup's repositories in `tx`, into a database emptied of them.
pub fn restore(
    tx: &rusqlite::Transaction<'_>,
    secrets: &SecretBox,
    repositories: &[BackedUpRepository],
) -> AppResult<()> {
    let seal = |opened: &Option<String>| {
        opened
            .as_ref()
            .map(|s| secrets.seal(s.as_bytes()))
            .transpose()
    };
    for repo in repositories {
        tx.execute(
            "INSERT INTO git_repositories
                 (id, url, auth, private_key, public_key, username, token, host_key)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                &repo.id,
                &repo.url,
                repo.auth.text(),
                seal(&repo.private_key)?,
                &repo.public_key,
                &repo.username,
                seal(&repo.token)?,
                &repo.host_key,
            ),
        )
        .map_err(sqlite_error)?;
    }
    Ok(())
}
