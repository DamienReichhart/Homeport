//! CPU and memory of Pods and Nodes, from the Metrics API
//! (`metrics.k8s.io/v1beta1`) that metrics-server serves. Homeport installs
//! nothing: a Cluster without it says so, and every page still works.
//!
//! An open page polls through [`On::stream_metrics`] every 15 s
//! ([`EVERY`], metrics-server's resolution) until its stream is cancelled
//! or its channel dropped; a Pod's or a Node's page keeps the samples read
//! since it opened ([`history::KEPT`] at most), in memory only.

pub mod history;
pub mod quantity;

use std::{collections::BTreeMap, time::Duration};

use k8s_openapi::apimachinery::pkg::apis::meta::v1::{APIGroupList, ObjectMeta};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::time::MissedTickBehavior;

use homeport_core::error::{AppError, ErrorCode};

use self::history::History;
use crate::{
    messages as msg,
    resources::{ApiType, On},
};

/// The Metrics API's group and version.
pub const GROUP: &str = "metrics.k8s.io";
pub const VERSION: &str = "v1beta1";

/// How often an open page reads metrics, and how long a read may take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pace {
    pub every: Duration,
    /// Past this, metrics-server is taken as not answering, and the next
    /// read comes at its time.
    pub wait: Duration,
}

/// Every 15 s, metrics-server's default resolution, so each read is
/// usually a new sample; a read gets 10 s.
pub const EVERY: Pace = Pace {
    every: Duration::from_secs(15),
    wait: Duration::from_secs(10),
};

/// What a container, Pod or Node uses.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUsage {
    /// In millicores: 1000 is one CPU core.
    pub cpu: f64,
    /// In bytes: the working set, which is what the kubelet evicts on.
    pub memory: u64,
}

impl ResourceUsage {
    /// From a metrics object's `usage` map; what's missing or unreadable is 0.
    pub fn from_map(usage: &BTreeMap<String, String>) -> Self {
        Self {
            cpu: usage
                .get("cpu")
                .and_then(|q| quantity::millicores(q))
                .unwrap_or(0.0),
            memory: usage
                .get("memory")
                .and_then(|q| quantity::bytes(q))
                .unwrap_or(0),
        }
    }
}

impl std::ops::Add for ResourceUsage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cpu: self.cpu + other.cpu,
            memory: self.memory.saturating_add(other.memory),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodContainerUsage {
    pub name: String,
    pub usage: ResourceUsage,
}

/// A Pod's latest sample: the sum of its containers, and each of them.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PodUsage {
    pub namespace: String,
    pub name: String,
    /// When metrics-server read it, in Unix seconds.
    pub time: i64,
    pub usage: ResourceUsage,
    pub containers: Vec<PodContainerUsage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NodeUsage {
    pub name: String,
    /// When metrics-server read it, in Unix seconds.
    pub time: i64,
    pub usage: ResourceUsage,
}

/// One reading, at its Unix time in seconds.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UsageSample {
    pub time: i64,
    pub usage: ResourceUsage,
}

/// A container's samples since its Pod's page opened.
#[derive(Debug, Clone, PartialEq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContainerHistory {
    pub name: String,
    pub samples: Vec<UsageSample>,
}

/// What a page shows metrics of.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MetricsTarget {
    /// The Pods of a Namespace, or of every one: for the Pods list.
    Pods { namespace: Option<String> },
    /// Every Node: for the Nodes list.
    Nodes,
    /// One Pod, its containers' samples kept while its page is open.
    Pod { namespace: String, name: String },
    /// One Node, its samples kept while its page is open.
    Node { name: String },
}

/// What a metrics stream sends, every 15 s.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MetricsEvent {
    /// The Cluster serves no Metrics API: metrics-server isn't installed.
    /// Sent once; the stream ends.
    NotInstalled,
    /// The credentials may not read metrics. Sent once; the stream ends.
    Forbidden { error: AppError },
    /// The Metrics API is registered but doesn't answer, as while
    /// metrics-server starts. The stream goes on.
    NotResponding { error: AppError },
    /// Any other failure, such as the Cluster being unreachable. The stream goes on.
    Failed { error: AppError },
    /// Every Pod's latest sample.
    Pods { pods: Vec<PodUsage> },
    /// Every Node's latest sample.
    Nodes { nodes: Vec<NodeUsage> },
    /// Each container's samples since the page opened.
    Pod { containers: Vec<ContainerHistory> },
    /// The Node's samples since the page opened.
    Node { samples: Vec<UsageSample> },
}

/// A `PodMetrics` object, as the Metrics API sends it.
#[derive(Debug, Clone, Deserialize)]
pub struct PodMetrics {
    pub metadata: ObjectMeta,
    pub timestamp: Option<String>,
    #[serde(default)]
    pub containers: Vec<ContainerMetrics>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerMetrics {
    pub name: String,
    #[serde(default)]
    pub usage: BTreeMap<String, String>,
}

/// A `NodeMetrics` object, as the Metrics API sends it.
#[derive(Debug, Clone, Deserialize)]
pub struct NodeMetrics {
    pub metadata: ObjectMeta,
    pub timestamp: Option<String>,
    #[serde(default)]
    pub usage: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct List<T> {
    items: Vec<T>,
}

/// A metrics object's time in Unix seconds; now when it has none.
fn unix(timestamp: Option<&str>) -> i64 {
    timestamp
        .and_then(|t| t.parse::<jiff::Timestamp>().ok())
        .unwrap_or_else(jiff::Timestamp::now)
        .as_second()
}

pub fn pod_usage(metrics: &PodMetrics) -> PodUsage {
    let containers: Vec<PodContainerUsage> = metrics
        .containers
        .iter()
        .map(|c| PodContainerUsage {
            name: c.name.clone(),
            usage: ResourceUsage::from_map(&c.usage),
        })
        .collect();
    PodUsage {
        namespace: metrics.metadata.namespace.clone().unwrap_or_default(),
        name: metrics.metadata.name.clone().unwrap_or_default(),
        time: unix(metrics.timestamp.as_deref()),
        usage: containers
            .iter()
            .fold(ResourceUsage::default(), |sum, c| sum + c.usage),
        containers,
    }
}

pub fn node_usage(metrics: &NodeMetrics) -> NodeUsage {
    NodeUsage {
        name: metrics.metadata.name.clone().unwrap_or_default(),
        time: unix(metrics.timestamp.as_deref()),
        usage: ResourceUsage::from_map(&metrics.usage),
    }
}

/// Whether the discovery document lists `metrics.k8s.io/v1beta1`.
pub fn serves_metrics(groups: &APIGroupList) -> bool {
    groups.groups.iter().any(|g| {
        g.name == GROUP
            && g.versions
                .iter()
                .any(|v| v.group_version == format!("{GROUP}/{VERSION}"))
    })
}

/// The type of `plural` in the Metrics API, as RBAC and errors name it.
fn metrics_type(kind: &str, plural: &str, namespaced: bool) -> ApiType {
    ApiType {
        group: GROUP.into(),
        version: VERSION.into(),
        kind: kind.into(),
        plural: plural.into(),
        namespaced,
    }
}

/// Why a read of the Metrics API failed, as the stream tells it.
#[derive(Debug)]
enum Failure {
    /// The API is gone: no metrics-server.
    Gone,
    Forbidden(AppError),
    NotResponding(AppError),
    Other(AppError),
}

impl On {
    /// Whether the Cluster serves the Metrics API, from its discovery
    /// document (`/apis`).
    pub async fn serves_metrics(&self) -> Result<bool, AppError> {
        let groups = self.client.list_api_groups().await.map_err(|e| {
            crate::classify::classify(&self.cluster.name, self.cluster.address(), &e)
        })?;
        Ok(serves_metrics(&groups))
    }

    /// Reads `path` of the Metrics API; `None` for an object it has no
    /// sample of yet (a Pod that just started, or isn't running).
    async fn read<T: DeserializeOwned>(
        &self,
        path: &str,
        api: &ApiType,
        namespace: Option<&str>,
        one: bool,
        wait: Duration,
    ) -> Result<Option<T>, Failure> {
        let request = http::Request::get(format!("/apis/{GROUP}/{VERSION}/{path}"))
            .body(Vec::new())
            .map_err(|e| {
                Failure::Other(
                    AppError::new(
                        ErrorCode::ClusterApiError,
                        msg::API_ERROR.with("name", &self.cluster.name),
                    )
                    .with_detail(e),
                )
            })?;
        let verb = if one { "get" } else { "list" };
        // The API server holds a request for metrics-server until its own
        // timeout when it's registered but unreachable.
        let error = match tokio::time::timeout(wait, self.client.request::<T>(request)).await {
            Ok(Ok(answer)) => return Ok(Some(answer)),
            Ok(Err(error)) => error,
            Err(_) => {
                return Err(Failure::NotResponding(AppError::new(
                    ErrorCode::Timeout,
                    msg::TIMEOUT.with("name", &self.cluster.name),
                )))
            }
        };
        let code = match &error {
            kube::Error::Api(status) => Some(status.code),
            _ => None,
        };
        Err(match code {
            Some(404) if one => return Ok(None),
            Some(404) => Failure::Gone,
            Some(403) => Failure::Forbidden(self.failed_on(verb, api, namespace, None, &error)),
            Some(502..=504) => {
                Failure::NotResponding(self.failed_on(verb, api, namespace, None, &error))
            }
            _ => Failure::Other(self.failed_on(verb, api, namespace, None, &error)),
        })
    }

    async fn pod_usages(
        &self,
        namespace: Option<&str>,
        wait: Duration,
    ) -> Result<Vec<PodUsage>, Failure> {
        let path = match namespace {
            Some(ns) => format!("namespaces/{ns}/pods"),
            None => "pods".into(),
        };
        let api = metrics_type("PodMetrics", "pods", true);
        let list: Option<List<PodMetrics>> = self.read(&path, &api, namespace, false, wait).await?;
        let mut pods: Vec<PodUsage> = list
            .map(|l| l.items)
            .unwrap_or_default()
            .iter()
            .map(pod_usage)
            .collect();
        pods.sort_by(|a, b| (&a.namespace, &a.name).cmp(&(&b.namespace, &b.name)));
        Ok(pods)
    }

    async fn node_usages(&self, wait: Duration) -> Result<Vec<NodeUsage>, Failure> {
        let api = metrics_type("NodeMetrics", "nodes", false);
        let list: Option<List<NodeMetrics>> = self.read("nodes", &api, None, false, wait).await?;
        let mut nodes: Vec<NodeUsage> = list
            .map(|l| l.items)
            .unwrap_or_default()
            .iter()
            .map(node_usage)
            .collect();
        nodes.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(nodes)
    }

    /// The next event for `target`, its samples added to `history`.
    async fn sample(
        &self,
        target: &MetricsTarget,
        history: &mut History,
        wait: Duration,
    ) -> Result<MetricsEvent, Failure> {
        Ok(match target {
            MetricsTarget::Pods { namespace } => MetricsEvent::Pods {
                pods: self.pod_usages(namespace.as_deref(), wait).await?,
            },
            MetricsTarget::Nodes => MetricsEvent::Nodes {
                nodes: self.node_usages(wait).await?,
            },
            MetricsTarget::Pod { namespace, name } => {
                let api = metrics_type("PodMetrics", "pods", true);
                let path = format!("namespaces/{namespace}/pods/{name}");
                let metrics: Option<PodMetrics> =
                    self.read(&path, &api, Some(namespace), true, wait).await?;
                if let Some(pod) = metrics.as_ref().map(pod_usage) {
                    for c in &pod.containers {
                        history.push(&c.name, pod.time, c.usage);
                    }
                }
                MetricsEvent::Pod {
                    containers: history.containers(),
                }
            }
            MetricsTarget::Node { name } => {
                let api = metrics_type("NodeMetrics", "nodes", false);
                let metrics: Option<NodeMetrics> = self
                    .read(&format!("nodes/{name}"), &api, None, true, wait)
                    .await?;
                if let Some(node) = metrics.as_ref().map(node_usage) {
                    history.push(name, node.time, node.usage);
                }
                MetricsEvent::Node {
                    samples: history.samples(name),
                }
            }
        })
    }

    /// Reads the metrics of `target` at the pace of [`EVERY`] and sends them to
    /// `sink`, until the sink is gone (returns `false`), the Cluster serves
    /// no Metrics API or forbids it, or the task is cancelled.
    pub async fn stream_metrics(self, target: MetricsTarget, sink: impl Fn(MetricsEvent) -> bool) {
        self.poll_metrics(target, EVERY, sink).await;
    }

    /// [`On::stream_metrics`] at another `pace`.
    pub async fn poll_metrics(
        self,
        target: MetricsTarget,
        pace: Pace,
        sink: impl Fn(MetricsEvent) -> bool,
    ) {
        let mut ticker = tokio::time::interval(pace.every);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        let mut served = false;
        let mut history = History::default();
        loop {
            ticker.tick().await;
            if !served {
                match self.serves_metrics().await {
                    Ok(true) => served = true,
                    Ok(false) => {
                        sink(MetricsEvent::NotInstalled);
                        return;
                    }
                    Err(error) => {
                        if !sink(MetricsEvent::Failed { error }) {
                            return;
                        }
                        continue;
                    }
                }
            }
            let event = match self.sample(&target, &mut history, pace.wait).await {
                Ok(event) => event,
                Err(Failure::Gone) => {
                    sink(MetricsEvent::NotInstalled);
                    return;
                }
                Err(Failure::Forbidden(error)) => {
                    sink(MetricsEvent::Forbidden { error });
                    return;
                }
                Err(Failure::NotResponding(error)) => MetricsEvent::NotResponding { error },
                Err(Failure::Other(error)) => MetricsEvent::Failed { error },
            };
            if !sink(event) {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests;
