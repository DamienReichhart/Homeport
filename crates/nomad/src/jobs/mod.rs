//! A Nomad Cluster's jobs: listing them with their task groups' counts,
//! a job's detail with its allocations, stopping (and purging) one, and
//! scaling a task group within its scaling policy.

mod model;
pub mod run;
mod wire;

use std::collections::BTreeMap;

use futures_util::{stream, StreamExt};
use serde_json::json;

use homeport_core::error::{AppError, AppResult, ErrorCode, Message, Text};

pub use self::model::{
    NomadAllocationView, NomadGroupCounts, NomadJobDetail, NomadJobView, NomadScaling,
    NomadTaskGroupView, NomadTaskState, NomadTaskView,
};
pub(crate) use self::wire::{AllocationStub, GroupSummary, Job};
use self::wire::{JobStub, JobSummary, TaskGroup};
use crate::{
    api::{
        client::{segment, Nomad},
        time::iso,
    },
    messages as msg,
};

/// How many jobs' definitions are read at once to learn their desired counts.
const DEFINITIONS_AT_ONCE: usize = 8;

/// The namespace query of a job call: the token's default without one.
pub(crate) fn namespace_query(namespace: Option<&str>) -> Vec<(&'static str, String)> {
    namespace
        .filter(|ns| !ns.is_empty())
        .map(|ns| vec![("namespace", ns.to_owned())])
        .unwrap_or_default()
}

fn counts(name: &str, desired: Option<u32>, summary: Option<&GroupSummary>) -> NomadGroupCounts {
    let s = summary.cloned().unwrap_or_default();
    NomadGroupCounts {
        name: name.to_owned(),
        desired,
        running: s.running,
        starting: s.starting,
        queued: s.queued,
        failed: s.failed,
        lost: s.lost + s.unknown,
        complete: s.complete,
    }
}

fn some(text: String) -> Option<String> {
    (!text.is_empty()).then_some(text)
}

/// A count Nomad gives as u64, as the frontend's numbers hold it.
fn small(n: u64) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

fn job_view(stub: JobStub, desired: &BTreeMap<String, u32>) -> NomadJobView {
    let summary = stub.job_summary.map(|s| s.summary).unwrap_or_default();
    NomadJobView {
        groups: summary
            .iter()
            .map(|(name, s)| counts(name, desired.get(name).copied(), Some(s)))
            .collect(),
        id: stub.id,
        name: stub.name,
        namespace: stub.namespace,
        job_type: stub.job_type,
        status: stub.status,
        status_description: some(stub.status_description),
        stopped: stub.stop,
        priority: stub.priority,
        periodic: stub.periodic,
        parameterized: stub.parameterized_job,
        parent_id: some(stub.parent_id),
        submitted: iso(stub.submit_time),
    }
}

pub(crate) fn allocation_view(a: AllocationStub) -> NomadAllocationView {
    NomadAllocationView {
        tasks: a
            .task_states
            .unwrap_or_default()
            .into_iter()
            .map(|(name, s)| NomadTaskState {
                name,
                state: s.state,
                failed: s.failed,
                restarts: s.restarts,
                started: s.started_at.filter(|t| !t.starts_with("0001-")),
            })
            .collect(),
        id: a.id,
        name: a.name,
        namespace: a.namespace,
        job_id: a.job_id,
        task_group: a.task_group,
        node_id: a.node_id,
        node_name: a.node_name,
        client_status: a.client_status,
        desired_status: a.desired_status,
        job_version: small(a.job_version),
        created: iso(a.create_time),
        modified: iso(a.modify_time),
    }
}

fn detail(job: Job, summary: JobSummary, allocations: Vec<AllocationStub>) -> NomadJobDetail {
    let groups = job
        .task_groups
        .iter()
        .map(|g| NomadTaskGroupView {
            counts: counts(&g.name, g.count, summary.summary.get(&g.name)),
            scaling: g.scaling.as_ref().map(|s| NomadScaling {
                min: s.min.map(small),
                max: s.max.map(small),
                enabled: s.enabled.unwrap_or(true),
            }),
            tasks: g
                .tasks
                .iter()
                .map(|t| NomadTaskView {
                    name: t.name.clone(),
                    driver: t.driver.clone(),
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    let mut allocations: Vec<_> = allocations.into_iter().map(allocation_view).collect();
    allocations.sort_by(|a, b| b.created.cmp(&a.created));
    NomadJobDetail {
        job: NomadJobView {
            groups: groups.iter().map(|g| g.counts.clone()).collect(),
            id: job.id,
            name: job.name,
            namespace: job.namespace,
            job_type: job.job_type,
            status: job.status,
            status_description: some(job.status_description),
            stopped: job.stop,
            priority: job.priority,
            periodic: job.periodic.is_some_and(|p| !p.is_null()),
            parameterized: job.parameterized_job.is_some_and(|p| !p.is_null()),
            parent_id: job.parent_id.filter(|p| !p.is_empty()),
            submitted: iso(job.submit_time),
        },
        version: small(job.version),
        datacenters: job.datacenters,
        node_pool: job.node_pool.filter(|p| !p.is_empty()),
        groups,
        allocations,
    }
}

/// Checks `count` against the scaling policy of `group`, if it has one.
fn check_bounds(job: &str, group: &TaskGroup, count: u32) -> AppResult<()> {
    let Some(scaling) = &group.scaling else {
        return Ok(());
    };
    if let Some(min) = scaling.min.filter(|min| u64::from(count) < *min) {
        return Err(AppError::new(
            ErrorCode::NomadScaleOutOfBounds,
            msg::SCALE_BELOW_MIN
                .with("group", &group.name)
                .with("job", job)
                .with("min", min),
        )
        .with_hint(msg::SCALE_BOUNDS_HINT));
    }
    if let Some(max) = scaling.max.filter(|max| u64::from(count) > *max) {
        return Err(AppError::new(
            ErrorCode::NomadScaleOutOfBounds,
            msg::SCALE_ABOVE_MAX
                .with("group", &group.name)
                .with("job", job)
                .with("max", max),
        )
        .with_hint(msg::SCALE_BOUNDS_HINT));
    }
    Ok(())
}

/// A denied error said for what was being done; others as they are.
fn denied(error: AppError, message: Text, hint: Message) -> AppError {
    if error.is(ErrorCode::NomadPermissionDenied) {
        AppError {
            detail: error.detail,
            ..AppError::new(ErrorCode::NomadPermissionDenied, message).with_hint(hint)
        }
    } else {
        error
    }
}

impl Nomad {
    /// A job that isn't found, named.
    pub(crate) fn job_not_found(&self, error: AppError, job: &str) -> AppError {
        if error.is(ErrorCode::NotFound) {
            AppError {
                detail: error.detail,
                ..AppError::new(
                    ErrorCode::NomadJobNotFound,
                    msg::JOB_NOT_FOUND
                        .with("name", self.name())
                        .with("job", job),
                )
                .with_hint(msg::JOB_NOT_FOUND_HINT)
            }
        } else {
            error
        }
    }

    fn read_denied(&self, error: AppError) -> AppError {
        denied(
            error,
            msg::READ_DENIED.with("name", self.name()),
            msg::READ_DENIED_HINT,
        )
    }

    /// The job `id`'s definition, as Nomad keeps it.
    pub(crate) async fn job_definition(&self, namespace: Option<&str>, id: &str) -> AppResult<Job> {
        self.get(
            &format!("/v1/job/{}", segment(id)),
            &namespace_query(namespace),
        )
        .await
        .map_err(|e| self.read_denied(self.job_not_found(e, id)))
    }

    /// The jobs in `namespace` (the token's default without one, every
    /// namespace with `*`), by name, each with its task groups' counts. The
    /// desired counts are read from each job's definition, a few at a time.
    pub async fn jobs(&self, namespace: Option<&str>) -> AppResult<Vec<NomadJobView>> {
        let stubs: Vec<JobStub> = self
            .get("/v1/jobs", &namespace_query(namespace))
            .await
            .map_err(|e| self.read_denied(e))?;
        let wanted: Vec<(String, String)> = stubs
            .iter()
            .map(|s| (s.namespace.clone(), s.id.clone()))
            .collect();
        let desired: Vec<BTreeMap<String, u32>> = stream::iter(wanted)
            .map(|(namespace, id)| {
                let nomad = self.clone();
                async move { nomad.desired_counts(&namespace, &id).await }
            })
            .buffered(DEFINITIONS_AT_ONCE)
            .collect()
            .await;
        let mut jobs: Vec<_> = stubs
            .into_iter()
            .zip(desired)
            .map(|(stub, desired)| job_view(stub, &desired))
            .collect();
        jobs.sort_by(|a, b| (&a.name, &a.namespace).cmp(&(&b.name, &b.namespace)));
        Ok(jobs)
    }

    /// The count each task group of job `id` wants; empty when it can't be
    /// read, as for a job gone since it was listed.
    async fn desired_counts(&self, namespace: &str, id: &str) -> BTreeMap<String, u32> {
        match self.job_definition(Some(namespace), id).await {
            Ok(job) => job
                .task_groups
                .into_iter()
                .filter_map(|g| Some((g.name, g.count?)))
                .collect(),
            Err(_) => BTreeMap::new(),
        }
    }

    /// The job `id` with its task groups and allocations, newest first.
    pub async fn job(&self, namespace: Option<&str>, id: &str) -> AppResult<NomadJobDetail> {
        let query = namespace_query(namespace);
        let path = format!("/v1/job/{}", segment(id));
        let job = self.job_definition(namespace, id).await?;
        let summary: JobSummary = self
            .get(&format!("{path}/summary"), &query)
            .await
            .map_err(|e| self.job_not_found(e, id))?;
        let allocations: Vec<AllocationStub> = self
            .get(&format!("{path}/allocations"), &query)
            .await
            .map_err(|e| self.job_not_found(e, id))?;
        Ok(detail(job, summary, allocations))
    }

    /// The allocations of the job `id`, newest first.
    pub async fn job_allocations(
        &self,
        namespace: Option<&str>,
        id: &str,
    ) -> AppResult<Vec<NomadAllocationView>> {
        let stubs: Vec<AllocationStub> = self
            .get(
                &format!("/v1/job/{}/allocations", segment(id)),
                &namespace_query(namespace),
            )
            .await
            .map_err(|e| self.read_denied(self.job_not_found(e, id)))?;
        let mut allocations: Vec<_> = stubs.into_iter().map(allocation_view).collect();
        allocations.sort_by(|a, b| b.created.cmp(&a.created));
        Ok(allocations)
    }

    /// Stops the job `id`, and with `purge` removes it from Nomad at once
    /// instead of leaving it to garbage collection.
    pub async fn stop_job(&self, namespace: Option<&str>, id: &str, purge: bool) -> AppResult<()> {
        let mut query = namespace_query(namespace);
        if purge {
            query.push(("purge", "true".into()));
        }
        self.delete::<serde_json::Value>(&format!("/v1/job/{}", segment(id)), &query)
            .await
            .map(drop)
            .map_err(|e| {
                denied(
                    self.job_not_found(e, id),
                    msg::STOP_DENIED.with("name", self.name()),
                    msg::STOP_DENIED_HINT,
                )
            })
    }

    /// Sets how many allocations task group `group` of job `id` runs,
    /// refusing a count outside its scaling policy before asking Nomad.
    pub async fn scale_task_group(
        &self,
        namespace: Option<&str>,
        id: &str,
        group: &str,
        count: u32,
    ) -> AppResult<()> {
        let job = self.job_definition(namespace, id).await?;
        if let Some(definition) = job.task_groups.iter().find(|g| g.name == group) {
            check_bounds(id, definition, count)?;
        }
        let body = json!({
            "Count": count,
            "Target": { "Group": group },
            "Message": "Scaled from Homeport",
        });
        self.post::<serde_json::Value>(
            &format!("/v1/job/{}/scale", segment(id)),
            &namespace_query(namespace),
            &body,
        )
        .await
        .map(drop)
        .map_err(|e| {
            let text = e.detail.clone().unwrap_or_default();
            if crate::api::error::mentions(&text, &["active deployment"]) {
                return AppError::new(ErrorCode::Conflict, msg::SCALE_DEPLOYING.with("job", id))
                    .with_hint(msg::SCALE_DEPLOYING_HINT)
                    .with_detail(text);
            }
            if e.is(ErrorCode::BadRequest) {
                AppError {
                    hint: AppError::new(ErrorCode::BadRequest, msg::BAD_REQUEST)
                        .with_hint(msg::SCALE_REFUSED_HINT)
                        .hint,
                    ..e
                }
            } else {
                denied(
                    self.job_not_found(e, id),
                    msg::SCALE_DENIED.with("name", self.name()),
                    msg::SCALE_DENIED_HINT,
                )
            }
        })
    }
}

#[cfg(test)]
mod tests;
