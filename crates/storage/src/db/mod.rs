//! The SQLite store: a single connection behind a mutex, migrated on open.
//! Docker state is never cached here; the daemon is the source of truth.

mod error;

use std::{path::Path, sync::Mutex};

use rusqlite::Connection;
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
];
const MIGRATIONS: Migrations<'static> = Migrations::from_slice(MIGRATION_SLICE);

pub struct Db {
    conn: Mutex<Connection>,
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
        conn.pragma_update(None, "foreign_keys", "ON")
            .map_err(sqlite_error)?;
        MIGRATIONS.to_latest(&mut conn).map_err(migration_error)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Runs `f` with exclusive access to the connection.
    pub fn with<T>(&self, f: impl FnOnce(&mut Connection) -> rusqlite::Result<T>) -> AppResult<T> {
        let mut conn = self.conn.lock().expect("db lock poisoned");
        f(&mut conn).map_err(sqlite_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_are_valid() {
        MIGRATIONS.validate().unwrap();
    }

    #[test]
    fn databases_from_0_11_keep_engine_events_until_their_host_goes() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();
        MIGRATIONS.to_version(&mut conn, 13).unwrap();
        let local = r#"{"kind":"local","socketPath":"/var/run/docker.sock"}"#;
        conn.execute(
            "INSERT INTO hosts (id, name, connection) VALUES ('h', 'local', ?1)",
            [local],
        )
        .unwrap();
        MIGRATIONS.to_latest(&mut conn).unwrap();

        let insert = "INSERT OR IGNORE INTO engine_events
            (host_id, time_nano, kind, action, actor_id, attributes)
            VALUES ('h', 42, 'container', 'start', 'abc', '{}')";
        assert_eq!(conn.execute(insert, []).unwrap(), 1);
        assert_eq!(conn.execute(insert, []).unwrap(), 0, "kept once");
        conn.execute("DELETE FROM hosts WHERE id = 'h'", []).unwrap();
        let left: i64 = conn
            .query_row("SELECT count(*) FROM engine_events", [], |r| r.get(0))
            .unwrap();
        assert_eq!(left, 0);
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
        MIGRATIONS.to_latest(&mut conn).unwrap();

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
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();
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
        MIGRATIONS.to_latest(&mut conn).unwrap();

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
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();
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
        MIGRATIONS.to_latest(&mut conn).unwrap();

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
