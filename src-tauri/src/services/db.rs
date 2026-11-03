//! The SQLite store: a single connection behind a mutex, migrated on open.
//! Docker state is never cached here; the daemon is the source of truth.

use std::{path::Path, sync::Mutex};

use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

use crate::error::{AppError, AppResult, ErrorCode};

const MIGRATION_SLICE: &[M<'static>] = &[
    M::up(include_str!("db_migrations/0001_settings.sql")),
    M::up(include_str!("db_migrations/0002_hosts.sql")),
    M::up(include_str!("db_migrations/0003_stacks.sql")),
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

    #[cfg(test)]
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

/// SQLite failed; the cases the user can act on get their own code.
pub fn sqlite_error(e: rusqlite::Error) -> AppError {
    use rusqlite::ffi;
    let sqlite = match &e {
        rusqlite::Error::SqliteFailure(f, _) => Some(f.extended_code),
        _ => None,
    };
    let error = match sqlite {
            // Callers replace these with a message naming what clashed.
            Some(ffi::SQLITE_CONSTRAINT_UNIQUE) => {
                AppError::new(ErrorCode::NameInUse, "That name is already used.")
            }
            Some(ffi::SQLITE_CONSTRAINT_FOREIGNKEY) => {
                AppError::not_found("Something this refers to no longer exists.")
            }
            Some(code) if matches!(code & 0xff, ffi::SQLITE_BUSY | ffi::SQLITE_LOCKED) => {
                AppError::new(ErrorCode::DbLocked, "Homeport's database is busy.")
                    .with_hint("Another Homeport may be running. Close it and try again.")
            }
            Some(code) if matches!(code & 0xff, ffi::SQLITE_CORRUPT | ffi::SQLITE_NOTADB) => {
                AppError::new(ErrorCode::DbCorrupt, "Homeport's database is damaged.").with_hint(
                    "Quit Homeport, move homeport.db out of its data folder and start again. Hosts and stacks will need to be added again.",
                )
            }
            Some(code) if code & 0xff == ffi::SQLITE_FULL => {
                AppError::new(ErrorCode::DiskFull, "The disk is full, so Homeport can't save.")
                    .with_hint("Free some disk space and try again.")
            }
            _ => AppError::new(ErrorCode::Db, "Homeport's database failed."),
        };
    error.with_cause(&e)
}

fn migration_error(e: rusqlite_migration::Error) -> AppError {
    AppError::new(
        ErrorCode::DbMigration,
        "Homeport couldn't upgrade its database.",
    )
    .with_cause(&e)
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
