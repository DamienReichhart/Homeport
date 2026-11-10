//! The SQLite store: a single connection behind a mutex, migrated on open.
//! Docker state is never cached here; the daemon is the source of truth.

mod error;
mod newer;
mod released;

use std::{path::Path, sync::Mutex};

use rusqlite::{Connection, OptionalExtension};
use rusqlite_migration::{Migrations, M};

use homeport_core::error::AppResult;

pub use self::error::sqlite_error;
pub use self::released::{release_of, released, Released};

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
    M::up(include_str!("migrations/0017_host_engines.sql")),
    M::up(include_str!("migrations/0018_database_info.sql")),
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
        .map_err(sqlite_error)?;
    record_required_release(conn)
}

/// Writes the oldest Homeport able to open this schema into the database,
/// for an older Homeport to name when it refuses the file (`newer`).
fn record_required_release(conn: &Connection) -> AppResult<()> {
    conn.execute(
        "INSERT INTO database_info (key, value) VALUES (?1, ?2)
         ON CONFLICT (key) DO UPDATE SET value = excluded.value
         WHERE value <> excluded.value",
        (newer::REQUIRES, release_of(MIGRATION_SLICE.len())),
    )
    .map(drop)
    .map_err(sqlite_error)
}

impl Db {
    /// Opens the database at `path`, creating it and migrating it as needed.
    ///
    /// A database from a newer Homeport is refused with
    /// `ErrorCode::DbFromNewerVersion` before anything is written.
    pub fn open(path: &Path) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        newer::refuse_newer(path, MIGRATION_SLICE.len())?;
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

    /// The schema version: the number of migrations this database has run.
    pub fn schema_version(&self) -> AppResult<u32> {
        self.with(|c| c.query_row("PRAGMA user_version", [], |r| r.get(0)))
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

    /// Whether any secret is stored sealed under the keychain's key: a
    /// missing keychain entry then means a lost key, not a first run.
    pub fn holds_sealed_data(&self) -> AppResult<bool> {
        self.with(|c| {
            c.query_row(
                "SELECT EXISTS (SELECT 1 FROM hosts WHERE secrets IS NOT NULL)
                     OR EXISTS (SELECT 1 FROM stacks WHERE env IS NOT NULL)
                     OR EXISTS (SELECT 1 FROM synced_stacks WHERE env IS NOT NULL)
                     OR EXISTS (SELECT 1 FROM stack_deploys WHERE env IS NOT NULL)
                     OR EXISTS (SELECT 1 FROM registries)
                     OR EXISTS (SELECT 1 FROM git_repositories
                                WHERE private_key IS NOT NULL OR token IS NOT NULL)
                     OR EXISTS (SELECT 1 FROM clusters WHERE secrets IS NOT NULL)",
                [],
                |r| r.get(0),
            )
        })
    }

    /// Writes a copy of the database as it is now to `path`, which must not
    /// exist (`VACUUM INTO`): a consistent snapshot, whatever is in the WAL.
    pub fn copy_to(&self, path: &Path) -> AppResult<()> {
        let target = path.to_string_lossy();
        self.with(|c| c.execute("VACUUM INTO ?1", [target.as_ref()]).map(drop))
    }
}

#[cfg(test)]
mod tests;
