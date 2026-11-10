//! Deployments, StatefulSets, DaemonSets, Jobs and CronJobs, as one kind of
//! view: how many Pods are ready of how many wanted, and whether that's healthy.

use std::collections::BTreeMap;

use k8s_openapi::api::{
    apps::v1::{DaemonSet, Deployment, StatefulSet},
    batch::v1::{CronJob, Job},
    core::v1::PodTemplateSpec,
};
use serde::{Deserialize, Serialize};

use crate::resources::common::{
    annotations, conditions, created, labels, owner, time, ConditionView, OwnerView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum WorkloadKind {
    Deployment,
    StatefulSet,
    DaemonSet,
    Job,
    CronJob,
}

impl WorkloadKind {
    pub fn resource(self) -> crate::resources::ResourceKind {
        use crate::resources::ResourceKind as R;
        match self {
            Self::Deployment => R::Deployment,
            Self::StatefulSet => R::StatefulSet,
            Self::DaemonSet => R::DaemonSet,
            Self::Job => R::Job,
            Self::CronJob => R::CronJob,
        }
    }
}

/// Whether a workload does what it's asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum WorkloadHealth {
    /// Every wanted Pod is ready (or a Job finished, a CronJob is scheduled).
    Ready,
    /// Rolling out, scheduling or running towards what it wants.
    Progressing,
    /// Stuck: a rollout past its deadline, Pods unavailable.
    Degraded,
    /// A Job that failed.
    Failed,
    /// A Job or CronJob that is suspended, or scaled to zero.
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WorkloadView {
    pub kind: WorkloadKind,
    pub namespace: String,
    pub name: String,
    /// Pods ready (succeeded for a Job, active for a CronJob).
    pub ready: u32,
    /// Pods wanted (completions for a Job; 0 for a CronJob).
    pub desired: u32,
    /// Pods running the newest template, for rollouts.
    pub up_to_date: Option<u32>,
    pub health: WorkloadHealth,
    pub images: Vec<String>,
    pub created: Option<String>,
    /// A CronJob's schedule.
    pub schedule: Option<String>,
    /// When a CronJob last started a Job.
    pub last_schedule: Option<String>,
    pub owner: Option<OwnerView>,
}

/// A workload with what its page shows beyond the list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WorkloadDetail {
    pub view: WorkloadView,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    /// The labels its Pods are selected by, as `key=value,…`; empty for a CronJob.
    pub selector: String,
    pub conditions: Vec<ConditionView>,
    pub containers: Vec<TemplateContainer>,
    /// How it rolls out (`RollingUpdate`, `Recreate`, `OnDelete`).
    pub strategy: Option<String>,
}

/// A container of a workload's Pod template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TemplateContainer {
    pub name: String,
    pub image: String,
    /// `8080/TCP`, …
    pub ports: Vec<String>,
}

fn images(template: Option<&PodTemplateSpec>) -> Vec<String> {
    template
        .and_then(|t| t.spec.as_ref())
        .map(|s| {
            s.containers
                .iter()
                .filter_map(|c| c.image.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn containers(template: Option<&PodTemplateSpec>) -> Vec<TemplateContainer> {
    template
        .and_then(|t| t.spec.as_ref())
        .map(|s| {
            s.containers
                .iter()
                .map(|c| TemplateContainer {
                    name: c.name.clone(),
                    image: c.image.clone().unwrap_or_default(),
                    ports: c
                        .ports
                        .iter()
                        .flatten()
                        .map(|p| {
                            format!(
                                "{}/{}",
                                p.container_port,
                                p.protocol.as_deref().unwrap_or("TCP")
                            )
                        })
                        .collect(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn count(n: Option<i32>) -> u32 {
    n.unwrap_or(0).max(0) as u32
}

fn base(
    kind: WorkloadKind,
    meta: &k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
) -> WorkloadView {
    WorkloadView {
        kind,
        namespace: meta.namespace.clone().unwrap_or_default(),
        name: meta.name.clone().unwrap_or_default(),
        ready: 0,
        desired: 0,
        up_to_date: None,
        health: WorkloadHealth::Ready,
        images: vec![],
        created: created(meta),
        schedule: None,
        last_schedule: None,
        owner: owner(meta),
    }
}

/// Ready when every wanted replica is ready and up to date; degraded once
/// a rollout passed its deadline.
pub fn deployment(d: &Deployment) -> WorkloadView {
    let spec = d.spec.as_ref();
    let status = d.status.as_ref();
    let desired = count(spec.and_then(|s| s.replicas).or(Some(1)));
    let ready = count(status.and_then(|s| s.ready_replicas));
    let updated = count(status.and_then(|s| s.updated_replicas));
    let stalled = status
        .and_then(|s| s.conditions.as_ref())
        .into_iter()
        .flatten()
        .any(|c| c.type_ == "Progressing" && c.status == "False");
    let health = if desired == 0 {
        WorkloadHealth::Paused
    } else if stalled {
        WorkloadHealth::Degraded
    } else if ready >= desired && updated >= desired {
        WorkloadHealth::Ready
    } else {
        WorkloadHealth::Progressing
    };
    WorkloadView {
        ready,
        desired,
        up_to_date: Some(updated),
        health,
        images: images(spec.map(|s| &s.template)),
        ..base(WorkloadKind::Deployment, &d.metadata)
    }
}

pub fn stateful_set(s: &StatefulSet) -> WorkloadView {
    let spec = s.spec.as_ref();
    let status = s.status.as_ref();
    let desired = count(spec.and_then(|s| s.replicas).or(Some(1)));
    let ready = count(status.and_then(|s| s.ready_replicas));
    let updated = count(status.and_then(|s| s.updated_replicas));
    WorkloadView {
        ready,
        desired,
        up_to_date: Some(updated),
        health: replicas_health(ready, desired),
        images: images(spec.map(|s| &s.template)),
        ..base(WorkloadKind::StatefulSet, &s.metadata)
    }
}

pub fn daemon_set(d: &DaemonSet) -> WorkloadView {
    let status = d.status.as_ref();
    let desired = count(status.map(|s| s.desired_number_scheduled));
    let ready = count(status.map(|s| s.number_ready));
    let updated = count(status.and_then(|s| s.updated_number_scheduled));
    let health = if desired == 0 {
        WorkloadHealth::Ready
    } else {
        replicas_health(ready, desired)
    };
    WorkloadView {
        ready,
        desired,
        up_to_date: Some(updated),
        health,
        images: images(d.spec.as_ref().map(|s| &s.template)),
        ..base(WorkloadKind::DaemonSet, &d.metadata)
    }
}

fn replicas_health(ready: u32, desired: u32) -> WorkloadHealth {
    match desired {
        0 => WorkloadHealth::Paused,
        d if ready >= d => WorkloadHealth::Ready,
        _ => WorkloadHealth::Progressing,
    }
}

/// Done once complete, failed once failed, paused while suspended.
pub fn job(j: &Job) -> WorkloadView {
    let spec = j.spec.as_ref();
    let status = j.status.as_ref();
    let desired = count(spec.and_then(|s| s.completions).or(Some(1)));
    let succeeded = count(status.and_then(|s| s.succeeded));
    let condition = |kind: &str| {
        status
            .and_then(|s| s.conditions.as_ref())
            .into_iter()
            .flatten()
            .any(|c| c.type_ == kind && c.status == "True")
    };
    let health = if condition("Failed") {
        WorkloadHealth::Failed
    } else if condition("Complete") {
        WorkloadHealth::Ready
    } else if spec.and_then(|s| s.suspend) == Some(true) {
        WorkloadHealth::Paused
    } else {
        WorkloadHealth::Progressing
    };
    WorkloadView {
        ready: succeeded,
        desired,
        health,
        images: images(spec.map(|s| &s.template)),
        ..base(WorkloadKind::Job, &j.metadata)
    }
}

pub fn cron_job(c: &CronJob) -> WorkloadView {
    let spec = c.spec.as_ref();
    let status = c.status.as_ref();
    let active = status
        .and_then(|s| s.active.as_ref())
        .map_or(0, |a| a.len() as u32);
    WorkloadView {
        ready: active,
        desired: 0,
        health: if spec.and_then(|s| s.suspend) == Some(true) {
            WorkloadHealth::Paused
        } else {
            WorkloadHealth::Ready
        },
        images: images(
            spec.and_then(|s| s.job_template.spec.as_ref())
                .map(|j| &j.template),
        ),
        schedule: spec.map(|s| s.schedule.clone()),
        last_schedule: time(status.and_then(|s| s.last_schedule_time.as_ref())),
        ..base(WorkloadKind::CronJob, &c.metadata)
    }
}

fn selector(s: Option<&k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>) -> String {
    s.and_then(|s| s.match_labels.as_ref())
        .map(crate::resources::common::selector_string)
        .unwrap_or_default()
}

pub fn deployment_detail(d: &Deployment) -> WorkloadDetail {
    let spec = d.spec.as_ref();
    WorkloadDetail {
        view: deployment(d),
        labels: labels(&d.metadata),
        annotations: annotations(&d.metadata),
        selector: selector(spec.map(|s| &s.selector)),
        conditions: conditions!(d.status.as_ref().and_then(|s| s.conditions.as_ref())),
        containers: containers(spec.map(|s| &s.template)),
        strategy: spec.and_then(|s| s.strategy.as_ref()?.type_.clone()),
    }
}

pub fn stateful_set_detail(s: &StatefulSet) -> WorkloadDetail {
    let spec = s.spec.as_ref();
    WorkloadDetail {
        view: stateful_set(s),
        labels: labels(&s.metadata),
        annotations: annotations(&s.metadata),
        selector: selector(spec.map(|s| &s.selector)),
        conditions: conditions!(s.status.as_ref().and_then(|s| s.conditions.as_ref())),
        containers: containers(spec.map(|s| &s.template)),
        strategy: spec.and_then(|s| s.update_strategy.as_ref()?.type_.clone()),
    }
}

pub fn daemon_set_detail(d: &DaemonSet) -> WorkloadDetail {
    let spec = d.spec.as_ref();
    WorkloadDetail {
        view: daemon_set(d),
        labels: labels(&d.metadata),
        annotations: annotations(&d.metadata),
        selector: selector(spec.map(|s| &s.selector)),
        conditions: conditions!(d.status.as_ref().and_then(|s| s.conditions.as_ref())),
        containers: containers(spec.map(|s| &s.template)),
        strategy: spec.and_then(|s| s.update_strategy.as_ref()?.type_.clone()),
    }
}

pub fn job_detail(j: &Job) -> WorkloadDetail {
    let spec = j.spec.as_ref();
    WorkloadDetail {
        view: job(j),
        labels: labels(&j.metadata),
        annotations: annotations(&j.metadata),
        selector: selector(spec.and_then(|s| s.selector.as_ref())),
        conditions: conditions!(j.status.as_ref().and_then(|s| s.conditions.as_ref())),
        containers: containers(spec.map(|s| &s.template)),
        strategy: None,
    }
}

pub fn cron_job_detail(c: &CronJob) -> WorkloadDetail {
    let template = c
        .spec
        .as_ref()
        .and_then(|s| s.job_template.spec.as_ref())
        .map(|j| &j.template);
    WorkloadDetail {
        view: cron_job(c),
        labels: labels(&c.metadata),
        annotations: annotations(&c.metadata),
        selector: String::new(),
        conditions: vec![],
        containers: containers(template),
        strategy: c.spec.as_ref().and_then(|s| s.concurrency_policy.clone()),
    }
}
