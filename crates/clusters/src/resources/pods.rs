//! Pods: their status as `kubectl get pods` sums it up, and their containers.

use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::{ContainerStatus, Namespace, Pod};
use serde::Serialize;

use super::common::{
    annotations, conditions, created, labels, owner, time, ConditionView, OwnerView,
};

/// How a Pod is doing, for its colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum PodHealth {
    Running,
    Pending,
    Succeeded,
    Failed,
    /// A container that keeps crashing, or can't pull its image.
    Crashing,
    Terminating,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodView {
    pub namespace: String,
    pub name: String,
    /// Its phase (`Running`, `Pending`…).
    pub phase: String,
    /// What `kubectl get pods` shows: the phase, or why a container waits
    /// or ended (`CrashLoopBackOff`, `ImagePullBackOff`, `OOMKilled`…).
    pub status: String,
    pub health: PodHealth,
    /// Containers ready, of `containers`.
    pub ready: u32,
    pub containers: u32,
    pub restarts: u32,
    pub node: Option<String>,
    pub ip: Option<String>,
    pub created: Option<String>,
    pub owner: Option<OwnerView>,
    pub images: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodDetail {
    pub view: PodView,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    pub conditions: Vec<ConditionView>,
    pub containers: Vec<PodContainer>,
    pub host_ip: Option<String>,
    pub qos_class: Option<String>,
    pub service_account: Option<String>,
}

/// A container of a Pod and its state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodContainer {
    pub name: String,
    pub image: String,
    /// An init container, run before the others.
    pub init: bool,
    /// `running`, `waiting` or `terminated`.
    pub state: String,
    pub reason: Option<String>,
    pub message: Option<String>,
    pub started: Option<String>,
    pub exit_code: Option<i32>,
    pub ready: bool,
    pub restarts: u32,
    /// Why it last ended, when it restarted.
    pub last_reason: Option<String>,
}

/// The status column of `kubectl get pods`, simplified: a terminating Pod,
/// then the first waiting or failed container's reason, then the phase.
fn status(pod: &Pod) -> (String, PodHealth) {
    let phase = pod
        .status
        .as_ref()
        .and_then(|s| s.phase.clone())
        .unwrap_or_else(|| "Unknown".into());
    if pod.metadata.deletion_timestamp.is_some() {
        return ("Terminating".into(), PodHealth::Terminating);
    }
    let status = pod.status.as_ref();
    let reason_of = |statuses: Option<&Vec<ContainerStatus>>, init: bool| {
        statuses.into_iter().flatten().find_map(|c| {
            let state = c.state.as_ref()?;
            let reason = if let Some(waiting) = &state.waiting {
                waiting.reason.clone()?
            } else if let Some(ended) = &state.terminated {
                // An init container that finished well lets the Pod go on.
                if init && ended.exit_code == 0 {
                    return None;
                }
                ended
                    .reason
                    .clone()
                    .unwrap_or_else(|| format!("ExitCode:{}", ended.exit_code))
            } else {
                return None;
            };
            Some(if init {
                format!("Init:{reason}")
            } else {
                reason
            })
        })
    };
    let reason = reason_of(
        status.and_then(|s| s.init_container_statuses.as_ref()),
        true,
    )
    .or_else(|| reason_of(status.and_then(|s| s.container_statuses.as_ref()), false))
    .or_else(|| status.and_then(|s| s.reason.clone()));
    let text = match &reason {
        Some(r) if !(phase == "Succeeded" && r == "Completed") => r.clone(),
        _ => phase.clone(),
    };
    let crashing = [
        "CrashLoopBackOff",
        "ImagePullBackOff",
        "ErrImagePull",
        "CreateContainerConfigError",
        "InvalidImageName",
        "OOMKilled",
        "Error",
    ];
    let health = if crashing.iter().any(|c| text.ends_with(c)) {
        PodHealth::Crashing
    } else {
        match phase.as_str() {
            "Running" => PodHealth::Running,
            "Pending" => PodHealth::Pending,
            "Succeeded" => PodHealth::Succeeded,
            "Failed" => PodHealth::Failed,
            _ => PodHealth::Unknown,
        }
    };
    (text, health)
}

pub fn view(pod: &Pod) -> PodView {
    let (status_text, health) = status(pod);
    let status = pod.status.as_ref();
    let spec = pod.spec.as_ref();
    let statuses = status.and_then(|s| s.container_statuses.as_ref());
    PodView {
        namespace: pod.metadata.namespace.clone().unwrap_or_default(),
        name: pod.metadata.name.clone().unwrap_or_default(),
        phase: status
            .and_then(|s| s.phase.clone())
            .unwrap_or_else(|| "Unknown".into()),
        status: status_text,
        health,
        ready: statuses.into_iter().flatten().filter(|c| c.ready).count() as u32,
        containers: spec.map_or(0, |s| s.containers.len() as u32),
        restarts: statuses
            .into_iter()
            .flatten()
            .map(|c| c.restart_count.max(0) as u32)
            .sum(),
        node: spec.and_then(|s| s.node_name.clone()),
        ip: status.and_then(|s| s.pod_ip.clone()),
        created: created(&pod.metadata),
        owner: owner(&pod.metadata),
        images: spec
            .map(|s| {
                s.containers
                    .iter()
                    .filter_map(|c| c.image.clone())
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn container(
    c: &k8s_openapi::api::core::v1::Container,
    status: Option<&ContainerStatus>,
    init: bool,
) -> PodContainer {
    let state = status.and_then(|s| s.state.as_ref());
    let (kind, reason, message, started, exit_code) = match state {
        Some(s) if s.running.is_some() => (
            "running",
            None,
            None,
            time(s.running.as_ref().and_then(|r| r.started_at.as_ref())),
            None,
        ),
        Some(s) if s.terminated.is_some() => {
            let t = s.terminated.as_ref().expect("checked");
            (
                "terminated",
                t.reason.clone(),
                t.message.clone(),
                time(t.started_at.as_ref()),
                Some(t.exit_code),
            )
        }
        Some(s) => {
            let w = s.waiting.as_ref();
            (
                "waiting",
                w.and_then(|w| w.reason.clone()),
                w.and_then(|w| w.message.clone()),
                None,
                None,
            )
        }
        None => ("waiting", None, None, None, None),
    };
    PodContainer {
        name: c.name.clone(),
        image: c.image.clone().unwrap_or_default(),
        init,
        state: kind.into(),
        reason,
        message,
        started,
        exit_code,
        ready: status.is_some_and(|s| s.ready),
        restarts: status.map_or(0, |s| s.restart_count.max(0) as u32),
        last_reason: status
            .and_then(|s| s.last_state.as_ref()?.terminated.as_ref()?.reason.clone()),
    }
}

pub fn detail(pod: &Pod) -> PodDetail {
    let spec = pod.spec.as_ref();
    let status = pod.status.as_ref();
    fn find<'a>(list: Option<&'a Vec<ContainerStatus>>, name: &str) -> Option<&'a ContainerStatus> {
        list.into_iter().flatten().find(|s| s.name == name)
    }
    let mut containers: Vec<PodContainer> = spec
        .and_then(|s| s.init_containers.as_ref())
        .into_iter()
        .flatten()
        .map(|c| {
            container(
                c,
                find(
                    status.and_then(|s| s.init_container_statuses.as_ref()),
                    &c.name,
                ),
                true,
            )
        })
        .collect();
    containers.extend(spec.into_iter().flat_map(|s| &s.containers).map(|c| {
        container(
            c,
            find(status.and_then(|s| s.container_statuses.as_ref()), &c.name),
            false,
        )
    }));
    PodDetail {
        view: view(pod),
        labels: labels(&pod.metadata),
        annotations: annotations(&pod.metadata),
        conditions: conditions!(status.and_then(|s| s.conditions.as_ref())),
        containers,
        host_ip: status.and_then(|s| s.host_ip.clone()),
        qos_class: status.and_then(|s| s.qos_class.clone()),
        service_account: spec.and_then(|s| s.service_account_name.clone()),
    }
}

/// A Namespace, as the filter and its list show it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceView {
    pub name: String,
    /// `Active` or `Terminating`.
    pub phase: String,
    pub created: Option<String>,
    pub labels: BTreeMap<String, String>,
}

pub fn namespace(ns: &Namespace) -> NamespaceView {
    NamespaceView {
        name: ns.metadata.name.clone().unwrap_or_default(),
        phase: ns
            .status
            .as_ref()
            .and_then(|s| s.phase.clone())
            .unwrap_or_else(|| "Active".into()),
        created: created(&ns.metadata),
        labels: labels(&ns.metadata),
    }
}
