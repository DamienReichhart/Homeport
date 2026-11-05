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
        let linked: i64 = conn
            .query_row("SELECT count(*) FROM linked_stacks", [], |r| r.get(0))
            .unwrap();
        assert_eq!(linked, 0);
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
