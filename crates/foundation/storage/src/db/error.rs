//! SQLite failures, sorted into the cases the user can act on.

use homeport_core::error::{AppError, ErrorCode};

use crate::messages as msg;

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
            AppError::new(ErrorCode::NameInUse, msg::NAME_IN_USE)
        }
        Some(ffi::SQLITE_CONSTRAINT_FOREIGNKEY) => AppError::not_found(msg::REFERENCE_GONE),
        Some(code) if matches!(code & 0xff, ffi::SQLITE_BUSY | ffi::SQLITE_LOCKED) => {
            AppError::new(ErrorCode::DbLocked, msg::DB_LOCKED).with_hint(msg::DB_LOCKED_HINT)
        }
        Some(code) if matches!(code & 0xff, ffi::SQLITE_CORRUPT | ffi::SQLITE_NOTADB) => {
            AppError::new(ErrorCode::DbCorrupt, msg::DB_CORRUPT).with_hint(msg::DB_CORRUPT_HINT)
        }
        Some(code) if code & 0xff == ffi::SQLITE_FULL => {
            AppError::new(ErrorCode::DiskFull, msg::DISK_FULL).with_hint(msg::DISK_FULL_HINT)
        }
        _ => AppError::new(ErrorCode::Db, msg::DB_FAILED),
    };
    error.with_cause(&e)
}

/// A migration left rows of `table` pointing at nothing.
pub(super) fn foreign_keys_broken(table: &str) -> AppError {
    AppError::new(ErrorCode::DbMigration, msg::DB_MIGRATION_FAILED)
        .with_detail(format!("foreign key check failed in {table}"))
}

pub(super) fn migration_error(e: rusqlite_migration::Error) -> AppError {
    use rusqlite_migration::{Error, MigrationDefinitionError};
    if matches!(
        e,
        Error::MigrationDefinition(MigrationDefinitionError::DatabaseTooFarAhead)
    ) {
        return from_newer_unknown().with_cause(&e);
    }
    AppError::new(ErrorCode::DbMigration, msg::DB_MIGRATION_FAILED).with_cause(&e)
}

/// The database was migrated by a newer Homeport: `required` is the oldest
/// version that can open it, when the file says so. The hint never suggests
/// removing the file, which is the one way to lose what's in it.
pub(super) fn from_newer(required: Option<&str>, found: usize, known: usize) -> AppError {
    let error = match required {
        Some(version) => AppError::new(
            ErrorCode::DbFromNewerVersion,
            msg::DB_FROM_NEWER.with("version", version),
        )
        .with_hint(msg::DB_FROM_NEWER_HINT.with("version", version)),
        None => from_newer_unknown(),
    };
    error.with_detail(format!(
        "the database is at schema version {found}; Homeport {} knows up to {known}",
        env!("CARGO_PKG_VERSION")
    ))
}

fn from_newer_unknown() -> AppError {
    AppError::new(
        ErrorCode::DbFromNewerVersion,
        msg::DB_FROM_NEWER_UNKNOWN.with("current", env!("CARGO_PKG_VERSION")),
    )
    .with_hint(msg::DB_FROM_NEWER_UNKNOWN_HINT)
}
