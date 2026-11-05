//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    /// A row clashes with a unique name; callers usually replace it with
    /// a message naming what clashed.
    NAME_IN_USE = "errors.storage.nameInUse" => "That name is already used.";
    /// A row refers to another that was deleted.
    REFERENCE_GONE = "errors.storage.referenceGone" => "Something this refers to no longer exists.";
    /// SQLite is busy or locked, usually by another Homeport.
    DB_LOCKED = "errors.storage.dbLocked" => "Homeport's database is busy.";
    DB_LOCKED_HINT = "errors.storage.dbLocked.hint" => "Another Homeport may be running. Close it and try again.";
    /// The database file is damaged or isn't a database.
    DB_CORRUPT = "errors.storage.dbCorrupt" => "Homeport's database is damaged.";
    DB_CORRUPT_HINT = "errors.storage.dbCorrupt.hint" => "Quit Homeport, move homeport.db out of its data folder and start again. Hosts and stacks will need to be added again.";
    /// The disk filled up while saving to the database.
    DISK_FULL = "errors.storage.diskFull" => "The disk is full, so Homeport can't save.";
    DISK_FULL_HINT = "errors.storage.diskFull.hint" => "Free some disk space and try again.";
    /// Any other SQLite failure.
    DB_FAILED = "errors.storage.dbFailed" => "Homeport's database failed.";
    /// A migration failed when the database was opened.
    DB_MIGRATION_FAILED = "errors.storage.dbMigrationFailed" => "Homeport couldn't upgrade its database.";
}
