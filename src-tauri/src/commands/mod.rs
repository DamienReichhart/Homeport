//! IPC surface. Commands are thin: validate input, call a service, map the result.
//! Register each new command in [`builder`]; the TypeScript bindings in
//! `src/shared/api/bindings.ts` are generated from it.

use tauri_specta::{collect_commands, ErrorHandlingMode};

pub mod settings;
pub mod system;

/// Path of the generated bindings, relative to the `src-tauri` crate root.
pub const BINDINGS_PATH: &str = "../src/shared/api/bindings.ts";

pub fn builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .error_handling(ErrorHandlingMode::Throw)
        .dangerously_cast_bigints_to_number()
        .commands(collect_commands![
            system::get_app_info,
            settings::get_settings,
            settings::update_settings,
        ])
}

pub fn export_bindings(path: impl AsRef<std::path::Path>) {
    builder()
        .export(
            specta_typescript::Typescript::default().header("// @ts-nocheck"),
            path,
        )
        .expect("failed to export TypeScript bindings");
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Fails when the committed bindings drift from the Rust command surface.
    /// Run with `UPDATE_BINDINGS=1` to regenerate them.
    #[test]
    fn bindings_are_up_to_date() {
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join(super::BINDINGS_PATH);
        if std::env::var_os("UPDATE_BINDINGS").is_some() {
            super::export_bindings(&committed);
            return;
        }
        let dir = tempfile::tempdir().unwrap();
        let fresh = dir.path().join("bindings.ts");
        super::export_bindings(&fresh);
        assert_eq!(
            std::fs::read_to_string(&committed).unwrap_or_default(),
            std::fs::read_to_string(&fresh).unwrap(),
            "bindings are stale: run `bun run bindings`"
        );
    }
}
