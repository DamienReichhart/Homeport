//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    /// Restoring Homeport's data from an export of some Hosts.
    BACKUP_IS_AN_EXPORT = "errors.backup.isAnExport" => "{path} is an export of Hosts, not a backup of Homeport's data.";
    BACKUP_IS_AN_EXPORT_HINT = "errors.backup.isAnExport.hint" => "Import its Hosts from the Hosts page instead, which adds them to the ones you have.";
}
