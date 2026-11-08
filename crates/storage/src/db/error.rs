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
    AppError::new(ErrorCode::DbMigration, msg::DB_MIGRATION_FAILED).with_cause(&e)
}
