//! Persistence for repositories. A deploy key's private half and a token
//! are sealed before they reach SQLite and leave this crate only as a
//! [`Secret`], handed to one git command at a time.

use std::{fmt, sync::Arc};

use rusqlite::{OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_storage::{Db, SecretBox};
use homeport_transport::ssh;

use crate::{
    messages as msg,
    url::{RepoUrl, Transport},
};

/// How Homeport authenticates to a repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum RepoAuth {
    /// A deploy key Homeport generated, over SSH.
    SshKey,
    /// A user name and access token, over HTTPS.
    HttpsToken,
    /// Nothing: a public repository over HTTPS.
    None,
}

impl RepoAuth {
    fn text(self) -> &'static str {
        match self {
            Self::SshKey => "ssh_key",
            Self::HttpsToken => "https_token",
            Self::None => "none",
        }
    }

    fn from_text(text: &str) -> Self {
        match text {
            "ssh_key" => Self::SshKey,
            "https_token" => Self::HttpsToken,
            _ => Self::None,
        }
    }
}

/// A stored repository, as the frontend sees it: never a secret.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: String,
    pub url: String,
    pub auth: RepoAuth,
    /// The deploy key to add to the repository, in OpenSSH format.
    pub public_key: Option<String>,
    /// Its `SHA256:…` fingerprint, as forges show it.
    pub key_fingerprint: Option<String>,
    pub username: Option<String>,
    pub has_token: bool,
    /// The `SHA256:…` fingerprint of the SSH server's key, once trusted.
    pub host_key_fingerprint: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// What the user typed to add a repository.
#[derive(Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryDraft {
    pub url: String,
    pub auth: RepoAuth,
    pub username: Option<String>,
    pub token: Option<String>,
}

impl fmt::Debug for RepositoryDraft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepositoryDraft")
            .field("url", &self.url)
            .field("auth", &self.auth)
            .field("username", &self.username)
            .field("token", &self.token.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

/// A repository's credentials, opened for one command. Its `Debug` hides
/// them, so they can't reach a log by accident.
#[derive(Clone, PartialEq, Eq)]
pub enum Secret {
    None,
    SshKey { private_key: String },
    Token { username: String, token: String },
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::SshKey { .. } => f.write_str("SshKey(<redacted>)"),
            Self::Token { username, .. } => write!(f, "Token({username}, <redacted>)"),
        }
    }
}

/// Everything one git command needs to reach a repository.
#[derive(Debug, Clone)]
pub struct Access {
    pub repository: Repository,
    pub secret: Secret,
    /// The trusted `known_hosts` line of an SSH server, if any yet.
    pub host_key: Option<String>,
}

/// A row's private key, user name, token and trusted host key, still sealed.
type SealedRow = (
    Option<Vec<u8>>,
    Option<String>,
    Option<Vec<u8>>,
    Option<String>,
);

pub struct RepositoryStore {
    db: Arc<Db>,
    secrets: Arc<SecretBox>,
}

const COLUMNS: &str =
    "id, url, auth, public_key, username, token IS NOT NULL, host_key, created_at, updated_at";

impl RepositoryStore {
    pub fn new(db: Arc<Db>, secrets: Arc<SecretBox>) -> Self {
        Self { db, secrets }
    }

    pub fn list(&self) -> AppResult<Vec<Repository>> {
        self.db.with(|c| {
            c.prepare(&format!(
                "SELECT {COLUMNS} FROM git_repositories ORDER BY url"
            ))?
            .query_map([], read_row)?
            .collect()
        })
    }

    pub fn get(&self, id: &str) -> AppResult<Repository> {
        self.db
            .with(|c| {
                c.query_row(
                    &format!("SELECT {COLUMNS} FROM git_repositories WHERE id = ?1"),
                    [id],
                    read_row,
                )
                .optional()
            })?
            .ok_or_else(|| AppError::not_found(msg::GONE))
    }

    /// Adds a repository: with a deploy key generated for it, a token, or
    /// nothing for a public HTTPS one.
    pub fn create(&self, draft: &RepositoryDraft) -> AppResult<Repository> {
        let url = draft.url.trim();
        let parsed = RepoUrl::parse(url)?;
        match (draft.auth, parsed.transport) {
            (RepoAuth::SshKey, Transport::Https) => {
                return Err(AppError::validation(msg::KEY_NEEDS_SSH))
            }
            (RepoAuth::HttpsToken, Transport::Ssh) => {
                return Err(AppError::validation(msg::TOKEN_NEEDS_HTTPS))
            }
            (RepoAuth::None, Transport::Ssh) => {
                return Err(AppError::validation(msg::SSH_NEEDS_KEY))
            }
            _ => {}
        }
        let (private_key, public_key) = match draft.auth {
            RepoAuth::SshKey => {
                let key = ssh::generate_key(&key_comment(&parsed));
                (
                    Some(self.secrets.seal(key.private_key.as_bytes())?),
                    Some(key.public_key),
                )
            }
            _ => (None, None),
        };
        let (username, token) = match draft.auth {
            RepoAuth::HttpsToken => {
                let (username, token) =
                    token_fields(draft.username.as_deref(), draft.token.as_deref())?;
                (Some(username), Some(self.secrets.seal(token.as_bytes())?))
            }
            _ => (None, None),
        };
        let id = uuid::Uuid::new_v4().to_string();
        self.db
            .with(|c| {
                c.execute(
                    "INSERT INTO git_repositories (id, url, auth, private_key, public_key, username, token)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    (&id, url, draft.auth.text(), &private_key, &public_key, &username, &token),
                )
            })
            .map_err(|e| match e.code {
                ErrorCode::NameInUse => e
                    .with_message(msg::REPOSITORY_EXISTS.with("url", url))
                    .with_hint(msg::REPOSITORY_EXISTS_HINT),
                _ => e,
            })?;
        self.get(&id)
    }

    /// Replaces a repository's deploy key with a new one, which the user
    /// adds to the repository in place of the old.
    pub fn rotate_key(&self, id: &str) -> AppResult<Repository> {
        let repository = self.get(id)?;
        if repository.auth != RepoAuth::SshKey {
            return Err(AppError::validation(msg::WRONG_AUTH));
        }
        let key = ssh::generate_key(&key_comment(&RepoUrl::parse(&repository.url)?));
        let sealed = self.secrets.seal(key.private_key.as_bytes())?;
        self.touch(
            "UPDATE git_repositories SET private_key = ?2, public_key = ?3,
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
            (id, &sealed, &key.public_key),
        )?;
        self.get(id)
    }

    /// Replaces a repository's token, and its user name.
    pub fn set_token(&self, id: &str, username: &str, token: &str) -> AppResult<Repository> {
        if self.get(id)?.auth != RepoAuth::HttpsToken {
            return Err(AppError::validation(msg::WRONG_AUTH));
        }
        let (username, token) = token_fields(Some(username), Some(token))?;
        let sealed = self.secrets.seal(token.as_bytes())?;
        self.touch(
            "UPDATE git_repositories SET username = ?2, token = ?3,
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
            (id, &username, &sealed),
        )?;
        self.get(id)
    }

    /// Trusts `line`, the SSH server's `known_hosts` entry, from now on.
    pub fn set_host_key(&self, id: &str, line: &str) -> AppResult<()> {
        self.touch(
            "UPDATE git_repositories SET host_key = ?2 WHERE id = ?1",
            (id, line),
        )
    }

    /// Forgets the trusted SSH server key, so the next command trusts the
    /// key the server presents then.
    pub fn reset_host_key(&self, id: &str) -> AppResult<Repository> {
        self.touch(
            "UPDATE git_repositories SET host_key = NULL,
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
            [id],
        )?;
        self.get(id)
    }

    /// Removes a repository, refused while a Synced Stack uses it.
    pub fn delete(&self, id: &str) -> AppResult<()> {
        let in_use = self.db.with(|c| {
            let tx = c.transaction()?;
            let in_use: bool = tx.query_row(
                "SELECT EXISTS (SELECT 1 FROM synced_stacks WHERE repository_id = ?1)",
                [id],
                |r| r.get(0),
            )?;
            if !in_use {
                tx.execute("DELETE FROM git_repositories WHERE id = ?1", [id])?;
            }
            tx.commit()?;
            Ok(in_use)
        })?;
        if in_use {
            return Err(AppError::new(ErrorCode::InUse, msg::IN_USE).with_hint(msg::IN_USE_HINT));
        }
        Ok(())
    }

    /// The repository with its credentials opened, for one command.
    pub fn access(&self, id: &str) -> AppResult<Access> {
        let repository = self.get(id)?;
        let (private_key, username, token, host_key): SealedRow = self.db.with(|c| {
            c.query_row(
                "SELECT private_key, username, token, host_key FROM git_repositories WHERE id = ?1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
        })?;
        let open = |sealed: Vec<u8>| -> AppResult<String> {
            Ok(String::from_utf8_lossy(&self.secrets.open(&sealed)?).into_owned())
        };
        let secret = match (repository.auth, private_key, token) {
            (RepoAuth::SshKey, Some(key), _) => Secret::SshKey {
                private_key: open(key)?,
            },
            (RepoAuth::HttpsToken, _, Some(token)) => Secret::Token {
                username: username.unwrap_or_default(),
                token: open(token)?,
            },
            _ => Secret::None,
        };
        Ok(Access {
            repository,
            secret,
            host_key,
        })
    }

    fn touch(&self, sql: &str, params: impl rusqlite::Params) -> AppResult<()> {
        let n = self.db.with(|c| c.execute(sql, params))?;
        if n == 0 {
            return Err(AppError::not_found(msg::GONE));
        }
        Ok(())
    }
}

/// `homeport-<repository name>`, so the key is recognisable in a forge's list.
fn key_comment(url: &RepoUrl) -> String {
    let name = url
        .path
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .trim_end_matches(".git");
    format!("homeport-{name}")
}

fn token_fields(username: Option<&str>, token: Option<&str>) -> AppResult<(String, String)> {
    let username = username.map(str::trim).unwrap_or_default();
    if username.is_empty() {
        return Err(AppError::validation(msg::USERNAME_EMPTY));
    }
    let token = token.map(str::trim).unwrap_or_default();
    if token.is_empty() {
        return Err(AppError::validation(msg::TOKEN_EMPTY));
    }
    Ok((username.to_owned(), token.to_owned()))
}

/// The fingerprint of a `known_hosts` line's key.
fn host_key_fingerprint(line: &str) -> Option<String> {
    let mut parts = line.split_whitespace().skip(1);
    let key = format!("{} {}", parts.next()?, parts.next()?);
    ssh::fingerprint(&key)
}

fn read_row(row: &Row<'_>) -> rusqlite::Result<Repository> {
    let public_key: Option<String> = row.get(3)?;
    let host_key: Option<String> = row.get(6)?;
    Ok(Repository {
        id: row.get(0)?,
        url: row.get(1)?,
        auth: RepoAuth::from_text(&row.get::<_, String>(2)?),
        key_fingerprint: public_key.as_deref().and_then(ssh::fingerprint),
        public_key,
        username: row.get(4)?,
        has_token: row.get(5)?,
        host_key_fingerprint: host_key.as_deref().and_then(host_key_fingerprint),
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
    })
}

#[cfg(test)]
mod tests {
    use homeport_storage::KeyStore;

    use super::*;

    fn store(secrets: SecretBox) -> (RepositoryStore, Arc<Db>) {
        let db = Arc::new(Db::open_in_memory().unwrap());
        (RepositoryStore::new(db.clone(), Arc::new(secrets)), db)
    }

    fn draft(url: &str, auth: RepoAuth) -> RepositoryDraft {
        RepositoryDraft {
            url: url.into(),
            auth,
            username: Some("deploy".into()),
            token: Some("glpat-hunter2".into()),
        }
    }

    #[test]
    fn deploy_keys_are_generated_and_sealed() {
        let (store, db) = store(SecretBox::from_key(&[4; 32]));
        let repo = store
            .create(&draft("git@github.com:owner/shop.git", RepoAuth::SshKey))
            .unwrap();
        let public = repo.public_key.clone().unwrap();
        assert!(public.starts_with("ssh-ed25519 ") && public.ends_with(" homeport-shop"));
        assert!(repo
            .key_fingerprint
            .as_deref()
            .unwrap()
            .starts_with("SHA256:"));
        assert_eq!((repo.username.as_deref(), repo.has_token), (None, false));

        let Secret::SshKey { private_key } = store.access(&repo.id).unwrap().secret else {
            panic!("no key");
        };
        assert!(private_key.contains("OPENSSH PRIVATE KEY"));
        // Nothing in the row reads as the key.
        let sealed: Vec<u8> = db
            .with(|c| c.query_row("SELECT private_key FROM git_repositories", [], |r| r.get(0)))
            .unwrap();
        assert!(!String::from_utf8_lossy(&sealed).contains("PRIVATE KEY"));

        let rotated = store.rotate_key(&repo.id).unwrap();
        assert_ne!(rotated.public_key, repo.public_key);
        assert!(store.set_token(&repo.id, "a", "b").is_err());
    }

    #[test]
    fn tokens_are_sealed_and_replaced() {
        let (store, db) = store(SecretBox::from_key(&[4; 32]));
        let repo = store
            .create(&draft(
                "https://gitlab.com/owner/shop.git",
                RepoAuth::HttpsToken,
            ))
            .unwrap();
        assert!(repo.has_token);
        assert_eq!(repo.username.as_deref(), Some("deploy"));
        assert!(!serde_json::to_string(&repo).unwrap().contains("hunter2"));
        assert!(!format!("{:?}", store.access(&repo.id).unwrap()).contains("hunter2"));
        let dump: String = db
            .with(|c| {
                c.query_row(
                    "SELECT quote(token) || username FROM git_repositories",
                    [],
                    |r| r.get(0),
                )
            })
            .unwrap();
        assert!(!dump.contains("hunter2"));

        store.set_token(&repo.id, "bot", "new-token").unwrap();
        assert_eq!(
            store.access(&repo.id).unwrap().secret,
            Secret::Token {
                username: "bot".into(),
                token: "new-token".into()
            }
        );
        assert!(store.set_token(&repo.id, "bot", " ").is_err());
        assert!(store.rotate_key(&repo.id).is_err());
    }

    #[test]
    fn the_url_and_the_kind_of_access_must_agree() {
        let (store, _) = store(SecretBox::from_key(&[4; 32]));
        for (url, auth) in [
            ("https://github.com/o/r.git", RepoAuth::SshKey),
            ("git@github.com:o/r.git", RepoAuth::HttpsToken),
            ("git@github.com:o/r.git", RepoAuth::None),
            ("http://github.com/o/r.git", RepoAuth::None),
        ] {
            assert!(
                matches!(store.create(&draft(url, auth)), Err(ref e) if e.is(ErrorCode::Validation)),
                "{url} {auth:?}"
            );
        }
        let public = store
            .create(&draft("https://github.com/o/r.git", RepoAuth::None))
            .unwrap();
        assert_eq!(store.access(&public.id).unwrap().secret, Secret::None);
        assert!(matches!(
            store.create(&draft(" https://github.com/o/r.git ", RepoAuth::None)),
            Err(ref e) if e.is(ErrorCode::NameInUse) && e.hint.is_some()
        ));
    }

    #[test]
    fn host_keys_are_trusted_then_reset() {
        let (store, _) = store(SecretBox::from_key(&[4; 32]));
        let repo = store
            .create(&draft("git@github.com:owner/shop.git", RepoAuth::SshKey))
            .unwrap();
        let server = ssh::generate_key("server").public_key;
        let line = format!(
            "github.com {}",
            server.split(' ').take(2).collect::<Vec<_>>().join(" ")
        );
        store.set_host_key(&repo.id, &line).unwrap();
        assert_eq!(
            store.access(&repo.id).unwrap().host_key.as_deref(),
            Some(line.as_str())
        );
        assert_eq!(
            store.get(&repo.id).unwrap().host_key_fingerprint,
            ssh::fingerprint(&server)
        );
        let reset = store.reset_host_key(&repo.id).unwrap();
        assert_eq!(reset.host_key_fingerprint, None);
        assert_eq!(store.access(&repo.id).unwrap().host_key, None);
    }

    #[test]
    fn repositories_in_use_are_not_removed() {
        let (store, db) = store(SecretBox::from_key(&[4; 32]));
        let repo = store
            .create(&draft("https://github.com/o/r.git", RepoAuth::None))
            .unwrap();
        db.with(|c| {
            c.execute(
                "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', '{\"kind\":\"local\",\"socketPath\":\"/var/run/docker.sock\"}')",
                [],
            )?;
            c.execute(
                "INSERT INTO synced_stacks (id, host_id, name, repository_id, ref_kind, ref_name, compose_path)
                 VALUES ('s', 'h', 'shop', ?1, 'branch', 'main', 'compose.yaml')",
                [&repo.id],
            )
        })
        .unwrap();
        assert!(matches!(store.delete(&repo.id), Err(ref e) if e.is(ErrorCode::InUse)));
        db.with(|c| c.execute("DELETE FROM synced_stacks", []))
            .unwrap();
        store.delete(&repo.id).unwrap();
        assert!(matches!(store.get(&repo.id), Err(ref e) if e.is(ErrorCode::NotFound)));
    }

    #[test]
    fn without_a_keychain_only_public_repositories_can_be_added() {
        struct NoKeychain;
        impl KeyStore for NoKeychain {
            fn load_or_create(&self) -> Result<homeport_storage::crypto::Key, String> {
                Err("none".into())
            }
        }
        let (store, _) = store(SecretBox::from_store(&NoKeychain));
        for (url, auth) in [
            ("git@github.com:o/r.git", RepoAuth::SshKey),
            ("https://gitlab.com/o/r.git", RepoAuth::HttpsToken),
        ] {
            assert!(matches!(
                store.create(&draft(url, auth)),
                Err(ref e) if e.is(ErrorCode::KeychainUnavailable)
            ));
        }
        assert!(store
            .create(&draft("https://github.com/o/r.git", RepoAuth::None))
            .is_ok());
    }
}
