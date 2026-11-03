//! IPC surface. Commands are thin: validate input, call a service, map the result.
//! Register each new command in [`builder`]; the TypeScript bindings in
//! `src/shared/api/bindings.ts` are generated from it.
//!
//! Tauri runs non-`async` commands on the main thread, outside the Tokio
//! runtime, so any command that reaches `tokio::spawn` (streams, Host
//! supervisors) must be `async`.

use tauri_specta::{collect_commands, collect_events, ErrorHandlingMode};

use crate::events;

pub mod containers;
pub mod engine;
pub mod exec;
pub mod hosts;
pub mod settings;
pub mod stacks;
pub mod streams;
pub mod system;
pub mod updates;

/// Path of the generated bindings, relative to the `src-tauri` crate root.
#[cfg(any(debug_assertions, test))]
pub const BINDINGS_PATH: &str = "../src/shared/api/bindings.ts";

pub fn builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .error_handling(ErrorHandlingMode::Throw)
        .dangerously_cast_bigints_to_number()
        // One TS type per Rust type; commands and channels only flow one way each.
        .disable_serde_phases()
        .commands(collect_commands![
            system::get_app_info,
            system::get_capabilities,
            system::get_licenses,
            system::read_text_file,
            system::write_text_file,
            settings::get_settings,
            settings::update_settings,
            updates::updates_check,
            updates::updates_install,
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
            containers::files_list,
            containers::files_download,
            containers::files_upload,
            streams::stream_cancel,
            streams::logs_subscribe,
            streams::stats_subscribe,
            streams::usage_subscribe,
            streams::images_pull_subscribe,
            exec::exec_start,
            exec::exec_input,
            exec::exec_resize,
            exec::exec_close,
            engine::images_list,
            engine::images_inspect,
            engine::images_history,
            engine::images_tag,
            engine::images_remove,
            engine::images_prune,
            engine::volumes_list,
            engine::volumes_inspect,
            engine::volumes_create,
            engine::volumes_remove,
            engine::volumes_prune,
            engine::networks_list,
            engine::networks_inspect,
            engine::networks_create,
            engine::networks_remove,
            engine::networks_connect,
            engine::networks_disconnect,
            engine::networks_prune,
            engine::system_info,
            engine::system_disk_usage,
            engine::system_prune,
            engine::containers_run,
            stacks::stacks_list,
            stacks::stacks_get,
            stacks::stacks_get_env,
            stacks::stacks_check,
            stacks::stacks_create,
            stacks::stacks_update,
            stacks::stacks_forget,
            stacks::stacks_act,
            stacks::stacks_remove,
            stacks::stacks_compose_subscribe,
        ])
        .events(collect_events![
            events::HostStatusChanged,
            events::EngineEventReceived,
        ])
}

#[cfg(any(debug_assertions, test))]
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
