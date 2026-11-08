//! The SQLite store: a single connection behind a mutex, migrated on open.
//! Docker state is never cached here; the daemon is the source of truth.

mod error;

use std::{path::Path, sync::Mutex};

use rusqlite::{Connection, OptionalExtension};
use rusqlite_migration::{Migrations, M};

use homeport_core::error::AppResult;

pub use self::error::sqlite_error;

use self::error::migration_error;

const MIGRATION_SLICE: &[M<'static>] = &[
    M::up(include_str!("migrations/0001_settings.sql")),
    M::up(include_str!("migrations/0002_hosts.sql")),
    M::up(include_str!("migrations/0003_stacks.sql")),
    M::up(include_str!("migrations/0004_registries.sql")),
    M::up(include_str!("migrations/0005_bastions.sql")),
    M::up(include_str!("migrations/0006_linked_stacks.sql")),
    M::up(include_str!("migrations/0007_stack_files.sql")),
    M::up(include_str!("migrations/0008_stack_profiles.sql")),
    M::up(include_str!("migrations/0009_stack_deploys.sql")),
    M::up(include_str!("migrations/0010_git_repositories.sql")),
    M::up(include_str!("migrations/0011_synced_stacks.sql")),
    M::up(include_str!("migrations/0012_stack_deploy_commits.sql")),
    M::up(include_str!("migrations/0013_synced_polling.sql")),
    M::up(include_str!("migrations/0014_engine_events.sql")),
    M::up(include_str!("migrations/0015_swarm_stacks.sql")),
    M::up(include_str!("migrations/0016_clusters.sql")),
];
const MIGRATIONS: Migrations<'static> = Migrations::from_slice(MIGRATION_SLICE);

pub struct Db {
    conn: Mutex<Connection>,
}

/// Brings `conn`'s schema to the latest version, then turns foreign keys on.
///
/// Migrations run with foreign keys off, as SQLite asks for rebuilding a
/// table (0015): dropping the old table would otherwise cascade to the rows
/// that point at it. They're checked once every migration has run.
fn migrate(conn: &mut Connection) -> AppResult<()> {
    conn.pragma_update(None, "foreign_keys", "OFF")
        .map_err(sqlite_error)?;
    MIGRATIONS.to_latest(conn).map_err(migration_error)?;
    let broken: Option<String> = conn
        .query_row("PRAGMA foreign_key_check", [], |r| r.get(0))
        .optional()
        .map_err(sqlite_error)?;
    if let Some(table) = broken {
        return Err(error::foreign_keys_broken(&table));
    }
    conn.pragma_update(None, "foreign_keys", "ON")
        .map_err(sqlite_error)
}

impl Db {
    pub fn open(path: &Path) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path).map_err(sqlite_error)?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(sqlite_error)?;
        Self::init(conn)
    }

    /// A database that lives as long as the value, for tests and throwaway stores.
    pub fn open_in_memory() -> AppResult<Self> {
        Self::init(Connection::open_in_memory().map_err(sqlite_error)?)
    }

    fn init(mut conn: Connection) -> AppResult<Self> {
        migrate(&mut conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Runs `f` with exclusive access to the connection.
    pub fn with<T>(&self, f: impl FnOnce(&mut Connection) -> rusqlite::Result<T>) -> AppResult<T> {
        let mut conn = self.conn.lock().expect("db lock poisoned");
        f(&mut conn).map_err(sqlite_error)
    }

    /// Runs `f` in one transaction, committed only if it succeeds: on an
    /// error, nothing it wrote is kept.
    pub fn transaction<T>(
        &self,
        f: impl FnOnce(&rusqlite::Transaction<'_>) -> AppResult<T>,
    ) -> AppResult<T> {
        let mut conn = self.conn.lock().expect("db lock poisoned");
        let tx = conn.transaction().map_err(sqlite_error)?;
        let value = f(&tx)?;
        tx.commit().map_err(sqlite_error)?;
        Ok(value)
    }

    /// Writes a copy of the database as it is now to `path`, which must not
    /// exist (`VACUUM INTO`): a consistent snapshot, whatever is in the WAL.
    pub fn copy_to(&self, path: &Path) -> AppResult<()> {
        let target = path.to_string_lossy();
        self.with(|c| c.execute("VACUUM INTO ?1", [target.as_ref()]).map(drop))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transactions_keep_nothing_of_a_failure() {
        let db = Db::open_in_memory().unwrap();
        let put = |tx: &rusqlite::Transaction<'_>, key: &str| {
            tx.execute("INSERT INTO settings (key, value) VALUES (?1, '{}')", [key])
                .map_err(sqlite_error)
        };
        let failed: AppResult<()> = db.transaction(|tx| {
            put(tx, "a")?;
            Err(homeport_core::error::AppError::validation(
                homeport_core::messages::CONTAINER_GONE,
            ))
        });
        assert!(failed.is_err());
        db.transaction(|tx| put(tx, "b")).unwrap();
        let keys: Vec<String> = db
            .with(|c| {
                c.prepare("SELECT key FROM settings")?
                    .query_map([], |r| r.get(0))?
                    .collect()
            })
            .unwrap();
        assert_eq!(keys, ["b"]);
    }

    #[test]
    fn copies_are_snapshots_of_the_database() {
        let dir = tempfile::tempdir().unwrap();
        let db = Db::open(&dir.path().join("homeport.db")).unwrap();
        db.with(|c| c.execute("INSERT INTO settings (key, value) VALUES ('k', '1')", []))
            .unwrap();
        let copy = dir.path().join("copy.db");
        db.copy_to(&copy).unwrap();
        let copied = Db::open(&copy).unwrap();
        let value: String = copied
            .with(|c| {
                c.query_row("SELECT value FROM settings WHERE key = 'k'", [], |r| {
                    r.get(0)
                })
            })
            .unwrap();
        assert_eq!(value, "1");
    }

    #[test]
    fn migrations_are_valid() {
        MIGRATIONS.validate().unwrap();
    }

    #[test]
    fn databases_from_0_11_keep_engine_events_until_their_host_goes() {
        let mut conn = Connection::open_in_memory().unwrap();
        MIGRATIONS.to_version(&mut conn, 13).unwrap();
        let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
        conn.execute(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', ?1)",
            [local],
        )
        .unwrap();
        migrate(&mut conn).unwrap();

        let insert = "INSERT OR IGNORE INTO engine_events
            (host_id, time_nano, kind, action, actor_id, attributes)
            VALUES ('h', 42, 'container', 'start', 'abc', '{}')";
        assert_eq!(conn.execute(insert, []).unwrap(), 1);
        assert_eq!(conn.execute(insert, []).unwrap(), 0, "kept once");
        conn.execute("DELETE FROM hosts WHERE id = 'h'", [])
            .unwrap();
        let left: i64 = conn
            .query_row("SELECT count(*) FROM engine_events", [], |r| r.get(0))
            .unwrap();
        assert_eq!(left, 0);
    }

    #[test]
    fn databases_from_0_14_gain_clusters_and_keep_their_hosts() {
        let mut conn = Connection::open_in_memory().unwrap();
        MIGRATIONS.to_version(&mut conn, 15).unwrap();
        let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
        conn.execute(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', ?1)",
            [local],
        )
        .unwrap();
        migrate(&mut conn).unwrap();

        let hosts: i64 = conn
            .query_row("SELECT count(*) FROM hosts", [], |r| r.get(0))
            .unwrap();
        assert_eq!(hosts, 1);
        conn.execute(
            "INSERT INTO clusters (id, name, connection) VALUES ('c', 'kind-dev', '{}')",
            [],
        )
        .unwrap();
        let (kind, enabled): (String, bool) = conn
            .query_row("SELECT kind, enabled FROM clusters", [], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .unwrap();
        assert_eq!((kind.as_str(), enabled), ("kubernetes", true));
        let duplicate = conn.execute(
            "INSERT INTO clusters (id, name, connection) VALUES ('d', 'KIND-DEV', '{}')",
            [],
        );
        assert!(duplicate.is_err(), "names are unique whatever their case");
    }

    #[test]
    fn stacks_from_0_13_keep_their_host_and_files() {
        let mut conn = Connection::open_in_memory().unwrap();
        MIGRATIONS.to_version(&mut conn, 14).unwrap();
        let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
        conn.execute_batch(&format!(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', '{local}');
             INSERT INTO stacks (id, host_id, name, env, profiles) VALUES ('m', 'h', 'shop', NULL, '[\"debug\"]');
             INSERT INTO stack_files (stack_id, position, name, content) VALUES ('m', 0, 'compose.yaml', 'services: {{}}');
             INSERT INTO linked_stacks (id, host_id, name, working_dir, config_files, fingerprint)
                 VALUES ('l', 'h', 'blog', '/srv/blog', '[]', 'f');
             INSERT INTO git_repositories (id, url, auth) VALUES ('r', 'https://example.com/a.git', 'none');
             INSERT INTO synced_stacks (id, host_id, name, repository_id, ref_kind, ref_name, compose_path, auto_deploy)
                 VALUES ('s', 'h', 'site', 'r', 'branch', 'main', 'compose.yaml', 1);
             INSERT INTO stack_deploys (id, host_id, stack_id, stack_kind, stack_name, action, definition)
                 VALUES ('d', 'h', 'm', 'managed', 'shop', 'up', '[]');"
        ))
        .unwrap();
        migrate(&mut conn).unwrap();

        let owner = |table: &str| -> (Option<String>, Option<String>) {
            conn.query_row(&format!("SELECT host_id, swarm_id FROM {table}"), [], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .unwrap()
        };
        for table in ["stacks", "linked_stacks", "synced_stacks", "stack_deploys"] {
            assert_eq!(owner(table), (Some("h".into()), None), "{table}");
        }
        let files: i64 = conn
            .query_row("SELECT count(*) FROM stack_files", [], |r| r.get(0))
            .unwrap();
        assert_eq!(files, 1, "rebuilding stacks keeps its files");
        let kept: (String, bool) = conn
            .query_row("SELECT profiles, auto_deploy FROM synced_stacks", [], |r| {
                Ok((r.get::<_, String>(0)?, r.get(1)?))
            })
            .unwrap();
        assert_eq!(kept, ("[]".to_owned(), true));

        // A Stack belongs to a Host or a Swarm, never both or neither.
        for (host, swarm) in [("NULL", "NULL"), ("'h'", "'s1'")] {
            let insert = format!(
                "INSERT INTO stacks (id, host_id, swarm_id, name) VALUES ('x', {host}, {swarm}, 'x')"
            );
            assert!(conn.execute(&insert, []).is_err(), "{host} {swarm}");
        }
        conn.execute(
            "INSERT INTO stacks (id, swarm_id, name) VALUES ('w', 's1', 'shop')",
            [],
        )
        .unwrap();
        // Removing the Host keeps the Swarm's Stack and drops its own.
        conn.execute("DELETE FROM hosts WHERE id = 'h'", [])
            .unwrap();
        let left: Vec<String> = conn
            .prepare("SELECT id FROM stacks")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(left, ["w"]);
    }

    #[test]
    fn ssh_hosts_from_before_bastions_get_an_empty_list() {
        let mut conn = Connection::open_in_memory().unwrap();
        MIGRATIONS.to_version(&mut conn, 4).unwrap();
        let ssh = r#"{"kind":"ssh","host":"h","port":22,"user":"u","socketPath":"/var/run/docker.sock","auth":"agent","publicKey":null}"#;
        let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
        conn.execute(
            "INSERT INTO hosts (id, name, connection, host_key) VALUES ('a', 'a', ?1, 'ssh-ed25519 KEY'), ('b', 'b', ?2, NULL)",
            (ssh, local),
        )
        .unwrap();
        migrate(&mut conn).unwrap();

        let read = |id: &str| -> (serde_json::Value, Option<String>) {
            conn.query_row(
                "SELECT connection, host_key FROM hosts WHERE id = ?1",
                [id],
                |r| {
                    Ok((
                        serde_json::from_str(&r.get::<_, String>(0)?).unwrap(),
                        r.get(1)?,
                    ))
                },
            )
            .unwrap()
        };
        let (a, key) = read("a");
        assert_eq!(a["bastions"], serde_json::json!([]));
        assert_eq!(a["host"], "h");
        assert_eq!(key.as_deref(), Some("ssh-ed25519 KEY"));
        assert!(read("b").0.get("bastions").is_none());
    }

    #[test]
    fn databases_from_0_9_keep_their_hosts_and_stacks() {
        let mut conn = Connection::open_in_memory().unwrap();
        MIGRATIONS.to_version(&mut conn, 5).unwrap();
        let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
        conn.execute(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', ?1)",
            [local],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO stacks (id, host_id, name, compose) VALUES ('s', 'h', 'shop', 'services: {}')",
            [],
        )
        .unwrap();
        migrate(&mut conn).unwrap();

        let stacks: i64 = conn
            .query_row("SELECT count(*) FROM stacks WHERE host_id = 'h'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(stacks, 1);
        // Its one definition is now its first compose file.
        let file: (i64, String, String) = conn
            .query_row(
                "SELECT position, name, content FROM stack_files WHERE stack_id = 's'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert_eq!(file, (0, "compose.yaml".into(), "services: {}".into()));
        // It deploys no profile until one is turned on.
        let profiles: String = conn
            .query_row("SELECT profiles FROM stacks WHERE id = 's'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(profiles, "[]");
        // No deploy was recorded before history existed.
        let deploys: i64 = conn
            .query_row("SELECT count(*) FROM stack_deploys", [], |r| r.get(0))
            .unwrap();
        assert_eq!(deploys, 0);
        let linked: i64 = conn
            .query_row("SELECT count(*) FROM linked_stacks", [], |r| r.get(0))
            .unwrap();
        assert_eq!(linked, 0);
    }

    #[test]
    fn databases_from_0_10_keep_their_stacks_and_history() {
        let mut conn = Connection::open_in_memory().unwrap();
        MIGRATIONS.to_version(&mut conn, 9).unwrap();
        let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
        conn.execute_batch(&format!(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', '{local}');
             INSERT INTO stacks (id, host_id, name) VALUES ('s', 'h', 'shop');
             INSERT INTO linked_stacks (id, host_id, name, working_dir, config_files, fingerprint)
                 VALUES ('l', 'h', 'blog', '/srv/blog', '[]', 'fp');
             INSERT INTO stack_deploys (id, host_id, stack_id, stack_kind, stack_name, action, definition)
                 VALUES ('d', 'h', 's', 'managed', 'shop', 'up', '[]');"
        ))
        .unwrap();
        migrate(&mut conn).unwrap();

        let count = |table: &str| -> i64 {
            conn.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))
                .unwrap()
        };
        assert_eq!(
            (
                count("stacks"),
                count("linked_stacks"),
                count("stack_deploys")
            ),
            (1, 1, 1)
        );
        assert_eq!((count("git_repositories"), count("synced_stacks")), (0, 0));
        let commit: Option<String> = conn
            .query_row(
                "SELECT commit_hash FROM stack_deploys WHERE id = 'd'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(commit, None, "deploys from before 0.11 have no commit");
        // A repository can't go while a Synced Stack uses it.
        conn.execute_batch(
            "INSERT INTO git_repositories (id, url, auth) VALUES ('r', 'https://example.com/r.git', 'none');
             INSERT INTO synced_stacks (id, host_id, name, repository_id, ref_kind, ref_name, compose_path)
                 VALUES ('y', 'h', 'site', 'r', 'branch', 'main', 'compose.yaml');",
        )
        .unwrap();
        assert!(conn
            .execute("DELETE FROM git_repositories WHERE id = 'r'", [])
            .is_err());
        // Synced Stacks poll by default, and never deploy on their own until asked.
        let (poll, auto): (bool, bool) = conn
            .query_row(
                "SELECT poll, auto_deploy FROM synced_stacks WHERE id = 'y'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!((poll, auto), (true, false));
        // Removing the Host removes its Synced Stacks.
        conn.execute("DELETE FROM hosts WHERE id = 'h'", [])
            .unwrap();
        assert_eq!(count("synced_stacks"), 0);
    }

    #[test]
    fn reopening_a_file_keeps_data() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("homeport.db");
        Db::open(&path)
            .unwrap()
            .with(|c| c.execute("INSERT INTO settings VALUES ('k', '1')", []))
            .unwrap();

        let value: String = Db::open(&path)
            .unwrap()
            .with(|c| {
                c.query_row("SELECT value FROM settings WHERE key = 'k'", [], |r| {
                    r.get(0)
                })
            })
            .unwrap();
        assert_eq!(value, "1");
    }
}
