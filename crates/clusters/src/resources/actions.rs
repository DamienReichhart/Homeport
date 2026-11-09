//! Changing what runs on a Cluster, as kubectl would: scaling workloads,
//! restarting their rollout, suspending and running CronJobs, and deleting
//! objects. Each goes straight to the Kubernetes API with the Cluster's own
//! credentials; the watches of the pages showing them tell them it changed.

use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::{DaemonSet, Deployment, StatefulSet},
        autoscaling::v2::HorizontalPodAutoscaler,
        batch::v1::{CronJob, Job},
    },
    apimachinery::pkg::apis::meta::v1::{ObjectMeta, OwnerReference},
};
use kube::api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, PropagationPolicy};
use serde_json::json;

use homeport_core::error::{AppError, AppResult};

use super::{ApiType, On, ResourceKind, WorkloadKind};
use crate::messages as msg;

/// The field manager Homeport's changes appear under.
pub const FIELD_MANAGER: &str = "homeport";

/// What kubectl puts on a Pod template to roll it out again.
pub const RESTARTED_AT: &str = "kubectl.kubernetes.io/restartedAt";

/// What marks a Job started by hand from a CronJob, as kubectl does.
pub const INSTANTIATE: &str = "cronjob.kubernetes.io/instantiate";

/// The longest name a Job may have, since it becomes a label value.
const JOB_NAME_MAX: usize = 63;

impl On {
    /// Sets how many Pods a Deployment or a StatefulSet runs.
    pub async fn scale(
        &self,
        kind: WorkloadKind,
        namespace: &str,
        name: &str,
        replicas: i32,
    ) -> AppResult<()> {
        if replicas < 0 {
            return Err(AppError::validation(msg::REPLICAS_NEGATIVE));
        }
        let ns = Some(namespace);
        let r = kind.resource();
        let patch = Patch::Merge(json!({ "spec": { "replicas": replicas } }));
        let params = PatchParams::default();
        let result = match kind {
            WorkloadKind::Deployment => self
                .api::<Deployment>(ns)
                .patch_scale(name, &params, &patch)
                .await
                .map(drop),
            WorkloadKind::StatefulSet => self
                .api::<StatefulSet>(ns)
                .patch_scale(name, &params, &patch)
                .await
                .map(drop),
            _ => return Err(not_supported(msg::SCALE_NOT_SUPPORTED, kind)),
        };
        result.map_err(|e| self.failed("patch", r, ns, Some(name), &e))
    }

    /// Rolls a Deployment, StatefulSet or DaemonSet out again, replacing
    /// its Pods like `kubectl rollout restart`.
    pub async fn restart(&self, kind: WorkloadKind, namespace: &str, name: &str) -> AppResult<()> {
        let ns = Some(namespace);
        let now = jiff::Timestamp::now().to_string();
        let patch = Patch::Strategic(restart_patch(&now));
        let params = PatchParams::default();
        let result = match kind {
            WorkloadKind::Deployment => self
                .api::<Deployment>(ns)
                .patch(name, &params, &patch)
                .await
                .map(drop),
            WorkloadKind::StatefulSet => self
                .api::<StatefulSet>(ns)
                .patch(name, &params, &patch)
                .await
                .map(drop),
            WorkloadKind::DaemonSet => self
                .api::<DaemonSet>(ns)
                .patch(name, &params, &patch)
                .await
                .map(drop),
            _ => return Err(not_supported(msg::RESTART_NOT_SUPPORTED, kind)),
        };
        result.map_err(|e| self.failed("patch", kind.resource(), ns, Some(name), &e))
    }

    /// Suspends a CronJob, so it starts no more Jobs, or resumes it.
    pub async fn set_suspended(
        &self,
        namespace: &str,
        name: &str,
        suspended: bool,
    ) -> AppResult<()> {
        let ns = Some(namespace);
        self.api::<CronJob>(ns)
            .patch(
                name,
                &PatchParams::default(),
                &Patch::Merge(json!({ "spec": { "suspend": suspended } })),
            )
            .await
            .map(drop)
            .map_err(|e| self.failed("patch", ResourceKind::CronJob, ns, Some(name), &e))
    }

    /// Starts a Job from a CronJob's template now; returns the Job's name.
    pub async fn trigger(&self, namespace: &str, name: &str) -> AppResult<String> {
        let ns = Some(namespace);
        let cron: CronJob = self
            .get(self.api(ns), ResourceKind::CronJob, ns, name)
            .await?;
        let suffix = uuid::Uuid::new_v4().simple().to_string();
        let job = job_from(&cron, &suffix[..5]);
        let job_name = job.metadata.name.clone().unwrap_or_default();
        self.api::<Job>(ns)
            .create(&PostParams::default(), &job)
            .await
            .map_err(|e| self.failed("create", ResourceKind::Job, ns, Some(&job_name), &e))?;
        Ok(job_name)
    }

    /// Deletes an object, its dependents in the background, as kubectl does;
    /// `grace` overrides how long a Pod is given to stop.
    pub async fn delete(
        &self,
        api: &ApiType,
        namespace: Option<&str>,
        name: &str,
        grace: Option<u32>,
    ) -> AppResult<()> {
        let params = delete_params(grace);
        api.api(self.client.clone(), namespace)
            .delete(name, &params)
            .await
            .map(drop)
            .map_err(|e| self.failed_on("delete", api, namespace, Some(name), &e))
    }

    /// The HorizontalPodAutoscaler that scales a workload, if one does
    /// (`None` too when the credentials can't list them).
    pub async fn autoscaler(
        &self,
        kind: WorkloadKind,
        namespace: &str,
        name: &str,
    ) -> AppResult<Option<String>> {
        let api: Api<HorizontalPodAutoscaler> = Api::namespaced(self.client.clone(), namespace);
        let list = match api.list(&ListParams::default()).await {
            Ok(list) => list,
            Err(kube::Error::Api(status)) if matches!(status.code, 403 | 404) => return Ok(None),
            Err(e) => return Err(self.failed("list", kind.resource(), Some(namespace), None, &e)),
        };
        let kind = kind.resource().kind();
        Ok(list.items.into_iter().find_map(|hpa| {
            let target = &hpa.spec.as_ref()?.scale_target_ref;
            (target.kind == kind && target.name == name).then(|| hpa.metadata.name.clone())?
        }))
    }
}

/// The patch that rolls a workload's Pods again: a new annotation on their template.
pub fn restart_patch(at: &str) -> serde_json::Value {
    json!({
        "spec": { "template": { "metadata": { "annotations": { RESTARTED_AT: at } } } }
    })
}

/// How objects are deleted: their dependents in the background.
pub fn delete_params(grace: Option<u32>) -> DeleteParams {
    DeleteParams {
        propagation_policy: Some(PropagationPolicy::Background),
        grace_period_seconds: grace,
        ..Default::default()
    }
}

/// A Job made from a CronJob's template, as `kubectl create job --from=cronjob/…`
/// makes it: owned by the CronJob and marked as started by hand.
pub fn job_from(cron: &CronJob, suffix: &str) -> Job {
    let cron_name = cron.metadata.name.clone().unwrap_or_default();
    let template = cron
        .spec
        .as_ref()
        .map(|s| s.job_template.clone())
        .unwrap_or_default();
    let meta = template.metadata.unwrap_or_default();
    let mut annotations: BTreeMap<String, String> = meta.annotations.unwrap_or_default();
    annotations.insert(INSTANTIATE.into(), "manual".into());
    let owner = cron.metadata.uid.as_ref().map(|uid| OwnerReference {
        api_version: "batch/v1".into(),
        kind: "CronJob".into(),
        name: cron_name.clone(),
        uid: uid.clone(),
        controller: Some(true),
        block_owner_deletion: Some(true),
    });
    Job {
        metadata: ObjectMeta {
            name: Some(manual_job_name(&cron_name, suffix)),
            namespace: cron.metadata.namespace.clone(),
            labels: meta.labels,
            annotations: Some(annotations),
            owner_references: owner.map(|o| vec![o]),
            ..Default::default()
        },
        spec: template.spec,
        status: None,
    }
}

/// `<cronjob>-manual-<suffix>`, the CronJob's name cut so it fits.
fn manual_job_name(cron: &str, suffix: &str) -> String {
    let tail = format!("-manual-{suffix}");
    let keep = JOB_NAME_MAX.saturating_sub(tail.len());
    let head: String = cron.chars().take(keep).collect();
    format!("{}{tail}", head.trim_end_matches('-'))
}

fn not_supported(message: homeport_core::error::Message, kind: WorkloadKind) -> AppError {
    AppError::validation(message.with("kind", kind.resource().kind()))
}

#[cfg(test)]
mod tests {
    use k8s_openapi::api::batch::v1::{CronJobSpec, JobSpec, JobTemplateSpec};

    use super::*;

    #[test]
    fn a_restart_annotates_the_pod_template_like_kubectl() {
        assert_eq!(
            restart_patch("2026-11-08T10:00:00Z"),
            json!({
                "spec": { "template": { "metadata": { "annotations": {
                    "kubectl.kubernetes.io/restartedAt": "2026-11-08T10:00:00Z"
                } } } }
            })
        );
    }

    #[test]
    fn deletes_leave_dependents_to_the_garbage_collector() {
        let params = delete_params(None);
        assert_eq!(
            params.propagation_policy,
            Some(PropagationPolicy::Background)
        );
        assert_eq!(params.grace_period_seconds, None);
        assert_eq!(delete_params(Some(0)).grace_period_seconds, Some(0));
    }

    fn cron() -> CronJob {
        CronJob {
            metadata: ObjectMeta {
                name: Some("nightly-report".into()),
                namespace: Some("shop".into()),
                uid: Some("u-1".into()),
                ..Default::default()
            },
            spec: Some(CronJobSpec {
                schedule: "0 3 * * *".into(),
                job_template: JobTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some([("app".to_owned(), "report".to_owned())].into()),
                        annotations: Some([("team".to_owned(), "data".to_owned())].into()),
                        ..Default::default()
                    }),
                    spec: Some(JobSpec {
                        backoff_limit: Some(2),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            status: None,
        }
    }

    #[test]
    fn a_triggered_job_copies_the_template_and_is_owned_by_its_cron_job() {
        let job = job_from(&cron(), "ab12c");
        assert_eq!(
            job.metadata.name.as_deref(),
            Some("nightly-report-manual-ab12c")
        );
        assert_eq!(job.metadata.namespace.as_deref(), Some("shop"));
        assert_eq!(job.metadata.labels.unwrap()["app"], "report");
        let annotations = job.metadata.annotations.unwrap();
        assert_eq!(annotations[INSTANTIATE], "manual");
        assert_eq!(annotations["team"], "data");
        let owner = &job.metadata.owner_references.unwrap()[0];
        assert_eq!(
            (owner.kind.as_str(), owner.name.as_str(), owner.uid.as_str()),
            ("CronJob", "nightly-report", "u-1")
        );
        assert_eq!(owner.controller, Some(true));
        assert_eq!(job.spec.unwrap().backoff_limit, Some(2));
    }

    #[test]
    fn a_long_cron_job_name_is_cut_to_fit_a_job_name() {
        let name = manual_job_name(&"a".repeat(80), "ab12c");
        assert_eq!(name.len(), JOB_NAME_MAX);
        assert!(name.ends_with("-manual-ab12c"));
        assert_eq!(manual_job_name("x-", "ab12c"), "x-manual-ab12c");
    }
}
