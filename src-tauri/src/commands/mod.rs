//! IPC surface. Commands are thin: validate input, call a service, map the result.
//! Register each new command in [`builder`]; the TypeScript bindings in
//! `src/shared/api/bindings.ts` are generated from it.

use tauri_specta::{collect_commands, collect_events, ErrorHandlingMode};

use crate::events;

pub mod containers;
pub mod exec;
pub mod hosts;
pub mod settings;
pub mod streams;
pub mod system;

/// Path of the generated bindings, relative to the `src-tauri` crate root.
pub const BINDINGS_PATH: &str = "../src/shared/api/bindings.ts";

pub fn builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .error_handling(ErrorHandlingMode::Throw)
        .dangerously_cast_bigints_to_number()
        .commands(collect_commands![
            system::get_app_info,
            system::get_capabilities,
            settings::get_settings,
            settings::update_settings,
            hosts::hosts_list,
            hosts::hosts_detect_local,
            hosts::hosts_create,
            hosts::hosts_update,
            hosts::hosts_delete,
            hosts::hosts_set_enabled,
            hosts::hosts_trust_key,
            hosts::hosts_reconnect,
            hosts::hosts_regenerate_key,
            hosts::hosts_ssh_setup,
            hosts::hosts_events,
            containers::containers_list,
            containers::containers_inspect,
            containers::containers_act,
            containers::containers_remove,
            containers::containers_rename,
            containers::containers_processes,
            streams::stream_cancel,
            streams::logs_subscribe,
            streams::stats_subscribe,
            streams::usage_subscribe,
            exec::exec_start,
            exec::exec_input,
            exec::exec_resize,
            exec::exec_close,
        ])
        .events(collect_events![
            events::HostStatusChanged,
            events::EngineEventReceived,
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
