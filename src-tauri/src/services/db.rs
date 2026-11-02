//! The SQLite store: a single connection behind a mutex, migrated on open.
//! Docker state is never cached here; the daemon is the source of truth.

use std::{path::Path, sync::Mutex};

use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

use crate::error::AppResult;

const MIGRATION_SLICE: &[M<'static>] = &[
    M::up(include_str!("db_migrations/0001_settings.sql")),
    M::up(include_str!("db_migrations/0002_hosts.sql")),
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
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        Self::init(conn)
    }

    #[cfg(test)]
    pub fn open_in_memory() -> AppResult<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(mut conn: Connection) -> AppResult<Self> {
        conn.pragma_update(None, "foreign_keys", "ON")?;
        MIGRATIONS.to_latest(&mut conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Runs `f` with exclusive access to the connection.
    pub fn with<T>(&self, f: impl FnOnce(&mut Connection) -> rusqlite::Result<T>) -> AppResult<T> {
        let mut conn = self.conn.lock().expect("db lock poisoned");
        Ok(f(&mut conn)?)
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
