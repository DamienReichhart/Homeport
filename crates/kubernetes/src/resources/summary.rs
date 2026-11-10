//! A Cluster at a glance, computed here so the Overview doesn't fetch every
//! Pod into the webview. Each part fails on its own: credentials that may
//! not list Nodes still see their Pods.

use serde::Serialize;

use homeport_core::error::AppError;

use super::{
    events::EventView,
    pods::PodHealth,
    workloads::{WorkloadHealth, WorkloadKind, WorkloadView},
    On,
};

/// The Warning events shown at most.
const WARNINGS: usize = 5;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NodeCounts {
    pub ready: u32,
    pub total: u32,
    /// The Nodes that aren't ready, by name.
    pub not_ready: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodCounts {
    pub total: u32,
    pub running: u32,
    pub pending: u32,
    pub succeeded: u32,
    pub failed: u32,
    /// Crashing, or unable to pull their image.
    pub crashing: u32,
    pub other: u32,
}

/// One part of the summary, or why it couldn't be read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum Part<T> {
    Read { value: T },
    Failed { error: AppError },
}

impl<T> From<homeport_core::error::AppResult<T>> for Part<T> {
    fn from(result: homeport_core::error::AppResult<T>) -> Self {
        match result {
            Ok(value) => Self::Read { value },
            Err(error) => Self::Failed { error },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummary {
    pub nodes: Part<NodeCounts>,
    pub pods: Part<PodCounts>,
    /// Deployments, StatefulSets and DaemonSets with fewer Pods ready than wanted.
    pub unready: Part<Vec<WorkloadView>>,
    /// The latest Warning events.
    pub warnings: Part<Vec<EventView>>,
}

pub fn count_pods(health: impl IntoIterator<Item = PodHealth>) -> PodCounts {
    let mut counts = PodCounts::default();
    for h in health {
        counts.total += 1;
        match h {
            PodHealth::Running => counts.running += 1,
            PodHealth::Pending => counts.pending += 1,
            PodHealth::Succeeded => counts.succeeded += 1,
            PodHealth::Failed => counts.failed += 1,
            PodHealth::Crashing => counts.crashing += 1,
            PodHealth::Terminating | PodHealth::Unknown => counts.other += 1,
        }
    }
    counts
}

/// Whether a workload has fewer Pods ready than it wants.
pub fn is_unready(w: &WorkloadView) -> bool {
    matches!(
        w.health,
        WorkloadHealth::Progressing | WorkloadHealth::Degraded
    ) && w.ready < w.desired
}

impl On {
    /// The Cluster at a glance, in `namespace` (every one with `None`) for
    /// what lives in Namespaces.
    pub async fn summary(&self, namespace: Option<&str>) -> ClusterSummary {
        let nodes = async {
            let nodes = self.nodes().await?;
            Ok(NodeCounts {
                ready: nodes.iter().filter(|n| n.ready).count() as u32,
                total: nodes.len() as u32,
                not_ready: nodes
                    .iter()
                    .filter(|n| !n.ready)
                    .map(|n| n.name.clone())
                    .collect(),
            })
        };
        let pods = async {
            let pods = self.pods(namespace, None, None).await?;
            Ok(count_pods(pods.iter().map(|p| p.health)))
        };
        let unready = async {
            let (d, s, ds) = tokio::join!(
                self.workloads(WorkloadKind::Deployment, namespace),
                self.workloads(WorkloadKind::StatefulSet, namespace),
                self.workloads(WorkloadKind::DaemonSet, namespace),
            );
            let mut unready: Vec<WorkloadView> = [d?, s?, ds?]
                .into_iter()
                .flatten()
                .filter(is_unready)
                .collect();
            unready.sort_by(|a, b| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)));
            Ok::<_, AppError>(unready)
        };
        let warnings = async {
            let mut events = self.events(namespace).await?;
            events.retain(|e| e.kind == "Warning");
            events.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
            events.truncate(WARNINGS);
            Ok(events)
        };
        let (nodes, pods, unready, warnings) = tokio::join!(nodes, pods, unready, warnings);
        ClusterSummary {
            nodes: nodes.into(),
            pods: pods.into(),
            unready: unready.into(),
            warnings: warnings.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pods_are_counted_by_how_they_do() {
        let counts = count_pods([
            PodHealth::Running,
            PodHealth::Running,
            PodHealth::Crashing,
            PodHealth::Pending,
            PodHealth::Succeeded,
            PodHealth::Terminating,
        ]);
        assert_eq!(
            counts,
            PodCounts {
                total: 6,
                running: 2,
                pending: 1,
                succeeded: 1,
                failed: 0,
                crashing: 1,
                other: 1,
            }
        );
    }

    #[test]
    fn only_workloads_short_of_pods_are_unready() {
        let view = |ready, desired, health| WorkloadView {
            kind: WorkloadKind::Deployment,
            namespace: "shop".into(),
            name: "web".into(),
            ready,
            desired,
            up_to_date: None,
            health,
            images: vec![],
            created: None,
            schedule: None,
            last_schedule: None,
            owner: None,
        };
        assert!(is_unready(&view(1, 3, WorkloadHealth::Progressing)));
        assert!(is_unready(&view(0, 2, WorkloadHealth::Degraded)));
        assert!(!is_unready(&view(3, 3, WorkloadHealth::Ready)));
        assert!(!is_unready(&view(0, 0, WorkloadHealth::Paused)));
        assert!(
            !is_unready(&view(3, 3, WorkloadHealth::Progressing)),
            "rolling out, all ready"
        );
    }

    #[test]
    fn a_part_is_read_or_says_why_not() {
        let read: Part<u32> = Ok(3).into();
        assert_eq!(read, Part::Read { value: 3 });
        let failed: Part<u32> = Err(super::super::forbidden(
            "list",
            super::super::ResourceKind::Node,
            None,
        ))
        .into();
        assert!(
            matches!(failed, Part::Failed { error } if error.is(homeport_core::error::ErrorCode::Forbidden))
        );
    }
}
