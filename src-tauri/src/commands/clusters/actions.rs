//! Changing what runs on a Cluster: scaling, restarting, suspending and
//! running workloads, deleting objects, and applying YAML. The pages'
//! watches show the result.

use tauri::State;

use homeport_clusters::{
    apply::ApplyResult,
    resources::{ApiType, ResourceKind, WorkloadKind},
};

use super::on;
use crate::{app::AppState, error::AppResult};

/// Sets how many Pods a Deployment or a StatefulSet runs.
#[tauri::command]
#[specta::specta]
pub async fn clusters_scale(
    state: State<'_, AppState>,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
    replicas: i32,
) -> AppResult<()> {
    on(&state, &id)?
        .scale(kind, &namespace, &name, replicas)
        .await
}

/// The HorizontalPodAutoscaler that scales a workload, if one does.
#[tauri::command]
#[specta::specta]
pub async fn clusters_autoscaler(
    state: State<'_, AppState>,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
) -> AppResult<Option<String>> {
    on(&state, &id)?.autoscaler(kind, &namespace, &name).await
}

/// Rolls a workload's Pods out again, like `kubectl rollout restart`.
#[tauri::command]
#[specta::specta]
pub async fn clusters_restart(
    state: State<'_, AppState>,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
) -> AppResult<()> {
    on(&state, &id)?.restart(kind, &namespace, &name).await
}

/// Suspends a CronJob or resumes it.
#[tauri::command]
#[specta::specta]
pub async fn clusters_cron_job_suspend(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
    suspended: bool,
) -> AppResult<()> {
    on(&state, &id)?
        .set_suspended(&namespace, &name, suspended)
        .await
}

/// Starts a Job from a CronJob now; returns the Job's name.
#[tauri::command]
#[specta::specta]
pub async fn clusters_cron_job_trigger(
    state: State<'_, AppState>,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<String> {
    on(&state, &id)?.trigger(&namespace, &name).await
}

/// Deletes an object shown on one of Homeport's own pages; `grace`
/// overrides how long a Pod is given to stop.
#[tauri::command]
#[specta::specta]
pub async fn clusters_delete(
    state: State<'_, AppState>,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
    grace: Option<u32>,
) -> AppResult<()> {
    on(&state, &id)?
        .delete(&kind.api_type(), namespace.as_deref(), &name, grace)
        .await
}

/// Deletes an object of any type the Cluster serves.
#[tauri::command]
#[specta::specta]
pub async fn clusters_api_delete(
    state: State<'_, AppState>,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    name: String,
) -> AppResult<()> {
    on(&state, &id)?
        .delete(&api, namespace.as_deref(), &name, None)
        .await
}

/// What applying `yaml` would change, document by document, from a dry
/// run; nothing changes on the Cluster.
#[tauri::command]
#[specta::specta]
pub async fn clusters_apply_preview(
    state: State<'_, AppState>,
    id: String,
    yaml: String,
    namespace: Option<String>,
) -> AppResult<ApplyResult> {
    on(&state, &id)?
        .apply_preview(&yaml, namespace.as_deref())
        .await
}

/// Applies `yaml` with server-side apply as the `homeport` field manager;
/// `force` takes over the fields other managers own.
#[tauri::command]
#[specta::specta]
pub async fn clusters_apply(
    state: State<'_, AppState>,
    id: String,
    yaml: String,
    namespace: Option<String>,
    force: bool,
) -> AppResult<ApplyResult> {
    on(&state, &id)?
        .apply(&yaml, namespace.as_deref(), force)
        .await
}

/// An object shown on one of Homeport's own pages, as YAML to edit.
#[tauri::command]
#[specta::specta]
pub async fn clusters_resource_edit_yaml(
    state: State<'_, AppState>,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
) -> AppResult<String> {
    on(&state, &id)?
        .edit_yaml(&kind.api_type(), namespace.as_deref(), &name)
        .await
}

/// An object of any type the Cluster serves, as YAML to edit.
#[tauri::command]
#[specta::specta]
pub async fn clusters_api_edit_yaml(
    state: State<'_, AppState>,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    name: String,
) -> AppResult<String> {
    on(&state, &id)?
        .edit_yaml(&api, namespace.as_deref(), &name)
        .await
}
