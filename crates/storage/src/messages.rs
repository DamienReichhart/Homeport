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
    /// The database was migrated by a newer Homeport, which says the oldest
    /// version able to open it. Nothing was written to it.
    DB_FROM_NEWER = "errors.storage.dbFromNewer" => "This database was last used by Homeport {version} or later.";
    DB_FROM_NEWER_HINT = "errors.storage.dbFromNewer.hint" => "Install Homeport {version} or newer from the download page. Your data hasn't been changed.";
    /// The same, for a database that doesn't name the version it needs.
    DB_FROM_NEWER_UNKNOWN = "errors.storage.dbFromNewerUnknown" => "This database was last used by a newer Homeport than this one ({current}).";
    DB_FROM_NEWER_UNKNOWN_HINT = "errors.storage.dbFromNewerUnknown.hint" => "Install the latest Homeport from the download page. Your data hasn't been changed.";
    /// A passphrase too short to protect an export.
    BUNDLE_PASSPHRASE_SHORT = "errors.storage.bundle.passphraseShort" => "Use a passphrase of at least {min} characters.";
    /// A file picked for import that isn't a Homeport export.
    BUNDLE_NOT_A_BUNDLE = "errors.storage.bundle.notABundle" => "This isn't a Homeport export file.";
    /// The passphrase doesn't open the file, or the file was changed.
    BUNDLE_UNREADABLE = "errors.storage.bundle.unreadable" => "The passphrase doesn't open this file, or the file is damaged.";
    BUNDLE_UNREADABLE_HINT = "errors.storage.bundle.unreadable.hint" => "Check the passphrase, including capitals and spaces. If it's right, copy the file again from where it was exported.";
    /// An export written by a newer Homeport.
    BUNDLE_TOO_NEW = "errors.storage.bundle.tooNew" => "This file comes from a newer Homeport.";
    BUNDLE_TOO_NEW_HINT = "errors.storage.bundle.tooNew.hint" => "Update Homeport on this computer, then import it again.";
}
