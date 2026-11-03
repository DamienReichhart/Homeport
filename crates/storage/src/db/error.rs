//! SQLite failures, sorted into the cases the user can act on.

use homeport_core::error::{AppError, ErrorCode};

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

pub(super) fn migration_error(e: rusqlite_migration::Error) -> AppError {
    AppError::new(
        ErrorCode::DbMigration,
        "Homeport couldn't upgrade its database.",
    )
    .with_cause(&e)
}
