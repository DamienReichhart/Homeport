//! Changing what runs on a Cluster: scaling, restarting, suspending and
//! running workloads, deleting objects, and applying YAML. The pages'
//! watches show the result.

use homeport_core::error::AppResult;
use homeport_kubernetes::{
    apply::ApplyResult,
    resources::{ApiType, ResourceKind, WorkloadKind},
};

use super::on;
use crate::app::AppState;

/// Sets how many Pods a Deployment or a StatefulSet runs.
pub async fn clusters_scale(
    state: &AppState,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
    replicas: i32,
) -> AppResult<()> {
    on(state, &id)?
        .scale(kind, &namespace, &name, replicas)
        .await
}

/// The HorizontalPodAutoscaler that scales a workload, if one does.
pub async fn clusters_autoscaler(
    state: &AppState,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
) -> AppResult<Option<String>> {
    on(state, &id)?.autoscaler(kind, &namespace, &name).await
}

/// Rolls a workload's Pods out again, like `kubectl rollout restart`.
pub async fn clusters_restart(
    state: &AppState,
    id: String,
    kind: WorkloadKind,
    namespace: String,
    name: String,
) -> AppResult<()> {
    on(state, &id)?.restart(kind, &namespace, &name).await
}

/// Suspends a CronJob or resumes it.
pub async fn clusters_cron_job_suspend(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
    suspended: bool,
) -> AppResult<()> {
    on(state, &id)?
        .set_suspended(&namespace, &name, suspended)
        .await
}

/// Starts a Job from a CronJob now; returns the Job's name.
pub async fn clusters_cron_job_trigger(
    state: &AppState,
    id: String,
    namespace: String,
    name: String,
) -> AppResult<String> {
    on(state, &id)?.trigger(&namespace, &name).await
}

/// Deletes an object shown on one of Homeport's own pages; `grace`
/// overrides how long a Pod is given to stop.
pub async fn clusters_delete(
    state: &AppState,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
    grace: Option<u32>,
) -> AppResult<()> {
    on(state, &id)?
        .delete(&kind.api_type(), namespace.as_deref(), &name, grace)
        .await
}

/// Deletes an object of any type the Cluster serves.
pub async fn clusters_api_delete(
    state: &AppState,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    name: String,
) -> AppResult<()> {
    on(state, &id)?
        .delete(&api, namespace.as_deref(), &name, None)
        .await
}

/// What applying `yaml` would change, document by document, from a dry
/// run; nothing changes on the Cluster.
pub async fn clusters_apply_preview(
    state: &AppState,
    id: String,
    yaml: String,
    namespace: Option<String>,
) -> AppResult<ApplyResult> {
    on(state, &id)?
        .apply_preview(&yaml, namespace.as_deref())
        .await
}

/// Applies `yaml` with server-side apply as the `homeport` field manager;
/// `force` takes over the fields other managers own.
pub async fn clusters_apply(
    state: &AppState,
    id: String,
    yaml: String,
    namespace: Option<String>,
    force: bool,
) -> AppResult<ApplyResult> {
    on(state, &id)?
        .apply(&yaml, namespace.as_deref(), force)
        .await
}

/// An object shown on one of Homeport's own pages, as YAML to edit.
pub async fn clusters_resource_edit_yaml(
    state: &AppState,
    id: String,
    kind: ResourceKind,
    namespace: Option<String>,
    name: String,
) -> AppResult<String> {
    on(state, &id)?
        .edit_yaml(&kind.api_type(), namespace.as_deref(), &name)
        .await
}

/// An object of any type the Cluster serves, as YAML to edit.
pub async fn clusters_api_edit_yaml(
    state: &AppState,
    id: String,
    api: ApiType,
    namespace: Option<String>,
    name: String,
) -> AppResult<String> {
    on(state, &id)?
        .edit_yaml(&api, namespace.as_deref(), &name)
        .await
}

#[cfg(test)]
mod tests {
    use super::super::tests::{gone, unknown};
    use super::*;
    use crate::app::testing::app;

    fn ns() -> String {
        "default".into()
    }

    fn name() -> String {
        "web".into()
    }

    #[tokio::test]
    async fn nothing_is_changed_on_an_unknown_cluster() {
        let app = app();
        let s = &app.state;
        let kind = WorkloadKind::Deployment;
        let pod = ResourceKind::Pod;
        let pods = || ResourceKind::Pod.api_type();
        assert!(gone(
            clusters_scale(s, unknown(), kind, ns(), name(), 2).await
        ));
        assert!(gone(
            clusters_autoscaler(s, unknown(), kind, ns(), name()).await
        ));
        assert!(gone(
            clusters_restart(s, unknown(), kind, ns(), name()).await
        ));
        assert!(gone(
            clusters_cron_job_suspend(s, unknown(), ns(), name(), true).await
        ));
        assert!(gone(
            clusters_cron_job_trigger(s, unknown(), ns(), name()).await
        ));
        assert!(gone(
            clusters_delete(s, unknown(), pod, Some(ns()), name(), Some(0)).await
        ));
        assert!(gone(
            clusters_api_delete(s, unknown(), pods(), None, name()).await
        ));
        let yaml = "kind: ConfigMap".to_owned();
        assert!(gone(
            clusters_apply_preview(s, unknown(), yaml.clone(), None).await
        ));
        assert!(gone(clusters_apply(s, unknown(), yaml, None, false).await));
        assert!(gone(
            clusters_resource_edit_yaml(s, unknown(), pod, None, name()).await
        ));
        assert!(gone(
            clusters_api_edit_yaml(s, unknown(), pods(), None, name()).await
        ));
    }
}
