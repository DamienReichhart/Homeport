//! IPC surface, one module per feature. Commands are thin: validate input,
//! call into a crate, map the result.
//! Register each new command in [`builder`]; the TypeScript bindings in
//! `src/shared/api/bindings.ts` are generated from it.
//!
//! Tauri runs non-`async` commands on the main thread, outside the Tokio
//! runtime, so any command that reaches `tokio::spawn` (streams, Host
//! supervisors) must be `async`.

use tauri_specta::{collect_commands, collect_events, ErrorHandlingMode};

use crate::app::events;

pub mod app;
pub mod backup;
pub mod cluster_actions;
pub mod cluster_api;
pub mod cluster_forwards;
pub mod cluster_streams;
pub mod clusters;
pub mod containerd;
pub mod containers;
pub mod exec;
pub mod files;
pub mod helm;
pub mod hosts;
pub mod images;
pub mod incus;
pub mod networks;
pub mod nomad;
pub mod podman;
pub mod registries;
pub mod repositories;
pub mod settings;
pub mod stacks;
pub mod streams;
pub mod swarm;
pub mod system;
pub mod updates;
pub mod volumes;

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
            app::get_app_info,
            app::get_capabilities,
            app::app_start_at_login,
            app::app_set_start_at_login,
            app::get_licenses,
            app::read_text_file,
            app::write_text_file,
            app::open_log_folder,
            settings::get_settings,
            settings::update_settings,
            registries::registries_list,
            registries::registries_save,
            registries::registries_remove,
            registries::registries_check,
            registries::registries_login_for,
            registries::registries_hub_tags,
            repositories::repositories_list,
            repositories::repositories_create,
            repositories::repositories_rotate_key,
            repositories::repositories_set_token,
            repositories::repositories_reset_host_key,
            repositories::repositories_remove,
            repositories::repositories_test,
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
            hosts::hosts_events_clear,
            clusters::clusters_list,
            clusters::clusters_get,
            clusters::clusters_update,
            clusters::clusters_set_enabled,
            clusters::clusters_remove,
            clusters::clusters_reconnect,
            clusters::clusters_kubeconfig_contexts,
            clusters::clusters_import,
            clusters::clusters_refresh,
            clusters::clusters_detect_local,
            nomad::clusters_nomad_add,
            nomad::clusters_nomad_update,
            nomad::clusters_nomad_test,
            nomad::clusters_nomad_jobs,
            nomad::clusters_nomad_job,
            nomad::clusters_nomad_job_stop,
            nomad::clusters_nomad_job_scale,
            nomad::clusters_nomad_watch_subscribe,
            nomad::clusters_nomad_job_plan,
            nomad::clusters_nomad_job_run,
            nomad::clusters_nomad_job_source,
            nomad::clusters_nomad_allocation,
            nomad::clusters_nomad_logs_subscribe,
            nomad::clusters_nomad_exec_start,
            nomad::clusters_nomad_exec_input,
            nomad::clusters_nomad_exec_resize,
            nomad::clusters_nomad_exec_close,
            clusters::clusters_namespaces,
            clusters::clusters_workloads_list,
            clusters::clusters_workload_get,
            clusters::clusters_cron_job_jobs,
            clusters::clusters_pods_list,
            clusters::clusters_pod_get,
            clusters::clusters_object_events,
            clusters::clusters_resource_yaml,
            clusters::clusters_watch_subscribe,
            clusters::clusters_services_list,
            clusters::clusters_service_get,
            clusters::clusters_ingresses_list,
            clusters::clusters_ingress_get,
            clusters::clusters_config_maps_list,
            clusters::clusters_config_map_get,
            clusters::clusters_secrets_list,
            clusters::clusters_secret_get,
            clusters::clusters_secret_reveal,
            clusters::clusters_nodes_list,
            clusters::clusters_node_get,
            clusters::clusters_events_list,
            clusters::clusters_summary,
            cluster_actions::clusters_scale,
            cluster_actions::clusters_autoscaler,
            cluster_actions::clusters_restart,
            cluster_actions::clusters_cron_job_suspend,
            cluster_actions::clusters_cron_job_trigger,
            cluster_actions::clusters_delete,
            cluster_actions::clusters_api_delete,
            cluster_actions::clusters_apply_preview,
            cluster_actions::clusters_apply,
            cluster_actions::clusters_resource_edit_yaml,
            cluster_actions::clusters_api_edit_yaml,
            cluster_api::clusters_api_resources,
            cluster_api::clusters_api_list,
            cluster_api::clusters_api_get,
            cluster_api::clusters_api_object_events,
            cluster_api::clusters_api_watch_subscribe,
            cluster_forwards::clusters_forward_start,
            cluster_forwards::clusters_forward_list,
            cluster_forwards::clusters_forward_stop,
            cluster_forwards::clusters_forward_port_free,
            helm::clusters_helm_releases,
            helm::clusters_helm_history,
            helm::clusters_helm_release_details,
            helm::helm_repos_list,
            helm::helm_repos_add,
            helm::helm_repos_update_subscribe,
            helm::helm_repos_remove,
            helm::helm_charts_search,
            helm::helm_chart_values,
            helm::helm_chart_versions,
            helm::clusters_helm_preview_install,
            helm::clusters_helm_install_subscribe,
            helm::helm_chart_candidates,
            helm::clusters_helm_preview_upgrade,
            helm::clusters_helm_upgrade_subscribe,
            helm::clusters_helm_rollback_subscribe,
            helm::clusters_helm_uninstall_subscribe,
            cluster_streams::clusters_pod_logs_subscribe,
            cluster_streams::clusters_exec_start,
            cluster_streams::clusters_exec_input,
            cluster_streams::clusters_exec_resize,
            cluster_streams::clusters_exec_close,
            backup::data_backup,
            backup::data_restore_preview,
            backup::data_restore,
            hosts::hosts_ssh_config_preview,
            hosts::hosts_ssh_config_import,
            hosts::hosts_docker_contexts_preview,
            hosts::hosts_docker_contexts_import,
            hosts::hosts_export,
            hosts::hosts_bundle_preview,
            hosts::hosts_bundle_import,
            containers::containers_list,
            containers::containers_inspect,
            containers::containers_act,
            containers::containers_remove,
            containers::containers_rename,
            containers::containers_processes,
            containers::containers_health,
            files::files_list,
            files::files_download,
            files::files_upload,
            streams::stream_cancel,
            containers::logs_subscribe,
            containers::stats_subscribe,
            system::usage_subscribe,
            images::images_pull_subscribe,
            images::images_push_subscribe,
            images::images_build_subscribe,
            images::images_save_subscribe,
            images::images_load_subscribe,
            images::images_import_subscribe,
            images::containers_export_subscribe,
            exec::exec_start,
            exec::exec_input,
            exec::exec_resize,
            exec::exec_close,
            images::images_list,
            images::images_inspect,
            images::images_history,
            images::images_search,
            images::images_freshness,
            images::images_tag,
            images::images_remove,
            images::images_prune,
            volumes::volumes_list,
            volumes::volumes_sizes,
            volumes::volumes_inspect,
            volumes::volumes_create,
            volumes::volumes_remove,
            volumes::volumes_prune,
            volumes::volumes_helper_image,
            volumes::volumes_helper_pull_subscribe,
            volumes::volumes_browse_start,
            volumes::volumes_browse_list,
            volumes::volumes_browse_download,
            volumes::volumes_browse_end,
            volumes::volumes_running_users,
            volumes::volumes_backup_subscribe,
            volumes::volumes_restore_subscribe,
            volumes::volumes_clone,
            networks::networks_list,
            networks::networks_inspect,
            networks::networks_create,
            networks::networks_remove,
            networks::networks_connect,
            networks::networks_disconnect,
            networks::networks_prune,
            system::system_info,
            system::system_disk_usage,
            system::system_prune,
            containers::containers_run,
            containerd::containerd_namespaces,
            containerd::containerd_namespace_create,
            containerd::containerd_namespace_remove,
            containerd::containerd_containers,
            containerd::containerd_container_inspect,
            containerd::containerd_container_act,
            containerd::containerd_container_kill,
            containerd::containerd_container_remove,
            containerd::containerd_logs_source,
            containerd::containerd_logs_subscribe,
            containerd::containerd_exec_start,
            containerd::containerd_exec_input,
            containerd::containerd_exec_resize,
            containerd::containerd_exec_close,
            containerd::containerd_images,
            containerd::containerd_image_remove,
            containerd::containerd_images_pull_subscribe,
            containerd::containerd_snapshots,
            containerd::containerd_snapshot_remove,
            incus::incus_trust_token_read,
            incus::incus_trust_add,
            incus::incus_projects,
            incus::incus_instances,
            incus::incus_instance,
            incus::incus_instance_act,
            incus::incus_instance_delete,
            incus::incus_instance_create_subscribe,
            incus::incus_images,
            incus::incus_remote_images,
            incus::incus_image_copy_subscribe,
            incus::incus_image_refresh,
            incus::incus_image_delete,
            incus::incus_image_alias_add,
            incus::incus_image_alias_remove,
            incus::incus_exec_start,
            incus::incus_console_start,
            incus::incus_console_log,
            incus::incus_terminal_input,
            incus::incus_terminal_resize,
            incus::incus_terminal_close,
            incus::incus_snapshots,
            incus::incus_snapshot_create,
            incus::incus_snapshot_restore,
            incus::incus_snapshot_delete,
            incus::incus_storage_pools,
            incus::incus_storage_volumes,
            incus::incus_volume_create,
            incus::incus_volume_delete,
            incus::incus_networks,
            incus::incus_network_state,
            incus::incus_bridge_create,
            incus::incus_network_delete,
            incus::incus_profiles,
            incus::incus_profile,
            incus::incus_profile_update,
            incus::incus_profile_create,
            incus::incus_profile_delete,
            podman::pods_list,
            podman::pods_inspect,
            podman::pods_act,
            podman::pods_remove,
            podman::pods_create,
            podman::quadlets_list,
            podman::quadlets_file,
            podman::quadlets_install,
            podman::quadlets_remove,
            podman::quadlets_act,
            containers::containers_run_spec,
            containers::containers_edit_losses,
            containers::containers_recreate,
            containers::containers_commit,
            stacks::stacks_list,
            stacks::stacks_get,
            stacks::stacks_get_env,
            stacks::stacks_check,
            stacks::stacks_create,
            stacks::stacks_update,
            stacks::stacks_forget,
            stacks::stacks_set_profiles,
            stacks::stacks_set_registry_auth,
            stacks::stacks_linked_inspect,
            stacks::stacks_linked_create,
            stacks::stacks_linked_read,
            stacks::stacks_linked_write,
            stacks::stacks_linked_set_files,
            stacks::stacks_linked_forget,
            stacks::stacks_synced_create,
            stacks::stacks_synced_get,
            stacks::stacks_synced_files,
            stacks::stacks_synced_get_env,
            stacks::stacks_synced_set_env,
            stacks::stacks_synced_sync,
            stacks::stacks_synced_deploy_subscribe,
            stacks::stacks_synced_set_sync,
            stacks::stacks_synced_forget,
            stacks::stacks_act,
            stacks::stacks_remove,
            stacks::stacks_compose_subscribe,
            stacks::stacks_diff,
            stacks::stacks_history,
            stacks::stacks_deploy,
            stacks::stacks_rollback_subscribe,
            swarm::swarms_list,
            swarm::swarm_inspect,
            swarm::swarm_nodes_list,
            swarm::swarm_nodes_inspect,
            swarm::swarm_nodes_update,
            swarm::swarm_nodes_remove,
            swarm::swarm_services_list,
            swarm::swarm_services_inspect,
            swarm::swarm_services_scale,
            swarm::swarm_services_rollback,
            swarm::swarm_services_remove,
            swarm::swarm_services_create,
            swarm::swarm_services_update,
            swarm::swarm_networks_list,
            swarm::swarm_init,
            swarm::swarm_join,
            swarm::swarm_leave,
            swarm::swarm_join_info,
            swarm::swarm_stacks_list,
            swarm::swarm_stacks_parts,
            swarm::swarm_stacks_remove,
            swarm::swarm_stacks_link,
            swarm::swarm_service_logs_subscribe,
            swarm::swarm_tasks_list,
            swarm::swarm_tasks_inspect,
            swarm::swarm_secrets_list,
            swarm::swarm_secrets_create,
            swarm::swarm_secrets_remove,
            swarm::swarm_configs_list,
            swarm::swarm_configs_create,
            swarm::swarm_configs_inspect,
            swarm::swarm_configs_remove,
        ])
        .events(collect_events![
            events::HostStatusChanged,
            events::ClusterStatusChanged,
            events::PortForwardChanged,
            events::EngineEventReceived,
            events::SyncedUpdateAvailable,
            events::SyncedAutoDeployed,
            events::TrayHostChosen,
            events::TrayAddHostChosen,
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

    /// A Synced Stack's compose files change only through git: no command
    /// for one takes files or file content (only its environment).
    #[test]
    fn no_command_writes_a_synced_stacks_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bindings.ts");
        super::export_bindings(&path);
        let bindings = std::fs::read_to_string(path).unwrap();
        let synced: Vec<&str> = bindings
            .lines()
            .filter(|l| l.trim_start().starts_with("stacksSynced"))
            .collect();
        assert!(synced.len() >= 8, "{synced:?}");
        for line in synced {
            let params = line.split("=>").next().unwrap_or_default();
            for forbidden in ["files", "content", "path:", "ComposeFile"] {
                assert!(!params.contains(forbidden), "{line}");
            }
        }
    }

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
